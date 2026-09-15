package dev.paperback.android

import android.content.Intent
import android.os.Build
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.view.KeyEvent
import android.widget.EditText
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.activity.viewModels
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalView
import dev.paperback.android.theme.MyApplicationTheme
import dev.paperback.android.ui.state.MainScreenViewModel

class MainActivity : ComponentActivity() {
	private val vm: MainScreenViewModel by viewModels()

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		System.setProperty("uniffi.component.paperback.libraryOverride", "paperback_core")
		Translations.load(this)
		enableEdgeToEdge()
		setContent {
			val view = LocalView.current
			LaunchedEffect(view) {
				// The delegate below exists only to relabel Compose Slider nodes -- which report
				// themselves to TalkBack as SeekBar -- as buttons, and that check reads
				// AccessibilityNodeInfo#getStateDescription, which is API 30. Below API 30 the
				// wrapper would be a pure pass-through, so don't install it at all rather than
				// call View#getAccessibilityDelegate (API 29) to build it: unguarded, that getter
				// threw NoSuchMethodError at launch on API 24-28.
				if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
					installSeekBarRoleDelegate(view)
				}
			}
			MyApplicationTheme {
				Surface(modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background) {
					MainNavigation()
				}
			}
		}
	}

	override fun onNewIntent(intent: Intent) {
		super.onNewIntent(intent)
		setIntent(intent)
	}

	override fun onDestroy() {
		super.onDestroy()
		headsethookHandler.removeCallbacks(headsethookRunnable)
	}

	private var headsethookClickCount = 0
	private val headsethookHandler = Handler(Looper.getMainLooper())
	private val headsethookRunnable = Runnable {
		when (headsethookClickCount) {
			1 -> vm.togglePlayPause()
			2 -> vm.playNextSegment()
			3 -> vm.playPrevSegment()
		}
		headsethookClickCount = 0
	}

	// ComponentActivity.dispatchKeyEvent is marked @RestrictTo(LIBRARY_GROUP_PREFIX) by AndroidX,
	// which makes lint flag every super call here even though overriding it and delegating is the
	// documented way to intercept key events on an Activity.
	@Suppress("RestrictedApi")
	override fun dispatchKeyEvent(event: KeyEvent): Boolean {
		if (event.action != KeyEvent.ACTION_DOWN) return super.dispatchKeyEvent(event)
		// Don't intercept when a text field has focus (e.g. Find or Go-To dialogs).
		if (currentFocus is EditText) return super.dispatchKeyEvent(event)
		val shortcut = shortcutFor(event.keyCode, event.isCtrlPressed, event.isShiftPressed)
			?: return super.dispatchKeyEvent(event)
		perform(shortcut)
		return true
	}

	private fun perform(shortcut: ReaderShortcut) {
		when (shortcut) {
			ReaderShortcut.TogglePlayback -> vm.togglePlayPause()
			ReaderShortcut.PlayOnly -> if (!vm.ttsManager.isSpeaking.value) vm.togglePlayPause()
			ReaderShortcut.PauseOnly -> if (vm.ttsManager.isSpeaking.value) vm.togglePlayPause()
			ReaderShortcut.HeadsetHook -> countHeadsetClick()
			ReaderShortcut.NextSegment -> vm.playNextSegment()
			ReaderShortcut.PreviousSegment -> vm.playPrevSegment()
			ReaderShortcut.FindNext -> vm.search.findNext()
			ReaderShortcut.FindPrevious -> vm.search.findPrevious()
			ReaderShortcut.OpenFind -> vm.findDialog.open()
			ReaderShortcut.OpenSettings -> vm.settingsRequest.request()
			ReaderShortcut.OpenBook -> vm.openBookRequest.request()
			ReaderShortcut.OpenToc -> vm.tocRequest.request()
			ReaderShortcut.OpenElements -> vm.openElements()
			ReaderShortcut.OpenWordCount -> vm.openWordCountDialog()
			ReaderShortcut.OpenAllDocuments -> vm.allDocumentsRequest.request()
			ReaderShortcut.OpenDocumentInfo -> vm.documentInfoDialog.open()
			ReaderShortcut.OpenSleepTimer -> vm.sleepTimerDialog.open()
			ReaderShortcut.ExportDocument -> vm.openExportDocumentDialog()
			ReaderShortcut.ExportSettings -> vm.exportSettingsRequest.request()
			ReaderShortcut.ImportSettings -> vm.importSettingsRequest.request()
			is ReaderShortcut.OpenGoTo -> vm.openGoToDialog(shortcut.mode)
			is ReaderShortcut.Navigate -> vm.navigateByType(shortcut.type, shortcut.direction)
		}
	}

	/** A headset's one button counts clicks rather than acting on each: the second and third land
	 * within [HEADSET_CLICK_WINDOW_MS] of the first, so acting straight away would play the book
	 * every time the reader meant to skip. */
	private fun countHeadsetClick() {
		headsethookClickCount++
		headsethookHandler.removeCallbacks(headsethookRunnable)
		headsethookHandler.postDelayed(headsethookRunnable, HEADSET_CLICK_WINDOW_MS)
	}

	private companion object {
		const val HEADSET_CLICK_WINDOW_MS = 300L
	}
}
