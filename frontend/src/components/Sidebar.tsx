import { NavLink } from 'react-router-dom'

const navItems = [
  { name: 'Home', path: '/' },
  { name: 'Explore', path: '/explore' },
  { name: 'Notifications', path: '/notifications' },
  { name: 'Messages', path: '/messages' },
  { name: 'Profile', path: '/profile/nirvanjain' },
  { name: 'Settings', path: '/settings' },
]

export default function Sidebar() {
  return (
    <div className="fixed h-screen w-64 flex flex-col py-6 px-4">
      {/* Logo */}
      <NavLink to="/" className="text-2xl font-bold text-[#1d9bf0] mb-8">
        BlockSmith
      </NavLink>

      {/* Navigation */}
      <nav className="space-y-2">
        {navItems.map((item) => (
          <NavLink
            key={item.name}
            to={item.path}
            className={({ isActive }) =>
              `block px-4 py-3 rounded-lg text-base transition-colors ${
                isActive
                  ? 'font-bold bg-[rgba(29,155,240,0.1)] text-[#1d9bf0]'
                  : 'text-[#e7e9ea] hover:bg-[rgba(231,233,234,0.1)]'
              }`
            }
          >
            {item.name}
          </NavLink>
        ))}
      </nav>

      {/* Post Button */}
      <button className="mt-8 w-full bg-[#1d9bf0] hover:bg-[#1a8cd8] text-white font-bold py-3 px-4 rounded-full transition-colors">
        New Post
      </button>

      {/* User Info */}
      <div className="mt-auto flex items-center gap-3 p-3 rounded-lg hover:bg-[rgba(231,233,234,0.1)] transition-colors cursor-pointer">
        <div className="w-10 h-10 bg-[#333639] rounded-full flex items-center justify-center text-[#e7e9ea] font-bold">
          NJ
        </div>
        <div className="flex-1 min-w-0">
          <div className="font-bold text-sm text-[#e7e9ea]">Nirvan Jain</div>
          <div className="text-[#71767b] text-sm">@NirvanJain</div>
        </div>
      </div>
    </div>
  )
}
