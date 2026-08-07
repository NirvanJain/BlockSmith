import { useState } from 'react'
import Post from '../components/Post'
import ComposePost from '../components/ComposePost'

const mockPosts = [
  {
    id: '1',
    author: { name: 'Nirvan Jain', username: 'NirvanJain' },
    content: 'Just merged a major PR that adds WebSocket support for real-time notifications! The new system uses a pub/sub pattern with Redis for horizontal scaling.',
    type: 'pr' as const,
    repository: 'BlockSmith/BlockSmith',
    timestamp: '2h',
    stats: { replies: 12, reposts: 24, likes: 89, views: 1240 },
  },
  {
    id: '2',
    author: { name: 'The Octocat', username: 'octocat' },
    content: 'Looking for contributors to help with the new GitHub Actions integration. We need someone experienced with Rust and the GitHub API.\n\nSkills needed:\n- Rust async/await\n- Octocrab or similar GitHub API client\n- Understanding of webhooks\n\nDM me if interested!',
    type: 'issue' as const,
    repository: 'BlockSmith/BlockSmith',
    timestamp: '4h',
    stats: { replies: 8, reposts: 15, likes: 42, views: 856 },
  },
  
  {
    id: '3',
    author: { name: 'Sarah Chen', username: 'sarahchen' },
    content: 'The reputation system is coming along nicely! Users can now earn XP for:\n- Merging PRs\n- Opening issues\n- Code reviews\n- Mentoring new contributors\n\nStill working on the leaderboard page.',
    type: 'post' as const,
    timestamp: '6h',
    stats: { replies: 5, reposts: 8, likes: 34, views: 420 },
  },
  {
    id: '4',
    author: { name: 'Alex Rivera', username: 'alexrivera' },
    content: 'feat: Implement Merkle tree verification for contribution history\n\nAdded cryptographic proof that a contribution was recorded at a specific time. This makes the reputation system tamper-proof!',
    type: 'commit' as const,
    repository: 'BlockSmith/BlockSmith',
    timestamp: '8h',
    stats: { replies: 3, reposts: 7, likes: 28, views: 312 },
  },
]

export default function Feed() {
  const [activeTab, setActiveTab] = useState<'foryou' | 'following'>('foryou')

  return (
    <div>
      {/* Header */}
      <div className="sticky top-0 z-10 bg-[rgba(0,0,0,0.65)] backdrop-blur-md border-b border-[#2f3336]">
        <h1 className="text-lg font-bold px-4 py-4">Home</h1>
        <div className="flex">
          <button
            onClick={() => setActiveTab('foryou')}
            className={`flex-1 py-3 text-sm transition-colors hover:bg-[rgba(231,233,234,0.1)] ${
              activeTab === 'foryou' ? 'font-bold border-b-2 border-[#1d9bf0]' : 'text-[#71767b]'
            }`}
          > 
            For you
          </button>
          <button
            onClick={() => setActiveTab('following')}
            className={`flex-1 py-3 text-sm transition-colors hover:bg-[rgba(231,233,234,0.1)] ${
              activeTab === 'following' ? 'font-bold border-b-2 border-[#1d9bf0]' : 'text-[#71767b]'
            }`}
          >
            Following
          </button>
        </div>
      </div>
      {/* Compose */}
      <ComposePost />

      {/* Posts */}
      <div>
        {mockPosts.map((post) => (
          <Post key={post.id} {...post} />
        ))}
      </div>
    </div>
  )
}
