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
    `<li><a href="${url}">${label}</a> <span class="count">${fmtCount(count)}</span></li>`;

  const render = (release, label, subtitle = "", showApk = false, showMac = false) => {
    const assets = (release.assets ?? []).filter(a => !legacyAssetNames.has(a.name.toLowerCase()));
    const exes = assets.filter(a => a.name.toLowerCase().endsWith(".exe"));
    const winZips = assets.filter(a => a.name.toLowerCase().endsWith(".zip"));
    const linuxTarballs = assets.filter(a => a.name.toLowerCase().endsWith(".tar.gz"));
    const linuxAppImages = assets.filter(a => a.name.toLowerCase().endsWith(".appimage"));
    const macDmg = assets.find(a => a.name.toLowerCase().endsWith(".dmg"));
    const apks = showApk ? assets.filter(a => a.name.toLowerCase().endsWith(".apk")) : [];
    const version = release.tag_name.replace(/^v/, "");
    const archLabel = name => name.toLowerCase().includes("arm64") ? " (ARM64)" : name.toLowerCase().includes("x64") ? " (x64)" : "";
    const archOrder = name => name.toLowerCase().includes("arm64") ? 1 : 0;
    const byArch = list => list.sort((a, b) => archOrder(a.name) - archOrder(b.name));
    const rows = [
      ...byArch(exes).map(a => row(a.browser_download_url, `Windows Installer${archLabel(a.name)} (.exe)`, a.download_count)),
      ...byArch(winZips).map(a => row(a.browser_download_url, `Windows Portable${archLabel(a.name)} (.zip)`, a.download_count)),
      ...(showMac && macDmg ? [row(macDmg.browser_download_url, "macOS (.dmg)", macDmg.download_count)] : []),
      ...byArch(linuxAppImages).map(a => row(a.browser_download_url, `Linux Installer${archLabel(a.name)} (.AppImage)`, a.download_count)),
      ...byArch(linuxTarballs).map(a => row(a.browser_download_url, `Linux Portable${archLabel(a.name)} (.tar.gz)`, a.download_count)),
      ...apks.map(a => {
        const apkLabel = a.name.includes("arm64") ? "Android APK (arm64-v8a)" : a.name.includes("arm") ? "Android APK (armeabi-v7a)" : "Android APK";
        return row(a.browser_download_url, apkLabel, a.download_count);
      }),
    ];
    return `
      <h3>${label} ${version}</h3>
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
    stableEl.innerHTML = stable ? render(stable, "Stable Version", "Recommended for most users", false, true) : "No stable release found.";
    devEl.innerHTML = dev ? render(dev, "Master Build", "Includes experimental features, may be unstable", true, true) : "No development builds found.";
    if (previousStable.length > 0) {
      const blocks = previousStable.map(r => `<div class="release">${render(r, "Stable Version", "", false, true)}</div>`).join("");
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
