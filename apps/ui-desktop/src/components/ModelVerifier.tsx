import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default function ModelVerifier() {
  const [status, setStatus] = useState<string>("Checking model integrity...");
  const [hash, setHash] = useState<string>("");

  async function checkModel() {
    try {
      const result = await invoke<string>("verify_active_model");
      setStatus(
        result.startsWith("✅ Model repaired")
          ? "🛠️ Model auto-repaired successfully"
          : "✅ Model verified OK"
      );
      const match = result.match(/SHA256:\s*([0-9a-f]+)/i);
      if (match) setHash(match[1]);
    } catch (err: any) {
      setStatus(`❌ ${err}`);
    }
  }

  useEffect(() => {
    checkModel();
    const id = setInterval(checkModel, 10 * 60 * 1000);
    return () => clearInterval(id);
  }, []);

  return (
    <div
      className={`p-2 text-sm rounded mb-2 ${
        status.includes("✅")
          ? "bg-green-700"
          : status.includes("🛠️")
          ? "bg-blue-700"
          : "bg-red-700"
      } text-white`}
    >
      {status}
      {hash && <div className="text-xs mt-1 break-all">SHA256: {hash}</div>}
    </div>
  );
}
