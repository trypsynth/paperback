package dev.paperback.android

import android.os.Build
import android.os.Bundle
import android.view.View
import android.view.ViewGroup
import android.view.accessibility.AccessibilityEvent
import android.view.accessibility.AccessibilityNodeInfo
import android.view.accessibility.AccessibilityNodeProvider
import androidx.annotation.RequiresApi

/**
 * Wraps the root view's accessibility delegate so Compose Sliders, which TalkBack would
 * otherwise announce as seek bars, are described as buttons. Every other method here is a
 * straight pass-through to the delegate Compose installed.
 */
@RequiresApi(Build.VERSION_CODES.R)
internal fun installSeekBarRoleDelegate(view: View) {
	val originalDelegate = view.accessibilityDelegate
	view.accessibilityDelegate = object : View.AccessibilityDelegate() {
		override fun getAccessibilityNodeProvider(host: View): AccessibilityNodeProvider? {
			val provider = originalDelegate?.getAccessibilityNodeProvider(host) ?: super.getAccessibilityNodeProvider(host)
			if (provider == null) return null
			return object : AccessibilityNodeProvider() {
				override fun createAccessibilityNodeInfo(virtualViewId: Int): AccessibilityNodeInfo? {
					val info = provider.createAccessibilityNodeInfo(virtualViewId)
					if (info != null &&
						info.className == "android.widget.SeekBar" &&
						info.stateDescription?.toString() == "​"
					) {
						info.extras.putCharSequence("AccessibilityNodeInfo.roleDescription", "button")
					}
					return info
				}

				override fun performAction(
					virtualViewId: Int,
					action: Int,
					arguments: Bundle?
				): Boolean =
					provider
						.performAction(virtualViewId, action, arguments)

				override fun findAccessibilityNodeInfosByText(
					text: String,
					virtualViewId: Int
				): MutableList<AccessibilityNodeInfo>? =
					provider
						.findAccessibilityNodeInfosByText(text, virtualViewId)

				override fun findFocus(focus: Int): AccessibilityNodeInfo? = provider.findFocus(focus)
			}
		}

		override fun sendAccessibilityEvent(
			host: View,
			eventType: Int
		) {
			originalDelegate?.sendAccessibilityEvent(host, eventType) ?: super.sendAccessibilityEvent(host, eventType)
		}

		override fun sendAccessibilityEventUnchecked(
			host: View,
			event: AccessibilityEvent
		) {
			originalDelegate?.sendAccessibilityEventUnchecked(host, event)
				?: super.sendAccessibilityEventUnchecked(host, event)
		}

		override fun dispatchPopulateAccessibilityEvent(
			host: View,
			event: AccessibilityEvent
		): Boolean =
			originalDelegate?.dispatchPopulateAccessibilityEvent(host, event)
				?: super.dispatchPopulateAccessibilityEvent(host, event)

		override fun onPopulateAccessibilityEvent(
			host: View,
			event: AccessibilityEvent
		) {
			originalDelegate?.onPopulateAccessibilityEvent(host, event) ?: super.onPopulateAccessibilityEvent(host, event)
		}

		override fun onInitializeAccessibilityEvent(
			host: View,
			event: AccessibilityEvent
		) {
			originalDelegate?.onInitializeAccessibilityEvent(host, event) ?: super.onInitializeAccessibilityEvent(host, event)
		}

		override fun onInitializeAccessibilityNodeInfo(
			host: View,
			info: AccessibilityNodeInfo
		) {
			originalDelegate?.onInitializeAccessibilityNodeInfo(host, info)
				?: super.onInitializeAccessibilityNodeInfo(host, info)
		}

		override fun onRequestSendAccessibilityEvent(
			host: ViewGroup,
			child: View,
			event: AccessibilityEvent
		): Boolean =
			originalDelegate?.onRequestSendAccessibilityEvent(host, child, event)
				?: super.onRequestSendAccessibilityEvent(host, child, event)

		override fun performAccessibilityAction(
			host: View,
			action: Int,
			args: Bundle?
		): Boolean =
			originalDelegate?.performAccessibilityAction(host, action, args)
				?: super.performAccessibilityAction(host, action, args)
	}
}
