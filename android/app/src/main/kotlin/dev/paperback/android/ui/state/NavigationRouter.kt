package dev.paperback.android.ui.state

import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

/**
 * Every screen and dialog the reading UI can put on top of itself, in one place: a [ScreenRequest] for the ones somewhere else has to carry out (a navigation destination for MainNavigation, a file picker for MainScreen), a [DialogState] for the rest.
 *
 * The ones that have work to do before they open (loading the element lists, choosing the Go To mode) are opened through [MainScreenViewModel], so nothing can open them straight into an empty or stale state.
 */
class NavigationRouter {
	val settingsRequest = ScreenRequest()
	val tocRequest = ScreenRequest()
	val allDocumentsRequest = ScreenRequest()
	val openBookRequest = ScreenRequest()
	val exportSettingsRequest = ScreenRequest()
	val importSettingsRequest = ScreenRequest()
	val elementsRequest = ScreenRequest()

	val findDialog = DialogState()
	val exportDocumentDialog = DialogState()
	val wordCountDialog = DialogState()
	val documentInfoDialog = DialogState()
	val sleepTimerDialog = DialogState()
	val permissionRationaleDialog = DialogState()

	private val goToDialogState = DialogState()
	val showGoToDialog: StateFlow<Boolean> = goToDialogState.isOpen

	private val _goToInitialMode = MutableStateFlow("Line")
	val goToInitialMode: StateFlow<String> = _goToInitialMode.asStateFlow()

	/**
	 * A document offset the reader has asked to jump to from another screen.
	 *
	 * Getting there means switching to Text Mode and scrolling the list, both of which belong to
	 * MainScreen, so a screen that is not MainScreen leaves the offset here and MainScreen acts
	 * on it when it comes back to the front.
	 */
	private val _pendingJumpOffset = MutableStateFlow<Long?>(null)
	val pendingJumpOffset: StateFlow<Long?> = _pendingJumpOffset.asStateFlow()

	internal fun openGoTo(mode: String) {
		_goToInitialMode.value = mode
		goToDialogState.open()
	}

	fun closeGoToDialog() {
		goToDialogState.close()
	}

	fun requestJumpToOffset(offset: Long) {
		_pendingJumpOffset.value = offset
	}

	fun consumeJumpRequest() {
		_pendingJumpOffset.value = null
	}
}
