package dev.paperback.mobile

import android.os.Bundle
import android.view.KeyEvent
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.ui.Modifier
import androidx.lifecycle.ViewModelProvider
import dev.paperback.mobile.theme.MyApplicationTheme
import dev.paperback.mobile.ui.MainScreenViewModel
import uniffi.paperback.SegmentDirectionFfi
import uniffi.paperback.SegmentTypeFfi

class MainActivity : ComponentActivity() {
	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		System.setProperty("uniffi.component.paperback.libraryOverride", "paperback_core")
		enableEdgeToEdge()
		setContent {
			val view = androidx.compose.ui.platform.LocalView.current
			androidx.compose.runtime.LaunchedEffect(view) {
				val originalDelegate = view.accessibilityDelegate
				view.accessibilityDelegate = object : android.view.View.AccessibilityDelegate() {
					override fun getAccessibilityNodeProvider(host: android.view.View): android.view.accessibility.AccessibilityNodeProvider? {
						val provider = originalDelegate?.getAccessibilityNodeProvider(host) ?: super.getAccessibilityNodeProvider(host)
						if (provider == null) return null
						return object : android.view.accessibility.AccessibilityNodeProvider() {
							override fun createAccessibilityNodeInfo(virtualViewId: Int): android.view.accessibility.AccessibilityNodeInfo? {
								val info = provider.createAccessibilityNodeInfo(virtualViewId)
								if (info != null && info.className == "android.widget.SeekBar" && info.stateDescription?.toString() == "\u200B") {
									info.extras.putCharSequence("AccessibilityNodeInfo.roleDescription", "button")
								}
								return info
							}
							override fun performAction(virtualViewId: Int, action: Int, arguments: android.os.Bundle?): Boolean {
								return provider.performAction(virtualViewId, action, arguments)
							}
							override fun findAccessibilityNodeInfosByText(text: String, virtualViewId: Int): MutableList<android.view.accessibility.AccessibilityNodeInfo>? {
								return provider.findAccessibilityNodeInfosByText(text, virtualViewId)
							}
							override fun findFocus(focus: Int): android.view.accessibility.AccessibilityNodeInfo? {
								return provider.findFocus(focus)
							}
						}
					}
					override fun sendAccessibilityEvent(host: android.view.View, eventType: Int) {
						originalDelegate?.sendAccessibilityEvent(host, eventType) ?: super.sendAccessibilityEvent(host, eventType)
					}
					override fun sendAccessibilityEventUnchecked(host: android.view.View, event: android.view.accessibility.AccessibilityEvent) {
						originalDelegate?.sendAccessibilityEventUnchecked(host, event) ?: super.sendAccessibilityEventUnchecked(host, event)
					}
					override fun dispatchPopulateAccessibilityEvent(host: android.view.View, event: android.view.accessibility.AccessibilityEvent): Boolean {
						return originalDelegate?.dispatchPopulateAccessibilityEvent(host, event) ?: super.dispatchPopulateAccessibilityEvent(host, event)
					}
					override fun onPopulateAccessibilityEvent(host: android.view.View, event: android.view.accessibility.AccessibilityEvent) {
						originalDelegate?.onPopulateAccessibilityEvent(host, event) ?: super.onPopulateAccessibilityEvent(host, event)
					}
					override fun onInitializeAccessibilityEvent(host: android.view.View, event: android.view.accessibility.AccessibilityEvent) {
						originalDelegate?.onInitializeAccessibilityEvent(host, event) ?: super.onInitializeAccessibilityEvent(host, event)
					}
					override fun onInitializeAccessibilityNodeInfo(host: android.view.View, info: android.view.accessibility.AccessibilityNodeInfo) {
						originalDelegate?.onInitializeAccessibilityNodeInfo(host, info) ?: super.onInitializeAccessibilityNodeInfo(host, info)
					}
					override fun onRequestSendAccessibilityEvent(host: android.view.ViewGroup, child: android.view.View, event: android.view.accessibility.AccessibilityEvent): Boolean {
						return originalDelegate?.onRequestSendAccessibilityEvent(host, child, event) ?: super.onRequestSendAccessibilityEvent(host, child, event)
					}
					override fun performAccessibilityAction(host: android.view.View, action: Int, args: android.os.Bundle?): Boolean {
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

	override fun onNewIntent(intent: android.content.Intent) {
		super.onNewIntent(intent)
		setIntent(intent)
	}

	override fun dispatchKeyEvent(event: KeyEvent): Boolean {
		if (event.action != KeyEvent.ACTION_DOWN) return super.dispatchKeyEvent(event)
		// Don't intercept when a text field has focus (e.g. Find or Go-To dialogs).
		if (currentFocus is android.widget.EditText) return super.dispatchKeyEvent(event)
		val vm = ViewModelProvider(this)[MainScreenViewModel::class.java]
		val dir = if (event.isShiftPressed) SegmentDirectionFfi.PREVIOUS else SegmentDirectionFfi.NEXT
		return when (event.keyCode) {
			KeyEvent.KEYCODE_HEADSETHOOK,
			KeyEvent.KEYCODE_SPACE -> { vm.togglePlayPause(); true }
			// Sections: [ = previous, ] = next (no shift needed, matching desktop)
			KeyEvent.KEYCODE_LEFT_BRACKET -> { vm.navigateByType(SegmentTypeFfi.SECTION, SegmentDirectionFfi.PREVIOUS); true }
			KeyEvent.KEYCODE_RIGHT_BRACKET -> { vm.navigateByType(SegmentTypeFfi.SECTION, SegmentDirectionFfi.NEXT); true }
			// Headings: H = next, Shift+H = previous
			KeyEvent.KEYCODE_H -> { vm.navigateByType(SegmentTypeFfi.HEADING, dir); true }
			// Pages: P = next, Shift+P = previous
			KeyEvent.KEYCODE_P -> { vm.navigateByType(SegmentTypeFfi.PAGE, dir); true }
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
