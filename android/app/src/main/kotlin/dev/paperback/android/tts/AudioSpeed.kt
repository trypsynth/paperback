package dev.paperback.android.tts

import java.util.Locale
import kotlin.math.roundToInt

/** Up to 4x because audiobook listeners commonly go past 3x; desktop and iOS keep to the same range. */
const val AUDIO_SPEED_MIN = 0.5f
const val AUDIO_SPEED_MAX = 4f
const val AUDIO_SPEED_STEP = 0.1f
val audioSpeedPresets = listOf(1f, 1.25f, 1.5f, 2f, 2.5f, 3f, 3.5f, 4f)

/** Rounded to hundredths, so stepping by 0.1 doesn't drift. */
fun clampAudioSpeed(speed: Float): Float = (speed.coerceIn(AUDIO_SPEED_MIN, AUDIO_SPEED_MAX) * 100).roundToInt() / 100f

/** "2.5x", as desktop says it. */
fun audioSpeedLabel(speed: Float): String = String.format(Locale.ROOT, "%.2f", speed).trimEnd('0').trimEnd('.') + "x"
