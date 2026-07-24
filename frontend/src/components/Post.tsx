import { useState } from 'react'

interface PostProps {
  id: string
  author: {
    name: string
    username: string
  }
  content: string
  type: 'issue' | 'pr' | 'commit' | 'post'
  repository?: string
  timestamp: string
  stats: {
    replies: number
    reposts: number
    likes: number
    views: number
  }
}

export default function Post({ author, content, type, repository, timestamp, stats }: PostProps) {
  const [liked, setLiked] = useState(false)
  const [likeCount, setLikeCount] = useState(stats.likes)

  const typeBadge = {
    issue: { label: 'Issue', color: 'bg-[#f85149]' },
    pr: { label: 'PR', color: 'bg-[#3fb950]' },
    commit: { label: 'Commit', color: 'bg-[#1d9bf0]' },
    post: { label: 'Post', color: 'bg-[#71767b]' },
  }

  const handleLike = () => {
    setLiked(!liked)
    setLikeCount(liked ? likeCount - 1 : likeCount + 1)
  }

  return (
    <article className="p-4 border-b border-[#2f3336] hover:bg-[rgba(231,233,234,0.03)] transition-colors">
      {/* Header */}
      <div className="flex items-center gap-3 mb-3">
        <div className="w-10 h-10 bg-[#333639] rounded-full flex items-center justify-center text-[#e7e9ea] font-bold text-sm flex-shrink-0">
          {author.name.split(' ').map(n => n[0]).join('')}
        </div>
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-1.5">
            <span className="font-bold text-[#e7e9ea] text-sm">{author.name}</span>
            <span className="text-[#71767b] text-sm">@{author.username}</span>
            <span className="text-[#71767b] text-sm">·</span>
            <span className="text-[#71767b] text-sm">{timestamp}</span>
          </div>
          <div className="flex items-center gap-2 mt-1">
            <span className={`${typeBadge[type].color} text-white text-xs font-bold px-2 py-0.5 rounded`}>
              {typeBadge[type].label}
            </span>
            {repository && (
              <span className="text-[#71767b] text-xs">{repository}</span>
            )}
          </div>
        </div>
      </div>

      {/* Content */}
      <div className="text-[#e7e9ea] text-sm leading-relaxed whitespace-pre-wrap mb-3">
        {content}
      </div>

      {/* Actions */}
      <div className="flex items-center gap-8 text-[#71767b] text-sm">
        <button className="flex items-center gap-1.5 hover:text-[#1d9bf0] transition-colors">
          <svg viewBox="0 0 24 24" className="w-4 h-4 fill-current">
            <path d="M1.751 10c0-4.42 3.584-8 8.005-8h4.366c4.49 0 8.129 3.64 8.129 8.13 0 2.25-.893 4.31-2.457 5.82l-9.994 7.74-9.996-7.74C2.646 14.44 1.751 12.29 1.751 10z" />
          </svg>
          <span>{stats.replies}</span>
        </button>
        <button className="flex items-center gap-1.5 hover:text-[#00ba7c] transition-colors">
          <svg viewBox="0 0 24 24" className="w-4 h-4 fill-current">
            <path d="M4.5 3.88l4.432 4.14-1.364 1.46L5.5 7.55V16c0 1.1.896 2 2 2H13v2H7.5c-2.209 0-4-1.79-4-4V7.55L1.432 9.48.068 8.02 4.5 3.88zM16.5 6H11V4h5.5c2.209 0 4 1.79 4 4v8.45l2.068-1.93 1.364 1.46-4.432 4.14-4.432-4.14 1.364-1.46 2.068 1.93V8c0-1.1-.896-2-2-2z" />
          </svg>
          <span>{stats.reposts}</span>
        </button>
        <button
          onClick={handleLike}
          className={`flex items-center gap-1.5 transition-colors ${liked ? 'text-[#f91880]' : 'hover:text-[#f91880]'}`}
        >
          <svg viewBox="0 0 24 24" className="w-4 h-4 fill-current">
            {liked ? (
              <path d="M20.884 13.19c-1.351 2.48-4.001 5.12-8.379 7.67l-.503.3-.504-.3c-4.379-2.55-7.029-5.19-8.382-7.67-1.36-2.5-1.41-4.86-.514-6.67.887-1.79 2.647-2.91 4.601-3.01 1.651-.09 3.368.56 4.798 2.01 1.429-1.45 3.146-2.1 4.796-2.01 1.954.1 3.714 1.22 4.601 3.01.896 1.81.846 4.17-.514 6.67z" />
            ) : (
              <path d="M16.697 5.5c-1.222-.06-2.679.51-3.89 2.16l-.805 1.09-.806-1.09C9.984 6.01 8.526 5.44 7.304 5.5c-1.243.07-2.349.78-2.91 1.91-.552 1.12-.633 2.78.479 4.82 1.074 1.97 3.257 4.27 7.129 6.61 3.87-2.34 6.052-4.64 7.126-6.61 1.111-2.04 1.03-3.7.477-4.82-.56-1.13-1.666-1.84-2.908-1.91zm4.187 7.69c-1.351 2.48-4.001 5.12-8.379 7.67l-.503.3-.504-.3c-4.379-2.55-7.029-5.19-8.382-7.67-1.36-2.5-1.41-4.86-.514-6.67.887-1.79 2.647-2.91 4.601-3.01 1.651-.09 3.368.56 4.798 2.01 1.429-1.45 3.146-2.1 4.796-2.01 1.954.1 3.714 1.22 4.601 3.01.896 1.81.846 4.17-.514 6.67z" />
            )}
          </svg>
          <span>{likeCount}</span>
        </button>
        <span className="flex items-center gap-1.5">
          <svg viewBox="0 0 24 24" className="w-4 h-4 fill-current">
            <path d="M8.75 21V3h2v18h-2zM18.75 21V8.5h2V21h-2zM13.75 21v-9h2v9h-2zM3.75 21v-4h2v4h-2z" />
          </svg>
          <span>{stats.views.toLocaleString()}</span>
        </span>
      </div>
    </article>
  )
}
