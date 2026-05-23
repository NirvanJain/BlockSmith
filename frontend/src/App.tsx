import { useEffect, useState } from "react";
import axios from "axios";

interface Block {
  index: number;
  timestamp: string;
  data: string;
  previous_hash: string;
  hash: string;
}

function App() {
  const [blocks, setBlocks] = useState<Block[]>([]);
  const [data, setData] = useState("");
  const [isValid, setIsValid] = useState<boolean | null>(
    null
  );
  const [loading, setLoading] = useState(false);

  const API = "http://localhost:3000";

  // Fetch blockchain
  const fetchBlocks = async () => {
    try {
      const res = await axios.get(
        `${API}/blocks`
      );

      setBlocks(res.data);
    } catch (error) {
      console.error(error);
    }
  };

  // Validate blockchain
  const validateChain = async () => {
    try {
      const res = await axios.get(
        `${API}/validate`
      );

      setIsValid(res.data.valid);
    } catch (error) {
      console.error(error);
    }
  };

  // Add block
  const addBlock = async () => {
    if (!data.trim()) return;

    try {
      setLoading(true);

      await axios.post(`${API}/add`, {
        data,
      });

      setData("");

      await fetchBlocks();
      await validateChain();
    } catch (error) {
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

useEffect(() => {
  const loadData = async () => {
    await fetchBlocks();
    await validateChain();
  };
  loadData();
}, []);

  return (
    <div className="min-h-screen bg-black text-white p-8">
      {/* Header */}
      <div className="mb-10">
        <h1 className="text-5xl font-bold">
          BlockSmith
        </h1>

        <p className="text-gray-400 mt-2">
          Crafting the chain, one block at a
          time.
        </p>
      </div>

      {/* Validation Status */}
      <div className="mb-8">
        <div className="inline-flex items-center gap-3 border border-gray-700 px-5 py-3 rounded-xl">
          <span className="font-semibold">
            Blockchain Status:
          </span>

          <span
            className={`font-bold ${
              isValid
                ? "text-green-400"
                : "text-red-400"
            }`}
          >
            {isValid ? "VALID" : "INVALID"}
          </span>
        </div>
      </div>

      {/* Add Block */}
      <div className="flex gap-4 mb-10">
        <input
          type="text"
          placeholder="Enter block data..."
          value={data}
          onChange={(e) =>
            setData(e.target.value)
          }
          className="flex-1 bg-gray-900 border border-gray-700 rounded-xl px-5 py-4 outline-none"
        />

        <button
          onClick={addBlock}
          disabled={loading}
          className="bg-green-500 hover:bg-green-600 transition px-6 py-4 rounded-xl font-semibold"
        >
          {loading ? "Adding..." : "Add Block"}
        </button>
      </div>

      {/* Blockchain */}
      <div className="grid gap-6">
        {blocks.map((block) => (
          <div
            key={block.hash}
            className="border border-gray-800 bg-gray-950 rounded-2xl p-6"
          >
            <div className="flex justify-between mb-4">
              <h2 className="text-2xl font-bold">
                Block #{block.index}
              </h2>

              <span className="text-sm text-gray-400">
                {new Date(
                  block.timestamp
                ).toLocaleString()}
              </span>
            </div>

            <div className="space-y-4">
              <div>
                <p className="text-gray-400 text-sm">
                  Data
                </p>

                <p className="text-lg">
                  {block.data}
                </p>
              </div>

              <div>
                <p className="text-gray-400 text-sm">
                  Hash
                </p>

                <p className="break-all text-green-400 text-sm">
                  {block.hash}
                </p>
              </div>

              <div>
                <p className="text-gray-400 text-sm">
                  Previous Hash
                </p>

                <p className="break-all text-yellow-400 text-sm">
                  {block.previous_hash}
                </p>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;