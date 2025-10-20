import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default function ModelVerifier() {
  const [status, setStatus] = useState<string>("Checking model integrity...");
  const [hash, setHash] = useState<string>("");

  async function checkModel() {
    try {
      const result = await invoke<string>("verify_active_model");
      setHash(result);
      setStatus("✅ Model integrity verified");
    } catch (err: any) {
      setStatus(`⚠️ ${err}`);
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
        status.startsWith("✅")
          ? "bg-green-700 text-white"
          : status.startsWith("⚠️")
          ? "bg-yellow-700 text-white"
          : "bg-gray-800 text-gray-200"
      }`}
    >
      {status}
      {hash && (
        <div className="text-xs text-gray-300 break-all mt-1">
          SHA256: {hash}
        </div>
      )}
    </div>
  );
}
