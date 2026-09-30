package dev.paperback.android.ui.components

import androidx.compose.ui.graphics.Color
import org.junit.Assert.assertEquals
import org.junit.Test

class HighContrastColorsTest {
	@Test
	fun `the light theme gets black text on white`() {
		assertEquals(HighContrastColors(text = Color.Black, background = Color.White), highContrastColors(darkTheme = false))
	}

	@Test
	fun `the dark theme gets white text on black`() {
		assertEquals(HighContrastColors(text = Color.White, background = Color.Black), highContrastColors(darkTheme = true))
	}
}
