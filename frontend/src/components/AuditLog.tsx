import type { LogEntry } from "../types";

interface AuditLogProps {
  logs: LogEntry[];
}

const LEVEL_LABELS: Record<LogEntry["level"], string> = {
  info: "[INFO]",
  ok: "[OK]  ",
  warn: "[WARN]",
  err: "[ERR] ",
};

export default function AuditLog({ logs }: AuditLogProps) {
  return (
    <div>
      <div style={{ marginBottom: "24px" }}>
        <div className="content-title">Audit Log</div>
        <div className="content-subtitle">
          Immutable record of all system events — block creations, verifications, auth, and errors.
        </div>
      </div>

      <div className="terminal-log" style={{ maxHeight: "none" }}>
        <div className="terminal-log-header">
          <span className="status-dot" />
          /var/log/blocksmith/audit.log — {logs.length} entries
        </div>
        <div className="terminal-log-body" style={{ gap: "2px" }}>
          {logs.length === 0 ? (
            <div className="log-entry">
              <span className="log-time">--:--:--</span>
              <span className="log-prefix info">[INFO]</span>
              <span className="log-msg">No audit events recorded yet.</span>
            </div>
          ) : (
            logs.map((log) => (
              <div key={log.id} className="log-entry" id={`audit-entry-${log.id}`}>
                <span className="log-time">{log.time}</span>
                <span className={`log-prefix ${log.level}`}>
                  {LEVEL_LABELS[log.level]}
                </span>
                <span className="log-msg">{log.message}</span>
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
}
