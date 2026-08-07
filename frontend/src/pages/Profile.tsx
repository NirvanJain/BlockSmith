import { useState } from 'react'
import Post from '../components/Post'

const userData = {
  name: 'Nirvan Jain',
  username: 'NirvanJain',
  bio: 'Building the future of open source developer reputation. Rust enthusiast | Open source contributor',
  location: 'San Francisco, CA',
  website: 'https://github.com/NirvanJain',
  following: 234,
  followers: 1250,
  level: 13,
  xp: 1250,
}

const userPosts = [
  {
    id: '1',
    author: { name: 'Nirvan Jain', username: 'NirvanJain' },
    content: 'Just shipped the reputation engine! Users can now earn XP for their open source contributions.\n\nFeatures:\n- Automatic PR tracking\n- Issue contribution counting\n- Badge system\n- Leaderboard',
    type: 'post' as const,
    timestamp: '3h',
    stats: { replies: 24, reposts: 45, likes: 156, views: 2340 },
  },
  
  {
    id: '2',
    author: { name: 'Nirvan Jain', username: 'NirvanJain' },
    content: 'feat: Add WebSocket support for real-time notifications\n\nThis PR adds WebSocket connection management and event broadcasting.',
    type: 'pr' as const,
    repository: 'BlockSmith/BlockSmith',
    timestamp: '1d',
    stats: { replies: 12, reposts: 24, likes: 89, views: 1240 },
  },
]

export default function Profile() {
  const [activeTab, setActiveTab] = useState<'posts' | 'replies' | 'issues' | 'pr'>('posts')
  const [isFollowing, setIsFollowing] = useState(false)

  return (
    <div>
      {/* Header */}
      <div className="sticky top-0 z-10 bg-[rgba(0,0,0,0.65)] backdrop-blur-md border-b border-[#2f3336]">
        <div className="flex items-center gap-6 px-4 py-3">
          <button className="p-2 -ml-2 rounded-full hover:bg-[rgba(231,233,234,0.1)]">
            <svg viewBox="0 0 24 24" className="w-5 h-5 fill-current">
              <path d="M7.414 13l5.043 5.04-1.414 1.42L3.586 12l7.457-7.46 1.414 1.42L7.414 11H21v2H7.414z" />
            </svg>
          </button>
          <div>
            <h1 className="font-bold text-lg leading-5">{userData.name}</h1>
            <div className="text-[#71767b] text-sm">1,234 posts</div>
          </div>
        </div>
      </div>

      {/* Profile Info */}
      <div className="px-4 py-4 border-b border-[#2f3336]">
        <div className="flex items-end justify-between mb-4">
          <div className="w-20 h-20 bg-[#333639] rounded-full flex items-center justify-center text-[#e7e9ea] font-bold text-2xl">
            NJ
          </div>
          <button
            onClick={() => setIsFollowing(!isFollowing)}
            className={`font-bold text-sm px-5 py-2 rounded-full transition-colors ${
              isFollowing
                ? 'border border-[#536471] hover:border-[#f4212e] hover:text-[#f4212e]'
                : 'bg-[#eff3f4] text-[#0f1419] hover:bg-[#d7dbdc]'
            }`}
          >
            {isFollowing ? 'Following' : 'Follow'}
          </button>
        </div>

        <h2 className="font-extrabold text-xl">{userData.name}</h2>
        <div className="text-[#71767b] text-sm mt-1">@{userData.username}</div>
        
        <p className="mt-3 text-sm leading-relaxed">{userData.bio}</p>

        <div className="flex flex-wrap items-center gap-x-3 gap-y-2 mt-3 text-[#71767b] text-sm">
          <span>{userData.location}</span>
          <a href={userData.website} className="text-[#1d9bf0] hover:underline">GitHub</a>
        </div>

        <div className="flex items-center gap-5 mt-3 text-sm">
          <span><strong>{userData.following}</strong> <span className="text-[#71767b]">Following</span></span>
          <span><strong>{userData.followers}</strong> <span className="text-[#71767b]">Followers</span></span>
        </div>

        <div className="flex items-center gap-4 mt-3 text-sm">
          <span className="text-[#71767b]">Level <strong className="text-[#1d9bf0]">{userData.level}</strong></span>
          <span className="text-[#71767b]">XP <strong className="text-[#1d9bf0]">{userData.xp}</strong></span>
        </div>
      </div>

      {/* Tabs */}
      <div className="flex border-b border-[#2f3336]">
        {(['posts', 'replies', 'issues', 'pr'] as const).map((tab) => (
          <button
            key={tab}
            onClick={() => setActiveTab(tab)}
            className={`flex-1 py-3 text-sm transition-colors hover:bg-[rgba(231,233,234,0.1)] ${
              activeTab === tab ? 'font-bold border-b-2 border-[#1d9bf0]' : 'text-[#71767b]'
            }`}
          >
            {tab.charAt(0).toUpperCase() + tab.slice(1)}
          </button>
        ))}
      </div>

      {/* Posts */}
      <div>
        {userPosts.map((post) => (
          <Post key={post.id} {...post} />
        ))}
      </div>
    </div>
  )
}
