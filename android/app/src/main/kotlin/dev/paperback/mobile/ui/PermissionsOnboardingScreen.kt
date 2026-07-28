package dev.paperback.mobile.ui

import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.Column
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
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import dev.paperback.mobile.t

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
						"Lets Paperback show playback controls in the notification shade while text-to-speech is " +
							"reading, so you can pause, resume, and skip without reopening the app."
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
						"Powers the optional in-app file browser, so you can open documents from anywhere on your " +
							"device — including network drives — instantly, with full screen-reader support. " +
							"You can skip this and use the system file picker instead."
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
