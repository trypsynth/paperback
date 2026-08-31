package dev.paperback.android.ui

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.InsertDriveFile
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.ErrorOutline
import androidx.compose.material.icons.filled.Search
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.semantics.*
import androidx.compose.ui.text.style.TextOverflow
import dev.paperback.android.t

/**
 * One entry in a recent-documents list, shared by the short preview on the no-document screen and
 * the full Recent Documents screen.
 */
@Composable
fun RecentDocumentItemRow(
	item: RecentDocumentItem,
	showClosedStatus: Boolean = true,
	onOpen: () -> Unit,
	onRemove: () -> Unit,
	onLocate: (() -> Unit)? = null
) {
	val statusText = when {
		item.isMissing ->
			// TRANSLATORS: Status label for a recent document: its file can't be found, it's open in a tab right now, or it's just closed
			t("File Missing")
		item.isOpen ->
			// TRANSLATORS: Status label for a recent document that is currently open in a tab
			t("Currently Open")
		showClosedStatus ->
			// TRANSLATORS: Status label for a recent document that is closed (not open in any tab)
			t("Closed")
		else -> null
	}

	ListItem(
		modifier = Modifier
			.clickable(
				onClickLabel = "open",
				role = Role.Button,
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
			},
		leadingContent = {
			Icon(
				imageVector = if (item.isMissing) Icons.Filled.ErrorOutline else Icons.AutoMirrored.Filled.InsertDriveFile,
				contentDescription = null,
				tint = if (item.isMissing) MaterialTheme.colorScheme.error else MaterialTheme.colorScheme.onSurfaceVariant
			)
		},
		headlineContent = {
			Text(
				text = item.displayName,
				color = if (item.isMissing) MaterialTheme.colorScheme.onSurface.copy(alpha = 0.5f) else Color.Unspecified
			)
		},
		supportingContent = if (!item.uri.startsWith("content://") || statusText != null) {
			{
				Column {
					if (!item.uri.startsWith("content://")) {
						Text(
							text = item.uri.removePrefix("file://"),
							maxLines = 2,
							overflow = TextOverflow.Ellipsis
						)
					}
					if (statusText != null) {
						Text(
							text = statusText,
							color = if (item.isMissing) MaterialTheme.colorScheme.error else Color.Unspecified
						)
					}
				}
			}
		} else {
			null
		},
		trailingContent = {
			Row {
				if (item.isMissing && onLocate != null) {
					IconButton(
						onClick = onLocate,
						modifier = Modifier.clearAndSetSemantics { }
					) {
						Icon(Icons.Filled.Search, contentDescription = null)
					}
				}
				IconButton(
					onClick = onRemove,
					modifier = Modifier.clearAndSetSemantics { }
				) {
					Icon(Icons.Filled.Delete, contentDescription = null)
				}
			}
		}
	)
}
