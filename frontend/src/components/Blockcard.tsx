import type { Block } from "../types/block";

interface BlockCardProps {
  block: Block;
}

function BlockCard({
  block,
}: BlockCardProps) {
  return (
    <div className="border border-gray-800 bg-gray-950 rounded-2xl p-6">
      {/* Header */}
      <div className="flex justify-between items-center mb-5">
        <h2 className="text-2xl font-bold">
          Block #{block.index}
        </h2>

        <span className="text-sm text-gray-400">
          {new Date(
            block.timestamp
          ).toLocaleString()}
        </span>
      </div>

      {/* Data */}
      <div className="mb-5">
        <p className="text-gray-400 text-sm mb-1">
          Data
        </p>

        <p className="text-lg text-white">
          {block.data}
        </p>
      </div>

      {/* Hash */}
      <div className="mb-5">
        <p className="text-gray-400 text-sm mb-1">
          Hash
        </p>

        <p className="break-all text-green-400 text-sm">
          {block.hash}
        </p>
      </div>

      {/* Previous Hash */}
      <div>
        <p className="text-gray-400 text-sm mb-1">
          Previous Hash
        </p>

        <p className="break-all text-yellow-400 text-sm">
          {block.previous_hash}
        </p>
      </div>
    </div>
  );
}

export default BlockCard;