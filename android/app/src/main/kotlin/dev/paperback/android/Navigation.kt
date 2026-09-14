package dev.paperback.android

import androidx.compose.runtime.Composable
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation3.runtime.entryProvider
import androidx.navigation3.runtime.rememberNavBackStack
import androidx.navigation3.ui.NavDisplay
import dev.paperback.android.ui.AllDocumentsScreen
import dev.paperback.android.ui.ElementsScreen
import dev.paperback.android.ui.MainScreen
import dev.paperback.android.ui.MainScreenViewModel
import dev.paperback.android.ui.OnScreenRequest
import dev.paperback.android.ui.SettingsScreen
import dev.paperback.android.ui.TocScreen

@Composable
fun MainNavigation() {
	val backStack = rememberNavBackStack(Main)
	val viewModel: MainScreenViewModel = viewModel()
	// Bridges non-UI triggers (e.g. the Ctrl+, keyboard shortcut in MainActivity) that
	// can't reach this composable's backStack directly into a real navigation push. Pushing a
	// route that is already on top would stack a second copy of the same screen.
	OnScreenRequest(viewModel.settingsRequest) {
		if (backStack.lastOrNull() != SettingsRoute) backStack.add(SettingsRoute)
	}
	OnScreenRequest(viewModel.tocRequest) {
		if (backStack.lastOrNull() != TocRoute) backStack.add(TocRoute)
	}
	OnScreenRequest(viewModel.allDocumentsRequest) {
		if (backStack.lastOrNull() != AllDocumentsRoute) backStack.add(AllDocumentsRoute)
	}
	OnScreenRequest(viewModel.elementsRequest) {
		if (backStack.lastOrNull() != ElementsRoute) backStack.add(ElementsRoute)
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
				entry<ElementsRoute> {
					ElementsScreen(
						viewModel = viewModel,
						onDismiss = { backStack.removeLastOrNull() }
					)
				}
			},
	)
}
