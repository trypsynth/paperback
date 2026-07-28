package dev.paperback.mobile

import android.Manifest
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.view.KeyEvent
import android.view.View
import android.view.ViewGroup
import android.widget.EditText
import android.view.accessibility.AccessibilityEvent
import android.view.accessibility.AccessibilityNodeInfo
import android.view.accessibility.AccessibilityNodeProvider
import androidx.activity.ComponentActivity
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.activity.result.contract.ActivityResultContracts
import androidx.activity.viewModels
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalView
import androidx.core.content.ContextCompat
import dev.paperback.mobile.theme.MyApplicationTheme
import dev.paperback.mobile.ui.MainScreenViewModel
import uniffi.paperback.SegmentDirectionFfi
import uniffi.paperback.SegmentTypeFfi

class MainActivity : ComponentActivity() {
	private val vm: MainScreenViewModel by viewModels()

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		System.setProperty("uniffi.component.paperback.libraryOverride", "paperback_core")
		Translations.load(this)
		enableEdgeToEdge()
		setContent {
			if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
				val permissionLauncher = rememberLauncherForActivityResult(
					ActivityResultContracts.RequestPermission()
				) { }
				LaunchedEffect(Unit) {
					if (ContextCompat.checkSelfPermission(
							this@MainActivity,
							Manifest.permission.POST_NOTIFICATIONS
						) != PackageManager.PERMISSION_GRANTED
					) {
						permissionLauncher.launch(Manifest.permission.POST_NOTIFICATIONS)
					}
				}
			}

