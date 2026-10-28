package dev.paperback.android.ui.state

import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import uniffi.paperback.ConfigManagerFfi

/** Persisted on every write because Android can kill the process without a callback; [sanitize] also applies to the stored value so a bad config file can't reach the UI. */
class Setting<T>(stored: T, private val sanitize: (T) -> T = { it }, private val persist: (T) -> Unit) {
	private val _state = MutableStateFlow(sanitize(stored))
	val state: StateFlow<T> = _state.asStateFlow()

	fun set(value: T) {
		val sanitized = sanitize(value)
		_state.value = sanitized
		persist(sanitized)
	}
}

/** Spacing and alignment share desktop's keys and values so a document reads the same everywhere; text size doesn't, since it scales the system font size rather than storing points. */
class ReaderSettings(private val config: ConfigManagerFfi) {
	val restorePreviousDocuments = boolSetting("restore_previous_documents", true)

	val useInAppFileBrowser = boolSetting("use_in_app_file_browser", false)

	val swipeUpMovesForward = boolSetting("swipe_up_moves_forward", true)

	// Off by default: the play button's swipe actions that replace these buttons aren't discoverable on their own.
	val hidePrevNextButtons = boolSetting("hide_prev_next_buttons", false)

	val stopAfterFindMatch = boolSetting("tts_stop_after_find_match", true)

	val textScalePercent = intSetting("text_scale_percent", 100) {
		it.coerceIn(MIN_TEXT_SCALE_PERCENT, MAX_TEXT_SCALE_PERCENT)
	}

	val lineSpacing = intSetting("line_spacing", 0)

	val paragraphSpacing = intSetting("paragraph_spacing", 0)

	val textAlignment = intSetting("text_alignment", 0)

	val highContrastText = boolSetting("high_contrast_text", false)

	val textMode = boolSetting("text_mode", false)

	private fun boolSetting(key: String, default: Boolean) = Setting(config.getAppBool(key, default)) {
		config.setAppBool(key, it)
		config.flush()
	}

	private fun intSetting(key: String, default: Int, sanitize: (Int) -> Int = { it }) =
		Setting(config.getAppInt(key, default), sanitize) {
			config.setAppInt(key, it)
			config.flush()
		}

	companion object {
		const val MIN_TEXT_SCALE_PERCENT = 70
		const val MAX_TEXT_SCALE_PERCENT = 300
		const val TEXT_SCALE_PERCENT_STEP = 10
	}
}
