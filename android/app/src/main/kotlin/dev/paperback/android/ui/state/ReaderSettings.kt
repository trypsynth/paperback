package dev.paperback.android.ui.state

import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import uniffi.paperback.ConfigManagerFfi

/**
 * One observable value that is persisted as soon as it changes. [persist] runs on every write
 * rather than at some later checkpoint because Android can kill this process with no lifecycle
 * callback at all, so there is no later point at which saving would be guaranteed to happen.
 *
 * [sanitize] applies to the stored value as well as to every write, so a config file holding
 * something outside the accepted range comes back in range rather than reaching the UI.
 */
class Setting<T>(
	stored: T,
	private val sanitize: (T) -> T = { it },
	private val persist: (T) -> Unit
) {
	private val _state = MutableStateFlow(sanitize(stored))
	val state: StateFlow<T> = _state.asStateFlow()

	fun set(value: T) {
		val sanitized = sanitize(value)
		_state.value = sanitized
		persist(sanitized)
	}
}

/**
 * The preferences the reading UI watches, each one a [Setting] over the same config file the
 * desktop app uses. Spacing and alignment share the desktop's keys and value meanings (spacing
 * 0/1/2; alignment 0 leading, 1 center, 2 trailing, 3 justify) so a document reads the same way
 * on every platform. Text size does not: the desktop stores an absolute point size, while this
 * scales whatever size the system font setting is already asking for.
 *
 * One-shot flags that nothing observes (the onboarding marker, the last file-manager directory)
 * stay on `ConfigManagerFfi` itself rather than becoming settings here.
 */
class ReaderSettings(
	private val config: ConfigManagerFfi
) {
	val restorePreviousDocuments = boolSetting("restore_previous_documents", true)

	val useInAppFileBrowser = boolSetting("use_in_app_file_browser", false)

	val swipeUpMovesForward = boolSetting("swipe_up_moves_forward", true)

	// Drops the previous and next buttons from the reading bar's TalkBack order. They duplicate
	// the swipe up and down actions already on the play button, so hiding them makes the bar
	// three stops instead of five. Off by default: the swipe is not discoverable on its own, so
	// nobody should lose the buttons without having chosen to.
	val hidePrevNextButtons = boolSetting("hide_prev_next_buttons", false)

	val textScalePercent = intSetting("text_scale_percent", 100) {
		it.coerceIn(MIN_TEXT_SCALE_PERCENT, MAX_TEXT_SCALE_PERCENT)
	}

	val lineSpacing = intSetting("line_spacing", 0)

	val paragraphSpacing = intSetting("paragraph_spacing", 0)

	val textAlignment = intSetting("text_alignment", 0)

	val highContrastText = boolSetting("high_contrast_text", false)

	private fun boolSetting(
		key: String,
		default: Boolean
	) = Setting(config.getAppBool(key, default)) {
		config.setAppBool(key, it)
		config.flush()
	}

	private fun intSetting(
		key: String,
		default: Int,
		sanitize: (Int) -> Int = { it }
	) = Setting(config.getAppInt(key, default), sanitize) {
		config.setAppInt(key, it)
		config.flush()
	}

	companion object {
		/** Bounds of the readability text size multiplier, shared with the settings slider. */
		const val MIN_TEXT_SCALE_PERCENT = 70
		const val MAX_TEXT_SCALE_PERCENT = 300
		const val TEXT_SCALE_PERCENT_STEP = 10
	}
}
