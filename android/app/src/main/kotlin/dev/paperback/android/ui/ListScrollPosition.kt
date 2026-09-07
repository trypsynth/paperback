package dev.paperback.android.ui

import kotlinx.coroutines.FlowPreview
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.debounce
import kotlinx.coroutines.flow.distinctUntilChanged
import kotlinx.coroutines.flow.drop

/** How long the list has to stand still before where it stopped becomes the reading position. */
private const val SCROLL_SETTLE_MS = 500L

/**
 * The scroll indices of a text list that are worth saving as the reading position.
 *
 * A list reports where it already is as soon as anything listens, and that first value is not a
 * place the reader went. MainScreen is composed afresh every time another screen is popped off
 * the back stack, and the list it builds then starts at the index the document was opened at
 * rather than wherever reading has since got to, so saving that value replaces a section chosen
 * in the table of contents with the start of the book. Everything after it is a real scroll,
 * reported once the list has settled.
 */
@OptIn(FlowPreview::class)
fun Flow<Int>.scrollsWorthSaving(): Flow<Int> = distinctUntilChanged().drop(1).debounce(SCROLL_SETTLE_MS)
