import { Outlet } from 'react-router-dom'
import Sidebar from './Sidebar'

export default function Layout() {
  return (
      <div className="flex min-h-screen max-w-[1200px] mx-auto">
        {/* Left Sidebar */}
        <aside className="w-64 flex-shrink-0 border-r border-[#2f3336] hidden md:block">
          <Sidebar />
        </aside>

        {/* Main Content */}
        <main className="flex-1 min-h-screen border-r border-[#2f3336]">
      <Outlet />
    </main>
  </div>
  )
}
