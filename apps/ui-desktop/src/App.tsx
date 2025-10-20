import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import ChatBox from "./components/ChatBox";
import FileUploader from "./components/FileUploader";
import ModelManager from "./components/ModelManager";
import axios from "axios";

type Message = {
  role: "user" | "assistant";
  text: string;
};

const API_URL = import.meta.env.VITE_MCP_API_URL || "http://localhost:5142";

export default function App() {
  // ---- Chat state ----
  const [messages, setMessages] = useState<Message[]>([]);
  const [loading, setLoading] = useState(false);

  // ---- Gateway + Version state ----
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

  // ---- Chat logic ----
  async function handleSend(query: string) {
    setMessages(prev => [...prev, { role: "user", text: query }]);
    setLoading(true);

    try {
      const { data: reasonReq } = await axios.post(`${API_URL}/query`, { text: query });
      const { data: reasonRes } = await axios.post(`${API_URL}/reason`, reasonReq);

      setMessages(prev => [
        ...prev,
        { role: "assistant", text: reasonRes.summary || "No response." },
      ]);

      await axios.post(`${API_URL}/store`, reasonRes);
    } catch (e: any) {
      setMessages(prev => [
        ...prev,
        { role: "assistant", text: `Error: ${e.message}` },
      ]);
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="h-screen flex flex-col bg-gray-900 text-gray-50">
      {/* ---- Header ---- */}
      <header className="p-4 border-b border-gray-700 flex justify-between items-center">
        <div className="text-xl font-semibold">⚖️ Counsel AI Desktop</div>
        <div className="text-xs text-gray-400">
          v{version} — {gatewayStatus}
        </div>
      </header>

      {/* ---- Gateway Controls ---- */}
      <div className="p-2 bg-gray-800 border-b border-gray-700 flex items-center justify-between">
        <button
          type="button"
          onClick={startLocalGateway}
          className="bg-blue-600 hover:bg-blue-700 text-white px-3 py-1 rounded text-sm"
        >
          Start Local MCP Gateway
        </button>
        {error && <span className="text-red-400 text-xs">{error}</span>}
      </div>

      {/* ---- Chat Window ---- */}
      <main className="flex-1 overflow-y-auto">
        <ChatBox messages={messages} loading={loading} />
      </main>

      {/* ---- Footer ---- */}
      <footer className="border-t border-gray-700 p-4 flex flex-col gap-4">
        <ModelManager />
        <FileUploader />
        <form
          onSubmit={e => {
            e.preventDefault();
            const input = e.currentTarget.elements.namedItem("q") as HTMLInputElement;
            if (input.value.trim()) handleSend(input.value.trim());
            input.value = "";
          }}
          className="flex gap-2"
        >
          <input
            type="text"
            name="q"
            placeholder="Ask about a legal article or case..."
            className="flex-1 bg-gray-800 border border-gray-700 rounded px-3 py-2 text-sm"
          />
          <button
            type="submit"
            disabled={loading}
            className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded"
          >
            {loading ? "..." : "Send"}
          </button>
        </form>
        <small className="text-xs text-gray-500">
          Runs entirely offline. Connects to localhost MCP Gateway for intelligence.
        </small>
      </footer>
    </div>
  );
}
