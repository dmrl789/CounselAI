import React from "react";
import MessageBubble from "./MessageBubble";

export default function ChatBox({
  messages,
  loading,
}: {
  messages: { role: string; text: string }[];
  loading: boolean;
}) {
  return (
    <div className="p-4 space-y-2">
      {messages.map((m, i) => (
        <MessageBubble key={i} role={m.role} text={m.text} />
      ))}
      {loading && <div className="italic text-gray-400">Thinking...</div>}
    </div>
  );
}
