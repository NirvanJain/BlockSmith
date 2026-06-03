// app.js – Minimal OpenHub frontend script
// This script provides basic interactions:
//   • Load recent activity feed from `/api/activity`
//   • Send chat messages to `/api/chat/send`
//   • Poll chat messages from `/api/chat/stream` (simple long‑poll)
//   • Display a static username placeholder (could be replaced by real auth data)

document.addEventListener('DOMContentLoaded', () => {
  const usernameEl = document.getElementById('username');
  const activityList = document.getElementById('activity-list');
  const chatWindow = document.getElementById('chat-window');
  const chatInput = document.getElementById('chat-input');
  const sendBtn = document.getElementById('send-btn');

  // -------------------------------------------------------------------
  // Helper: create a DOM element for an activity item
  // -------------------------------------------------------------------
  function addActivityItem(text) {
    const li = document.createElement('li');
    li.textContent = text;
    activityList.appendChild(li);
  }

  // -------------------------------------------------------------------
  // Load activity feed – expects JSON: [{ id, description }]
  // -------------------------------------------------------------------
  async function loadActivity() {
    try {
      const resp = await fetch('/api/activity');
      if (!resp.ok) throw new Error('Failed to load activity');
      const data = await resp.json();
      activityList.innerHTML = '';
      data.forEach(item => addActivityItem(item.description || item.title || 'Activity'));
    } catch (e) {
      console.error(e);
      addActivityItem('Unable to load activity feed');
    }
  }

  // -------------------------------------------------------------------
  // Chat handling – simple POST for sending, GET for polling
  // -------------------------------------------------------------------
  async function sendMessage() {
    const content = chatInput.value.trim();
    if (!content) return;
    try {
      const resp = await fetch('/api/chat/send', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ content }),
      });
      if (!resp.ok) throw new Error('Send failed');
      chatInput.value = '';
      // Optimistically add the message (you are "me")
      addChatMessage('me', content);
    } catch (e) {
      console.error(e);
    }
  }

  function addChatMessage(sender, text) {
    const div = document.createElement('div');
    div.classList.add('msg', sender === 'me' ? 'me' : 'other');
    div.textContent = text;
    chatWindow.appendChild(div);
    chatWindow.scrollTop = chatWindow.scrollHeight;
  }

  async function pollChat() {
    try {
      const resp = await fetch('/api/chat/stream'); // simple endpoint that returns JSON array of new messages
      if (!resp.ok) throw new Error('Chat poll failed');
      const msgs = await resp.json(); // [{ sender, content }]
      msgs.forEach(m => {
        if (m.sender !== 'me') addChatMessage('other', m.content);
      });
    } catch (e) {
      console.error(e);
    } finally {
      // poll again after a brief pause
      setTimeout(pollChat, 3000);
    }
  }

  // -------------------------------------------------------------------
  // Initialize UI
  // -------------------------------------------------------------------
  // Placeholder username – in a real app this would be fetched from /api/me
  usernameEl.textContent = 'Friend';

  loadActivity();
  pollChat();

  sendBtn.addEventListener('click', sendMessage);
  chatInput.addEventListener('keypress', e => {
    if (e.key === 'Enter') sendMessage();
  });
});
