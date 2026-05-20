# LibreConty as Featured Project — Design

**Date:** 2026-05-19
**Topic:** Add LibreConty to the portfolio Projects section as the new featured project; demote Bloomia to a regular card.

## Context

The portfolio (`index.html`) currently has four project cards under `#projects`:

1. **Bloomia** — featured (`project-card featured` with a mono spec sheet)
2. Bevy Game Projects
3. Sudoku Solver
4. Egor (contribution, non-link `<div>`)

LibreConty is the user's shipped desktop accounting product:

- Source: `~/personal/ledger`, GitHub: `github.com/diegoQuinas/LibreConty` (treated as non-public for portfolio purposes)
- Stack: Tauri 2 (Rust + rusqlite/SQLite) with a React 19 + TypeScript + Vite + Tailwind UI
- Version 0.6.0
- A marketing landing also exists (`personal/libreconty-web`, Next.js 16) but is out of scope here

## Goal

Surface LibreConty as the most prominent project in the portfolio's Projects section while preserving the established visual language, bilingual (EN/ES) text mechanism, and card patterns.

## Changes

### 1. New featured card — LibreConty (non-clickable)

A `<div class="project-card featured">` (not `<a>`, because there is no public link target yet) inserted as the **first child** of `.projects-grid`. Structure mirrors the existing featured card:

```html
<div class="project-card featured">
  <div class="project-badge" data-en="Featured" data-es="Destacado">Featured</div>
  <div class="project-featured-inner">
    <div class="project-featured-left">
      <h3>LibreConty</h3>
      <p data-en="..." data-es="...">...</p>
      <div class="project-tags">
        <span>Rust</span><span>Tauri 2</span><span>React</span><span>TypeScript</span><span>SQLite</span>
      </div>
    </div>
    <div class="project-spec">
      <div class="spec-row"><span class="spec-key">lang</span><span class="spec-val">Rust · TS</span></div>
      <div class="spec-row"><span class="spec-key">stack</span><span class="spec-val">Tauri 2 · React · rusqlite</span></div>
      <div class="spec-row"><span class="spec-key">db</span><span class="spec-val">SQLite (local)</span></div>
      <div class="spec-row"><span class="spec-key">status</span><span class="spec-val">v0.6.0</span></div>
    </div>
  </div>
</div>
```

**Copy:**

- **EN:** "Desktop double-entry accounting application for Uruguayan SMEs. Built with Tauri 2 — a Rust core with rusqlite backing a fully local, single-file SQLite ledger, and a React/TypeScript UI. Multi-currency with BCU exchange rates, accounting periods, formal financial statements, TAFACE/DGI invoice import, and PDF/Excel export. 100% offline, with built-in backups and auto-update."
- **ES:** "Aplicación de escritorio de contabilidad por partida doble para PyMEs uruguayas. Construida con Tauri 2 — un núcleo en Rust con rusqlite sobre un libro contable SQLite 100% local en un único archivo, y una interfaz en React/TypeScript. Multi-moneda con tipo de cambio del BCU, períodos contables, estados contables formales, importación de comprobantes TAFACE/DGI y exportación a PDF y Excel. 100% offline, con respaldos integrados y auto-actualización."

### 2. Demote Bloomia to a regular card

Convert the existing Bloomia `<a class="project-card featured">` (with `project-featured-inner` and spec sheet) into a standard regular card following the Sudoku/Bevy pattern:

```html
<a href="https://github.com/diegoQuinas/bloomia-backend" target="_blank" rel="noopener" class="project-card">
  <span class="project-lang">rs</span>
  <h3>Bloomia</h3>
  <p data-en="..." data-es="...">[existing EN/ES copy preserved verbatim]</p>
  <div class="project-tags">
    <span>Rust</span><span>Axum</span><span>Dioxus</span><span>PostgreSQL</span><span>sqlx</span>
  </div>
</a>
```

- Drop the `featured` class, the `project-badge`, the `project-featured-inner` wrapper, and the `project-spec` block.
- Add `<span class="project-lang">rs</span>` to match the regular-card lang chip pattern.
- Preserve description copy, tags, and `href` unchanged.

### 3. Final grid order

```
[ LibreConty — featured, full-width ]
[ Bloomia ]   [ Bevy Game Projects ]
[ Sudoku  ]   [ Egor               ]
```

(`projects-grid` is 2-column on desktop and collapses to 1-column on mobile via the existing media query.)

## Out of scope

- About-section stat cards (e.g., `28+ public repos`) are not updated.
- The marketing site `libreconty-web` is not added as its own entry.
- No CSS changes — the existing rules already cover a `<div class="project-card featured">` (the Egor card already uses a non-link `<div>` in the regular variant).
- No `script.js` changes — bilingual swapping is driven by `data-en`/`data-es` attributes which the new card uses.

## Verification

- Open `index.html` in a browser; confirm LibreConty appears full-width at the top of Projects with badge, copy, tags, and a 4-row mono spec sheet on the right.
- Toggle EN/ES; confirm the description swaps and the badge swaps between `Featured` and `Destacado`.
- Confirm Bloomia now renders as a standard half-width card with the `rs` lang chip, and its GitHub link still works.
- Resize to mobile width; confirm the featured card collapses to single-column (spec sheet stacks under the description) per the existing `@media` rule.
