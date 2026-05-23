interface ChainStatusProps {
  isValid: boolean | null;
}

function ChainStatus({
  isValid,
}: ChainStatusProps) {
  return (
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
  );
}

export default ChainStatus;