package dev.paperback.android.ui.components

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.Immutable
import androidx.compose.runtime.remember
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp

/** [background] stays [Color.Unspecified] unless high contrast is on, so the theme's surface shows through. */
@Immutable
data class ReadabilityStyle(
	val textStyle: TextStyle,
	val paragraphSpacing: Dp,
	val background: Color = Color.Unspecified
)

@Immutable
data class HighContrastColors(val text: Color, val background: Color)

fun highContrastColors(darkTheme: Boolean): HighContrastColors = if (darkTheme) {
	HighContrastColors(text = Color.White, background = Color.Black)
} else {
	HighContrastColors(text = Color.Black, background = Color.White)
}

/** Size multiplies the sp body style rather than setting points, so a device-wide font scale is kept and scaled further. */
@Composable
fun rememberReadabilityStyle(
	scalePercent: Int,
	lineSpacingChoice: Int,
	paragraphSpacingChoice: Int,
	alignmentChoice: Int,
	highContrast: Boolean
): ReadabilityStyle {
	val base = MaterialTheme.typography.bodyLarge
	val density = LocalDensity.current
	val darkTheme = isSystemInDarkTheme()
	return remember(
		base,
		density,
		scalePercent,
		lineSpacingChoice,
		paragraphSpacingChoice,
		alignmentChoice,
		highContrast,
		darkTheme
	) {
		val fontSize = base.fontSize * (scalePercent / 100f)
		// Added to the body line height rather than replacing it, since a bare multiple of the font size would make Normal tighter than the rest of the app.
		val extraLineSpacing = when (lineSpacingChoice) {
			1 -> 0.5f
			2 -> 1f
			else -> 0f
		}
		val paragraphSpacing = with(density) {
			when (paragraphSpacingChoice) {
				1 -> (fontSize * 0.35f).toDp()
				2 -> (fontSize * 0.7f).toDp()
				else -> 4.dp
			}
		}
		val alignment = when (alignmentChoice) {
			1 -> TextAlign.Center
			2 -> TextAlign.End
			3 -> TextAlign.Justify
			else -> TextAlign.Start
		}
		val contrast = if (highContrast) highContrastColors(darkTheme) else null
		ReadabilityStyle(
			textStyle = base.copy(
				fontSize = fontSize,
				lineHeight = fontSize * (base.lineHeight.value / base.fontSize.value + extraLineSpacing),
				textAlign = alignment,
				color = contrast?.text ?: base.color
			),
			paragraphSpacing = paragraphSpacing,
			background = contrast?.background ?: Color.Unspecified
		)
	}
}