			val view = LocalView.current
			LaunchedEffect(view) {
				val originalDelegate = view.accessibilityDelegate
				view.accessibilityDelegate = object : View.AccessibilityDelegate() {
					override fun getAccessibilityNodeProvider(host: View): AccessibilityNodeProvider? {
						val provider = originalDelegate?.getAccessibilityNodeProvider(host) ?: super.getAccessibilityNodeProvider(host)
						if (provider == null) return null
						return object : AccessibilityNodeProvider() {
							override fun createAccessibilityNodeInfo(virtualViewId: Int): AccessibilityNodeInfo? {
								val info = provider.createAccessibilityNodeInfo(virtualViewId)
								if (info != null && info.className == "android.widget.SeekBar" && info.stateDescription?.toString() == "\u200B") {
									info.extras.putCharSequence("AccessibilityNodeInfo.roleDescription", "button")
								}
								return info
							}
							override fun performAction(virtualViewId: Int, action: Int, arguments: Bundle?): Boolean {
								return provider.performAction(virtualViewId, action, arguments)
							}
							override fun findAccessibilityNodeInfosByText(text: String, virtualViewId: Int): MutableList<AccessibilityNodeInfo>? {
								return provider.findAccessibilityNodeInfosByText(text, virtualViewId)
							}
							override fun findFocus(focus: Int): AccessibilityNodeInfo? {
								return provider.findFocus(focus)
							}
						}
					}
					override fun sendAccessibilityEvent(host: View, eventType: Int) {
						originalDelegate?.sendAccessibilityEvent(host, eventType) ?: super.sendAccessibilityEvent(host, eventType)
					}
					override fun sendAccessibilityEventUnchecked(host: View, event: AccessibilityEvent) {
						originalDelegate?.sendAccessibilityEventUnchecked(host, event) ?: super.sendAccessibilityEventUnchecked(host, event)
					}
					override fun dispatchPopulateAccessibilityEvent(host: View, event: AccessibilityEvent): Boolean {
						return originalDelegate?.dispatchPopulateAccessibilityEvent(host, event) ?: super.dispatchPopulateAccessibilityEvent(host, event)
					}
					override fun onPopulateAccessibilityEvent(host: View, event: AccessibilityEvent) {
						originalDelegate?.onPopulateAccessibilityEvent(host, event) ?: super.onPopulateAccessibilityEvent(host, event)
					}
					override fun onInitializeAccessibilityEvent(host: View, event: AccessibilityEvent) {
						originalDelegate?.onInitializeAccessibilityEvent(host, event) ?: super.onInitializeAccessibilityEvent(host, event)
					}
					override fun onInitializeAccessibilityNodeInfo(host: View, info: AccessibilityNodeInfo) {
						originalDelegate?.onInitializeAccessibilityNodeInfo(host, info) ?: super.onInitializeAccessibilityNodeInfo(host, info)
					}
					override fun onRequestSendAccessibilityEvent(host: ViewGroup, child: View, event: AccessibilityEvent): Boolean {
						return originalDelegate?.onRequestSendAccessibilityEvent(host, child, event) ?: super.onRequestSendAccessibilityEvent(host, child, event)
					}
					override fun performAccessibilityAction(host: View, action: Int, args: Bundle?): Boolean {
						return originalDelegate?.performAccessibilityAction(host, action, args) ?: super.performAccessibilityAction(host, action, args)
					}
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

	override fun dispatchKeyEvent(event: KeyEvent): Boolean {
		if (event.action != KeyEvent.ACTION_DOWN) return super.dispatchKeyEvent(event)
		// Don't intercept when a text field has focus (e.g. Find or Go-To dialogs).
		if (currentFocus is EditText) return super.dispatchKeyEvent(event)
		// F7: elements list (matches desktop)
		if (event.keyCode == KeyEvent.KEYCODE_F7) {
			vm.openElementsDialog()
			return true
		}
		// Ctrl shortcuts: parity with desktop app
		if (event.isCtrlPressed) {
			return when (event.keyCode) {
				KeyEvent.KEYCODE_F -> { vm.openFindDialog(); true }
				KeyEvent.KEYCODE_COMMA -> { vm.openSettingsDialog(); true }
				KeyEvent.KEYCODE_T -> { vm.openTocDialog(); true }
				KeyEvent.KEYCODE_P -> { vm.openGoToDialog("Page"); true }
				KeyEvent.KEYCODE_G -> {
					if (event.isShiftPressed) vm.openGoToDialog("Percentage")
					else vm.openGoToDialog("Line")
					true
				}
				KeyEvent.KEYCODE_W -> { vm.openWordCountDialog(); true }
				KeyEvent.KEYCODE_I -> { vm.openDocumentInfoDialog(); true }
				KeyEvent.KEYCODE_S -> {
					if (event.isShiftPressed) { vm.openSleepTimerDialog(); true }
					else super.dispatchKeyEvent(event)
				}
				else -> super.dispatchKeyEvent(event)
			}
		}

		if (event.keyCode == KeyEvent.KEYCODE_F3) {
			if (event.isShiftPressed) {
				vm.triggerFindPrevious()
			} else {
				vm.triggerFindNext()
			}
			return true
		}

		val dir = if (event.isShiftPressed) SegmentDirectionFfi.PREVIOUS else SegmentDirectionFfi.NEXT
		return when (event.keyCode) {
			KeyEvent.KEYCODE_HEADSETHOOK -> {
				headsethookClickCount++
				headsethookHandler.removeCallbacks(headsethookRunnable)
				headsethookHandler.postDelayed(headsethookRunnable, 300)
				true
			}
			KeyEvent.KEYCODE_SPACE -> { vm.togglePlayPause(); true }
			KeyEvent.KEYCODE_MEDIA_PLAY_PAUSE -> {
				vm.togglePlayPause()
				true
			}
			KeyEvent.KEYCODE_MEDIA_PLAY -> {
				if (!vm.ttsManager.isSpeaking.value) {
					vm.togglePlayPause()
				}
				true
			}
			KeyEvent.KEYCODE_MEDIA_PAUSE -> {
				if (vm.ttsManager.isSpeaking.value) {
					vm.togglePlayPause()
				}
				true
			}
			KeyEvent.KEYCODE_MEDIA_NEXT -> {
				vm.playNextSegment()
				true
			}
			KeyEvent.KEYCODE_MEDIA_PREVIOUS -> {
				vm.playPrevSegment()
				true
			}
			// Sections: [ = previous, ] = next (no shift needed, matching desktop)
			KeyEvent.KEYCODE_LEFT_BRACKET -> { vm.navigateByType(SegmentTypeFfi.SECTION, SegmentDirectionFfi.PREVIOUS); true }
			KeyEvent.KEYCODE_RIGHT_BRACKET -> { vm.navigateByType(SegmentTypeFfi.SECTION, SegmentDirectionFfi.NEXT); true }
			// Headings: H = next, Shift+H = previous
			KeyEvent.KEYCODE_H -> { vm.navigateByType(SegmentTypeFfi.HEADING, dir); true }
			// Pages: P = next, Shift+P = previous
			KeyEvent.KEYCODE_P -> { vm.navigateByType(SegmentTypeFfi.PAGE, dir); true }
			// Images: G = next, Shift+G = previous
			KeyEvent.KEYCODE_G -> { vm.navigateByType(SegmentTypeFfi.IMAGE, dir); true }
			// Figures: F = next, Shift+F = previous
			KeyEvent.KEYCODE_F -> { vm.navigateByType(SegmentTypeFfi.FIGURE, dir); true }
			// Links: K = next, Shift+K = previous
			KeyEvent.KEYCODE_K -> { vm.navigateByType(SegmentTypeFfi.LINK, dir); true }
			// Tables: T = next, Shift+T = previous
			KeyEvent.KEYCODE_T -> { vm.navigateByType(SegmentTypeFfi.TABLE, dir); true }
			// Separators: S = next, Shift+S = previous
			KeyEvent.KEYCODE_S -> { vm.navigateByType(SegmentTypeFfi.SEPARATOR, dir); true }
			// Lists: L = next, Shift+L = previous
			KeyEvent.KEYCODE_L -> { vm.navigateByType(SegmentTypeFfi.LIST, dir); true }
			// List items: I = next, Shift+I = previous
			KeyEvent.KEYCODE_I -> { vm.navigateByType(SegmentTypeFfi.LIST_ITEM, dir); true }
			else -> super.dispatchKeyEvent(event)
		}
	}
}
