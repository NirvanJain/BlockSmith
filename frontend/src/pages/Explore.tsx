import { useState } from 'react'

export default function Explore() {
  const [searchQuery, setSearchQuery] = useState('')
  const [activeTab, setActiveTab] = useState<'trending' | 'issues' | 'developers'>('trending')

  return (
    <div>
      {/* Header */}
      <div className="sticky top-0 z-10 bg-[rgba(0,0,0,0.65)] backdrop-blur-md border-b border-[#2f3336]">
        <div className="p-4">
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Search BlockSmith"
            className="w-full bg-[#202327] text-[#e7e9ea] placeholder-[#71767b] py-3 px-4 rounded-full text-sm outline-none focus:ring-1 focus:ring-[#1d9bf0]"
          />
        </div>
        <div className="flex">

          <button
            onClick={() => setActiveTab('trending')}
            className={`flex-1 py-3 text-sm transition-colors hover:bg-[rgba(231,233,234,0.1)] ${
              activeTab === 'trending' ? 'font-bold border-b-2 border-[#1d9bf0]' : 'text-[#71767b]'
            }`}
          >
            Trending
          </button>

          <button
            onClick={() => setActiveTab('issues')}
            className={`flex-1 py-3 text-sm transition-colors hover:bg-[rgba(231,233,234,0.1)] ${
              activeTab === 'issues' ? 'font-bold border-b-2 border-[#1d9bf0]' : 'text-[#71767b]'
            }`}
          >
            Issues
          </button>
          
          <button
            onClick={() => setActiveTab('developers')}
            className={`flex-1 py-3 text-sm transition-colors hover:bg-[rgba(231,233,234,0.1)] ${
              activeTab === 'developers' ? 'font-bold border-b-2 border-[#1d9bf0]' : 'text-[#71767b]'
            }`}
          >
            Developers
          </button>
        </div>
      </div>

      {/* Content */}
      <div className="p-4 space-y-4">
        {activeTab === 'trending' && (
          <>
            <div className="bg-[#16181c] rounded-lg p-4">
              <div className="text-[#71767b] text-xs mb-2">Trending in Open Source</div>
              <div className="font-bold text-base">#WebAssembly</div>
              <div className="text-[#71767b] text-sm mt-1">12.5K posts</div>
            </div>
            <div className="bg-[#16181c] rounded-lg p-4">
              <div className="text-[#71767b] text-xs mb-2">Technology · Trending</div>
              <div className="font-bold text-base">#AIAssistants</div>
              <div className="text-[#71767b] text-sm mt-1">8.2K posts</div>
            </div>
            <div className="bg-[#16181c] rounded-lg p-4">
              <div className="text-[#71767b] text-xs mb-2">Programming · Trending</div>
              <div className="font-bold text-base">#TypeScript5</div>
              <div className="text-[#71767b] text-sm mt-1">5.8K posts</div>
            </div>
          </>
        )}

        {activeTab === 'issues' && (
          <>
            <div className="bg-[#16181c] rounded-lg p-4 border-l-4 border-[#3fb950]">
              <div className="flex items-center gap-2 mb-3">
                <span className="bg-[#3fb950] text-white text-xs font-bold px-2 py-1 rounded">Good First Issue</span>
                <span className="text-[#71767b] text-sm">BlockSmith/BlockSmith</span>
              </div>
              <h3 className="font-bold text-base">Add Clerk authentication middleware</h3>
              <p className="text-[#71767b] text-sm mt-2">We need to authenticate user endpoints using Clerk JWKS key validation.</p>
              <div className="flex items-center gap-4 mt-3 text-sm">
                <span className="text-[#3fb950]">AI Match: 92%</span>
                <span className="text-[#71767b]">Easy</span>
              </div>
            </div>
            <div className="bg-[#16181c] rounded-lg p-4 border-l-4 border-[#f85149]">
              <div className="flex items-center gap-2 mb-3">
                <span className="bg-[#f85149] text-white text-xs font-bold px-2 py-1 rounded">Bug</span>
                <span className="text-[#71767b] text-sm">BlockSmith/BlockSmith</span>
              </div>
              <h3 className="font-bold text-base">WebSocket disconnects on network change</h3>
              <p className="text-[#71767b] text-sm mt-2">Users are experiencing WebSocket disconnections when switching networks.</p>
              <div className="flex items-center gap-4 mt-3 text-sm">
                <span className="text-[#3fb950]">AI Match: 78%</span>
                <span className="text-[#71767b]">Medium</span>
              </div>
            </div>
          </>
        )}

        {activeTab === 'developers' && (
          <>
            <div className="flex items-center gap-4 bg-[#16181c] rounded-lg p-4">
              <div className="w-12 h-12 bg-[#333639] rounded-full flex items-center justify-center text-[#e7e9ea] font-bold">NJ</div>
              <div className="flex-1">
                <div className="font-bold text-base">Nirvan Jain</div>
                <div className="text-[#71767b] text-sm mt-0.5">@NirvanJain · Rust, TypeScript</div>
              </div>
              <button className="bg-[#eff3f4] text-[#0f1419] font-bold text-sm px-4 py-2 rounded-full hover:bg-[#d7dbdc]">
                Follow
              </button>
            </div>
            <div className="flex items-center gap-4 bg-[#16181c] rounded-lg p-4">
              <div className="w-12 h-12 bg-[#333639] rounded-full flex items-center justify-center text-[#e7e9ea] font-bold">SC</div>
              <div className="flex-1">
                <div className="font-bold text-base">Sarah Chen</div>
                <div className="text-[#71767b] text-sm mt-0.5">@sarahchen · Go, React</div>
              </div>
              <button className="bg-[#eff3f4] text-[#0f1419] font-bold text-sm px-4 py-2 rounded-full hover:bg-[#d7dbdc]">
                Follow
              </button>
            </div>
          </>
        )}
      </div>
    </div>
  )
}
