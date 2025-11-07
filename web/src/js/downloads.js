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

	function render(release, label, subtitle = "") {
		const assets = release.assets || [];
		const zip = assets.find(a => a.name.toLowerCase().endsWith(".zip"));
		const exe = assets.find(a => a.name.toLowerCase().endsWith(".exe"));
		const fmtCount = (n) => `downloaded ${n} ${n === 1 ? "time" : "times"}`;
		const version = release.tag_name.replace(/^v/, "");
		return `
			<h3>${label} ${version})</h3>
			${subtitle ? `<p>${subtitle}</p>` : ""}
			<ul>
				${exe ? `<li><a href="${exe.browser_download_url}">Windows Installer (.exe)</a> – ${fmtCount(exe.download_count)}</li>` : ""}
				${zip ? `<li><a href="${zip.browser_download_url}">Portable ZIP (.zip)</a> – ${fmtCount(zip.download_count)}</li>` : ""}
			</ul>
			<p><a href="${release.html_url}">View on GitHub</a></p>
		`;
	}

	try {
		const releases = await getReleases();
		if (!Array.isArray(releases) || releases.length === 0) {
			throw 0;
		}
		const stable = releases.find(r => /^v?\d+(\.\d+){1,2}$/.test(r.tag_name));
		const dev = releases[0];
		set(stableEl, stable ? render(stable, "Stable Version", "Recommended for most users") : "No stable release found.");
		set(devEl, dev ? render(dev, "Latest Development Build", "Includes experimental features, may be unstable") : "No development builds found.");
	} catch {
		const msg = 'Unable to load releases. See <a href="https://github.com/trypsynth/paperback/releases">GitHub</a>.';
		set(stableEl, msg);
		set(devEl, msg);
	}
})();
