import type { Block } from "../types";
import BlockCard from "./BlockCard";

interface ChainExplorerProps {
  blocks: Block[];
  loading: boolean;
  onRefresh: () => void;
}

export default function ChainExplorer({ blocks, loading, onRefresh }: ChainExplorerProps) {
  return (
    <div>
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          marginBottom: "24px",
        }}
      >
        <div>
          <div className="content-title">Block Explorer</div>
          <div className="content-subtitle">
            Full tamper-resistant contribution chain — click any block to inspect hashes.
          </div>
        </div>
        <div style={{ display: "flex", gap: "10px", alignItems: "center" }}>
          <span style={{ fontSize: "10px", color: "var(--text-dim)", textTransform: "uppercase", letterSpacing: "0.1em" }}>
            {blocks.length} block{blocks.length !== 1 ? "s" : ""}
          </span>
          <button
            id="btn-refresh-chain"
            className="btn btn-ghost"
            onClick={onRefresh}
            disabled={loading}
          >
            {loading ? <span className="spinner" /> : "Refresh"}
          </button>
        </div>
      </div>

      {/* Legend */}
      <div
        style={{
          display: "flex",
          gap: "20px",
          marginBottom: "18px",
          fontSize: "9px",
          textTransform: "uppercase",
          letterSpacing: "0.1em",
          color: "var(--text-dim)",
        }}
      >
        <span><span className="block-type-tag tag-pr" style={{ border: "none", padding: 0, fontSize: "9px" }}>PULL_REQ</span> = 10 pts</span>
        <span><span className="block-type-tag tag-issue" style={{ border: "none", padding: 0, fontSize: "9px" }}>ISSUE</span> = 5 pts</span>
        <span><span className="block-type-tag tag-commit" style={{ border: "none", padding: 0, fontSize: "9px" }}>COMMIT</span> = 3 pts</span>
      </div>

      {loading && blocks.length === 0 ? (
        <div className="empty-state">
          <div className="empty-state-icon">⠋</div>
          <div className="empty-state-msg cursor-blink">Loading chain</div>
        </div>
      ) : blocks.length === 0 ? (
        <div className="empty-state">
          <div className="empty-state-icon" style={{ fontSize: "28px" }}>◻</div>
          <div className="empty-state-msg">No blocks found — genesis block only</div>
        </div>
      ) : (
        <div className="chain-container">
          {[...blocks].reverse().map((block, i) => (
            <div key={block.hash + i}>
              <BlockCard block={block} />
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
