import { useState, useRef, useEffect } from 'react'

const conversations = [
  { id: 1, name: 'The Octocat', username: 'octocat', lastMessage: 'Hey! Would love to collaborate.', time: '2h', unread: true },
  { id: 2, name: 'Sarah Chen', username: 'sarahchen', lastMessage: 'The PR looks great!', time: '5h', unread: false },
  { id: 3, name: 'Alex Rivera', username: 'alexrivera', lastMessage: 'Thanks for the mentorship!', time: '1d', unread: false },
]

const messages = [
  { id: 1, sender: 'octocat', content: 'Hey Nirvan! I saw your PR on WebSocket implementation.', time: '2:30 PM' },
  { id: 2, sender: 'me', content: 'Thanks! The pub/sub pattern with Redis was key.', time: '2:32 PM' },
  { id: 3, sender: 'octocat', content: 'Would love to collaborate on extending it. What do you think?', time: '2:35 PM' },
  { id: 4, sender: 'me', content: "That's a great idea! Let me create an issue for it.", time: '2:38 PM' },
]

export default function Messages() {
  const [selectedConv, setSelectedConv] = useState<number | null>(1)
  const [newMessage, setNewMessage] = useState('')
  const [chatMessages, setChatMessages] = useState(messages)
  const chatEndRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    chatEndRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [chatMessages])

  const handleSend = () => {
    if (!newMessage.trim()) return
    setChatMessages([
      ...chatMessages,
      {
        id: chatMessages.length + 1,
        sender: 'me',
        content: newMessage.trim(),
        time: new Date().toLocaleTimeString([], { hour: 'numeric', minute: '2-digit' }),
      },
    ])
    setNewMessage('')
  }

  return (
    <div className="flex h-screen">
      {/* Conversations List */}
      <div className={`w-80 border-r border-[#2f3336] flex flex-col ${selectedConv ? 'hidden md:flex' : 'flex'}`}>
        <div className="sticky top-0 z-10 bg-[rgba(0,0,0,0.65)] backdrop-blur-md border-b border-[#2f3336] px-4 py-4">
          <h1 className="font-bold text-lg">Messages</h1>
        </div>
        <div className="flex-1 overflow-y-auto">
          {conversations.map((conv) => (
            <button
              key={conv.id}
              onClick={() => setSelectedConv(conv.id)}
              className={`w-full flex items-center gap-3 px-4 py-4 hover:bg-[rgba(231,233,234,0.03)] transition-colors text-left border-b border-[#2f3336] ${
                selectedConv === conv.id ? 'bg-[rgba(231,233,234,0.03)]' : ''
              }`}
            >
              <div className="w-12 h-12 bg-[#333639] rounded-full flex items-center justify-center text-[#e7e9ea] font-bold text-sm flex-shrink-0">
                {conv.name.split(' ').map(n => n[0]).join('')}
              </div>
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2">
                  <span className="font-bold text-sm truncate">{conv.name}</span>
                  <span className="text-[#71767b] text-xs">· {conv.time}</span>
                </div>
                <div className={`text-sm truncate mt-0.5 ${conv.unread ? 'text-[#e7e9ea]' : 'text-[#71767b]'}`}>
                  {conv.lastMessage}
                </div>
              </div>
              {conv.unread && <div className="w-2.5 h-2.5 bg-[#1d9bf0] rounded-full flex-shrink-0" />}
            </button>
          ))}
        </div>
      </div>

      {/* Chat Area */}
      {selectedConv ? (
        <div className="flex-1 flex flex-col">
          <div className="sticky top-0 z-10 bg-[rgba(0,0,0,0.65)] backdrop-blur-md border-b border-[#2f3336] px-4 py-4">
            <div className="font-bold text-base">
              {conversations.find(c => c.id === selectedConv)?.name}
            </div>
            <div className="text-[#71767b] text-sm">
              @{conversations.find(c => c.id === selectedConv)?.username}
            </div>
          </div>

          <div className="flex-1 overflow-y-auto p-4 space-y-4">
            {chatMessages.map((msg) => (
              <div key={msg.id} className={`flex ${msg.sender === 'me' ? 'justify-end' : 'justify-start'}`}>
                <div className={`max-w-[75%] px-4 py-3 rounded-2xl text-sm ${
                  msg.sender === 'me'
                    ? 'bg-[#1d9bf0] text-white rounded-br-md'
                    : 'bg-[#2f3336] text-[#e7e9ea] rounded-bl-md'
                }`}>
                  <div className="leading-relaxed">{msg.content}</div>
                  <div className={`text-xs mt-2 ${msg.sender === 'me' ? 'text-white/70' : 'text-[#71767b]'}`}>
                    {msg.time}
                  </div>
                </div>
              </div>
            ))}
            <div ref={chatEndRef} />
          </div>

          <div className="border-t border-[#2f3336] p-4">
            <div className="flex gap-3">
              <input
                type="text"
                value={newMessage}
                onChange={(e) => setNewMessage(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleSend()}
                placeholder="Type a message..."
                className="flex-1 bg-[#202327] text-[#e7e9ea] placeholder-[#71767b] py-3 px-4 rounded-full text-sm outline-none"
              />
              <button
                onClick={handleSend}
                disabled={!newMessage.trim()}
                className="bg-[#1d9bf0] hover:bg-[#1a8cd8] text-white font-bold text-sm py-3 px-6 rounded-full transition-colors disabled:opacity-50"
              >
                Send
              </button>
            </div>
          </div>
        </div>
      ) : (
        <div className="hidden md:flex flex-1 items-center justify-center">
          <p className="text-[#71767b] text-lg">Select a conversation</p>
        </div>
      )}
    </div>
  )
}
