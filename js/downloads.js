(async () => {
  const owner = "trypsynth";
  const repo = "paperback";
  const stableEl = document.getElementById("latest-stable");
  const devEl = document.getElementById("latest-dev");

  const set = (el, html) => el.innerHTML = html;

  const fetchJSON = async url => {
    const res = await fetch(url);
    if (!res.ok) {
      throw new Error(`HTTP ${res.status}`);
    }
    return res.json();
  };

  const getReleases = () => fetchJSON(`https://api.github.com/repos/${owner}/${repo}/releases?per_page=20`);

  const fmtCount = (n) => `downloaded ${n} ${n === 1 ? "time" : "times"}`;

  const render = (release, label, subtitle = "") => {
    const assets = release.assets ?? [];
    const zip = assets.find((a) => a.name.toLowerCase().endsWith(".zip"));
    const exe = assets.find((a) => a.name.toLowerCase().endsWith(".exe"));
    const version = release.tag_name.replace(/^v/, "");
    return `
      <h3>${label} ${version}</h3>
      ${subtitle ? `<p>${subtitle}</p>` : ""}
      <ul>
        ${exe ? `<li><a href="${exe.browser_download_url}">Windows Installer (.exe)</a> – ${fmtCount(exe.download_count)}</li>` : ""}
        ${zip ? `<li><a href="${zip.browser_download_url}">Portable ZIP (.zip)</a> – ${fmtCount(zip.download_count)}</li>` : ""}
      </ul>
      <p><a href="${release.html_url}">View on GitHub</a></p>
    `.trim();
  };

  try {
    const releases = await getReleases();
    if (!Array.isArray(releases) || releases.length === 0) {
      throw new Error("empty");
    }
    const stable = releases.find(r => /^v?\d+(\.\d+){1,3}$/.test(r.tag_name));
    const dev = releases.find((r) => r.tag_name === "latest");
    set(stableEl, stable ? render(stable, "Stable Version", "Recommended for most users") : "No stable release found.");
    set(devEl, dev ? render(dev, "Master Build", "Includes experimental features, may be unstable") : "No development builds found.");
  } catch (err) {
    const msg = `Unable to load releases. See <a href="https://github.com/${owner}/${repo}/releases">GitHub</a>.`;
    set(stableEl, msg);
    set(devEl, msg);
  }
})();
