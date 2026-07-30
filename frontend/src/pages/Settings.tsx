import { useState } from 'react'

export default function Settings() {
  const [activeTab, setActiveTab] = useState<'account' | 'privacy' | 'notifications'>('account')

  return (
    <div>
      <div className="sticky top-0 z-10 bg-[rgba(0,0,0,0.65)] backdrop-blur-md border-b border-[#2f3336] px-4 py-4">
        <h1 className="font-bold text-lg">Settings</h1>
      </div>

      <div className="flex border-b border-[#2f3336]">
        {(['account', 'privacy', 'notifications'] as const).map((tab) => (
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
      
      <div className="p-4 space-y-6">
        {activeTab === 'account' && (
          <>
            <div>
              <label className="block text-[#71767b] text-sm mb-2">Name</label>
              <input
                type="text"
                defaultValue="Nirvan Jain"
                className="w-full bg-[#202327] text-[#e7e9ea] py-3 px-4 rounded text-sm outline-none focus:ring-1 focus:ring-[#1d9bf0]"
              />
            </div>
            <div>
              <label className="block text-[#71767b] text-sm mb-2">Bio</label>
              <textarea
                defaultValue="Building the future of open source developer reputation."
                className="w-full bg-[#202327] text-[#e7e9ea] py-3 px-4 rounded text-sm outline-none focus:ring-1 focus:ring-[#1d9bf0] resize-none h-24"
              />
            </div>
            <div>
              <label className="block text-[#71767b] text-sm mb-2">Location</label>
              <input
                type="text"
                defaultValue="San Francisco, CA"
                className="w-full bg-[#202327] text-[#e7e9ea] py-3 px-4 rounded text-sm outline-none focus:ring-1 focus:ring-[#1d9bf0]"
              />
            </div>
            <div>
              <label className="block text-[#71767b] text-sm mb-2">Website</label>
              <input
                type="url"
                defaultValue="https://github.com/NirvanJain"
                className="w-full bg-[#202327] text-[#e7e9ea] py-3 px-4 rounded text-sm outline-none focus:ring-1 focus:ring-[#1d9bf0]"
              />
            </div>
            <button className="bg-[#1d9bf0] hover:bg-[#1a8cd8] text-white font-bold text-sm py-3 px-6 rounded-full">
              Save changes
            </button>
          </>
        )}

        {activeTab === 'privacy' && (
          <div className="space-y-4">
            <div className="bg-[#16181c] rounded-lg p-4">
              <div className="flex items-center justify-between">
                <div>
                  <div className="font-bold text-sm">Protect your posts</div>
                  <div className="text-[#71767b] text-sm mt-1">Only approved followers can see your posts</div>
                </div>
                <button className="w-12 h-6 bg-[#1d9bf0] rounded-full relative">
                  <span className="absolute right-1 top-1 w-4 h-4 bg-white rounded-full" />
                </button>
              </div>
            </div>
            <div className="bg-[#16181c] rounded-lg p-4">
              <div className="flex items-center justify-between">
                <div>
                  <div className="font-bold text-sm">Allow photo tags</div>
                  <div className="text-[#71767b] text-sm mt-1">Allow anyone to tag you in photos</div>
                </div>
                <button className="w-12 h-6 bg-[#333639] rounded-full relative">
                  <span className="absolute left-1 top-1 w-4 h-4 bg-[#71767b] rounded-full" />
                </button>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'notifications' && (
          <div className="space-y-4">
            <div className="bg-[#16181c] rounded-lg p-4">
              <div className="flex items-center justify-between">
                <div>
                  <div className="font-bold text-sm">Push notifications</div>
                  <div className="text-[#71767b] text-sm mt-1">Receive push notifications for activity</div>
                </div>
                <button className="w-12 h-6 bg-[#1d9bf0] rounded-full relative">
                  <span className="absolute right-1 top-1 w-4 h-4 bg-white rounded-full" />
                </button>
              </div>
            </div>
            <div className="bg-[#16181c] rounded-lg p-4">
              <div className="flex items-center justify-between">
                <div>
                  <div className="font-bold text-sm">Email notifications</div>
                  <div className="text-[#71767b] text-sm mt-1">Receive email for important updates</div>
                </div>
                <button className="w-12 h-6 bg-[#1d9bf0] rounded-full relative">
                  <span className="absolute right-1 top-1 w-4 h-4 bg-white rounded-full" />
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}
