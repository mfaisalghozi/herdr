#!/usr/bin/env bash
# Build the sidebar-sections fork, install it, and live-handoff the running server.
set -euo pipefail
cd "$(dirname "$0")/.."

export PATH="$HOME/.local/share/mise/installs/zig/0.16.0:$HOME/.cargo/bin:$PATH"
export ZIG="${ZIG:-$HOME/.local/share/mise/installs/zig/0.16.0/zig}"

if [[ "${1:-}" != "--no-test" ]]; then
  cargo test --release --locked --bin herdr client::shell
fi

cargo build --release --locked

install -m 755 target/release/herdr "$HOME/.local/bin/herdr.new"
mv "$HOME/.local/bin/herdr.new" "$HOME/.local/bin/herdr"
echo "installed $(herdr --version) at $HOME/.local/bin/herdr"

socket="${HERDR_SOCKET_PATH:-$HOME/.config/herdr/herdr.sock}"
if [[ -S "$socket" ]]; then
  /usr/bin/python3 - "$socket" "$HOME/.local/bin/herdr" <<'PY'
import json
import os
import socket
import sys

sock_path, exe = sys.argv[1], sys.argv[2]
request = {
    "id": "reinstall-fork",
    "method": "server.live_handoff",
    "params": {"import_exe": exe},
}
client = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
client.settimeout(60)
client.connect(sock_path)
client.sendall((json.dumps(request) + "\n").encode())
response = b""
while True:
    try:
        chunk = client.recv(4096)
    except socket.timeout:
        break
    if not chunk:
        break
    response += chunk
client.close()
print(response.decode("utf-8", "replace").strip() or "(no response)")
PY
fi

# Re-send the servers section so a freshly handed-off server gets the current
# `row_actions` payload instead of the watcher's cached copy.
plugin_report="$HOME/.config/herdr/plugins/local/server-panel/scripts/report_servers.py"
if [[ -S "$socket" && -f "$plugin_report" ]]; then
  HERDR_SOCKET_PATH="$socket" HERDR_SERVER_PANEL_WATCH=0 \
    /usr/bin/python3 "$plugin_report" >/dev/null 2>&1 || true
fi

echo
echo "Reload the running client with: ctrl+b q, then herdr"
