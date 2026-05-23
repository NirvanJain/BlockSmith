interface AddBlockFormProps {
  data: string;
  setData: React.Dispatch<
    React.SetStateAction<string>
  >;
  addBlock: () => void;
  loading: boolean;
}

function AddBlockForm({
  data,
  setData,
  addBlock,
  loading,
}: AddBlockFormProps) {
  return (
    <div className="flex gap-4 mb-10">
      <input
        type="text"
        placeholder="Enter block data..."
        value={data}
        onChange={(e) =>
          setData(e.target.value)
        }
        className="flex-1 bg-gray-900 border border-gray-700 rounded-xl px-5 py-4 outline-none text-white"
      />

      <button
        onClick={addBlock}
        disabled={loading}
        className="bg-green-500 hover:bg-green-600 transition px-6 py-4 rounded-xl font-semibold"
      >
        {loading ? "Adding..." : "Add Block"}
      </button>
    </div>
  );
}

export default AddBlockForm;