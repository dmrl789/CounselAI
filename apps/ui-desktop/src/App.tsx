import React, { useState } from "react";
import ChatBox from "./components/ChatBox";
import FileUploader from "./components/FileUploader";
import axios from "axios";

type Message = {
  role: "user" | "assistant";
  text: string;
};

const API_URL = import.meta.env.VITE_MCP_API_URL || "http://localhost:5142";

export default function App() {
  const [messages, setMessages] = useState<Message[]>([]);
  const [loading, setLoading] = useState(false);

  async function handleSend(query: string) {
    setMessages(prev => [...prev, { role: "user", text: query }]);
    setLoading(true);

    try {
      const { data: reasonReq } = await axios.post(`${API_URL}/query`, {
        text: query,
      });

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
      <header className="p-4 text-xl font-semibold border-b border-gray-700">
        ⚖️ Counsel AI — Local Legal Reasoner
      </header>

      <main className="flex-1 overflow-y-auto">
        <ChatBox messages={messages} loading={loading} />
      </main>

      <footer className="border-t border-gray-700 p-4 flex flex-col gap-2">
        <FileUploader />
        <form
          onSubmit={e => {
            e.preventDefault();
            const input = (e.currentTarget.elements.namedItem("q") as HTMLInputElement);
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
      </footer>
    </div>
  );
}
