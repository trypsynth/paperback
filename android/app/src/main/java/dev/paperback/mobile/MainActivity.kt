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
		if (event.keyCode == KeyEvent.KEYCODE_HEADSETHOOK && event.action == KeyEvent.ACTION_DOWN) {
			ViewModelProvider(this)[MainScreenViewModel::class.java].togglePlayPause()
			return true
		}
		return super.dispatchKeyEvent(event)
	}
}
