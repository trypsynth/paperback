---
layout: default.liquid
title: Downloads
permalink: /downloads
---
## Downloads

### Latest Stable Release
<div id="latest-stable">Loading…</div>

### Latest Development Build
<div id="latest-dev">Loading…</div>

<script>
(async () => {
	const owner = "trypsynth";
	const repo = "paperback";
	const stableEl = document.getElementById("latest-stable");
	const devEl = document.getElementById("latest-dev");

	const set = (el, html) => el.innerHTML = html;

	async function getReleases() {
		const res = await fetch(`https://api.github.com/repos/${owner}/${repo}/releases?per_page=20`);
		if (!res.ok) {
			throw new Error("GitHub API error");
		}
		return res.json();
	}

	function render(release) {
		const assets = release.assets || [];
		const zip = assets.find(a => a.name.toLowerCase().endsWith(".zip"));
		const exe = assets.find(a => a.name.toLowerCase().endsWith(".exe"));
		return `
			<h2>${release.tag_name}</h2>
			<ul>
				${exe ? `<li><a href="${exe.browser_download_url}">paperback_setup.exe</a></li>` : ""}
				${zip ? `<li><a href="${zip.browser_download_url}">paperback.zip</a></li>` : ""}
			</ul>
		`;
	}

	try {
		const releases = await getReleases();
		if (!Array.isArray(releases) || releases.length === 0) {
			throw 0;
		}
		const stable = releases.find(r => /^v?\d+(\.\d+){1,2}$/.test(r.tag_name));
		const dev = releases[0];
		set(stableEl, stable ? render(stable) : "No stable release found.");
		set(devEl, dev ? render(dev) : "No development builds found.");
	} catch {
		set(stableEl, 'Unable to load releases. See <a href="https://github.com/trypsynth/paperback/releases">GitHub</a>.');
		set(devEl, 'Unable to load releases. See <a href="https://github.com/trypsynth/paperback/releases">GitHub</a>.');
	}
})();
</script>
