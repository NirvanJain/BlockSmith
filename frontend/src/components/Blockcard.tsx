import { useState } from "react";
import type { Block } from "../types";

interface BlockCardProps {
  block: Block;
}

const typeTag = (type: string) => {
  if (type === "pull_request") return { label: "PULL_REQ", cls: "tag-pr" };
  if (type === "commit") return { label: "COMMIT", cls: "tag-commit" };
  if (type === "issue") return { label: "ISSUE", cls: "tag-issue" };
  return { label: type.toUpperCase(), cls: "tag-genesis" };
};

const formatTime = (ts: string) => {
  try {
    const d = new Date(ts);
    return d.toISOString().replace("T", " ").slice(0, 19) + " UTC";
  } catch {
    return ts;
  }
};

export default function BlockCard({ block }: BlockCardProps) {
  const [open, setOpen] = useState(false);
  const tag = typeTag(block.contribution_type);

  return (
    <div className="block-card">
      <div
        className="block-header"
        onClick={() => setOpen((o) => !o)}
        role="button"
        aria-expanded={open}
        id={`block-header-${block.index}`}
      >
        <div className="block-index">
          <span className="block-index-num">#{String(block.index).padStart(4, "0")}</span>
          <span>{block.contributor || "blocksmith"}</span>
          <span className={`block-type-tag ${tag.cls}`}>{tag.label}</span>
        </div>
        <div className="block-timestamp">
          <span>{formatTime(block.timestamp)}</span>
          <span style={{ color: "var(--text-muted)", fontSize: "11px" }}>
            {open ? "▲" : "▼"}
          </span>
        </div>
      </div>

      <div className={`block-body ${open ? "open" : ""}`}>
        <div className="block-field">
          <div className="block-field-key">Repository</div>
          <div className="block-field-val">{block.repository || "—"}</div>
        </div>
        <div className="block-field">
          <div className="block-field-key">Contributor</div>
          <div className="block-field-val">{block.contributor || "—"}</div>
        </div>
        <div className="block-field">
          <div className="block-field-key">Type</div>
          <div className="block-field-val">
            <span className={`block-type-tag ${tag.cls}`}>{tag.label}</span>
          </div>
        </div>
        {block.contribution_link && block.contribution_link !== "None" && (
          <div className="block-field">
            <div className="block-field-key">Link</div>
            <div className="block-field-val link-val">
              <a href={block.contribution_link} target="_blank" rel="noreferrer">
                {block.contribution_link}
              </a>
            </div>
          </div>
        )}
        <div className="block-field">
          <div className="block-field-key">Hash</div>
          <div className="block-field-val hash-val">{block.hash}</div>
        </div>
        <div className="block-field">
          <div className="block-field-key">Prev Hash</div>
          <div className="block-field-val prev-hash-val">{block.previous_hash}</div>
        </div>
      </div>
    </div>
  );
}