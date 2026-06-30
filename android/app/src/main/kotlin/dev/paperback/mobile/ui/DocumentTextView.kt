package dev.paperback.mobile.ui

import android.content.Intent
import android.net.Uri
import androidx.compose.foundation.focusable
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyListState
import androidx.compose.foundation.relocation.BringIntoViewRequester
import androidx.compose.foundation.relocation.bringIntoViewRequester
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.semantics.CustomAccessibilityAction
import androidx.compose.ui.semantics.customActions
import androidx.compose.ui.semantics.heading
import androidx.compose.ui.semantics.isTraversalGroup
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.stateDescription
import androidx.compose.ui.text.LinkAnnotation
import androidx.compose.ui.text.SpanStyle
import androidx.compose.ui.text.TextLinkStyles
import androidx.compose.ui.text.buildAnnotatedString
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextDecoration
import androidx.compose.ui.text.withLink
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
import uniffi.paperback.LinkAction
import uniffi.paperback.MarkerTypeFfi
import uniffi.paperback.SearchOptionsFfi

@Composable
fun DocumentTextView(
	docState: DocumentTabState,
	listState: LazyListState,
	lineIndexToFocus: Int?,
	onLineIndexChange: (Int?) -> Unit,
	activeSearchQuery: String?,
	activeSearchOptions: SearchOptionsFfi?,
	onCloseSearch: () -> Unit
) {
	val context = LocalContext.current
	val scope = rememberCoroutineScope()
	LazyColumn(
		state = listState,
		modifier = Modifier.fillMaxSize().semantics { isTraversalGroup = true },
		contentPadding = PaddingValues(start = 16.dp, end = 16.dp, top = 4.dp, bottom = 16.dp)
	) {
		items(
			count = docState.lineCount.toInt(),
			key = { it }
		) { index ->
			val lineNum = (index + 1).toLong()
			val pos = docState.session.positionFromLine(lineNum)
			val lineText = docState.session.getLineText(pos).trimEnd()
			val markers = docState.session.getLineMarkers(lineNum)
			if (lineText.isNotBlank()) {
				val bringIntoViewRequester = remember { BringIntoViewRequester() }
				var isTemporaryFocusTarget by remember { mutableStateOf(lineIndexToFocus == index) }
				LaunchedEffect(lineIndexToFocus) {
					if (lineIndexToFocus == index) {
						isTemporaryFocusTarget = true
					}
				}
				var textModifier = Modifier.padding(vertical = 4.dp).semantics(mergeDescendants = true) {}
				var isHeading = false
				var headingLevel = 0
				val annotatedString = buildAnnotatedString {
					var currentIdx = 0
					val sortedMarkers = markers.sortedBy { it.position }
					sortedMarkers.forEach { marker ->
						when (marker.mtype) {
							MarkerTypeFfi.HEADING1 -> { isHeading = true; headingLevel = 1 }
							MarkerTypeFfi.HEADING2 -> { isHeading = true; headingLevel = 2 }
							MarkerTypeFfi.HEADING3 -> { isHeading = true; headingLevel = 3 }
							MarkerTypeFfi.HEADING4 -> { isHeading = true; headingLevel = 4 }
							MarkerTypeFfi.HEADING5 -> { isHeading = true; headingLevel = 5 }
							MarkerTypeFfi.HEADING6 -> { isHeading = true; headingLevel = 6 }
							MarkerTypeFfi.LINK -> {
								val markerStartInLine = (marker.position - pos).toInt().coerceAtLeast(0)
								val markerTextLength = marker.text.length
								if (markerStartInLine > currentIdx) {
									append(lineText.substring(currentIdx, markerStartInLine.coerceAtMost(lineText.length)))
									currentIdx = markerStartInLine
								}
								if (currentIdx < lineText.length) {
									val linkEnd = (currentIdx + markerTextLength).coerceAtMost(lineText.length)
									val linkText = lineText.substring(currentIdx, linkEnd)
									val linkAnnotation = LinkAnnotation.Clickable(
										tag = marker.position.toString(),
										styles = TextLinkStyles(
											style = SpanStyle(
												color = MaterialTheme.colorScheme.primary,
												textDecoration = TextDecoration.Underline
											)
										)
									) {
										val result = docState.session.activateLinkFfi(marker.position)
										if (result.found) {
											when (result.action) {
												LinkAction.EXTERNAL -> {
													val intent = Intent(Intent.ACTION_VIEW, Uri.parse(result.url))
													context.startActivity(intent)
												}
												LinkAction.INTERNAL -> {
													val targetLine = docState.session.lineFromPosition(result.offset)
													val targetIndex = (targetLine - 1).toInt().coerceAtLeast(0)
													scope.launch {
														listState.scrollToItem(targetIndex)
														onLineIndexChange(targetIndex)
													}
												}
												else -> {}
											}
										}
									}
									withLink(linkAnnotation) {
										append(linkText)
									}
									currentIdx = linkEnd
								}
							}
							else -> {}
						}
					}
					if (currentIdx < lineText.length) {
						append(lineText.substring(currentIdx))
					}
				}
				if (isHeading) {
					textModifier = textModifier.semantics {
						heading()
						if (headingLevel > 0) {
							stateDescription = "Heading $headingLevel"
						}
					}
				}
				if (isTemporaryFocusTarget) {
					textModifier = textModifier.bringIntoViewRequester(bringIntoViewRequester).focusable()
				}
				val currentOptions = activeSearchOptions
				val currentQuery = activeSearchQuery
				if (currentQuery != null && currentOptions != null) {
					textModifier = textModifier.semantics {
						customActions = listOf(
							CustomAccessibilityAction(t("Find Next")) {
								val nextLine = (index + 2).toLong().coerceAtMost(docState.lineCount)
								val searchPos = docState.session.positionFromLine(nextLine)
								val res = docState.session.searchFfi(currentQuery, searchPos, currentOptions.copy(forward = true))
								if (res.found) {
									val targetLine = docState.session.lineFromPosition(res.position)
									val targetIndex = (targetLine - 1).toInt().coerceAtLeast(0)
									scope.launch {
										listState.scrollToItem(targetIndex)
										onLineIndexChange(targetIndex)
									}
								}
								true
							},
							CustomAccessibilityAction(t("Find Previous")) {
								val searchPos = docState.session.positionFromLine((index + 1).toLong())
								val res = docState.session.searchFfi(currentQuery, searchPos, currentOptions.copy(forward = false))
								if (res.found) {
									val targetLine = docState.session.lineFromPosition(res.position)
									val targetIndex = (targetLine - 1).toInt().coerceAtLeast(0)
									scope.launch {
										listState.scrollToItem(targetIndex)
										onLineIndexChange(targetIndex)
									}
								}
								true
							},
							CustomAccessibilityAction(t("Close Search")) {
								onCloseSearch()
								true
							}
						)
					}
				}
				val textStyle = if (isHeading) {
					MaterialTheme.typography.bodyLarge.copy(fontWeight = FontWeight.Bold)
				} else {
					MaterialTheme.typography.bodyLarge
				}
				Text(
					text = annotatedString,
					style = textStyle,
					modifier = textModifier
				)
				if (isTemporaryFocusTarget) {
					LaunchedEffect(Unit) {
						try {
							bringIntoViewRequester.bringIntoView()
						} catch (e: Exception) {
						}
						isTemporaryFocusTarget = false
						if (lineIndexToFocus == index) {
							onLineIndexChange(null)
						}
					}
				}
			}
		}
	}
}
