import SwiftUI

/// Size multiplies Dynamic Type rather than fixing a point size, so text already enlarged system-wide scales further here.
struct ReadabilityStyle: ViewModifier {
	let scalePercent: Int
	let lineSpacingChoice: Int
	let paragraphSpacingChoice: Int
	let alignmentChoice: Int
	let highContrast: Bool

	@Environment(\.colorScheme) private var colorScheme
	/// Increase Contrast in iOS Settings counts as asking for high contrast here too.
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

	/// SwiftUI's Text can't justify, so desktop's justified choice carried in from elsewhere reads as leading.
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

/// Without this, high contrast would put pure white text on the system's off-white background.
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
	var preferredColorSchemeChoice: ColorScheme? {
		switch self {
		case 1: return .light
		case 2: return .dark
		default: return nil
		}
	}
}
