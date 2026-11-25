package dev.paperback.android.ui.state

import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.net.ConnectivityManager
import android.net.Network
import android.net.NetworkCapabilities
import android.net.Uri
import android.provider.OpenableColumns
import androidx.core.net.toUri
import com.google.android.gms.auth.api.identity.AuthorizationRequest
import com.google.android.gms.auth.api.identity.Identity
import com.google.android.gms.common.api.ApiException
import com.google.android.gms.common.api.CommonStatusCodes
import com.google.android.gms.common.api.Scope
import com.google.android.gms.tasks.Tasks
import dev.paperback.android.configWrites
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.FlowPreview
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.debounce
import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.coroutines.withContext
import uniffi.paperback.CloudBookKey
import uniffi.paperback.Command
import uniffi.paperback.ConfigManagerFfi
import uniffi.paperback.DriveBookItem
import uniffi.paperback.ResumePosition
import uniffi.paperback.SyncReport
import java.io.File
import java.util.concurrent.ExecutionException
import java.util.concurrent.TimeUnit

enum class DriveStatus {
	Disconnected,
	NeedsConnection,
	Syncing,
	Synced,
	WaitingForFiles,
	Unavailable,
	Failed
}

data class DriveUpload(val locator: String, val account: String, val bytes: ULong)

enum class DriveNotice {
	NeedsConnection,
	AccountPending,
	FileUnavailable
}

