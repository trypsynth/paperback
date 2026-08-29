import SwiftUI

/// Applies the reader's readability preferences to document text: size, line spacing, the
/// spacing between paragraphs, and alignment.
///
/// The size is a multiplier on the Dynamic Type body size rather than a fixed point size, so a
/// reader who has already enlarged text system-wide keeps that and scales it further here.
struct ReadabilityStyle: ViewModifier {
	let scalePercent: Int
	let lineSpacingChoice: Int
	let paragraphSpacingChoice: Int
	let alignmentChoice: Int

	@ScaledMetric(relativeTo: .body) private var baseSize: CGFloat = 17

	private var fontSize: CGFloat { baseSize * CGFloat(scalePercent) / 100 }

	private var lineSpacing: CGFloat {
		switch lineSpacingChoice {
		case 1: return fontSize * 0.5
		case 2: return fontSize
		default: return 0
		}
	}

	private var paragraphPadding: CGFloat {
		switch paragraphSpacingChoice {
		case 1: return fontSize * 0.35
		case 2: return fontSize * 0.7
		default: return 2
		}
	}

	/// SwiftUI's Text has no justified alignment, so the desktop's fourth choice is not offered
	/// and a justified value carried in from elsewhere reads as leading.
	private var alignment: TextAlignment {
		switch alignmentChoice {
		case 1: return .center
		case 2: return .trailing
		default: return .leading
		}
	}

	private var frameAlignment: Alignment {
		switch alignmentChoice {
		case 1: return .center
		case 2: return .trailing
		default: return .leading
		}
	}

	func body(content: Content) -> some View {
		content
			.font(.system(size: fontSize))
			.lineSpacing(lineSpacing)
			.multilineTextAlignment(alignment)
			.padding(.vertical, paragraphPadding)
			.frame(maxWidth: .infinity, alignment: frameAlignment)
	}
}

extension View {
	func readabilityStyle(_ viewModel: AppViewModel) -> some View {
		modifier(
			ReadabilityStyle(
				scalePercent: viewModel.textScalePercent,
				lineSpacingChoice: viewModel.lineSpacingChoice,
				paragraphSpacingChoice: viewModel.paragraphSpacingChoice,
				alignmentChoice: viewModel.textAlignmentChoice
			)
		)
	}
}
