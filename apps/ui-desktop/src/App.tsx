import React from "react";
import ModelManager from "./components/ModelManager";

export default function App() {
  return (
    <div className="min-h-screen bg-gray-950 text-white flex flex-col">
      <header className="p-4 border-b border-gray-800">
        <h1 className="text-2xl font-semibold">Counsel AI Desktop</h1>
      </header>
      <main className="flex-1 p-4 overflow-y-auto">
        <p className="text-gray-400">
          Chat content will appear here. Integrate chat components as needed.
        </p>
      </main>
      <footer className="p-4 border-t border-gray-800 space-y-4">
        <ModelManager />
        <form className="flex gap-2">
          <input
            type="text"
            placeholder="Type your query"
            className="flex-1 px-3 py-2 rounded bg-gray-900 border border-gray-700"
          />
          <button
            type="submit"
            className="px-4 py-2 rounded bg-blue-600 hover:bg-blue-700"
          >
            Send
          </button>
        </form>
      </footer>
    </div>
  );
}
