package dev.paperback.android.ui

import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import androidx.core.net.toUri
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import dev.paperback.android.t
import androidx.compose.foundation.lazy.itemsIndexed as lazyItemsIndexed

/**
 * The full recent-documents list. The no-document screen shows a five-item preview of the same
 * list; "Show All" pushes this screen, which is a real destination rather than a dialog so the
 * list can use the whole window and the system back gesture leaves it.
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun AllDocumentsScreen(
	viewModel: MainScreenViewModel = viewModel(),
	onDismiss: () -> Unit
) {
	val state by viewModel.uiState.collectAsStateWithLifecycle()
	val recentDocuments = (state as? MainScreenUiState.Success)?.recentDocuments.orEmpty()
	val supportedMimeTypes by viewModel.supportedMimeTypes.collectAsStateWithLifecycle()

	var locateTargetUri by remember { mutableStateOf<String?>(null) }
	val locateFilePickerLauncher = rememberLauncherForActivityResult(
		contract = ActivityResultContracts.OpenDocument(),
		onResult = { uri ->
			val target = locateTargetUri
			if (uri != null && target != null) {
				viewModel.locateRecentDocument(target, uri)
			}
			locateTargetUri = null
		}
	)

	Surface(
		modifier = Modifier.fillMaxSize().semantics { paneTitle = "Recent Documents" },
		color = MaterialTheme.colorScheme.surface
	) {
		Column(modifier = Modifier.fillMaxSize()) {
			TopAppBar(
				title = {
					// TRANSLATORS: Title of the screen listing every previously opened document
					Text(t("Recent Documents"))
				},
				navigationIcon = {
					IconButton(onClick = onDismiss) {
						// TRANSLATORS: Accessibility label for the back button that leaves the settings screen
						Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = t("Back"))
					}
				}
			)
			if (recentDocuments.isEmpty()) {
				Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
					// TRANSLATORS: Announced when opening "All Documents" while the recent-documents list is empty
					Text(t("No recent documents."))
				}
			} else {
				LazyColumn(
					modifier = Modifier.fillMaxSize(),
					contentPadding = PaddingValues(bottom = 32.dp)
				) {
					lazyItemsIndexed(recentDocuments) { index, recentDoc ->
						RecentDocumentItemRow(
							item = recentDoc,
							onOpen = {
								onDismiss()
								viewModel.openDocument(recentDoc.uri.toUri())
							},
							onRemove = { viewModel.removeRecentDocument(recentDoc.uri) },
							onLocate = {
								locateTargetUri = recentDoc.uri
								locateFilePickerLauncher.launch(supportedMimeTypes)
							}
						)
						if (index < recentDocuments.lastIndex) {
							HorizontalDivider()
						}
					}
				}
			}
		}
	}
}
