package dev.paperback.android.ui.components

import androidx.compose.ui.semantics.SemanticsActions
import androidx.compose.ui.semantics.SemanticsProperties
import androidx.compose.ui.test.SemanticsMatcher
import androidx.compose.ui.test.assert
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onNodeWithContentDescription
import androidx.compose.ui.test.performSemanticsAction
import dev.paperback.android.ui.state.NavUnit
import org.junit.Assert.assertEquals
import org.junit.Rule
import org.junit.Test
import uniffi.paperback.SegmentTypeFfi

/**
 * The reading bar is the one screen a TalkBack user spends a whole book in, so what it does and
 * does not expose to the semantics tree is the behaviour worth pinning down. None of this can be
 * checked from a plain unit test: the tree only exists once the bar is composed.
 */
class TtsBottomBarTest {
	@get:Rule
	val compose = createComposeRule()

	private val paragraph = NavUnit.Segment(SegmentTypeFfi.PARAGRAPH)

	private fun bar(
		hidePrevNextButtons: Boolean = false,
		isSpeaking: Boolean = false,
		speechRatePercent: Int = 50,
		onPrev: () -> Unit = {},
		onNext: () -> Unit = {},
		onPlayPause: () -> Unit = {},
		onSpeechRateChange: (Int) -> Unit = {},
		swipeUpMovesForward: Boolean = true
	) {
		compose.setContent {
			TtsBottomBar(
				isSpeaking = isSpeaking,
				onPlayPause = onPlayPause,
				onPrev = onPrev,
				onNext = onNext,
				onPrevButton = {},
				onNextButton = {},
				currentUnit = paragraph,
				navUnits = listOf(paragraph, NavUnit.Segment(SegmentTypeFfi.LINE)),
				onNavUnitChange = {},
				speechRatePercent = speechRatePercent,
				onSpeechRateChange = onSpeechRateChange,
				swipeUpMovesForward = swipeUpMovesForward,
				hidePrevNextButtons = hidePrevNextButtons
			)
		}
	}

	@Test
	fun theBarOffersPreviousAndNextUntilAskedNotTo() {
		bar()
		compose.onNodeWithContentDescription("Previous Paragraph").assertExists()
		compose.onNodeWithContentDescription("Next Paragraph").assertExists()
	}

	@Test
	fun hidingThePreviousAndNextButtonsTakesThemOutOfTheSwipeOrder() {
		bar(hidePrevNextButtons = true)
		compose.onNodeWithContentDescription("Previous Paragraph").assertDoesNotExist()
		compose.onNodeWithContentDescription("Next Paragraph").assertDoesNotExist()
	}

	/**
	 * The one thing hiding the buttons must never do. Losing play would leave a TalkBack user
	 * with no way to start the book at all, which is far worse than the extra swipe stops the
	 * option exists to remove.
	 */
	@Test
	fun hidingThePreviousAndNextButtonsLeavesPlayReachable() {
		bar(hidePrevNextButtons = true)
		compose.onNodeWithContentDescription("Play").assertExists().assertIsDisplayed()
	}

	@Test
	fun playAndPauseReadAsWhicheverTheButtonWillDo() {
		bar(isSpeaking = false)
		compose.onNodeWithContentDescription("Play").assertExists()
		compose.onNodeWithContentDescription("Pause").assertDoesNotExist()
	}

	@Test
	fun theBarReadsAsPauseWhileItIsSpeaking() {
		bar(isSpeaking = true)
		compose.onNodeWithContentDescription("Pause").assertExists()
		compose.onNodeWithContentDescription("Play").assertDoesNotExist()
	}

	/**
	 * Play carries the seek gesture, which is what makes hiding the previous and next buttons
	 * safe in the first place. Without this the option would strand people.
	 */
	@Test
	fun swipingOnPlayMovesByTheCurrentUnit() {
		var forward = 0
		var back = 0
		bar(onNext = { forward++ }, onPrev = { back++ }, hidePrevNextButtons = true)

		val play = compose.onNodeWithContentDescription("Play")
		play.performSemanticsAction(SemanticsActions.SetProgress) { it(10000f) }
		assertEquals("swiping up moved forward", 1, forward)
		play.performSemanticsAction(SemanticsActions.SetProgress) { it(0f) }
		assertEquals("swiping down moved back", 1, back)
	}

	/** With the direction reversed, the same gestures have to mean the opposite. */
	@Test
	fun theSeekDirectionFollowsTheSwipeUpSetting() {
		var forward = 0
		var back = 0
		bar(onNext = { forward++ }, onPrev = { back++ }, swipeUpMovesForward = false)

		val play = compose.onNodeWithContentDescription("Play")
		play.performSemanticsAction(SemanticsActions.SetProgress) { it(10000f) }
		assertEquals("swiping up moved back", 1, back)
		play.performSemanticsAction(SemanticsActions.SetProgress) { it(0f) }
		assertEquals("swiping down moved forward", 1, forward)
	}

	@Test
	fun theSpeechRateReadsItsCurrentValue() {
		bar(speechRatePercent = 65)
		compose
			.onNodeWithContentDescription("Speech Rate")
			.assert(SemanticsMatcher.expectValue(SemanticsProperties.StateDescription, "65%"))
	}

	/** Swiping the rate steps it rather than jumping to wherever the gesture landed. */
	@Test
	fun swipingTheSpeechRateStepsItUpAndDown() {
		val asked = mutableListOf<Int>()
		bar(speechRatePercent = 50, onSpeechRateChange = { asked.add(it) })

		val rate = compose.onNodeWithContentDescription("Speech Rate")
		rate.performSemanticsAction(SemanticsActions.SetProgress) { it(10000f) }
		rate.performSemanticsAction(SemanticsActions.SetProgress) { it(0f) }
		assertEquals(listOf(55, 45), asked)
	}

	/**
	 * Both chips report themselves as ranges so TalkBack offers the swipe at all, rather than
	 * announcing them as plain buttons.
	 */
	@Test
	fun theAdjustableControlsReportThemselvesAsRanges() {
		bar()
		for (label in listOf("Navigation unit", "Speech Rate", "Play")) {
			compose.onNodeWithContentDescription(label).assert(
				SemanticsMatcher.keyIsDefined(SemanticsProperties.ProgressBarRangeInfo)
			)
		}
	}

	@Test
	fun theNavigationUnitReadsTheUnitItIsOn() {
		bar()
		compose
			.onNodeWithContentDescription("Navigation unit")
			.assert(SemanticsMatcher.expectValue(SemanticsProperties.StateDescription, "Paragraph"))
	}
}
