import Post from '../components/Post'

const bookmarkedPosts = [
  {
    id: '1',
    author: { name: 'The Octocat', username: 'octocat' },
    content: 'Great thread on building scalable WebSocket systems with Rust and Redis.',
    type: 'post' as const,
    timestamp: '2d',
    stats: { replies: 32, reposts: 67, likes: 234, views: 5670 },
  },

  {
    id: '2',
    author: { name: 'Sarah Chen', username: 'sarahchen' },
    content: 'Just open-sourced our GraphQL API client with automatic query batching.',
    type: 'post' as const,
    timestamp: '3d',
    stats: { replies: 18, reposts: 42, likes: 156, views: 3200 },
  },
]

export default function Bookmarks() {
  return (
    <div>
      <div className="sticky top-0 z-10 bg-[rgba(0,0,0,0.65)] backdrop-blur-md border-b border-[#2f3336] px-4 py-3">
        <h1 className="font-bold">Bookmarks</h1>
        <div className="text-[#71767b] text-xs">@NirvanJain</div>
      </div>
      
      {bookmarkedPosts.map((post) => (
        <Post key={post.id} {...post} />
      ))}
    </div>
  )
}
