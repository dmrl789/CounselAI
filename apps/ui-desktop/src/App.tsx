import React from "react";
import ModelVerifier from "./components/ModelVerifier";
import ModelManager from "./components/ModelManager";

export default function App() {
  return (
    <div className="min-h-screen bg-gray-900 text-white flex flex-col">
      <main className="flex-1 p-4">{/* Application content goes here */}</main>
      <footer className="border-t border-gray-700 p-4 flex flex-col gap-2">
        <ModelVerifier />
        <ModelManager />
      </footer>
    </div>
  );
}
