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

  const classifyAsset = asset => {
    const name = asset.name.toLowerCase();

    if (name === "paperback_setup.exe" || (name.includes("setup") && name.endsWith(".exe"))) {
      return "Windows Installer (.exe)";
    }
    if (name === "paperback_windows.zip" || name === "paperback.zip" || (name.includes("windows") && name.endsWith(".zip"))) {
      return "Windows Portable (.zip)";
    }
    if (name === "paperback_mac.zip" || name === "paperback_macos.zip" || (name.includes("mac") && name.endsWith(".zip"))) {
      return "macOS Portable (.zip)";
    }
    if (name.endsWith(".dmg")) {
      return "macOS Installer (.dmg)";
    }
    if (name === "paperback_linux.zip" || (name.includes("linux") && name.endsWith(".zip"))) {
      return "Linux Portable (.zip)";
    }
    if (name.includes("linux") && name.endsWith(".tar.gz")) {
      return "Linux Archive (.tar.gz)";
    }
    if (name.endsWith(".appimage")) {
      return "Linux AppImage";
    }
    if (name.endsWith(".deb")) {
      return "Linux Debian Package (.deb)";
    }
    if (name.endsWith(".rpm")) {
      return "Linux RPM Package (.rpm)";
    }
    return null;
  };

  const renderDownloads = release => {
    const links = (release.assets ?? [])
      .map(asset => {
        const label = classifyAsset(asset) || asset.name;
        return `<li><a href="${asset.browser_download_url}">${label}</a> - ${fmtCount(asset.download_count)}</li>`;
      })
      .filter(Boolean);
    if (links.length === 0) return "<p>No downloadable assets were published for this release.</p>";
    return `<ul>${links.join("")}</ul>`;
  };

  const render = (release, label, subtitle = "") => {
    const version = release.tag_name.replace(/^v/, "");
    return `
      <div>
        <h3>${label} ${version}</h3>
        ${subtitle ? `<p>${subtitle}</p>` : ""}
        ${renderDownloads(release)}
        <p><a href="${release.html_url}">View on GitHub</a></p>
      </div>
    `.trim();
  };

  try {
    const releases = await getReleases();
    if (!Array.isArray(releases) || releases.length === 0) throw new Error("empty");
    const isStable = r => /^v?\d+(\.\d+){1,3}$/.test(r.tag_name);
    const stable = releases.find(isStable);
    const dev = releases.find(r => r.tag_name === "latest");
    const previousStable = releases.filter(isStable).slice(1);
    stableEl.innerHTML = stable ? render(stable, "Stable Version", "Recommended for most users") : "No stable release found.";
    devEl.innerHTML = dev ? render(dev, "Master Build", "Includes experimental features, may be unstable") : "No development builds found.";
    if (previousStable.length > 0) {
      const blocks = previousStable.map(r => render(r, "Stable Version")).join("");
      historyEl.innerHTML = `
        <details>
          <summary>Previous Stable Releases</summary>
          <div>
            ${blocks}
          </div>
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
