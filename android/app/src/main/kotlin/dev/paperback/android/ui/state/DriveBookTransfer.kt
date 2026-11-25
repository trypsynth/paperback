package dev.paperback.android.ui.state

import android.content.ContentResolver
import android.net.ConnectivityManager
import android.net.Network
import android.net.NetworkCapabilities
import android.provider.DocumentsContract
import androidx.core.net.toUri
import uniffi.paperback.BookTransfer
import uniffi.paperback.BookTransferRequest
import uniffi.paperback.BookTransferResult
import java.io.EOFException
import java.io.IOException
import java.io.RandomAccessFile
import java.net.HttpURLConnection
import java.net.URI
import java.security.MessageDigest
import java.util.concurrent.ConcurrentHashMap

internal class DriveBookTransfer(
	private val connectivity: ConnectivityManager,
	private val resolver: ContentResolver,
	private val enabled: () -> Boolean,
	private val mobileData: () -> Boolean
) : BookTransfer {
	private data class Fingerprint(val modified: Long, val size: Long, val hash: String)

	private val fingerprints = ConcurrentHashMap<String, Fingerprint>()

	private fun network(): Network? {
		if (!enabled()) return null
		val network = connectivity.activeNetwork ?: return null
		val capabilities = connectivity.getNetworkCapabilities(network) ?: return null
		return network.takeIf {
			capabilities.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET) &&
				permitsBookTransfer(capabilities.hasTransport(NetworkCapabilities.TRANSPORT_WIFI), mobileData())
		}
	}

	override fun available(): Boolean = network() != null

	override fun localContentHash(locator: String): String? = runCatching {
		val uri = locator.toUri()
		if (!uri.isLocalDocument()) return null
		val attributes = resolver.query(uri, null, null, null, null)?.use { cursor ->
			if (!cursor.moveToFirst()) return@use null
			val modifiedColumn = cursor.getColumnIndex(DocumentsContract.Document.COLUMN_LAST_MODIFIED)
			val sizeColumn = cursor.getColumnIndex(DocumentsContract.Document.COLUMN_SIZE)
			if (modifiedColumn < 0 || sizeColumn < 0) return@use null
			val modified = cursor.getLong(modifiedColumn)
			val size = cursor.getLong(sizeColumn)
			if (modified > 0 && size > 0) modified to size else null
		}
		val previous = fingerprints[locator]
		if (attributes != null &&
			previous != null &&
			attributes.first == previous.modified &&
			attributes.second == previous.size
		) {
			return previous.hash
		}
		val digest = MessageDigest.getInstance("SHA-256")
		resolver.openInputStream(uri)?.use { input ->
			val buffer = ByteArray(128 * 1024)
			var count = input.read(buffer)
			while (count >= 0) {
				digest.update(buffer, 0, count)
				count = input.read(buffer)
			}
		} ?: return null
		val hash = digest.digest().joinToString("") { "%02x".format(it) }
		if (attributes != null) fingerprints[locator] = Fingerprint(attributes.first, attributes.second, hash)
		hash
	}.getOrNull()

	override fun transfer(request: BookTransferRequest): BookTransferResult {
		val network = network() ?: return unavailable()
		val connection = try {
			network.openConnection(URI(request.url).toURL()) as HttpURLConnection
		} catch (_: IOException) {
			return unavailable()
		}
		return try {
			connection.instanceFollowRedirects = false
			connection.connectTimeout = 10_000
			connection.readTimeout = 30_000
			connection.setRequestProperty("Authorization", "Bearer ${request.token}")
			connection.setRequestProperty("Accept-Encoding", "identity")
			if (request.upload) upload(connection, request, network) else download(connection, request, network)
		} catch (_: IOException) {
			unavailable()
		} finally {
			connection.disconnect()
		}
	}

	private fun upload(connection: HttpURLConnection, request: BookTransferRequest, network: Network): BookTransferResult {
		val length = request.length.toLong()
		connection.requestMethod = "PUT"
		connection.doOutput = true
		connection.setFixedLengthStreamingMode(length)
		val range = if (request.probe) {
			"bytes */${request.total}"
		} else {
			val end = request.offset + request.length - 1uL
			"bytes ${request.offset}-$end/${request.total}"
		}
		connection.setRequestProperty("Content-Range", range)
		connection.outputStream.use { output ->
			if (!request.probe) {
				RandomAccessFile(request.path, "r").use { file ->
					file.seek(request.offset.toLong())
					copyExactly(length, network, file::read) { bytes, count -> output.write(bytes, 0, count) }
				}
			}
		}
		val status = connection.responseCode
		val body = if (status in 200..299) connection.inputStream.use { it.readBytes() } else byteArrayOf()
		return BookTransferResult(status.toUShort(), connection.getHeaderField("Range").orEmpty(), body)
	}

	private fun download(
		connection: HttpURLConnection,
		request: BookTransferRequest,
		network: Network
	): BookTransferResult {
		connection.setRequestProperty("Range", "bytes=${request.offset}-${request.offset + request.length - 1uL}")
		val status = connection.responseCode
		if (status == 206 || (status == 200 && request.offset == 0uL)) {
			val count = if (status == 206) request.length else request.total
			connection.inputStream.use { input ->
				RandomAccessFile(request.path, "rw").use { file ->
					file.setLength(request.offset.toLong())
					file.seek(request.offset.toLong())
					copyExactly(count.toLong(), network, input::read) { bytes, size -> file.write(bytes, 0, size) }
				}
			}
		}
		return BookTransferResult(status.toUShort(), "", byteArrayOf())
	}

	private fun copyExactly(
		length: Long,
		connection: Network,
		read: (ByteArray, Int, Int) -> Int,
		write: (ByteArray, Int) -> Unit
	) {
		val buffer = ByteArray(64 * 1024)
		var remaining = length
		while (remaining > 0) {
			// openConnection binds the transfer to this network. The check also stops a transfer if the user changes the setting.
			if (!enabled() || (!mobileData() && network() != connection)) throw IOException("Book transfer paused")
			val count = read(buffer, 0, minOf(remaining, buffer.size.toLong()).toInt())
			if (count < 0) throw EOFException("Incomplete book transfer")
			write(buffer, count)
			remaining -= count
		}
	}

	private fun unavailable() = BookTransferResult(0u, "", byteArrayOf())
}

internal fun permitsBookTransfer(wifi: Boolean, mobileData: Boolean): Boolean = wifi || mobileData
