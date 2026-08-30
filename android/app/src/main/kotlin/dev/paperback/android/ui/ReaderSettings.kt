package dev.paperback.android.ui

import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import uniffi.paperback.ConfigManagerFfi

/**
 * One observable preference, backed by a key in the shared config file. The stored value is read
 * once when the setting is built and written back (and flushed) on every change, because Android
 * can kill this process with no lifecycle callback at all, so there is no later point at which
 * saving would be guaranteed to happen.
 */
class Setting<T>(
	private val config: ConfigManagerFfi,
	private val key: String,
	stored: T,
	private val write: ConfigManagerFfi.(String, T) -> Unit,
	private val sanitize: (T) -> T
) {
	private val _state = MutableStateFlow(sanitize(stored))
	val state: StateFlow<T> = _state.asStateFlow()

	fun set(value: T) {
		val sanitized = sanitize(value)
		_state.value = sanitized
		config.write(key, sanitized)
		config.flush()
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

	val textScalePercent = intSetting("text_scale_percent", 100) {
		it.coerceIn(MIN_TEXT_SCALE_PERCENT, MAX_TEXT_SCALE_PERCENT)
	}

	val lineSpacing = intSetting("line_spacing", 0)

	val paragraphSpacing = intSetting("paragraph_spacing", 0)

	val textAlignment = intSetting("text_alignment", 0)

	private fun boolSetting(
		key: String,
		default: Boolean
	) = Setting(config, key, config.getAppBool(key, default), ConfigManagerFfi::setAppBool) { it }

	private fun intSetting(
		key: String,
		default: Int,
		sanitize: (Int) -> Int = { it }
	) = Setting(config, key, config.getAppInt(key, default), ConfigManagerFfi::setAppInt, sanitize)

	companion object {
		/** Bounds of the readability text size multiplier, shared with the settings slider. */
		const val MIN_TEXT_SCALE_PERCENT = 70
		const val MAX_TEXT_SCALE_PERCENT = 300
		const val TEXT_SCALE_PERCENT_STEP = 10
	}
}
