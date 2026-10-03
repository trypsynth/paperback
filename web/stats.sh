#!/usr/bin/env bash
# Writes the GitHub numbers the home page shows into web/_data/stats.yml, where cobalt reads them as site.data.stats. Needs gh with a token (GH_TOKEN in CI).
# They are fetched at build time rather than in the browser, so visitors get plain HTML and never hit GitHub's rate limit; the site workflow rebuilds daily to keep them fresh.
set -euo pipefail
repo="trypsynth/paperback"
out="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/_data/stats.yml"

count() { gh api -X GET search/issues -f q="repo:$repo $1" --jq .total_count; }

# Cobalt's liquid has no filter for thousands separators.
thousands() { printf '%s' "$1" | sed -E ':a;s/([0-9])([0-9]{3})($|,)/\1,\2\3/;ta'; }

read -r stars forks < <(gh api "repos/$repo" --jq '"\(.stargazers_count) \(.forks_count)"')
contributors="$(gh api --paginate "repos/$repo/contributors?per_page=100" --jq '.[].login' | wc -l | tr -d ' ')"
open_issues="$(count "is:issue is:open")"
closed_issues="$(count "is:issue is:closed")"
merged_prs="$(count "is:pr is:merged")"
# The last page number of a one-per-page listing is the commit count.
commits="$(gh api "repos/$repo/commits?per_page=1" -i | sed -nE 's/^[Ll]ink:.*[?&]page=([0-9]+)>; rel="last".*/\1/p')"
downloads="$(gh api --paginate "repos/$repo/releases?per_page=100" --jq '.[].assets[].download_count' | awk '{t += $1} END {print t + 0}')"
issues=$((open_issues + closed_issues))
mkdir -p "$(dirname "$out")"
cat > "$out" << EOF
stars: "$(thousands "$stars")"
forks: "$(thousands "$forks")"
contributors: "$(thousands "$contributors")"
merged_prs: "$(thousands "$merged_prs")"
commits: "$(thousands "$commits")"
downloads: "$(thousands "$downloads")"
closed_issues: "$(thousands "$closed_issues")"
issues: "$(thousands "$issues")"
closed_percent: "$(( (closed_issues * 100 + issues / 2) / issues ))"
updated: "$(date -u '+%B %-d, %Y')"
EOF
echo "wrote $out"
