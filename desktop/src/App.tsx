import { useCallback, useState } from 'react';
import axios from 'axios';

type Message = {
  role: 'user' | 'assistant';
  text: string;
};

const API_URL = process.env.TAURI_APP_MCP_URL ?? 'http://localhost:8080';

export function App() {
  const [messages, setMessages] = useState<Message[]>([]);
  const [loading, setLoading] = useState(false);
  const [offlineMode, setOfflineMode] = useState(false);

  const handleSend = useCallback(
    async (query: string) => {
      if (!query.trim()) {
        return;
      }

      setMessages((prev) => [...prev, { role: 'user', text: query }]);
      setLoading(true);

      try {
        const endpoint = offlineMode ? '/reason_local' : '/reason';
        const { data: reasonReq } = await axios.post(`${API_URL}/query`, { text: query });
        const { data: reasonRes } = await axios.post(`${API_URL}${endpoint}`, reasonReq);

        setMessages((prev) => [...prev, { role: 'assistant', text: reasonRes.summary }]);
        await axios.post(`${API_URL}/store`, reasonRes);
      } catch (error) {
        const message = error instanceof Error ? error.message : 'Unknown error';
        setMessages((prev) => [
          ...prev,
          { role: 'assistant', text: `Error: ${message}` },
        ]);
      } finally {
        setLoading(false);
      }
    },
    [offlineMode]
  );

  return (
    <div className="flex h-screen flex-col bg-slate-950 text-slate-100">
      <main className="flex-1 overflow-y-auto p-4 space-y-3">
        {messages.map((msg, index) => (
          <div
            key={index}
            className={`rounded-lg border border-slate-800 p-3 ${
              msg.role === 'user' ? 'bg-slate-900 text-right' : 'bg-slate-800'
            }`}
          >
            {msg.text}
          </div>
        ))}
        {loading && <div className="text-xs text-slate-400">Thinking…</div>}
      </main>
      <footer className="border-t border-slate-800 p-3 flex items-center justify-between">
        <div className="flex items-center gap-2 text-xs text-slate-400">
          <input
            id="offline"
            type="checkbox"
            checked={offlineMode}
            onChange={(event) => setOfflineMode(event.target.checked)}
          />
          <label htmlFor="offline">Offline Mode (Local LLM)</label>
          <span
            className={`inline-flex items-center gap-1 rounded-full px-2 py-1 text-[10px] ${
              offlineMode ? 'bg-emerald-500/20 text-emerald-300' : 'bg-sky-500/20 text-sky-300'
            }`}
          >
            {offlineMode ? 'Local Mistral 7B' : 'OpenAI GPT-5'}
          </span>
        </div>
      </footer>
    </div>
  );
}

export default App;
