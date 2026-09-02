#!/usr/bin/env bash
# Renders every doc/readme*.md into the built site, one page per language, and links
# them together with a language bar.
#
# This mirrors crates/paperback/build/docs.rs, which runs the same conversion for the
# copy embedded in the binary for the Help menu: same pandoc defaults, same per-file
# `lang` metadata. Keep the two in step, or the manual on the site and the manual in
# the app will drift apart.
#
# English lands at readme.html and every other language at readme-<code>.html. The
# English path is what the site nav and every published link point at, so it stays put.
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
doc_dir="$root/doc"
out_dir="${1:-$root/web/_site}"
base_url="https://paperback.dev"
work_dir=$(mktemp -d)
trap 'rm -rf "$work_dir"' EXIT

# Native language names for the selector. patois' `language_name` is the canonical
# table (crates/patois/src/lang.rs in the patois repo); this covers the languages that
# have a manual. An unknown code falls back to the code itself, so a newly added readme
# still appears in the bar, just with a plainer label than it deserves.
native_name() {
	case "$1" in
	en) printf 'English' ;;
	bs) printf 'Bosanski' ;;
	cs) printf 'Čeština' ;;
	de) printf 'Deutsch' ;;
	es) printf 'Español' ;;
	fi) printf 'Suomi' ;;
	fr) printf 'Français' ;;
	ja) printf '日本語' ;;
	nl) printf 'Nederlands' ;;
	pl) printf 'Polski' ;;
	pt_br) printf 'Português (Brasil)' ;;
	ru) printf 'Русский' ;;
	sr) printf 'Српски' ;;
	vi) printf 'Tiếng Việt' ;;
	zh_CN) printf '中文（简体）' ;;
	*) printf '%s' "$1" ;;
	esac
}

# gettext locale codes use an underscore (zh_CN); HTML wants BCP 47 (zh-CN).
bcp47() { printf '%s' "${1//_/-}"; }

page_for() {
	if [ "$1" = en ]; then printf 'readme.html'; else printf 'readme-%s.html' "$1"; fi
}

# English first, then whatever translations exist, in code order.
langs=(en)
for path in "$doc_dir"/readme-*.md; do
	[ -e "$path" ] || continue
	stem=$(basename "$path" .md)
	langs+=("${stem#readme-}")
done

# hreflang alternates, identical on every page. x-default points at English, which is
# also what a language we have no manual for should land on.
header_file="$work_dir/hreflang.html"
: >"$header_file"
for lang in "${langs[@]}"; do
	printf '<link rel="alternate" hreflang="%s" href="%s/%s">\n' \
		"$(bcp47 "$lang")" "$base_url" "$(page_for "$lang")" >>"$header_file"
done
printf '<link rel="alternate" hreflang="x-default" href="%s/readme.html">\n' "$base_url" >>"$header_file"
# Just enough to keep the bar from rendering as a bulleted column above the title.
# These pages use pandoc's stock template, so there is no site stylesheet to inherit.
cat >>"$header_file" <<'CSS'
<style>
.language-bar ul { list-style: none; padding: 0; margin: 0 0 1.5em; display: flex; flex-wrap: wrap; gap: 0.4em 1em; }
.language-bar [aria-current] { font-weight: bold; }
</style>
CSS

# Each link carries its own `lang`, so a screen reader announces "日本語" in a Japanese
# voice rather than spelling it out in the voice of whatever page you are on. Same
# reasoning as the `-M lang=` metadata in build/docs.rs.
language_bar() {
	local current=$1 lang tag name
	printf '<nav class="language-bar" aria-label="Language">\n<ul>\n'
	for lang in "${langs[@]}"; do
		tag=$(bcp47 "$lang")
		name=$(native_name "$lang")
		if [ "$lang" = "$current" ]; then
			printf '<li><span lang="%s" aria-current="true">%s</span></li>\n' "$tag" "$name"
		else
			printf '<li><a href="/%s" lang="%s" hreflang="%s">%s</a></li>\n' \
				"$(page_for "$lang")" "$tag" "$tag" "$name"
		fi
	done
	printf '</ul>\n</nav>\n'
}

mkdir -p "$out_dir"
for lang in "${langs[@]}"; do
	if [ "$lang" = en ]; then source_file="$doc_dir/readme.md"; else source_file="$doc_dir/readme-$lang.md"; fi
	bar_file="$work_dir/bar-$lang.html"
	language_bar "$lang" >"$bar_file"
	pandoc "--defaults=$doc_dir/pandoc.yaml" \
		-M "lang=$(bcp47 "$lang")" \
		"--include-in-header=$header_file" \
		"--include-before-body=$bar_file" \
		"$source_file" \
		-o "$out_dir/$(page_for "$lang")"
	echo "built $(page_for "$lang") from $(basename "$source_file")"
done
