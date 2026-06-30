package dev.paperback.mobile.ui.dialogs

import android.os.Environment
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Description
import androidx.compose.material.icons.filled.Folder
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.role
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.platform.LocalContext
import androidx.core.content.ContextCompat
import androidx.compose.ui.window.Dialog
import androidx.compose.ui.input.key.*
import android.os.storage.StorageManager
import android.content.Context
import androidx.compose.ui.window.DialogProperties
import java.io.File
import java.text.SimpleDateFormat
import java.util.Date
import java.util.Locale
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import dev.paperback.mobile.t


@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun FileManagerDialog(
	supportedExtensions: List<String>,
	initialDirectory: File = Environment.getExternalStorageDirectory(),
	onDirectoryChanged: (File) -> Unit = {},
	onFileSelected: (File) -> Unit,
	onDismiss: () -> Unit
) {
	var currentDirectory by remember { mutableStateOf(initialDirectory) }

	var isFirstLaunch by remember { mutableStateOf(true) }
	LaunchedEffect(currentDirectory) {
		if (isFirstLaunch) {
			isFirstLaunch = false
		} else {
			onDirectoryChanged(currentDirectory)
		}
	}

	var files by remember { mutableStateOf<List<File>>(emptyList()) }
	var isLoading by remember { mutableStateOf(true) }

	val context = LocalContext.current
	val storageRoots = remember(context) {
		if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.R) {
			val sm = context.getSystemService(Context.STORAGE_SERVICE) as StorageManager
			sm.storageVolumes.mapNotNull { it.directory }
		} else {
			ContextCompat.getExternalFilesDirs(context, null).mapNotNull {
				val path = it?.absolutePath ?: return@mapNotNull null
				val index = path.indexOf("/Android/data/")
				if (index != -1) {
					File(path.substring(0, index))
				} else null
			}.distinct()
		}
	}

	val virtualParent = remember(currentDirectory, storageRoots) {
		if (currentDirectory.absolutePath == "/storage") null
		else if (storageRoots.any { it.absolutePath == currentDirectory.absolutePath }) File("/storage")
		else currentDirectory.parentFile
	}

	LaunchedEffect(currentDirectory, supportedExtensions, storageRoots) {
		isLoading = true
		files = withContext(Dispatchers.IO) {
			if (currentDirectory.absolutePath == "/storage") {
				storageRoots.sortedBy { it.name.lowercase() }
			} else {
				var list = currentDirectory.listFiles()?.toList() ?: emptyList()
				val folders = list.filter { it.isDirectory && !it.isHidden }.sortedBy { it.name.lowercase() }
				val docs = list.filter { file ->
					!file.isDirectory && !file.isHidden &&
					supportedExtensions.any { ext -> file.name.lowercase().endsWith(".$ext") }
				}.sortedBy { it.name.lowercase() }
				folders + docs
			}
		}
		isLoading = false
	}

	Dialog(
		onDismissRequest = onDismiss,
		properties = DialogProperties(usePlatformDefaultWidth = false)
	) {
		Surface(
			modifier = Modifier
				.fillMaxSize()
				.semantics { paneTitle = "File Manager" }
				.onKeyEvent { event ->
					if (event.type == KeyEventType.KeyDown && event.key == Key.Backspace) {
						if (virtualParent != null) {
							currentDirectory = virtualParent
							return@onKeyEvent true
						}
					}
					false
				},
			color = MaterialTheme.colorScheme.surface
		) {
		Column(modifier = Modifier.fillMaxSize()) {
			TopAppBar(
				title = {
					Text(
						text = if (currentDirectory.absolutePath == "/storage") "Storage Devices"
							   else if (currentDirectory.absolutePath == Environment.getExternalStorageDirectory().absolutePath) "Internal Storage"
							   else currentDirectory.name.ifBlank { "Storage" },
						maxLines = 1,
						overflow = TextOverflow.Ellipsis
					)
				},
				actions = {
					TextButton(onClick = onDismiss) {
						Text("Cancel")
					}
				}
			)


			Text(
				text = currentDirectory.absolutePath,
				style = MaterialTheme.typography.bodySmall,
				color = MaterialTheme.colorScheme.onSurfaceVariant,
				modifier = Modifier.padding(horizontal = 16.dp, vertical = 4.dp).semantics {
					contentDescription = "Current path: ${currentDirectory.absolutePath}"
				}
			)

			Column(modifier = Modifier.fillMaxSize().verticalScroll(rememberScrollState())) {
				if (virtualParent != null) {
					Row(
						modifier = Modifier
							.fillMaxWidth()
							.clickable { currentDirectory = virtualParent }
							.padding(16.dp)
							.clearAndSetSemantics {
								role = Role.Button
								contentDescription = "Go up to parent directory: ${if (virtualParent.absolutePath == "/storage") "Storage Devices" else virtualParent.name}"
							},
						verticalAlignment = Alignment.CenterVertically
					) {
						Icon(
							imageVector = Icons.Default.Folder,
							contentDescription = null,
							tint = MaterialTheme.colorScheme.primary,
							modifier = Modifier.size(32.dp).padding(end = 16.dp)
						)
						Text(
							text = ".. (Parent Directory)",
							style = MaterialTheme.typography.bodyLarge,
							maxLines = 1,
							overflow = TextOverflow.Ellipsis
						)
					}
				}

				if (isLoading) {
					Box(
						modifier = Modifier.fillMaxWidth().padding(32.dp),
						contentAlignment = Alignment.Center
					) {
						CircularProgressIndicator()
					}
				} else {
					files.forEach { file ->
						FileListItem(
							file = file,
							onClick = {
								if (file.isDirectory) {
									currentDirectory = file
								} else {
									onFileSelected(file)
								}
							}
						)
					}
					if (files.isEmpty()) {
						Box(
							modifier = Modifier.fillMaxWidth().padding(32.dp),
							contentAlignment = Alignment.Center
						) {
							Text(t("No supported books or folders found here."))
						}
					}
				}
			}
		}
	}
	}
}
@Composable
fun FileListItem(file: File, onClick: () -> Unit) {
	val dateFormat = remember { SimpleDateFormat("MMM dd, yyyy", Locale.getDefault()) }
	val dateString = remember(file) { dateFormat.format(Date(file.lastModified())) }
	val sizeString = remember(file) {
		if (file.isDirectory) "" else {
			val kb = file.length() / 1024
			if (kb > 1024) "${kb / 1024} MB" else "$kb KB"
		}
	}

	Row(
		modifier = Modifier
			.fillMaxWidth()
			.clickable(onClick = onClick)
			.padding(16.dp)
			.clearAndSetSemantics {
				role = Role.Button
				val typeStr = if (file.isDirectory) t("Folder") else t("File")
				val sizeDesc = if (file.isDirectory) "" else ", $sizeString"
				val displayName = if (file.absolutePath == Environment.getExternalStorageDirectory().absolutePath) t("Internal Storage") else file.name
				contentDescription = "$displayName, $typeStr, modified $dateString$sizeDesc"
			},
		verticalAlignment = Alignment.CenterVertically
	) {
		Icon(
			imageVector = if (file.isDirectory) Icons.Default.Folder else Icons.Default.Description,
			contentDescription = null,
			tint = if (file.isDirectory) MaterialTheme.colorScheme.primary else MaterialTheme.colorScheme.onSurfaceVariant,
			modifier = Modifier.size(32.dp).padding(end = 16.dp)
		)
		Column {
			val displayName = if (file.absolutePath == Environment.getExternalStorageDirectory().absolutePath) t("Internal Storage") else file.name
			Text(
				text = displayName,
				style = MaterialTheme.typography.bodyLarge,
				maxLines = 1,
				overflow = TextOverflow.Ellipsis
			)
			Row {
				Text(
					text = dateString,
					style = MaterialTheme.typography.bodySmall,
					color = MaterialTheme.colorScheme.onSurfaceVariant
				)
				if (!file.isDirectory) {
					Text(
						text = " • $sizeString",
						style = MaterialTheme.typography.bodySmall,
						color = MaterialTheme.colorScheme.onSurfaceVariant
					)
				}
			}
		}
	}
}
