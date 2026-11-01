package dev.paperback.android.ui.state

import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

/** Ones with work to do before opening (element lists, Go To mode) go through [MainScreenViewModel], so nothing opens them into an empty or stale state. */
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

	/** Left here for MainScreen to act on when it's back in front, since only it can switch to Text Mode and scroll the list. */
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
