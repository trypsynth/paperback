(async () => {
  const owner = "trypsynth";
  const repo = "paperback";
  const stableEl = document.getElementById("latest-stable");
  const devEl = document.getElementById("latest-dev");
  const historyEl = document.getElementById("stable-history");

  const set = (el, html) => el.innerHTML = html;

  const fetchJSON = async url => {
    const res = await fetch(url);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    return res.json();
  };

  const getReleases = () =>
    fetchJSON(`https://api.github.com/repos/${owner}/${repo}/releases?per_page=50`);

  const fmtCount = n => `downloaded ${n} ${n === 1 ? "time" : "times"}`;

  const render = (release, label, subtitle = "") => {
    const assets = release.assets ?? [];
    const zip = assets.find(a => a.name.toLowerCase().endsWith(".zip"));
    const exe = assets.find(a => a.name.toLowerCase().endsWith(".exe"));
    const version = release.tag_name.replace(/^v/, "");
    return `
      <div>
        <h3>${label} ${version}</h3>
        ${subtitle ? `<p>${subtitle}</p>` : ""}
        <p>${exe ? `<p><a href="${exe.browser_download_url}">Windows Installer (.exe)</a> – ${fmtCount(exe.download_count)}</p>` : ""}</p>
        <p>${zip ? `<p><a href="${zip.browser_download_url}">Windows Portable (.zip)</a> – ${fmtCount(zip.download_count)}</p>` : ""}</p>
        <p><a href="${release.html_url}">View on GitHub</a></p>
      </div>
    `.trim();
  };

  try {
    const releases = await getReleases();
    if (!Array.isArray(releases) || releases.length === 0) {
      throw new Error("empty");
    }
    const isStable = r => /^v?\d+(\.\d+){1,3}$/.test(r.tag_name);
    const stable = releases.find(isStable);
    const dev = releases.find(r => r.tag_name === "latest");
    const previousStable = releases.filter(isStable).slice(1);
    set(stableEl, stable ? render(stable, "Stable Version", "Recommended for most users") : "No stable release found.");
    set(devEl, dev ? render(dev, "Master Build", "Includes experimental features, may be unstable") : "No development builds found.");
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
    set(stableEl, msg);
    set(devEl, msg);
    if (historyEl) set(historyEl, "");
  }
})();
