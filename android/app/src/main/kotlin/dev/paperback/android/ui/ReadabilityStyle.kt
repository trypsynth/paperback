package dev.paperback.android.ui

import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.Immutable
import androidx.compose.runtime.remember
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp

/**
 * The reader's readability preferences, resolved into the values a text view needs: a [TextStyle]
 * carrying size, line height and alignment, and the padding to put above and below each paragraph.
 */
@Immutable
data class ReadabilityStyle(
	val textStyle: TextStyle,
	val paragraphSpacing: Dp
)

/**
 * Builds the [ReadabilityStyle] for the reader's current preferences.
 *
 * The size is a multiplier on the body text style rather than a fixed point size. Compose measures
 * that style in sp, which already carries the system font scale, so a reader who has enlarged text
 * device-wide keeps that and scales it further here.
 */
@Composable
fun rememberReadabilityStyle(
	scalePercent: Int,
	lineSpacingChoice: Int,
	paragraphSpacingChoice: Int,
	alignmentChoice: Int
): ReadabilityStyle {
	val base = MaterialTheme.typography.bodyLarge
	val density = LocalDensity.current
	return remember(base, density, scalePercent, lineSpacingChoice, paragraphSpacingChoice, alignmentChoice) {
		val fontSize = base.fontSize * (scalePercent / 100f)
		// The body style's own line height is what "Normal" means, so the wider choices add to it
		// instead of replacing it. Dropping to a bare multiple of the font size would make Normal
		// tighter than the rest of the app.
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
		ReadabilityStyle(
			textStyle = base.copy(
				fontSize = fontSize,
				lineHeight = fontSize * (base.lineHeight.value / base.fontSize.value + extraLineSpacing),
				textAlign = alignment
			),
			paragraphSpacing = paragraphSpacing
		)
	}
}
