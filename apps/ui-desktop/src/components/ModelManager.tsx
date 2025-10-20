import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

type Model = {
  name: string;
  file: string;
  installed: boolean;
  path: string;
};

export default function ModelManager() {
  const [models, setModels] = useState<Model[]>([]);
  const [activePath, setActivePath] = useState<string>("");
  const [status, setStatus] = useState<string>("");

  async function loadModels() {
    try {
      const result = await invoke<Model[]>("list_local_models");
      setModels(result);
    } catch (error: any) {
      const message =
        typeof error === "string"
          ? error
          : error?.toString?.() ?? "Unknown error";
      setStatus(`Error loading models: ${message}`);
    }
  }

  async function handleInstall(name: string) {
    setStatus(`Installing ${name} ...`);
    try {
      const res = await invoke<string>("install_model", { modelName: name });
      setStatus(res);
      await loadModels();
    } catch (error: any) {
      const message =
        typeof error === "string"
          ? error
          : error?.toString?.() ?? "Unknown error";
      setStatus(`Error: ${message}`);
    }
  }

  async function handleActivate(path: string) {
    try {
      const res = await invoke<string>("set_active_model", { modelPath: path });
      setStatus(res);
      setActivePath(path);
    } catch (error: any) {
      const message =
        typeof error === "string"
          ? error
          : error?.toString?.() ?? "Unknown error";
      setStatus(`Error: ${message}`);
    }
  }

  useEffect(() => {
    loadModels();
  }, []);

  return (
    <div className="p-4 bg-gray-800 rounded-lg border border-gray-700">
      <h2 className="text-lg font-semibold mb-3">🧠 Local Models</h2>
      <div className="space-y-2">
        {models.map((m) => (
          <div
            key={m.name}
            className="flex items-center justify-between bg-gray-900 p-2 rounded-md"
          >
            <div>
              <div className="font-medium text-gray-100">{m.name}</div>
              <div className="text-xs text-gray-400">{m.file}</div>
              {m.installed ? (
                <div className="text-xs text-green-400">Installed ✅</div>
              ) : (
                <div className="text-xs text-red-400">Not installed</div>
              )}
            </div>
            <div className="flex gap-2">
              {!m.installed && (
                <button
                  onClick={() => handleInstall(m.name)}
                  className="bg-blue-600 hover:bg-blue-700 px-3 py-1 rounded text-sm"
                >
                  Install
                </button>
              )}
              {m.installed && (
                <button
                  onClick={() => handleActivate(m.path)}
                  className={`px-3 py-1 rounded text-sm ${
                    activePath === m.path
                      ? "bg-green-600"
                      : "bg-gray-700 hover:bg-green-700"
                  }`}
                >
                  {activePath === m.path ? "Active" : "Activate"}
                </button>
              )}
            </div>
          </div>
        ))}
      </div>
      {status && (
        <div className="mt-3 text-xs text-gray-300 border-t border-gray-700 pt-2">
          {status}
        </div>
      )}
    </div>
  );
}
