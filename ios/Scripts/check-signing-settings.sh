#!/bin/sh
# Fails when the Xcode project file sets signing values itself. A value there overrides
# ios/Config/Signing.xcconfig and each contributor's own team, so one person's team or bundle ID
# would end up in everyone's build. Xcode's Signing & Capabilities tab writes these, and so does
# an agent "fixing" a signing error, which can also use up a personal team's 10 App IDs a week.
#
# Runs on every build as the app target's first build phase, locally and in CI.
# Usage: check-signing-settings.sh path/to/project.pbxproj

set -eu

awk -v project="$1" '
	/^[[:space:]]*"?(DEVELOPMENT_TEAM|CODE_SIGN_STYLE|CODE_SIGN_IDENTITY|PROVISIONING_PROFILE|PROVISIONING_PROFILE_SPECIFIER)(\[[^]]*\])?"?[[:space:]]*=/ {
		printf "%s:%d: error: the project file sets %s\n", project, NR, $1
		failed = 1
	}
	/^[[:space:]]*"?PRODUCT_BUNDLE_IDENTIFIER/ && !/= "\$\(PAPERBACK_BUNDLE_ID\)(\.[A-Za-z]+)?";$/ {
		printf "%s:%d: error: PRODUCT_BUNDLE_IDENTIFIER must be $(PAPERBACK_BUNDLE_ID), optionally with a suffix\n", project, NR
		failed = 1
	}
	END {
		if (failed) {
			print "error: Remove these settings from the project file. Signing comes from ios/Config/Signing.xcconfig, and your own team from ~/.config/paperback-dev/ios-signing.xcconfig, which `cargo ios run --device` writes. See \"Run on your iPhone or iPad\" in README.md."
			exit 1
		}
	}
' "$1"
