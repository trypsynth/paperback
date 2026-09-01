package dev.paperback.android.ui

/**
 * Whether leaving Text Mode should take the reading position from the text list's scroll offset.
 *
 * Only a real switch out of Text Mode should: the reader scrolled somewhere and read-aloud picks
 * up from there. The effect that asks this also runs whenever MainScreen is composed afresh,
 * which happens every time another screen is popped off the back stack, and the list is rebuilt
 * at the index the document was opened at rather than wherever the reader has since got to.
 * Syncing then discards the position, so a chapter chosen in the table of contents becomes the
 * start of the book again.
 *
 * @param previousMode the last mode this screen saw, or null if it has not seen one yet.
 */
fun shouldSyncPositionFromList(
	previousMode: Boolean?,
	isTextMode: Boolean
): Boolean = previousMode == true && !isTextMode
