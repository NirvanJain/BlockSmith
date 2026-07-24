import { useState } from 'react'

export default function ComposePost() {
  const [content, setContent] = useState('')

  const handleSubmit = () => {
    if (content.trim()) {
      console.log('Posting:', content)
      setContent('')
    }
  }

  return (
    <div className="p-4 border-b border-[#2f3336]">
      <div className="flex gap-3">
        <div className="w-10 h-10 bg-[#333639] rounded-full flex items-center justify-center text-[#e7e9ea] font-bold text-sm flex-shrink-0">
          NJ
        </div>
        <div className="flex-1">
          <textarea
            value={content}
            onChange={(e) => setContent(e.target.value)}
            placeholder="What's happening?"
            className="w-full bg-transparent text-[#e7e9ea] placeholder-[#71767b] text-sm resize-none outline-none min-h-[80px]"
            rows={3}
          />
          <div className="flex items-center justify-between pt-3 mt-2 border-t border-[#2f3336]">
            <span className="text-sm text-[#71767b]">
              {content.length > 0 && `${content.length}/280`}
            </span>
            <button
              onClick={handleSubmit}
              disabled={!content.trim() || content.length > 280}
              className="bg-[#1d9bf0] hover:bg-[#1a8cd8] text-white font-bold text-sm py-2 px-5 rounded-full transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Post
            </button>
          </div>
        </div>
      </div>
    </div>
  )
}
