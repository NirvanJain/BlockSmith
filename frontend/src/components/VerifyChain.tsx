import type { Block } from "../types";

interface VerifyChainProps {
  blocks: Block[];
  chainValid: boolean | null;
  onVerify: () => void;
  verifying: boolean;
}

export default function VerifyChain({
  blocks,
  chainValid,
  onVerify,
  verifying,
}: VerifyChainProps) {
  return (
    <div>
      <div style={{ marginBottom: "28px" }}>
        <div className="content-title">Verify Chain Integrity</div>
        <div className="content-subtitle">
          Re-compute and validate all block hashes from genesis to tip.<br />
          Any tampered block will break the hash linkage and flag the chain as invalid.
        </div>
      </div>

      {/* Status panel */}
      <div
        style={{
          border: `1px solid ${
            chainValid === null
              ? "var(--border)"
              : chainValid
              ? "var(--accent-green)"
              : "var(--accent-red)"
          }`,
          background: "var(--bg-card)",
          padding: "32px",
          marginBottom: "28px",
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          gap: "16px",
          textAlign: "center",
        }}
      >
        <div
          style={{
            fontSize: "48px",
            fontWeight: 900,
            letterSpacing: "-0.02em",
            color:
              chainValid === null
                ? "var(--text-dim)"
                : chainValid
                ? "var(--accent-green)"
                : "var(--accent-red)",
          }}
        >
          {chainValid === null ? "—" : chainValid ? "VALID" : "INVALID"}
        </div>
        <div
          style={{
            fontSize: "10px",
            textTransform: "uppercase",
            letterSpacing: "0.14em",
            color: "var(--text-dim)",
          }}
        >
          {chainValid === null
            ? "Run verification to check chain integrity"
            : chainValid
            ? `All ${blocks.length} blocks verified — no tampering detected`
            : "Hash mismatch detected — chain integrity compromised"}
        </div>
        <button
          id="btn-verify-chain"
          className="btn btn-primary"
          onClick={onVerify}
          disabled={verifying}
          style={{ marginTop: "8px" }}
        >
          {verifying ? (
            <>
              <span className="spinner" />
              Validating Hashes...
            </>
          ) : (
            "Run Chain Validation"
          )}
        </button>
      </div>

      {/* Algorithm explanation */}
      <div className="section-title">Validation Algorithm</div>
      <div
        style={{
          border: "1px solid var(--border)",
          background: "var(--bg-card)",
          padding: "20px",
          fontFamily: "var(--font-mono)",
          fontSize: "10px",
          color: "var(--text-dim)",
          lineHeight: "2",
        }}
      >
        <div style={{ color: "var(--accent-green)", marginBottom: "8px" }}>
          $ blocksmith validate --chain all
        </div>
        <div>for block in chain[1..] :</div>
        <div style={{ paddingLeft: "24px" }}>assert block.previous_hash == chain[i-1].hash</div>
        <div style={{ paddingLeft: "24px" }}>assert block.hash == SHA256(index|ts|contributor|repo|type|link|prev_hash)</div>
        <div style={{ paddingLeft: "24px", color: "var(--accent-green)" }}>→ PASS or FAIL</div>
      </div>

      {/* Block inventory */}
      {blocks.length > 0 && (
        <>
          <div className="section-title" style={{ marginTop: "24px" }}>
            Block Inventory ({blocks.length})
          </div>
          <div className="table-wrapper">
            <table className="bs-table" id="verify-block-table">
              <thead>
                <tr>
                  <th>Index</th>
                  <th>Contributor</th>
                  <th>Repository</th>
                  <th>Hash (truncated)</th>
                  <th>Prev Hash (truncated)</th>
                  <th>Status</th>
                </tr>
              </thead>
              <tbody>
                {blocks.map((block) => (
                  <tr key={block.hash}>
                    <td style={{ color: "var(--accent-green)", fontWeight: 700 }}>
                      #{String(block.index).padStart(4, "0")}
                    </td>
                    <td>{block.contributor || "blocksmith"}</td>
                    <td style={{ color: "var(--text-dim)" }}>{block.repository || "—"}</td>
                    <td style={{ color: "var(--accent-green)", fontSize: "10px" }}>
                      {block.hash.slice(0, 16)}...
                    </td>
                    <td style={{ color: "var(--accent-amber)", fontSize: "10px" }}>
                      {block.previous_hash.slice(0, 16)}...
                    </td>
                    <td>
                      <span
                        style={{
                          fontSize: "9px",
                          textTransform: "uppercase",
                          letterSpacing: "0.1em",
                          color:
                            chainValid === null
                              ? "var(--text-dim)"
                              : chainValid
                              ? "var(--accent-green)"
                              : "var(--accent-red)",
                        }}
                      >
                        {chainValid === null ? "UNCHECKED" : chainValid ? "OK" : "ERR"}
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </>
      )}
    </div>
  );
}