@OptIn(FlowPreview::class)
class DriveSync(
	context: Context,
	private val config: ConfigManagerFfi,
	private val documentCache: DocumentCache,
	private val scope: CoroutineScope,
	private val openDocuments: () -> List<String>,
	private val onLibraryChanged: () -> Unit
) {
	private val root = File(context.noBackupFilesDir, "google-drive").absolutePath
	private val resolver = context.contentResolver
	private val authorization = Identity.getAuthorizationClient(context)
	private val request = AuthorizationRequest
		.builder()
		.setRequestedScopes(
			listOf(Scope("https://www.googleapis.com/auth/drive.appdata"), Scope("https://www.googleapis.com/auth/drive.file"))
		).build()
	private val gate = Mutex()
	private val _enabled = MutableStateFlow(config.getAppBool(ENABLED, false))
	val enabled = _enabled.asStateFlow()
	private val _connected = MutableStateFlow(config.getAppBool(CONNECTED, false))
	val connected = _connected.asStateFlow()
	private val _mobileData = MutableStateFlow(config.getAppBool(MOBILE_DATA, false))
	val mobileData = _mobileData.asStateFlow()
	private val _status = MutableStateFlow(DriveStatus.Disconnected)
	val status = _status.asStateFlow()
	private val _books = MutableStateFlow(emptyList<DriveBookItem>())
	val books = _books.asStateFlow()
	private val _account = MutableStateFlow("")
	val account = _account.asStateFlow()
	private val _authorizationPrompt = MutableStateFlow<PendingIntent?>(null)
	val authorizationPrompt = _authorizationPrompt.asStateFlow()
	private val _upload = MutableStateFlow<DriveUpload?>(null)
	val upload = _upload.asStateFlow()
	private val _notice = MutableStateFlow<DriveNotice?>(null)
	val notice = _notice.asStateFlow()
	private val _libraryOpen = MutableStateFlow(false)
	val libraryOpen = _libraryOpen.asStateFlow()
	private var report: SyncReport? = null
	private val connectivity = requireNotNull(context.getSystemService(ConnectivityManager::class.java))
	private val transport = DriveBookTransfer(connectivity, resolver, { _enabled.value }, { _mobileData.value })
	private var fileJob: Job? = null
	private var lastConnection: Pair<Network, Boolean>? = null
	private val networkCallback = object : ConnectivityManager.NetworkCallback() {
		override fun onCapabilitiesChanged(network: Network, networkCapabilities: NetworkCapabilities) {
			if (_enabled.value && networkCapabilities.hasCapability(NetworkCapabilities.NET_CAPABILITY_VALIDATED)) {
				val connection = network to networkCapabilities.hasTransport(NetworkCapabilities.TRANSPORT_WIFI)
				scope.launch {
					if (lastConnection != connection) {
						lastConnection = connection
						sync(Command.Refresh, interactive = false)
					}
				}
			}
		}

		override fun onLost(network: Network) {
			scope.launch {
				if (lastConnection?.first == network) lastConnection = null
			}
		}
	}

	init {
		connectivity.registerDefaultNetworkCallback(networkCallback)
		if (_enabled.value) {
			scope.launch {
				val cached = withContext(Dispatchers.IO) { runCatching { config.cachedGoogleDrive(root, transport) }.getOrNull() }
				if (report == null && cached != null) accept(cached)
			}
		}
		scope.launch {
			configWrites
				.debounce { immediate ->
					if (immediate) 0L else 3_000L
				}.collect { sync(Command.Refresh, interactive = false) }
		}
	}

	fun close() {
		connectivity.unregisterNetworkCallback(networkCallback)
	}

	fun setEnabled(value: Boolean) {
		_enabled.value = value
		config.setAppBool(ENABLED, value)
		when {
			!value -> {
				_status.value = DriveStatus.Disconnected
				_authorizationPrompt.value = null
			}
			_connected.value -> refresh()
			else -> _status.value = DriveStatus.NeedsConnection
		}
		scope.launch(Dispatchers.IO) { config.flush() }
	}

	fun setMobileData(value: Boolean) {
		_mobileData.value = value
		config.setAppBool(MOBILE_DATA, value)
		scope.launch {
			withContext(Dispatchers.IO) { config.flush() }
			sync(Command.Refresh, interactive = false)
		}
	}

	fun refresh() {
		if (_enabled.value) scope.launch { sync(Command.Refresh, interactive = false) }
	}

	fun connect() {
		if (_enabled.value) scope.launch { sync(Command.Refresh, interactive = true) }
	}

	fun takeAuthorizationPrompt() {
		_authorizationPrompt.value = null
	}

	fun finishAuthorization(data: Intent?) {
		scope.launch {
			gate.withLock {
				if (!_enabled.value) return@withLock
				try {
					val token = data?.let { authorization.getAuthorizationResultFromIntent(it).accessToken }
					if (token == null) {
						_status.value = DriveStatus.NeedsConnection
					} else {
						report = null
						_books.value = emptyList()
						_account.value = ""
						setConnected(true)
						perform(token, Command.Refresh)
					}
				} catch (error: ApiException) {
					_status.value = authorizationFailure(error)
				}
			}
		}
	}

	private fun setConnected(value: Boolean) {
		if (_connected.value == value) return
		_connected.value = value
		if (!value) _account.value = ""
		config.setAppBool(CONNECTED, value)
		scope.launch(Dispatchers.IO) { config.flush() }
	}

	private fun requireConnection(): Boolean {
		if (_connected.value) return true
		_notice.value = DriveNotice.NeedsConnection
		return false
	}

	fun dismissNotice() {
		_notice.value = null
	}

	fun showLibrary(value: Boolean) {
		if (value && !requireConnection()) return
		_libraryOpen.value = value
		if (value) refresh()
	}

	fun selectUpload(locator: String) {
		if (!requireConnection()) return
		val accountKey = report?.driveAccountKey()?.takeIf(String::isNotEmpty)
		if (accountKey == null) {
			_notice.value = DriveNotice.AccountPending
			refresh()
			return
		}
		scope.launch {
			val bytes = withContext(Dispatchers.IO) {
				config.googleDriveBookSize(locator) ?: localDocumentSize(locator.toUri())
			}
			if (bytes == null) {
				_notice.value = DriveNotice.FileUnavailable
			} else {
				_upload.value = DriveUpload(locator, accountKey, bytes)
			}
		}
	}

	fun finishUpload(accepted: Boolean) {
		val selected = _upload.value
		_upload.value = null
		if (accepted && selected != null && _enabled.value && requireConnection()) {
			scope.launch {
				val queued = withContext(Dispatchers.IO) {
					runCatching {
						if (config.googleDriveBookSize(selected.locator) == null) {
							val uri = selected.locator.toUri()
							if (!uri.isLocalDocument()) return@runCatching false
							val file = documentCache.materialize(resolver, uri, reuse = true) ?: return@runCatching false
							config.associateUriWithLocalFile(selected.locator, file.absolutePath)
						}
						config.requestGoogleDriveBookFile(selected.locator, selected.account, selected.bytes)
					}.getOrDefault(false)
				}
				if (queued) sync(Command.Refresh, interactive = false) else _notice.value = DriveNotice.FileUnavailable
			}
		}
	}

	private fun localDocumentSize(uri: Uri): ULong? = runCatching {
		if (!uri.isLocalDocument()) return null
		resolver.query(uri, arrayOf(OpenableColumns.DISPLAY_NAME, OpenableColumns.SIZE), null, null, null)?.use { cursor ->
			if (!cursor.moveToFirst() || cursor.getString(0).orEmpty().endsWith(".opf", ignoreCase = true)) return@use null
			cursor.getLong(1).takeIf { it > 0 }?.toULong()
		}
	}.getOrNull()

	fun canRemoveOfflineCopy(path: String): Boolean = openDocuments().none { it.toUri().path == path || it == path }

	fun removeFile(key: CloudBookKey, cloud: Boolean) {
		if (cloud && !requireConnection()) return
		scope.launch {
			val book = _books.value.firstOrNull { it.key == key } ?: return@launch
			if (!cloud && book.offlinePath?.let(::canRemoveOfflineCopy) != true) return@launch
			val command = if (cloud) Command.RemoveCloudFile(key) else Command.RemoveOfflineFile(key)
			sync(command, interactive = true)
		}
	}

	suspend fun prepareOpen(key: CloudBookKey, choice: ResumePosition?): String? {
		val command = if (choice == null) Command.Prioritize(key) else Command.ChoosePosition(key, choice)
		val visible = _books.value.firstOrNull { it.key == key } ?: return null
		if (choice != null || (visible.localLocator == null && visible.offlinePath == null)) {
			if (!sync(command, interactive = true)) return null
		}
		while (transport.available() && _status.value == DriveStatus.WaitingForFiles) {
			val pending = _books.value.firstOrNull { it.key == key } ?: break
			if (pending.cloudBytes == null || pending.localLocator != null || pending.offlinePath != null) break
			if (!sync(Command.ContinueFiles, interactive = false)) break
		}
		val latest = report ?: return null
		val book = latest.driveBooks().firstOrNull { it.key == key } ?: return null
		val locator = book.localLocator ?: book.offlinePath ?: return null
		return withContext(Dispatchers.IO) {
			val applied = config.applyGoogleDriveBook(latest, key.id, locator)
			if (choice != null && !applied) return@withContext null
			if (locator.toUri().scheme != null) return@withContext locator
			val uri = Uri.fromFile(File(locator)).toString()
			config.associateUriWithLocalFile(uri, locator)
			uri
		}
	}

	private suspend fun sync(command: Command, interactive: Boolean): Boolean {
		if (!_enabled.value || !interactive && !_connected.value) return false
		if (connectivity.activeNetwork == null) {
			_status.value = DriveStatus.Disconnected
			return false
		}
		return gate.withLock {
			if (!_enabled.value || !interactive && !_connected.value) return@withLock false
			try {
				val result = withContext(Dispatchers.IO) {
					Tasks.await(authorization.authorize(request), 25, TimeUnit.SECONDS)
				}
				val token = result.accessToken
				if (result.hasResolution() || token == null) {
					setConnected(false)
					if (interactive) _authorizationPrompt.value = result.pendingIntent
					_status.value = DriveStatus.NeedsConnection
					return@withLock false
				}
				setConnected(true)
				perform(token, command)
			} catch (cancelled: CancellationException) {
				throw cancelled
			} catch (error: Exception) {
				if (_enabled.value) _status.value = authorizationFailure(error)
				false
			}
		}
	}

	private fun authorizationFailure(error: Exception): DriveStatus {
		val cause = (error as? ExecutionException)?.cause ?: error
		return if (cause is ApiException && cause.statusCode == CommonStatusCodes.DEVELOPER_ERROR) {
			setConnected(false)
			DriveStatus.Unavailable
		} else {
			DriveStatus.Failed
		}
	}

	private suspend fun perform(token: String, command: Command): Boolean {
		if (!_enabled.value) return false
		if (command != Command.ContinueFiles) _status.value = DriveStatus.Syncing
		return try {
			val result = withContext(Dispatchers.IO) { config.syncGoogleDrive(root, token, command, transport) }
			if (!_enabled.value) return false
			val protected = openDocuments()
			withContext(Dispatchers.IO) {
				val published = config.applyGoogleDriveReport(result, protected)
				if (published.isNotEmpty()) {
					val opened = config.getOpenedDocuments()
					for (locator in published) {
						if (locator !in opened && locator !in protected) documentCache.forget(locator)
					}
				}
			}
			accept(result)
			_status.value = when {
				result.driveFileError() -> DriveStatus.Failed
				result.driveFilesWaiting() -> DriveStatus.WaitingForFiles
				else -> DriveStatus.Synced
			}
			onLibraryChanged()
			if (result.driveFilesWaiting() && !result.driveFileError() && transport.available()) {
				scheduleNextFilePart()
			}
			true
		} catch (cancelled: CancellationException) {
			throw cancelled
		} catch (_: Exception) {
			if (_enabled.value) _status.value = DriveStatus.Failed
			false
		}
	}

	private fun scheduleNextFilePart() {
		if (fileJob?.isActive == true) return
		fileJob = scope.launch {
			delay(200)
			fileJob = null
			if (transport.available()) sync(Command.ContinueFiles, interactive = false)
		}
	}

	private fun accept(result: SyncReport) {
		report = result
		_books.value = result.driveBooks()
		result.driveAccount().takeIf(String::isNotEmpty)?.let { _account.value = it }
	}

	private companion object {
		const val ENABLED = "google_drive_book_sync"
		const val CONNECTED = "google_drive_account_connected"
		const val MOBILE_DATA = "google_drive_books_mobile_data"
	}
}
