package dev.paperback.mobile.ui.dialogs

import android.net.Uri
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.*
import androidx.compose.ui.unit.dp
import dev.paperback.mobile.ui.RecentDocumentItem
import androidx.compose.foundation.lazy.items as lazyItems
import dev.paperback.mobile.t

@Composable
fun AllDocumentsDialog(
	recentDocuments: List<RecentDocumentItem>,
	onDismiss: () -> Unit,
	onOpenDocument: (Uri) -> Unit,
	onRemoveDocument: (String) -> Unit,
	onLocateDocument: (String) -> Unit
) {
	AlertDialog(
		onDismissRequest = onDismiss,
		modifier = Modifier.semantics { paneTitle = "Recent Documents" },
		// TRANSLATORS: Title of the dialog listing every previously opened document
		title = { Text(t("Recent Documents")) },
		text = {
			LazyColumn(
				modifier = Modifier.fillMaxWidth()
			) {
				lazyItems(recentDocuments) { recentDoc ->
					RecentDocumentItemRow(
						item = recentDoc,
						onOpen = {
							onDismiss()
							onOpenDocument(Uri.parse(recentDoc.uri))
						},
						onRemove = { onRemoveDocument(recentDoc.uri) },
						onLocate = { onLocateDocument(recentDoc.uri) }
					)
				}
			}
		},
		confirmButton = {
			TextButton(onClick = onDismiss) {
				Text("Close")
			}
		}
	)
}

@Composable
fun RecentDocumentItemRow(
	item: RecentDocumentItem,
	showClosedStatus: Boolean = true,
	onOpen: () -> Unit,
	onRemove: () -> Unit,
	onLocate: (() -> Unit)? = null
) {
	Row(
		modifier = Modifier
			.fillMaxWidth()
			.clickable(
				onClickLabel = "open",
				onClick = { if (!item.isMissing) onOpen() }
			).semantics {
				customActions = mutableListOf<CustomAccessibilityAction>().apply {
					if (item.isMissing && onLocate != null) {
						add(
							// TRANSLATORS: Accessibility action to pick a new file location for a document whose file can no longer be found
							CustomAccessibilityAction(t("Locate")) {
								onLocate()
								true
							}
						)
					}
					add(
						// TRANSLATORS: Accessibility action to remove a document from the recent documents list
						CustomAccessibilityAction(t("Remove")) {
							onRemove()
							true
						}
					)
				}
			}.padding(vertical = 12.dp, horizontal = 8.dp),
		verticalAlignment = Alignment.CenterVertically
	) {
		Column(modifier = Modifier.weight(1f)) {
			Text(
				text = item.displayName,
				style = MaterialTheme.typography.bodyLarge,
				color = if (item.isMissing) {
					MaterialTheme.colorScheme.onSurface.copy(alpha = 0.5f)
				} else {
					MaterialTheme.colorScheme.onSurface
				}
			)
			if (!item.uri.startsWith("content://")) {
				Text(
					text = item.uri.removePrefix("file://"),
					style = MaterialTheme.typography.bodySmall,
					color = MaterialTheme.colorScheme.onSurfaceVariant,
					maxLines = 2,
					overflow = androidx.compose.ui.text.style.TextOverflow.Ellipsis
				)
			}
			if (item.isMissing || item.isOpen || showClosedStatus) {
				Text(
					text = if (item.isMissing) {
						// TRANSLATORS: Status label for a recent document: its file can't be found, it's open in a tab right now, or it's just closed
						t("File Missing")
					} else if (item.isOpen) {
						t("Currently Open")
					} else {
						t("Closed")
					},
					style = MaterialTheme.typography.bodySmall,
					color = if (item.isMissing) MaterialTheme.colorScheme.error else MaterialTheme.colorScheme.onSurfaceVariant
				)
			}
		}
		if (item.isMissing && onLocate != null) {
			TextButton(
				onClick = onLocate,
				modifier = Modifier.clearAndSetSemantics { }
			) {
				Text(t("Locate"))
			}
		}
		TextButton(
			onClick = onRemove,
			modifier = Modifier.clearAndSetSemantics { }
		) {
			Text(t("Remove"))
		}
	}
}
