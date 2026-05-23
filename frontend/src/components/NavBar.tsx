function Navbar() {
  return (
    <nav className="w-full border-b border-gray-800 bg-black px-8 py-5 mb-10">
      <div className="flex items-center justify-between">
        {/* Logo */}
        <div>
          <h1 className="text-3xl font-bold text-white">
            BlockSmith
          </h1>

          <p className="text-sm text-gray-400">
            Crafting the chain, one block at a
            time.
          </p>
        </div>

        {/* Status */}
        <div className="hidden md:flex items-center gap-3">
          <div className="w-3 h-3 rounded-full bg-green-400 animate-pulse"></div>

          <span className="text-gray-300 text-sm">
            Blockchain Network Active
          </span>
        </div>
      </div>
    </nav>
  );
}

export default Navbar;