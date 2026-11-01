package dev.paperback.android.ui.state

import kotlinx.coroutines.FlowPreview
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.debounce
import kotlinx.coroutines.flow.distinctUntilChanged
import kotlinx.coroutines.flow.drop

private const val SCROLL_SETTLE_MS = 500L

/** Drops the first value: after a pop the rebuilt list reports its opening index, and saving that would replace a TOC jump with the start of the book. */
@OptIn(FlowPreview::class)
fun Flow<Int>.scrollsWorthSaving(): Flow<Int> = distinctUntilChanged().drop(1).debounce(SCROLL_SETTLE_MS)
