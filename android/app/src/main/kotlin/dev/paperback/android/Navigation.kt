package dev.paperback.android

import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation3.runtime.entryProvider
import androidx.navigation3.runtime.rememberNavBackStack
import androidx.navigation3.ui.NavDisplay
import dev.paperback.android.ui.AllDocumentsScreen
import dev.paperback.android.ui.MainScreen
import dev.paperback.android.ui.MainScreenViewModel
import dev.paperback.android.ui.SettingsScreen
import dev.paperback.android.ui.TocScreen

@Composable
fun MainNavigation() {
	val backStack = rememberNavBackStack(Main)
	val viewModel: MainScreenViewModel = viewModel()

	// Bridges non-UI triggers (e.g. the Ctrl+, keyboard shortcut in MainActivity) that
	// can't reach this composable's backStack directly into a real navigation push.
	val settingsRequested by viewModel.settingsRequest.isRequested.collectAsStateWithLifecycle()
	LaunchedEffect(settingsRequested) {
		if (settingsRequested) {
			if (backStack.lastOrNull() != SettingsRoute) {
				backStack.add(SettingsRoute)
			}
			viewModel.settingsRequest.consume()
		}
	}

	val tocRequested by viewModel.tocRequest.isRequested.collectAsStateWithLifecycle()
	LaunchedEffect(tocRequested) {
		if (tocRequested) {
			if (backStack.lastOrNull() != TocRoute) {
				backStack.add(TocRoute)
			}
			viewModel.tocRequest.consume()
		}
	}

	NavDisplay(
		backStack = backStack,
		onBack = { backStack.removeLastOrNull() },
		entryProvider =
			entryProvider {
				entry<Main> {
					MainScreen(
						viewModel = viewModel,
						onItemClick = { navKey ->
							backStack.add(navKey)
						}
					)
				}
				entry<SettingsRoute> {
					SettingsScreen(
						viewModel = viewModel,
						onDismiss = { backStack.removeLastOrNull() }
					)
				}
				entry<TocRoute> {
					TocScreen(
						viewModel = viewModel,
						onDismiss = { backStack.removeLastOrNull() }
					)
				}
				entry<AllDocumentsRoute> {
					AllDocumentsScreen(
						viewModel = viewModel,
						onDismiss = { backStack.removeLastOrNull() }
					)
				}
			},
	)
}
