import React from "react";

export default function MessageBubble({
  role,
  text,
}: {
  role: string;
  text: string;
}) {
  const isUser = role === "user";
  return (
    <div
      className={`flex ${isUser ? "justify-end" : "justify-start"} transition-all`}
    >
      <div
        className={`max-w-[70%] px-4 py-2 rounded-2xl text-sm ${
          isUser
            ? "bg-blue-600 text-white rounded-br-none"
            : "bg-gray-700 text-gray-100 rounded-bl-none"
        }`}
      >
        {text}
      </div>
    </div>
  );
}
