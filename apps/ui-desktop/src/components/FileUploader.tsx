import React, { useState } from "react";

export default function FileUploader() {
  const [fileNames, setFileNames] = useState<string[]>([]);

  const handleFiles = (files: FileList | null) => {
    if (!files) return;
    const names = Array.from(files).map(f => f.name);
    setFileNames(prev => [...prev, ...names]);
  };

  return (
    <div className="flex flex-col gap-1">
      <label className="text-xs text-gray-400">Attach legal files (PDF, DOCX)</label>
      <input
        type="file"
        multiple
        onChange={e => handleFiles(e.target.files)}
        className="text-sm text-gray-300"
      />
      {fileNames.length > 0 && (
        <div className="text-xs text-gray-400">
          Attached: {fileNames.join(", ")}
        </div>
      )}
    </div>
  );
}
