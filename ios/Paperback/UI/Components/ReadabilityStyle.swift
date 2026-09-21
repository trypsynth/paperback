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
	let highContrast: Bool

	@Environment(\.colorScheme) private var colorScheme
	/// True when the reader has asked for stronger contrast in iOS Settings, which counts as
	/// asking for it here as well.
	@Environment(\.colorSchemeContrast) private var systemContrast

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

	/// Black on white, or white on black, rather than the softer system label colour. Nil
	/// leaves the system's own colours alone, which already adapt to light and dark.
	private var textColor: Color? {
		guard highContrast || systemContrast == .increased else { return nil }
		return colorScheme == .dark ? .white : .black
	}

	func body(content: Content) -> some View {
		content
			.font(.system(size: fontSize))
			.lineSpacing(lineSpacing)
			.multilineTextAlignment(alignment)
			.padding(.vertical, paragraphPadding)
			.frame(maxWidth: .infinity, alignment: frameAlignment)
			.foregroundStyle(textColor ?? .primary)
	}
}

/// The page behind the text, which has to move with the text colour or high contrast would put
/// pure white on the system's off-white background.
struct ReadingBackground: ViewModifier {
	let highContrast: Bool

	@Environment(\.colorScheme) private var colorScheme
	@Environment(\.colorSchemeContrast) private var systemContrast

	func body(content: Content) -> some View {
		if highContrast || systemContrast == .increased {
			content.background(colorScheme == .dark ? Color.black : Color.white)
		} else {
			content
		}
	}
}

extension View {
	func readabilityStyle(_ viewModel: AppViewModel) -> some View {
		modifier(
			ReadabilityStyle(
				scalePercent: viewModel.textScalePercent,
				lineSpacingChoice: viewModel.lineSpacingChoice,
				paragraphSpacingChoice: viewModel.paragraphSpacingChoice,
				alignmentChoice: viewModel.textAlignmentChoice,
				highContrast: viewModel.highContrastText
			)
		)
	}

	func readingBackground(_ viewModel: AppViewModel) -> some View {
		modifier(ReadingBackground(highContrast: viewModel.highContrastText))
	}
}

extension Int {
	/// The scheme an appearance choice forces, or nil to follow the system.
	var preferredColorSchemeChoice: ColorScheme? {
		switch self {
		case 1: return .light
		case 2: return .dark
		default: return nil
		}
	}
}
