package dev.paperback.android.ui.state

/** Only a real switch out of Text Mode: MainScreen recomposes after every pop with the list at its opening index, and syncing then would undo a chapter chosen in the TOC. */
fun shouldSyncPositionFromList(previousMode: Boolean?, isTextMode: Boolean): Boolean =
	previousMode == true && !isTextMode
