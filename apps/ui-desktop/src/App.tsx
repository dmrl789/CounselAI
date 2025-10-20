import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const [version, setVersion] = useState<string>("loading…");
  const [gatewayStatus, setGatewayStatus] = useState<string>("Gateway idle");
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    invoke<string>("app_version")
      .then(setVersion)
      .catch(() => setVersion("unknown"));
  }, []);

  async function startLocalGateway() {
    try {
      const response = await invoke<string>("start_mcp_gateway");
      setGatewayStatus(response);
      setError(null);
    } catch (err) {
      console.error(err);
      setError(err instanceof Error ? err.message : String(err));
    }
  }

  return (
    <main className="container">
      <header>
        <h1>⚖️ Counsel AI Desktop</h1>
        <p className="tagline">Local-first legal copilot</p>
      </header>

      <section className="card">
        <h2>Status</h2>
        <dl>
          <div>
            <dt>App version</dt>
            <dd>{version}</dd>
          </div>
          <div>
            <dt>MCP Gateway</dt>
            <dd>{gatewayStatus}</dd>
          </div>
        </dl>
        <button type="button" onClick={startLocalGateway}>
          Start Local MCP Gateway
        </button>
        {error && <p className="error">{error}</p>}
      </section>

      <footer>
        <small>Runs entirely offline. Connect to localhost MCP Gateway for intelligence.</small>
      </footer>
    </main>
  );
}

export default App;
