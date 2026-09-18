import { NavLink, Outlet } from 'react-router-dom'
import Sidebar from './Sidebar'

const mobileNavItems = [
  { name: 'Home', path: '/' },
  { name: 'Explore', path: '/explore' },
  { name: 'Messages', path: '/messages' },
  { name: 'Profile', path: '/profile/nirvanjain' },
]

export default function Layout() {
  return (
      <div className="flex min-h-screen max-w-[1200px] mx-auto">
        {/* Left Sidebar */}
        <aside className="w-64 flex-shrink-0 border-r border-[#2f3336] hidden md:block">
          <Sidebar />
        </aside>

        {/* Main Content */}
        <main className="flex-1 min-h-screen border-r border-[#2f3336] pb-16 md:pb-0">
      <Outlet />
    </main>
    <nav
      aria-label="Mobile navigation"
      className="fixed inset-x-0 bottom-0 z-20 flex border-t border-[#2f3336] bg-black/95 backdrop-blur-md md:hidden"
    >
      {mobileNavItems.map((item) => (
        <NavLink
          key={item.name}
          to={item.path}
          className={({ isActive }) =>
            `flex-1 px-2 py-3 text-center text-xs transition-colors ${
              isActive ? 'font-bold text-[#1d9bf0]' : 'text-[#71767b] hover:text-[#e7e9ea]'
            }`
          }
        >
          {item.name}
        </NavLink>
      ))}
    </nav>
  </div>
  )
}
