package dev.paperback.android.ui.screens

import android.Manifest
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.os.Environment
import android.provider.Settings
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.core.content.ContextCompat
import androidx.core.net.toUri
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import androidx.lifecycle.compose.LocalLifecycleOwner
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import dev.paperback.android.t
import dev.paperback.android.ui.state.MainScreenViewModel

private const val AUTO_ENABLED_IN_APP_FILE_BROWSER_KEY = "auto_enabled_in_app_file_browser"
private const val ONBOARDING_SHOWN_KEY = "permissions_onboarding_shown"

/** True once Android enforces scoped storage (R+) and the app still lacks "All files access". */
internal fun needsAllFilesAccessPermission(): Boolean =
	Build.VERSION.SDK_INT >= Build.VERSION_CODES.R && !Environment.isExternalStorageManager()

/** True only on R+ devices where "All files access" has already been granted. */
internal fun hasAllFilesAccessOnR(): Boolean =
	Build.VERSION.SDK_INT >= Build.VERSION_CODES.R && Environment.isExternalStorageManager()

/** True once Android requires a runtime prompt (Tiramisu+) and notifications aren't yet allowed. */
internal fun needsNotificationPermission(context: Context): Boolean =
	Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU &&
		ContextCompat.checkSelfPermission(context, Manifest.permission.POST_NOTIFICATIONS) !=
		PackageManager.PERMISSION_GRANTED

/**
 * Shows the permissions onboarding over the reading view until the reader has dealt with it, and
 * keeps an eye on the permissions afterwards.
 *
 * The OS grants these from its own settings screens, which this app cannot see the result of, so
 * every resume re-reads them rather than trusting what was true when this last recomposed. A
 * resume that finds All Files Access newly granted also switches on the in-app file browser,
 * once: doing it on every resume would fight a reader who turned it back off.
 */
@Composable
fun PermissionsGate(viewModel: MainScreenViewModel) {
	val context = LocalContext.current
	val settings = viewModel.settings
	val useInAppFileBrowser by settings.useInAppFileBrowser.state.collectAsStateWithLifecycle()
	var hasAutoEnabledInAppFileBrowser by remember {
		mutableStateOf(viewModel.configManager.getAppBool(AUTO_ENABLED_IN_APP_FILE_BROWSER_KEY, false))
	}
	var onboardingCompleted by remember {
		mutableStateOf(viewModel.configManager.getAppBool(ONBOARDING_SHOWN_KEY, false))
	}
	var resumeCount by remember { mutableStateOf(0) }
	var notificationRequested by remember { mutableStateOf(false) }
	val notificationPermissionLauncher = rememberLauncherForActivityResult(
		ActivityResultContracts.RequestPermission()
	) { notificationRequested = true }
	val notificationsApplicable = Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU
	val allFilesAccessApplicable = Build.VERSION.SDK_INT >= Build.VERSION_CODES.R
	val notificationsGranted = remember(resumeCount, notificationRequested) { !needsNotificationPermission(context) }
	val allFilesAccessGranted = remember(resumeCount) { !needsAllFilesAccessPermission() }
	val lifecycleOwner = LocalLifecycleOwner.current
	DisposableEffect(lifecycleOwner) {
		val observer = LifecycleEventObserver { _, event ->
			if (event == Lifecycle.Event.ON_RESUME) {
				resumeCount++
				if (hasAllFilesAccessOnR() && !useInAppFileBrowser && !hasAutoEnabledInAppFileBrowser) {
					settings.useInAppFileBrowser.set(true)
					hasAutoEnabledInAppFileBrowser = true
					viewModel.configManager.setAppBool(AUTO_ENABLED_IN_APP_FILE_BROWSER_KEY, true)
					viewModel.configManager.flush()
				}
			}
		}
		lifecycleOwner.lifecycle.addObserver(observer)
		onDispose { lifecycleOwner.lifecycle.removeObserver(observer) }
	}
	val showOnboarding = !onboardingCompleted &&
		(
			(notificationsApplicable && !notificationsGranted) ||
				(allFilesAccessApplicable && !allFilesAccessGranted)
		)
	if (!showOnboarding) return
	PermissionsOnboardingScreen(
		showNotificationsSection = notificationsApplicable,
		notificationsGranted = notificationsGranted,
		onEnableNotifications = { notificationPermissionLauncher.launch(Manifest.permission.POST_NOTIFICATIONS) },
		showAllFilesAccessSection = allFilesAccessApplicable,
		allFilesAccessGranted = allFilesAccessGranted,
		onEnableAllFilesAccess = {
			val intent = Intent(Settings.ACTION_MANAGE_APP_ALL_FILES_ACCESS_PERMISSION)
			intent.data = "package:${context.packageName}".toUri()
			context.startActivity(intent)
		},
		onContinue = {
			onboardingCompleted = true
			viewModel.configManager.setAppBool(ONBOARDING_SHOWN_KEY, true)
			viewModel.configManager.flush()
		}
	)
}

