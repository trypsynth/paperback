package dev.paperback.android.ui

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.CustomAccessibilityAction
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.semantics.customActions
import androidx.compose.ui.semantics.heading
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.unit.dp
import dev.paperback.android.t
import androidx.compose.foundation.lazy.items as lazyItems

/** How many of the recent documents the no-document screen previews before "Show All". */
private const val RECENT_DOCUMENTS_PREVIEW_COUNT = 5

/**
 * What fills the reading area when no document is open: a short preview of the recent documents
 * with a way through to the full list, or a bare message when nothing has been opened yet.
 */
@Composable
fun NoDocumentPane(
	recentDocuments: List<RecentDocumentItem>,
	onOpenDocument: (String) -> Unit,
	onRemoveDocument: (String) -> Unit,
	onLocateDocument: (String) -> Unit,
	onShowAllDocuments: () -> Unit
) {
	Column(
		modifier = Modifier.fillMaxSize().padding(16.dp),
		horizontalAlignment = Alignment.CenterHorizontally,
		verticalArrangement = Arrangement.Center
	) {
		if (recentDocuments.isEmpty()) {
			Text(
				// TRANSLATORS: Shown on the main screen when no document is open and there are no recent documents to list
				t("No Documents"),
				style = MaterialTheme.typography.titleLarge,
				modifier = Modifier.padding(bottom = 24.dp)
			)
			return@Column
		}
		Text(
			// TRANSLATORS: Heading above the list of recently opened documents, shown when no document is currently open
			t("Recent Documents"),
			style = MaterialTheme.typography.titleMedium,
			modifier = Modifier.padding(bottom = 8.dp).semantics { heading() }
		)
		LazyColumn(
			modifier = Modifier.weight(1f).fillMaxWidth(),
			contentPadding = PaddingValues(vertical = 8.dp)
		) {
			lazyItems(recentDocuments.take(RECENT_DOCUMENTS_PREVIEW_COUNT)) { recentDoc ->
				RecentDocumentItemRow(
					item = recentDoc,
					showClosedStatus = false,
					onOpen = { onOpenDocument(recentDoc.uri) },
					onRemove = { onRemoveDocument(recentDoc.uri) },
					onLocate = { onLocateDocument(recentDoc.uri) }
				)
			}
		}
		TextButton(
			onClick = onShowAllDocuments,
			modifier = Modifier.padding(top = 8.dp)
		) {
			// TRANSLATORS: Button below the short recent-documents preview that opens the full Recent Documents screen
			Text(t("Show All"))
		}
	}
}

/**
 * The read-aloud view: the segment being spoken (or the name of the narration file being played),
 * how far through the document that segment sits, and the sleep timer countdown while one is
 * running. Find is a navigation unit in this mode, with its own buttons in the bottom bar, so the
 * text needs no find actions of its own.
 */
@Composable
fun ReadAloudPane(
	segmentText: String,
	textStyle: TextStyle,
	progressPercent: Int?,
	sleepTimerRemaining: Int?,
	onCancelSleepTimer: () -> Unit
) {
	Column(
		modifier = Modifier.fillMaxSize().padding(24.dp),
		horizontalAlignment = Alignment.CenterHorizontally,
		verticalArrangement = Arrangement.Center
	) {
		// A segment is usually one short sentence but can be a whole paragraph, which would run off
		// both ends of a centred column with no way to reach either. Giving the card what room it
		// asks for up to the height of the pane, and scrolling beyond that, keeps both ends reachable.
		Surface(
			modifier = Modifier.fillMaxWidth().weight(1f, fill = false),
			shape = MaterialTheme.shapes.large,
			color = MaterialTheme.colorScheme.surfaceContainerHigh,
			contentColor = MaterialTheme.colorScheme.onSurface
		) {
			Text(
				text = segmentText,
				style = textStyle,
				modifier = Modifier.fillMaxWidth().verticalScroll(rememberScrollState()).padding(24.dp)
			)
		}
		if (progressPercent != null) {
			Spacer(modifier = Modifier.height(24.dp))
			// The reading below says the same number, so the bar is decoration for the eye only.
			LinearProgressIndicator(
				progress = { progressPercent / 100f },
				modifier = Modifier.fillMaxWidth().clearAndSetSemantics { }
			)
			Spacer(modifier = Modifier.height(8.dp))
			Text(
				// TRANSLATORS: How far through the document the reader has got, shown under the read-aloud progress bar; {} is a whole-number percentage
				t("{}% through", progressPercent.toString()),
				style = MaterialTheme.typography.labelMedium,
				color = MaterialTheme.colorScheme.onSurfaceVariant
			)
		}
		if (sleepTimerRemaining == null) return@Column
		Spacer(modifier = Modifier.height(16.dp))
		val minutes = sleepTimerRemaining / 60
		val seconds = sleepTimerRemaining % 60
		Text(
			// TRANSLATORS: Countdown shown while the reading sleep timer is active; {} is the remaining time as minutes:seconds
			t("Sleep timer: {}", "%d:%02d".format(minutes, seconds)),
			style = MaterialTheme.typography.labelMedium,
			color = MaterialTheme.colorScheme.onSurfaceVariant,
			modifier = Modifier.semantics {
				customActions = listOf(
					// TRANSLATORS: Accessibility action to cancel the active reading sleep timer
					CustomAccessibilityAction(t("Cancel sleep timer")) {
						onCancelSleepTimer()
						true
					}
				)
			}
		)
	}
}
