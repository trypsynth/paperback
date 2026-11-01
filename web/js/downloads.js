(async () => {
  const owner = "trypsynth";
  const repo = "paperback";
  const stableEl = document.getElementById("latest-stable");
  const devEl = document.getElementById("latest-dev");
  const historyEl = document.getElementById("stable-history");

  const fetchJSON = async url => {
    const res = await fetch(url);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    return res.json();
  };

  const getReleases = () => fetchJSON(`https://api.github.com/repos/${owner}/${repo}/releases?per_page=50`);

  const fmtCount = n => `downloaded ${n} ${n === 1 ? "time" : "times"}`;

  // "paperback.zip"/"paperback_setup.exe" are unsuffixed duplicates of the x64 build, kept
  // in the release only so pre-ship-shape (0.8.5 and earlier) clients' exact-name update
  // check still finds a match. Hide them here so the site doesn't list x64 twice.
  const legacyAssetNames = new Set(["paperback.zip", "paperback_setup.exe"]);

  // One row per download: the link, then the count as quiet text beside it.
  const row = (url, label, count) =>
    count === null ? `<li><a href="${url}">${label}</a></li>` : `<li><a href="${url}">${label}</a> <span class="count">${fmtCount(count)}</span></li>`;

  // Each development build replaces the last, so its counts start again from nothing every time and say nothing about it.
  const render = (release, label, subtitle = "", showMac = false, showCounts = true) => {
    const assets = (release.assets ?? []).filter(a => !legacyAssetNames.has(a.name.toLowerCase()));
    const exes = assets.filter(a => a.name.toLowerCase().endsWith(".exe"));
    const winZips = assets.filter(a => a.name.toLowerCase().endsWith(".zip"));
    const linuxTarballs = assets.filter(a => a.name.toLowerCase().endsWith(".tar.gz"));
    const linuxAppImages = assets.filter(a => a.name.toLowerCase().endsWith(".appimage"));
    const macDmg = assets.find(a => a.name.toLowerCase().endsWith(".dmg"));
    const version = release.tag_name.replace(/^v/, "");
    const archLabel = name => name.toLowerCase().includes("arm64") ? " (ARM64)" : name.toLowerCase().includes("x64") ? " (x64)" : "";
    const count = a => showCounts ? a.download_count : null;
    const archOrder = name => name.toLowerCase().includes("arm64") ? 1 : 0;
    const byArch = list => list.sort((a, b) => archOrder(a.name) - archOrder(b.name));
    const rows = [
      ...byArch(exes).map(a => row(a.browser_download_url, `Windows Installer${archLabel(a.name)} (.exe)`, count(a))),
      ...byArch(winZips).map(a => row(a.browser_download_url, `Windows Portable${archLabel(a.name)} (.zip)`, count(a))),
      ...(showMac && macDmg ? [row(macDmg.browser_download_url, "macOS (.dmg)", count(macDmg))] : []),
      ...byArch(linuxAppImages).map(a => row(a.browser_download_url, `Linux Installer${archLabel(a.name)} (.AppImage)`, count(a))),
      ...byArch(linuxTarballs).map(a => row(a.browser_download_url, `Linux Portable${archLabel(a.name)} (.tar.gz)`, count(a))),
    ];
    // The development build's tag is "latest", which says nothing a reader needs after its name.
    const heading = release.tag_name === "latest" ? label : `${label} ${version}`;
    return `
      <h3>${heading}</h3>
      ${subtitle ? `<p class="subtitle">${subtitle}</p>` : ""}
      <ul>
        ${rows.join("\n        ")}
        <li><a href="${release.html_url}">Release notes on GitHub</a></li>
      </ul>
    `.trim();
  };

  try {
    const releases = await getReleases();
    if (!Array.isArray(releases) || releases.length === 0) throw new Error("empty");
    const isStable = r => /^v?\d+(\.\d+){1,3}$/.test(r.tag_name);
    const stable = releases.find(isStable);
    const dev = releases.find(r => r.tag_name === "latest");
    const previousStable = releases.filter(isStable).slice(1);
    stableEl.innerHTML = stable ? render(stable, "Stable Version", "Recommended for most users", true) : "No stable release found.";
    devEl.innerHTML = dev ? render(dev, "Development Build", "Includes experimental features, may be unstable", true, false) : "No development builds found.";
    if (previousStable.length > 0) {
      const blocks = previousStable.map(r => `<div class="release">${render(r, "Stable Version", "", true)}</div>`).join("");
      historyEl.innerHTML = `
        <details>
          <summary>Previous stable releases</summary>
          ${blocks}
        </details>
      `;
    } else {
      historyEl.innerHTML = "";
    }
  } catch (err) {
    console.log(err);
    const msg = `Unable to load releases. See <a href="https://github.com/${owner}/${repo}/releases">GitHub</a>.`;
    stableEl.innerHTML = msg;
    devEl.innerHTML = msg;
    if (historyEl) historyEl.innerHTML = "";
  }
})();
