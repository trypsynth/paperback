import XCTest
@testable import Paperback

final class SegmentTypeNameTests: XCTestCase {
	private static let allTypes: [SegmentTypeFfi] = [
		.paragraph, .line, .heading, .link, .section, .page,
		.list, .listItem, .table, .separator, .image, .figure, .formula,
	]

	/// The reading bar names the current unit and builds "Previous {}" and "Next {}" from it, so
	/// a type with no name of its own would leave a button announcing nothing useful.
	func testEveryTypeTheCoreCanReportHasAName() {
		for type in Self.allTypes {
			XCTAssertFalse(segmentTypeName(type).isEmpty, "\(type) has no name")
		}
	}

	/// Two units reading alike in the picker would be indistinguishable to a screen reader,
	/// which is the only way most readers tell them apart.
	func testNoTwoTypesShareAName() {
		let names = Self.allTypes.map { segmentTypeName($0) }
		XCTAssertEqual(Self.allTypes.count, Set(names).count, "duplicate names in \(names)")
	}
}

final class NavUnitNameTests: XCTestCase {
	/// Find joins the unit list whenever a search is active, so it has to be distinguishable
	/// from every structural unit it sits beside.
	func testFindDoesNotShareANameWithAStructuralUnit() {
		let findName = NavUnit.find.name
		XCTAssertFalse(findName.isEmpty)
		for type in [SegmentTypeFfi.paragraph, .line, .heading, .section] {
			XCTAssertNotEqual(findName, NavUnit.segment(type).name)
		}
	}

	/// A recorded book offers seek amounts in the same list as its structural units, so the two
	/// kinds must not produce the same label either.
	func testSeekAmountsDoNotShareNamesWithStructuralUnits() {
		let segmentNames = Set([SegmentTypeFfi.paragraph, .line, .section].map { NavUnit.segment($0).name })
		for seconds in audioSeekAmountsSeconds {
			XCTAssertFalse(segmentNames.contains(NavUnit.time(seconds: seconds).name))
		}
	}

	/// A unit's name is what the previous and next buttons announce, so an unnamed one would
	/// leave them reading "Previous" and "Next" with nothing after.
	func testEveryOfferedSeekAmountIsNamed() {
		for seconds in audioSeekAmountsSeconds {
			XCTAssertFalse(NavUnit.time(seconds: seconds).name.isEmpty, "\(seconds)s has no name")
		}
	}
}

final class AppearanceChoiceTests: XCTestCase {
	/// Zero has to mean "follow the system" rather than picking a scheme, or a fresh install
	/// would override whatever the phone is set to.
	func testTheDefaultChoiceFollowsTheSystem() {
		XCTAssertNil(0.preferredColorSchemeChoice)
	}

	func testTheTwoPinnedChoicesMapToTheirSchemes() {
		XCTAssertEqual(.light, 1.preferredColorSchemeChoice)
		XCTAssertEqual(.dark, 2.preferredColorSchemeChoice)
	}

	/// A value stored by a future build with more options must fall back to the system rather
	/// than pinning the app to something arbitrary.
	func testAnUnknownChoiceFallsBackToTheSystem() {
		XCTAssertNil(99.preferredColorSchemeChoice)
		XCTAssertNil((-1).preferredColorSchemeChoice)
	}
}
