package dev.paperback.android.ui.dialogs

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.window.DialogProperties
import dev.paperback.android.t
import dev.paperback.android.ui.state.LinkOpener
import dev.paperback.android.ui.state.LinkStage

/** The & marks desktop's keyboard accelerator, which a button here has no use for. */
private fun withoutAccelerator(label: String) = label.replace("&", "")

@Composable
fun OpenFromUrlDialogs(opener: LinkOpener, stage: LinkStage?, failure: String?) {
	if (stage != null) OpenFromUrlDialog(opener, stage)
	if (stage is LinkStage.Confirming) {
		AlertDialog(
			onDismissRequest = { opener.decline() },
			modifier = Modifier.semantics { paneTitle = t("Security Warning") },
			title = { Text(t("Security Warning")) },
			text = { Text(stage.warning) },
			confirmButton = { TextButton(onClick = { opener.confirm() }) { Text(withoutAccelerator(t("&Yes"))) } },
			dismissButton = { TextButton(onClick = { opener.decline() }) { Text(withoutAccelerator(t("&No"))) } }
		)
	}
	if (failure != null) {
		AlertDialog(
			onDismissRequest = { opener.dismissFailure() },
			modifier = Modifier.semantics { paneTitle = t("Open from URL") },
			title = { Text(t("Open from URL")) },
			text = { Text(failure) },
			confirmButton = { TextButton(onClick = { opener.dismissFailure() }) { Text(t("OK")) } }
		)
	}
}

@Composable
private fun OpenFromUrlDialog(opener: LinkOpener, stage: LinkStage) {
	var link by remember { mutableStateOf("") }
	val focusRequester = remember { FocusRequester() }
	val entering = stage == LinkStage.Entering || stage is LinkStage.Confirming
	LaunchedEffect(entering) {
		if (entering) focusRequester.requestFocus()
	}
	AlertDialog(
		onDismissRequest = { opener.close() },
		// A tap outside shouldn't drop a download that is under way.
		properties = DialogProperties(dismissOnClickOutside = entering),
		modifier = Modifier.semantics { paneTitle = t("Open from URL") },
		title = { Text(t("Open from URL")) },
		text = {
			when (stage) {
				LinkStage.Entering, is LinkStage.Confirming -> OutlinedTextField(
					value = link,
					onValueChange = { link = it },
					label = { Text(t("Link:")) },
					keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Uri, imeAction = ImeAction.Go),
					keyboardActions = KeyboardActions(onGo = { if (link.isNotBlank()) opener.open(link) }),
					singleLine = true,
					modifier = Modifier.fillMaxWidth().focusRequester(focusRequester)
				)
				LinkStage.Connecting -> Column {
					Text(t("Connecting..."))
					LinearProgressIndicator(modifier = Modifier.fillMaxWidth())
				}
				is LinkStage.Downloading -> Column {
					Text(t("Downloading %s").replace("%s", stage.name))
					val fraction = stage.fraction
					if (fraction == null) {
						LinearProgressIndicator(modifier = Modifier.fillMaxWidth())
					} else {
						LinearProgressIndicator(progress = { fraction }, modifier = Modifier.fillMaxWidth())
					}
				}
			}
		},
		confirmButton = {
			if (entering) {
				Button(onClick = { opener.open(link) }, enabled = link.isNotBlank()) { Text(t("Open")) }
			}
		},
		dismissButton = {
			TextButton(onClick = { opener.close() }) { Text(t("Cancel")) }
		}
	)
}