@Composable
private fun PermissionSection(
	title: String,
	description: String,
	granted: Boolean,
	enableLabel: String,
	onEnableClick: () -> Unit
) {
	Text(text = title, style = MaterialTheme.typography.titleMedium, fontWeight = FontWeight.Bold)
	Spacer(modifier = Modifier.height(4.dp))
	Text(text = description, style = MaterialTheme.typography.bodyMedium)
	Spacer(modifier = Modifier.height(8.dp))
	if (granted) {
		// TRANSLATORS: Shown next to a permission on the onboarding screen once it has been granted
		Text(text = t("✓ Granted"), style = MaterialTheme.typography.bodyMedium, fontWeight = FontWeight.Bold)
	} else {
		Button(onClick = onEnableClick) { Text(enableLabel) }
	}
}

/**
 * First-run screen explaining both permissions up front. Both sections are shown
 * together at all times (each reflecting its own granted state independently);
 * the user leaves via the Continue button whenever they're ready, whether or not
 * either permission ended up granted.
 */
@Composable
fun PermissionsOnboardingScreen(
	showNotificationsSection: Boolean,
	notificationsGranted: Boolean,
	onEnableNotifications: () -> Unit,
	showAllFilesAccessSection: Boolean,
	allFilesAccessGranted: Boolean,
	onEnableAllFilesAccess: () -> Unit,
	onContinue: () -> Unit,
	modifier: Modifier = Modifier
) {
	Surface(modifier = modifier.fillMaxSize()) {
		Column(
			modifier = Modifier
				.fillMaxSize()
				.verticalScroll(rememberScrollState())
				.padding(24.dp)
		) {
			// TRANSLATORS: Heading shown on the first-run permissions onboarding screen
			Text(text = t("Before You Start"), style = MaterialTheme.typography.headlineSmall, fontWeight = FontWeight.Bold)
			Spacer(modifier = Modifier.height(8.dp))
			Text(
				// TRANSLATORS: Intro paragraph explaining that the following sections describe requested permissions
				text = t("Paperback works best with a couple of permissions. Here's what we ask for and why:"),
				style = MaterialTheme.typography.bodyLarge
			)
			Spacer(modifier = Modifier.height(24.dp))

			if (showNotificationsSection) {
				PermissionSection(
					// TRANSLATORS: Heading for the notifications permission section on the onboarding screen
					title = t("Notifications"),
					// TRANSLATORS: Explanation of why the app requests the notifications permission
					description = t(
						"Lets Paperback show playback controls in the notification shade while text-to-speech is reading, so you can pause, resume, and skip without reopening the app."
					),
					granted = notificationsGranted,
					// TRANSLATORS: Button to grant the notifications permission during onboarding
					enableLabel = t("Enable Notifications"),
					onEnableClick = onEnableNotifications
				)
				Spacer(modifier = Modifier.height(20.dp))
			}

			if (showAllFilesAccessSection) {
				PermissionSection(
					// TRANSLATORS: Heading for the all files access permission section on the onboarding screen
					title = t("All Files Access"),
					// TRANSLATORS: Explanation of why the app requests the all files access permission
					description = t(
						"Powers the optional in-app file browser, so you can open documents from anywhere on your device — including network drives — instantly, with full screen-reader support. You can skip this and use the system file picker instead."
					),
					granted = allFilesAccessGranted,
					// TRANSLATORS: Button to open system settings for the all files access permission during onboarding
					enableLabel = t("Enable File Access"),
					onEnableClick = onEnableAllFilesAccess
				)
				Spacer(modifier = Modifier.height(20.dp))
			}

			Spacer(modifier = Modifier.height(12.dp))
			Button(onClick = onContinue, modifier = Modifier.fillMaxWidth()) {
				// TRANSLATORS: Button to leave the onboarding screen and continue into the app, whether or not permissions were granted
				Text(t("Continue"))
			}
		}
	}
}
