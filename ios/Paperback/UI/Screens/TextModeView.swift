import SwiftUI

struct TextModeView: View {
	@Environment(AppViewModel.self) private var viewModel
	let tab: DocumentTab

	var body: some View {
		if let model = viewModel.bookModel(for: tab) {
			ReadingScreen(model: model, reading: viewModel.reading, style: ReadingStyle(scalePercent: viewModel.textScalePercent, lineSpacingChoice: viewModel.lineSpacingChoice, paragraphSpacingChoice: viewModel.paragraphSpacingChoice, alignmentChoice: viewModel.textAlignmentChoice, highContrast: viewModel.highContrastText))
				.id(tab.id)
		} else {
			Text("No document open.")
				.padding()
				.foregroundStyle(.secondary)
		}
	}
}

private struct ReadingScreen: UIViewControllerRepresentable {
	let model: BookModel
	let reading: ReadingController
	let style: ReadingStyle

	func makeUIViewController(context: Context) -> ReadingViewController {
		let controller = ReadingViewController(model: model, position: reading.ttsPosition)
		controller.bookView.style = style
		controller.onPositionChange = { [weak reading] position in
			reading?.textModeMoved(to: position)
		}
		reading.onTextModeJump = { [weak controller] position in
			controller?.jump(toPosition: position)
		}
		reading.onTextModeFollow = { [weak controller] position in
			controller?.follow(position: position)
		}
		return controller
	}

	func updateUIViewController(_ controller: ReadingViewController, context: Context) {
		controller.bookView.style = style
	}
}
