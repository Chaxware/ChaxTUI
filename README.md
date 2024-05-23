# chax-tui

A TUI (Terminal User Interface) for the Chax app made in Rust using [Ratatui](https://github.com/ratatui-org/ratatui)

## Build and run

_Prerequisites:_ `cargo`, `npm`

### Backend

```bash
git clone https://github.com/Chaxware/backend
cd backend
npm install
npm run dev
```

### Frontend

```bash
git clone https://github.com/Chaxware/ChaxTUI chax-tui
cd chax-tui
git checkout ratatui
cargo run # Runs on default port 8787
```

If the backend is attached to a different port, you should specify it:

```bash
cargo run "http://localhost:<port>" # Replace <port> with the port
```
