# Development

The web crate defines the entrypoint for the web app along with any assets, components and dependencies that are specific to web builds. The web crate starts out something like this:

```
web/
├─ assets/ # Assets used by the web app - Any platform specific assets should go in this folder
├─ src/
│  ├─ main.rs # The entrypoint for the web app.It also defines the routes for the web platform
│  ├─ api_client.rs # HTTP types and calls to the Packrat Axum API
│  ├─ views/
│  │  ├─ mod.rs
│  │  ├─ inventory.rs # Main inventory UI (create / look up / delete items)
│  │  ├─ home.rs # Mounts `Inventory` at `/`
├─ Cargo.toml # The web crate's Cargo.toml - This should include all web specific dependencies
```

## Dependencies
Since you have fullstack enabled, the web crate will be built two times:
1. Once for the server build with the `server` feature enabled
2. Once for the client build with the `web` feature enabled

You should make **web-only** dependencies optional under the `wasm32` target where possible (this crate uses target-specific `reqwest` features: lightweight on WASM, `rustls` on the server build).

### Packrat API

Run `packrat_api` (default `http://127.0.0.1:3000`) with Postgres so `/api/health`, `/api/ready`, and `/api/items` work. The UI defaults to that base URL; change it in the page and click **Refresh status**.

### Serving Your Web App

From the **Packrat repo root**, **`just serve-web`** starts Tailwind in watch mode (so RSX class changes update `tailwind.css`) and runs **`dx serve`** in one process group; Ctrl+C stops both.

Set **`PACKRAT_WEB_NO_TAILWIND=1`** if you already run `npm run watch` in `packages/ui` yourself (the recipe still runs **`npm run build`** once so `../ui/assets/tailwind.css` exists — that file is **gitignored** and must be produced locally).

For a **one-shot CSS build** without the dev server (e.g. CI), use **`just build-ui-css`**.

From `packages/web` only, run **`just build-ui-css`** from the repo root first, then:

```bash
dx serve
```
