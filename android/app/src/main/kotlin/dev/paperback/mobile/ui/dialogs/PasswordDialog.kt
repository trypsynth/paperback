package dev.paperback.mobile.ui.dialogs

import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
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
import androidx.compose.ui.text.input.PasswordVisualTransformation

@Composable
fun PasswordDialog(
	onConfirm: (String) -> Unit,
	onDismiss: () -> Unit
) {
	var password by remember { mutableStateOf("") }
	val focusRequester = remember { FocusRequester() }

	// Automatically focus the password field when the dialog opens
	LaunchedEffect(Unit) {
		focusRequester.requestFocus()
	}

	AlertDialog(
		onDismissRequest = onDismiss,
		modifier = Modifier.semantics { paneTitle = "Document Password" },
		title = { Text(t("Document Password")) },
		text = {
			OutlinedTextField(
				value = password,
				onValueChange = { password = it },
				label = { Text(t("Password:")) },
				visualTransformation = PasswordVisualTransformation(),
				keyboardOptions = KeyboardOptions(
					keyboardType = KeyboardType.Password,
					imeAction = ImeAction.Done
				),
				keyboardActions = KeyboardActions(
					onDone = {
						if (password.isNotEmpty()) {
							onConfirm(password)
						}
					}
				),
				singleLine = true,
				modifier = Modifier
					.fillMaxWidth()
					.focusRequester(focusRequester)
			)
		},
		confirmButton = {
			Button(
				onClick = { onConfirm(password) },
				enabled = password.isNotEmpty()
			) {
				Text(t("OK"))
			}
		},
		dismissButton = {
			TextButton(onClick = onDismiss) {
				Text(t("Cancel"))
			}
		}
	)
}
