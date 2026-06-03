import React, { useEffect, useRef, useState } from 'react';
import { fetchConversations, createConversation, fetchMessages, WS_BASE } from '../api';
import type { Conversation, Message } from '../types';

type Props = {
  token: string | null;
  currentUsername?: string | null;
};

const ChatBox: React.FC<Props> = ({ token, currentUsername }) => {
  const [conversations, setConversations] = useState<Conversation[]>([]);
  const [activeConv, setActiveConv] = useState<string | null>(null);
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState('');
  const [newRecipient, setNewRecipient] = useState('');
  const [showNewDm, setShowNewDm] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Load conversations
  useEffect(() => {
    if (!token) return;
    fetchConversations(token)
      .then((convs) => {
        setConversations(convs);
        if (convs.length > 0 && !activeConv) setActiveConv(convs[0].id);
      })
      .catch(console.error);
  }, [token]);

  // Connect WebSocket
  useEffect(() => {
    if (!token) return;
    const ws = new WebSocket(`${WS_BASE}/ws?token=${encodeURIComponent(token)}`);
    wsRef.current = ws;

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data) as Partial<Message>;
        if (data.content && data.conversation_id === activeConv) {
          setMessages((prev) => [
            ...prev,
            {
              sender_username: data.sender_username ?? 'other',
              sender_avatar: data.sender_avatar ?? '',
              content: data.content!,
              created_at: data.created_at ?? new Date().toISOString(),
              conversation_id: data.conversation_id,
            },
          ]);
        }
      } catch {
        // non-JSON frame (e.g. ping)
      }
    };

    return () => {
      ws.close();
      wsRef.current = null;
    };
  }, [token, activeConv]);

  // Load messages when active conversation changes
  useEffect(() => {
    if (!token || !activeConv) return;
    fetchMessages(token, activeConv)
      .then(setMessages)
      .catch(console.error);
  }, [token, activeConv]);

  // Scroll to bottom on new messages
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const handleSend = () => {
    if (!wsRef.current || !activeConv || !input.trim()) return;
    const content = input.trim();
    wsRef.current.send(JSON.stringify({ conversation_id: activeConv, content }));
    setMessages((prev) => [
      ...prev,
      {
        sender_username: currentUsername ?? 'me',
        sender_avatar: '',
        content,
        created_at: new Date().toISOString(),
        conversation_id: activeConv,
      },
    ]);
    setInput('');
  };

  const handleNewDm = async () => {
    if (!token || !newRecipient.trim()) return;
    try {
      const { id } = await createConversation(token, newRecipient.trim());
      const convs = await fetchConversations(token);
      setConversations(convs);
      setActiveConv(id);
      setNewRecipient('');
      setShowNewDm(false);
    } catch (e) {
      console.error(e);
      alert('User not found or error creating conversation.');
    }
  };

  if (!token) {
    return <p className="empty-state">Sign in to use chat.</p>;
  }

  return (
    <div className="chat-layout">
      <div className="chat-sidebar">
        <div className="chat-sidebar-header">
          <span>Conversations</span>
          <button className="btn-icon" onClick={() => setShowNewDm((v) => !v)} title="New DM">+</button>
        </div>
        {showNewDm && (
          <div className="new-dm-form">
            <input
              type="text"
              placeholder="GitHub username"
              value={newRecipient}
              onChange={(e) => setNewRecipient(e.target.value)}
              onKeyDown={(e) => { if (e.key === 'Enter') handleNewDm(); }}
            />
            <button className="btn primary" onClick={handleNewDm}>Start</button>
          </div>
        )}
        {conversations.length === 0 ? (
          <p className="empty-state" style={{ fontSize: '0.8rem', padding: '0.5rem' }}>No conversations</p>
        ) : (
          <ul className="conv-list">
            {conversations.map((c) => (
              <li
                key={c.id}
                className={`conv-item ${c.id === activeConv ? 'active' : ''}`}
                onClick={() => setActiveConv(c.id)}
              >
                {c.name ?? c.participants.filter((p) => p !== currentUsername).join(', ') ?? 'DM'}
              </li>
            ))}
          </ul>
        )}
      </div>

      <div className="chat-main">
        <div className="chat-window">
          {messages.map((msg, i) => {
            const isMe = msg.sender_username === (currentUsername ?? 'me');
            return (
              <div key={i} className={`msg ${isMe ? 'me' : 'other'}`}>
                {!isMe && <span className="msg-sender">{msg.sender_username}</span>}
                <span>{msg.content}</span>
              </div>
            );
          })}
          <div ref={messagesEndRef} />
        </div>
        <div className="chat-input-row">
          <input
            type="text"
            value={input}
            placeholder={activeConv ? 'Say something…' : 'Select a conversation'}
            disabled={!activeConv}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => { if (e.key === 'Enter') handleSend(); }}
          />
          <button className="btn primary" onClick={handleSend} disabled={!activeConv}>Send</button>
        </div>
      </div>
    </div>
  );
};

export default ChatBox;
