export default function Lists() {
  const lists = [
    { id: 1, name: 'Rust Contributors', members: 24 },
    { id: 2, name: 'BlockSmith Team', members: 8 },
    { id: 3, name: 'Open Source Mentors', members: 15 },
  ]

  return (
    <div>
      <div className="sticky top-0 z-10 bg-[rgba(0,0,0,0.65)] backdrop-blur-md border-b border-[#2f3336] px-4 py-3">
        <h1 className="font-bold">Lists</h1>
      </div>
      <div>
        {lists.map((list) => (
          <div key={list.id} className="p-4 border-b border-[#2f3336] hover:bg-[rgba(231,233,234,0.03)]">
            <h3 className="font-bold text-sm">{list.name}</h3>
            <div className="text-[#71767b] text-xs mt-1">{list.members} members</div>
          </div>
        ))}
      </div>
    </div>
  )
}
