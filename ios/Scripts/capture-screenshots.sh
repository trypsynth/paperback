#!/usr/bin/env bash
# Captures the App Store screenshots into target/screenshots/<class>/.
#   ios/Scripts/capture-screenshots.sh [all|iphone|ipad]
# Paperback is universal, so App Store Connect needs both a 6.9" iPhone and a 13" iPad set. These simulators render an accepted size natively, so nothing is resampled; override them with IPHONE_SIMULATOR / IPAD_SIMULATOR.
set -euo pipefail
cd "$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BUNDLE_ID="dev.paperback.ios"
DERIVED="target/screenshots-build"
OUT_ROOT="target/screenshots"
BOOKS="target/screenshot-books"
DEVICES=("iphone-6.9|${IPHONE_SIMULATOR:-iPhone Air}" "ipad-13|${IPAD_SIMULATOR:-iPad Pro 13-inch (M5)}")
# Saved under the names ScreenshotMode.books opens them by.
BOOK_SOURCES=(
	"Pride and Prejudice.epub|jane-austen/pride-and-prejudice/downloads/jane-austen_pride-and-prejudice.epub"
	"Alice's Adventures in Wonderland.epub|lewis-carroll/alices-adventures-in-wonderland/john-tenniel/downloads/lewis-carroll_alices-adventures-in-wonderland_john-tenniel.epub"
	"The Adventures of Sherlock Holmes.epub|arthur-conan-doyle/the-adventures-of-sherlock-holmes/downloads/arthur-conan-doyle_the-adventures-of-sherlock-holmes.epub"
)
SHOTS=("01-listen|reader" "02-read|text" "03-contents|toc" "04-recent|recents" "05-settings|settings")
only="${1:-all}"

fail() { echo "error: $*" >&2; exit 1; }

accepted() {
	case "$1" in
		iphone-6.9) echo "1260x2736 1290x2796 1320x2868 2736x1260 2796x1290 2868x1320" ;;
		ipad-13) echo "2064x2752 2048x2732 2752x2064 2732x2048" ;;
	esac
}

fetch_books() {
	mkdir -p "$BOOKS"
	local entry name path
	for entry in "${BOOK_SOURCES[@]}"; do
		IFS='|' read -r name path <<< "$entry"
		[[ -s "$BOOKS/$name" ]] || { echo "==> downloading $name"; curl -fsSL -o "$BOOKS/$name" "https://standardebooks.org/ebooks/$path?source=download" || fail "could not download $name"; }
		# A failed download can leave an HTML error page behind under the book's name.
		[[ "$(unzip -p "$BOOKS/$name" mimetype 2> /dev/null)" == "application/epub+zip" ]] || { rm -f "$BOOKS/$name"; fail "$name is not an EPUB; rerun to download it again"; }
	done
}

capture_device() {
	local class="$1" device="$2" out="$OUT_ROOT/$1" app data entry label screen
	echo "==> $class: $device"
	xcrun simctl list devices available | grep -q "    $device (" || fail "no simulator named '$device'. Available: $(xcrun simctl list devices available | grep -E 'iPhone|iPad' | sed -E 's/ \([0-9A-F-]{36}\).*//; s/^ +//' | sort -u | paste -sd, -)"
	rm -rf "$out"
	mkdir -p "$out"
	echo "    building the app"
	xcodebuild -project ios/Paperback.xcodeproj -scheme Paperback -configuration Debug -destination "platform=iOS Simulator,name=$device" -derivedDataPath "$DERIVED" -quiet build > "$DERIVED.log" 2>&1 || fail "build failed for $device; see $DERIVED.log"
	app="$(find "$DERIVED/Build/Products" -maxdepth 2 -name 'Paperback.app' -path '*iphonesimulator*' | head -1)"
	[[ -n "$app" ]] || fail "could not find Paperback.app under $DERIVED"
	# Erased so nothing left from using the simulator by hand ends up in a shot.
	echo "    erasing and booting"
	xcrun simctl shutdown "$device" 2> /dev/null || true
	xcrun simctl erase "$device"
	xcrun simctl boot "$device"
	xcrun simctl bootstatus "$device" -b > /dev/null
	xcrun simctl status_bar "$device" override --time "9:41" --dataNetwork wifi --wifiMode active --wifiBars 3 --cellularMode active --cellularBars 4 --batteryState charged --batteryLevel 100
	xcrun simctl install "$device" "$app"
	data="$(xcrun simctl get_app_container "$device" "$BUNDLE_ID" data)"
	mkdir -p "$data/Documents/Screenshots"
	cp "$BOOKS"/*.epub "$data/Documents/Screenshots/"
	# A freshly erased device posts its own first-boot notifications.
	echo "    letting the device settle"
	sleep 25
	for entry in "${SHOTS[@]}"; do
		IFS='|' read -r label screen <<< "$entry"
		shoot "$device" "$class" "$out/$label.png" -PaperbackScreenshotMode -PaperbackScreenshotScreen "$screen"
	done
	xcrun simctl shutdown "$device"
	echo "    wrote $(find "$out" -name '*.png' | wc -l | tr -d ' ') screenshots to $out/"
}

shoot() {
	local device="$1" class="$2" path="$3" label settled=0 attempt w h got
	shift 3
	label="$(basename "$path" .png)"
	xcrun simctl launch "$device" "$BUNDLE_ID" "$@" > /dev/null
	sleep 6
	# Two identical frames three seconds apart: the status bar is pinned and every screen is still, so any difference is something passing over it, like a notification banner.
	for attempt in 1 2 3 4 5 6; do
		xcrun simctl io "$device" screenshot "$path.a" > /dev/null 2>&1
		sleep 3
		xcrun simctl io "$device" screenshot "$path.b" > /dev/null 2>&1
		if cmp -s "$path.a" "$path.b"; then mv "$path.b" "$path"; settled=1; break; fi
		[[ "$attempt" -eq 1 ]] && printf '    ...  %-12s waiting for a still frame\n' "$label"
	done
	rm -f "$path.a" "$path.b"
	[[ "$settled" -eq 1 ]] || fail "$label never held still across 6 attempts"
	swift ios/Scripts/flatten-png.swift "$path"
	w="$(sips -g pixelWidth "$path" | awk -F': ' '/pixelWidth/ {print $2}')"
	h="$(sips -g pixelHeight "$path" | awk -F': ' '/pixelHeight/ {print $2}')"
	got="${w}x${h}"
	grep -qw "$got" <<< "$(accepted "$class")" || fail "$label is $got, which App Store Connect does not take for $class. Is '$device' the right simulator?"
	[[ "$(sips -g hasAlpha "$path" | awk -F': ' '/hasAlpha/ {print $2}')" == "no" ]] || fail "$label still has an alpha channel"
	printf '    ok   %-12s %s\n' "$label" "$got"
	xcrun simctl terminate "$device" "$BUNDLE_ID" > /dev/null 2>&1 || true
}

[[ "$only" == all || "$only" == iphone || "$only" == ipad ]] || fail "unknown target '$only' (expected all, iphone or ipad)"
fetch_books
# The Rust core, its Swift bindings and the translations are generated rather than committed.
echo "==> building the Rust core"
cargo xtask ios --release > /dev/null 2>&1 || fail "cargo xtask ios --release failed; run it by hand to see why"
mkdir -p target
for entry in "${DEVICES[@]}"; do
	IFS='|' read -r class device <<< "$entry"
	[[ "$only" == all || "$class" == "$only"-* ]] || continue
	capture_device "$class" "$device"
done
echo "==> Done. Screenshots are in $OUT_ROOT/"
