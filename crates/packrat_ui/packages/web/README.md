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
│  │  ├─ inventory.rs # Main inventory UI (create / look up / delete assets)
│  │  ├─ home.rs # Mounts `Inventory` at `/`
├─ Cargo.toml # The web crate's Cargo.toml - This should include all web specific dependencies
```

## Dependencies

This crate uses **`dioxus`** with **`router`** and **`web`** (client-side WASM only — no `fullstack`, so static hosting from `packrat_api` does not require SSR hydration data).

Optional **`--features server`** enables the native server build (`dioxus/server`, `ui/server`) if you use fullstack tooling; day-to-day **`dx serve`** targets WASM only.

Make **web-only** dependencies optional under the `wasm32` target where possible (this crate uses target-specific `reqwest`: lightweight on WASM, `rustls` on native).

### Packrat API

With **`packrat_api`** serving the built UI on the same origin, the WASM client defaults to an **empty** API base (requests go to `/api/...` on the current host). For **`just run-api`** with the UI on another origin (e.g. `dx serve`), use **`http://127.0.0.1:3000`** or set `localStorage` key `packrat_api_base_v1` if needed.

### Serving Your Web App

From the **Packrat repo root**, **`just serve-web`** starts Tailwind in watch mode (so RSX class changes update `tailwind.css`) and runs **`dx serve`** in one process group; Ctrl+C stops both.

Set **`PACKRAT_WEB_NO_TAILWIND=1`** if you already run `npm run watch` in `packages/ui` yourself (the recipe still runs **`npm run build`** once so `../ui/assets/tailwind.css` exists — that file is **gitignored** and must be produced locally).

For a **one-shot CSS build** without the dev server (e.g. CI), use **`just build-ui-css`**.

From `packages/web` only, run **`just build-ui-css`** from the repo root first, then:

```bash
dx serve
```
