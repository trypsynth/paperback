import XCTest
@testable import Paperback

final class LinkDownloadTests: XCTestCase {
	func testTextThatIsNotALinkIsRefusedWithAMessageNamingIt() {
		XCTAssertThrowsError(try LinkDownload(link: "see chapter 3")) { error in
			guard case LinkDownloadError.Failed(let message) = error else { return XCTFail("\(error)") }
			XCTAssertTrue(message.hasPrefix("see chapter 3: "), message)
		}
	}
}
