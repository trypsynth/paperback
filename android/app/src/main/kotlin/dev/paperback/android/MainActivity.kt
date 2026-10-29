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
				// The delegate's check needs API 30 anyway, and building it calls View#getAccessibilityDelegate (API 29), which threw NoSuchMethodError at launch on API 24-28.
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
			1 -> vm.reading.togglePlayPause()
			2 -> vm.reading.playNextSegment()
			3 -> vm.reading.playPrevSegment()
		}
		headsethookClickCount = 0
	}

	// AndroidX marks ComponentActivity.dispatchKeyEvent @RestrictTo, so lint flags the super calls even though this is the documented way to intercept keys.
	@Suppress("RestrictedApi")
	override fun dispatchKeyEvent(event: KeyEvent): Boolean {
		if (event.action != KeyEvent.ACTION_DOWN) return super.dispatchKeyEvent(event)
		if (currentFocus is EditText) return super.dispatchKeyEvent(event)
		val shortcut = shortcutFor(event.keyCode, event.isCtrlPressed, event.isShiftPressed, event.isAltPressed)
			?: return super.dispatchKeyEvent(event)
		perform(shortcut)
		return true
	}

	private fun perform(shortcut: ReaderShortcut) {
		when (shortcut) {
			ReaderShortcut.TogglePlayback -> vm.reading.togglePlayPause()
			ReaderShortcut.PlayOnly -> if (!vm.ttsManager.isSpeaking.value) vm.reading.togglePlayPause()
			ReaderShortcut.PauseOnly -> if (vm.ttsManager.isSpeaking.value) vm.reading.togglePlayPause()
			ReaderShortcut.HeadsetHook -> countHeadsetClick()
			ReaderShortcut.NextSegment -> vm.reading.playNextSegment()
			ReaderShortcut.PreviousSegment -> vm.reading.playPrevSegment()
			ReaderShortcut.FindNext -> vm.search.findNext()
			ReaderShortcut.FindPrevious -> vm.search.findPrevious()
			ReaderShortcut.OpenFind -> vm.navigation.findDialog.open()
			ReaderShortcut.OpenSettings -> vm.navigation.settingsRequest.request()
			ReaderShortcut.OpenBook -> vm.navigation.openBookRequest.request()
			ReaderShortcut.OpenFromUrl -> vm.links.prompt()
			ReaderShortcut.OpenToc -> vm.navigation.tocRequest.request()
			ReaderShortcut.OpenElements -> vm.openElements()
			ReaderShortcut.OpenWordCount -> vm.openWordCountDialog()
			ReaderShortcut.OpenAllDocuments -> vm.navigation.allDocumentsRequest.request()
			ReaderShortcut.OpenDocumentInfo -> vm.navigation.documentInfoDialog.open()
			ReaderShortcut.OpenSleepTimer -> vm.navigation.sleepTimerDialog.open()
			ReaderShortcut.ExportDocument -> vm.openExportDocumentDialog()
			ReaderShortcut.ExportSettings -> vm.navigation.exportSettingsRequest.request()
			ReaderShortcut.ImportSettings -> vm.navigation.importSettingsRequest.request()
			is ReaderShortcut.OpenGoTo -> vm.openGoToDialog(shortcut.mode)
			is ReaderShortcut.Navigate -> vm.reading.navigateByType(shortcut.type, shortcut.direction)
		}
	}

	// Clicks are counted before acting, or a double click meant to skip would first play the book.
	private fun countHeadsetClick() {
		headsethookClickCount++
		headsethookHandler.removeCallbacks(headsethookRunnable)
		headsethookHandler.postDelayed(headsethookRunnable, HEADSET_CLICK_WINDOW_MS)
	}

	private companion object {
		const val HEADSET_CLICK_WINDOW_MS = 300L
	}
}
