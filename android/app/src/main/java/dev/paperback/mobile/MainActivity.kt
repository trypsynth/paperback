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
