.PHONY: help build test hires run-hires run-hires-text run-hires-gui run-hires-gui-x11 run-hello run-loops run-load mcp-demo validate-demo

help:
	@echo "Targets:"
	@echo "  build         Build rust-gwbasic"
	@echo "  test          Run focused parser/interpreter tests"
	@echo "  hires/run-hires Run mandelbrot-hires.bas in GUI mode"
	@echo "  run-hires-text Run mandelbrot-hires.bas in text mode"
	@echo "  run-hires-gui Run mandelbrot-hires.bas with auto GUI backend selection"
	@echo "  run-hires-gui-x11 Run mandelbrot-hires.bas with GUI forced to X11"
	@echo "  run-load      Run load_runner.bas (LOAD + RUN flow)"
	@echo "  run-hello     Run hello.bas"
	@echo "  run-loops     Run loops.bas"
	@echo "  mcp-demo      Exercise MCP initialize/list/call flow"
	@echo "  validate-demo Run build, test, program runs, and MCP demo"

build:
	cargo build --manifest-path rust-gwbasic/Cargo.toml

test:
	cargo test --manifest-path rust-gwbasic/Cargo.toml test_parse_load_statement
	cargo test --manifest-path rust-gwbasic/Cargo.toml test_parse_open_and_close
	cargo test --manifest-path rust-gwbasic/Cargo.toml test_load_program_replaces_lines
	cargo test --manifest-path rust-gwbasic/Cargo.toml test_save_then_load_roundtrip

run-hires:
	@$(MAKE) run-hires-gui

hires: run-hires

run-hires-text:
	cargo run --manifest-path rust-gwbasic/Cargo.toml -- rust-gwbasic/examples/mandelbrot-hires.bas --input 16

run-hires-quality:
	cargo run --manifest-path rust-gwbasic/Cargo.toml -- rust-gwbasic/examples/mandelbrot-hires.bas --input 64

run-hires-gui:
	@bash -lc 'set -euo pipefail; \
	if [ -n "$${DISPLAY:-}" ]; then \
	  echo "Using X11 display: $$DISPLAY"; \
	  WAYLAND_DISPLAY= XDG_SESSION_TYPE=x11 cargo run --manifest-path rust-gwbasic/Cargo.toml -- --gui rust-gwbasic/examples/mandelbrot-hires.bas --input 8; \
	elif [ -n "$${WAYLAND_DISPLAY:-}" ]; then \
	  echo "Using Wayland display: $$WAYLAND_DISPLAY"; \
	  cargo run --manifest-path rust-gwbasic/Cargo.toml -- --gui rust-gwbasic/examples/mandelbrot-hires.bas --input 8; \
	else \
	  echo "ERROR: no GUI display detected (DISPLAY/WAYLAND_DISPLAY unset)." >&2; \
	  exit 2; \
	fi'

run-hires-gui-x11:
	WAYLAND_DISPLAY= XDG_SESSION_TYPE=x11 cargo run --manifest-path rust-gwbasic/Cargo.toml -- --gui rust-gwbasic/examples/mandelbrot-hires.bas --input 8

run-load:
	cd rust-gwbasic && cargo run -- examples/load_runner.bas --input 16

run-hello:
	cargo run --manifest-path rust-gwbasic/Cargo.toml -- rust-gwbasic/examples/hello.bas

run-loops:
	cargo run --manifest-path rust-gwbasic/Cargo.toml -- rust-gwbasic/examples/loops.bas

mcp-demo:
	printf '%s\n' \
	'{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' \
	'{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}' \
	'{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' \
	'{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"list_examples","arguments":{}}}' \
	'{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"list_program","arguments":{"path":"examples/mandelbrot-hires.bas"}}}' \
	'{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"run_program","arguments":{"path":"rust-gwbasic/examples/mandelbrot-hires.bas","inputs":["16"],"timeout":240}}}' \
	'{"jsonrpc":"2.0","id":6,"method":"tools/call","params":{"name":"run_program","arguments":{"path":"rust-gwbasic/examples/hello.bas","timeout":60}}}' \
	'{"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"run_program","arguments":{"path":"rust-gwbasic/examples/loops.bas","timeout":60}}}' \
	| python rust-gwbasic/tools/mcp_server.py

validate-demo: build test run-hires run-load run-hello run-loops mcp-demo
