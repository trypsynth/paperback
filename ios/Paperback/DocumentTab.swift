import Foundation

struct DocumentTab: Identifiable, Equatable {
	let id: UUID
	var title: String
	var url: URL
	var lineScrollIndex: Int = 0
	var ttsPosition: Int64 = 0
	// var session: DocumentSession  // added when UniFFI is wired up

	init(title: String, url: URL) {
		self.id = UUID()
		self.title = title
		self.url = url
	}

	static func == (lhs: DocumentTab, rhs: DocumentTab) -> Bool {
		lhs.id == rhs.id
	}
}

struct RecentDocument: Identifiable {
	let id: UUID
	var title: String
	var url: URL

	init(title: String, url: URL) {
		self.id = UUID()
		self.title = title
		self.url = url
	}
}
