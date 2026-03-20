#!/usr/bin/env python3
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXAMPLES = ROOT / "examples"


def send(msg):
    print(json.dumps(msg), flush=True)


def ok(req_id, result):
    send({"jsonrpc": "2.0", "id": req_id, "result": result})


def err(req_id, code, message):
    send({"jsonrpc": "2.0", "id": req_id, "error": {"code": code, "message": message}})


def list_examples():
    return sorted([p.name for p in EXAMPLES.glob("*.bas")])


def list_program(path):
    p = (ROOT / path).resolve()
    if not str(p).startswith(str(ROOT)):
        raise ValueError("Path escapes project root")
    if not p.exists():
        raise FileNotFoundError(path)
    lines = p.read_text().splitlines()
    return [{"line": i + 1, "text": t} for i, t in enumerate(lines)]


def run_program(path, inputs=None, timeout=180):
    cmd = [
        "cargo",
        "run",
        "--manifest-path",
        str(ROOT / "Cargo.toml"),
        "--",
        str(path),
    ]
    for val in inputs or []:
        cmd.extend(["--input", str(val)])
    proc = subprocess.run(
        cmd,
        cwd=str(ROOT.parent),
        capture_output=True,
        text=True,
        timeout=timeout,
    )
    return {
        "exit_code": proc.returncode,
        "stdout_tail": "\n".join(proc.stdout.splitlines()[-60:]),
        "stderr_tail": "\n".join(proc.stderr.splitlines()[-60:]),
        "command": " ".join(cmd),
    }


def tools_list():
    return {
        "tools": [
            {
                "name": "list_examples",
                "description": "List BASIC example programs",
                "inputSchema": {"type": "object", "properties": {}},
            },
            {
                "name": "list_program",
                "description": "List a BASIC program with line numbers",
                "inputSchema": {
                    "type": "object",
                    "properties": {"path": {"type": "string"}},
                    "required": ["path"],
                },
            },
            {
                "name": "run_program",
                "description": "Run a BASIC program through rust-gwbasic",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "path": {"type": "string"},
                        "inputs": {"type": "array", "items": {"type": "string"}},
                        "timeout": {"type": "integer"},
                    },
                    "required": ["path"],
                },
            },
        ]
    }


def handle(req):
    req_id = req.get("id")
    method = req.get("method")
    params = req.get("params", {})
    try:
        if method == "initialize":
            ok(
                req_id,
                {
                    "protocolVersion": "2024-11-05",
                    "serverInfo": {"name": "gwbasic-mcp", "version": "0.1.0"},
                    "capabilities": {"tools": {}},
                },
            )
        elif method == "notifications/initialized":
            return
        elif method == "tools/list":
            ok(req_id, tools_list())
        elif method == "tools/call":
            name = params.get("name")
            args = params.get("arguments", {})
            if name == "list_examples":
                out = {"examples": list_examples()}
            elif name == "list_program":
                out = {"lines": list_program(args["path"])}
            elif name == "run_program":
                out = run_program(args["path"], args.get("inputs"), args.get("timeout", 180))
            else:
                raise ValueError(f"Unknown tool: {name}")
            ok(req_id, {"content": [{"type": "text", "text": json.dumps(out, indent=2)}]})
        else:
            err(req_id, -32601, f"Method not found: {method}")
    except Exception as e:
        err(req_id, -32000, str(e))


def main():
    import sys

    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            req = json.loads(line)
        except Exception:
            continue
        handle(req)


if __name__ == "__main__":
    main()
