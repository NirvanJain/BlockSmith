import { useState } from 'react'
const notifications = [
  { id: 1, type: 'like', user: 'The Octocat', content: 'liked your post', time: '2h' },
  { id: 2, type: 'follow', user: 'Sarah Chen', content: 'followed you', time: '4h' },
  { id: 3, type: 'repost', user: 'Alex Rivera', content: 'reposted your post', time: '6h' },
  { id: 4, type: 'mention', user: 'Maya Patel', content: 'mentioned you in a post', time: '1d' },
]
export default function Notifications() {
  const [activeTab, setActiveTab] = useState<'all' | 'mentions'>('all')
  return (
    <div>
      <div className="sticky top-0 z-10 bg-[rgba(0,0,0,0.65)] backdrop-blur-md border-b border-[#2f3336]">
        <h1 className="font-bold text-lg px-4 py-4">Notifications</h1>
        <div className="flex">
          <button
            onClick={() => setActiveTab('all')}
            className={`flex-1 py-3 text-sm transition-colors hover:bg-[rgba(231,233,234,0.1)] ${
              activeTab === 'all' ? 'font-bold border-b-2 border-[#1d9bf0]' : 'text-[#71767b]'
            }`}
          >
            All
          </button>
          <button
            onClick={() => setActiveTab('mentions')}
            className={`flex-1 py-3 text-sm transition-colors hover:bg-[rgba(231,233,234,0.1)] ${
              activeTab === 'mentions' ? 'font-bold border-b-2 border-[#1d9bf0]' : 'text-[#71767b]'
            }`}
          >
            Mentions
          </button>
        </div>
      </div>
      <div>
        {notifications
          .filter((n) => activeTab === 'all' || n.type === 'mention')
          .map((n) => (
            <div key={n.id} className="flex items-center gap-4 p-4 border-b border-[#2f3336] hover:bg-[rgba(231,233,234,0.03)]">
              <div className="w-10 h-10 bg-[#333639] rounded-full flex items-center justify-center text-[#e7e9ea] font-bold text-sm flex-shrink-0">
                {n.user.split(' ').map(w => w[0]).join('')}
              </div>
              <div className="flex-1">
                <span className="font-bold text-sm">{n.user}</span>
                <span className="text-sm"> {n.content}</span>
                <span className="text-[#71767b] text-sm ml-2">· {n.time}</span>
              </div>
            </div>

          ))}
      </div>
    </div>
  )
}
