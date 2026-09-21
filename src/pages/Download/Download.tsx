import { useEffect, useState } from "react";
import { Monitor, Download as DownloadIcon, ArrowLeft } from "lucide-react";
import "./Download.css";

const REPO = "BastienMosse/network-learning";

interface Asset {
  name: string;
  browser_download_url: string;
  size: number;
}

interface Release {
  tag_name: string;
  published_at: string;
  assets: Asset[];
}

type Platform = "windows" | "linux" | "unknown";

function detectPlatform(): Platform {
  const ua = navigator.userAgent.toLowerCase();
  if (ua.includes("win")) return "windows";
  if (ua.includes("linux")) return "linux";
  return "unknown";
}

function matchAsset(assets: Asset[], platform: Platform): Asset | null {
  for (const a of assets) {
    const n = a.name.toLowerCase();
    if (platform === "windows" && (n.endsWith(".msi") || n.endsWith(".exe"))) return a;
    if (platform === "linux" && (n.endsWith(".deb") || n.endsWith(".appimage"))) return a;
  }
  return null;
}

function formatSize(bytes: number): string {
  const mb = bytes / (1024 * 1024);
  return `${mb.toFixed(1)} MB`;
}

const PLATFORM_LABELS: Record<Platform, string> = {
  windows: "Windows",
  linux: "Linux",
  unknown: "votre OS",
};

export function Download() {
  const [release, setRelease] = useState<Release | null>(null);
  const [error, setError] = useState(false);
  const platform = detectPlatform();

  useEffect(() => {
    fetch(`https://api.github.com/repos/${REPO}/releases/latest`)
      .then((r) => (r.ok ? r.json() : Promise.reject()))
      .then(setRelease)
      .catch(() => setError(true));
  }, []);

  const recommended = release ? matchAsset(release.assets, platform) : null;

  const windowsAssets = release?.assets.filter((a) => {
    const n = a.name.toLowerCase();
    return n.endsWith(".msi") || n.endsWith(".exe");
  }) ?? [];

  const linuxAssets = release?.assets.filter((a) => {
    const n = a.name.toLowerCase();
    return n.endsWith(".deb") || n.endsWith(".appimage");
  }) ?? [];

  return (
    <main className="dl-page">
      <div className="dl-container">
        <a href="." className="dl-back">
          <ArrowLeft size={14} /> Retour au site
        </a>

        <div className="dl-hero">
          <div className="dl-icon">
            <Monitor size={32} />
          </div>
          <h1>Network Learning</h1>
          <p className="dl-lead">
            Le simulateur réseau nécessite l'application desktop.
            <br />
            Téléchargez-la pour accéder au playground.
          </p>
        </div>

        {error && (
          <div className="dl-notice">
            Aucune release disponible pour le moment.
          </div>
        )}

        {!release && !error && (
          <div className="dl-notice">Chargement...</div>
        )}

        {release && (
          <>
            {recommended && (
              <section className="dl-primary">
                <a href={recommended.browser_download_url} className="dl-btn-primary">
                  <DownloadIcon size={18} />
                  Télécharger pour {PLATFORM_LABELS[platform]}
                </a>
                <span className="dl-meta">
                  {release.tag_name} · {formatSize(recommended.size)} · {recommended.name}
                </span>
              </section>
            )}

            <section className="dl-all">
              <h2>Tous les téléchargements</h2>

              {windowsAssets.length > 0 && (
                <div className="dl-group">
                  <h3>Windows</h3>
                  <ul>
                    {windowsAssets.map((a) => (
                      <li key={a.name}>
                        <a href={a.browser_download_url}>{a.name}</a>
                        <span className="dl-size">{formatSize(a.size)}</span>
                      </li>
                    ))}
                  </ul>
                  <details className="dl-instructions">
                    <summary>Installation & désinstallation</summary>
                    <div className="dl-instructions-body">
                      <p><strong>Installer :</strong> lancez le fichier <code>.msi</code>, suivez l'assistant.</p>
                      <p><strong>Désinstaller :</strong> Paramètres → Applications → Network Learning → Désinstaller.</p>
                    </div>
                  </details>
                </div>
              )}

              {linuxAssets.length > 0 && (
                <div className="dl-group">
                  <h3>Linux</h3>
                  <ul>
                    {linuxAssets.map((a) => (
                      <li key={a.name}>
                        <a href={a.browser_download_url}>{a.name}</a>
                        <span className="dl-size">{formatSize(a.size)}</span>
                      </li>
                    ))}
                  </ul>
                  <details className="dl-instructions">
                    <summary>Installation & désinstallation</summary>
                    <div className="dl-instructions-body">
                      <p><strong>.deb (Debian/Ubuntu) :</strong></p>
                      <pre>sudo dpkg -i network-learning_*.deb</pre>
                      <p>Désinstaller : <code>sudo apt remove network-learning</code></p>
                      <p><strong>.AppImage :</strong></p>
                      <pre>chmod +x Network-Learning_*.AppImage{"\n"}./Network-Learning_*.AppImage</pre>
                      <p>Désinstaller : supprimez simplement le fichier.</p>
                    </div>
                  </details>
                </div>
              )}
            </section>
          </>
        )}
      </div>
    </main>
  );
}
