# Blueprint Refinement Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Transform the portfolio from a generic dark dev template into a distinctive, warm-toned site that feels like a systems engineer's crafted tool.

**Architecture:** Pure CSS variable swap + targeted HTML restructuring per section. No new dependencies. All changes are in 4 files: `style.css`, `index.html`, `script.js`, `favicon.svg`.

**Tech Stack:** Vanilla HTML, CSS, JS (no frameworks)

---

### Task 1: Color Palette and Base Styles

**Files:**
- Modify: `style.css:1-20` (CSS variables)
- Modify: `style.css:37-47` (nav background rgba)
- Modify: `style.css:138-153` (mobile menu background rgba)
- Modify: `favicon.svg` (accent color)

- [ ] **Step 1: Replace CSS custom properties with warm palette**

In `style.css`, replace the `:root` block (lines 2-19):

```css
:root {
  --bg: #0c0a09;
  --bg-alt: #1c1917;
  --surface: #292524;
  --surface-hover: #33302e;
  --border: #44403c;
  --text: #e7e5e4;
  --text-muted: #a8a29e;
  --accent: #e8762a;
  --accent-soft: #f59e0b;
  --accent-glow: rgba(232, 118, 42, 0.12);
  --accent-secondary: #f59e0b;
  --radius: 12px;
  --radius-sm: 8px;
  --transition: 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  --font: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
  --font-mono: 'JetBrains Mono', monospace;
  --max-width: 1100px;
}
```

- [ ] **Step 2: Update nav background rgba to warm tone**

In `style.css`, update `.nav` background:

```css
background: rgba(12, 10, 9, 0.8);
```

Update `.mobile-menu` background:

```css
background: rgba(12, 10, 9, 0.95);
```

- [ ] **Step 3: Update all hardcoded indigo rgba values throughout style.css**

Search and replace these hardcoded color values:

- `rgba(99, 102, 241, 0.3)` → `rgba(232, 118, 42, 0.3)` (btn-primary hover shadow)
- `rgba(99, 102, 241, 0.1)` → `rgba(232, 118, 42, 0.1)` (highlight-card hover shadow, project-card hover shadow)
- `rgba(99, 102, 241, 0.2)` → `rgba(232, 118, 42, 0.2)` (timeline-tags border, project-tags border)
- `rgba(99, 102, 241, 0.05)` → `rgba(232, 118, 42, 0.05)` (featured project background)
- `rgba(99, 102, 241, 0.3)` → `rgba(232, 118, 42, 0.3)` (skill-tags highlight border)

- [ ] **Step 4: Update favicon accent color**

In `favicon.svg`, replace:
- `fill="#0a0a0f"` → `fill="#0c0a09"` (background)
- `fill="#6366f1"` → `fill="#e8762a"` (accent circle)

- [ ] **Step 5: Verify in browser**

Open `index.html` in a browser. Confirm:
- Warm black background (not blue-black)
- Amber/orange accents everywhere (nav underlines, buttons, tags)
- No remaining indigo/purple tones
- Favicon shows amber dot

- [ ] **Step 6: Commit**

```bash
git add style.css favicon.svg
git commit -m "style: replace indigo palette with warm amber/stone tones"
```

---

### Task 2: Hero Section — Remove Typing Effect, Add Status Line

**Files:**
- Modify: `index.html:70-100` (hero section HTML)
- Modify: `script.js:1-84` (remove typing code)
- Modify: `style.css:215-227` (hero-subtitle and typed-cursor styles)

- [ ] **Step 1: Replace hero subtitle HTML**

In `index.html`, replace the hero subtitle block (lines 75-80):

```html
      <p class="hero-subtitle">
        <span class="typed-wrapper">
          <span class="typed-text" id="typedText"></span>
          <span class="typed-cursor">|</span>
        </span>
      </p>
```

With:

```html
      <p class="hero-subtitle"
         data-en="> building microservices @ Cencosud · Montevideo, UY"
         data-es="> construyendo microservicios @ Cencosud · Montevideo, UY">
        > building microservices @ Cencosud · Montevideo, UY
      </p>
```

- [ ] **Step 2: Remove typing effect JS**

In `script.js`, remove the entire typing effect section (lines 27-84):
- The `phrases` object
- The `typingTimeout` variable
- The `startTyping()` function

Also remove the `startTyping()` call from `setLanguage()` function (line 18).

Also remove `startTyping()` from the DOMContentLoaded handler.

The resulting `setLanguage` function should be:

```js
function setLanguage(lang) {
  currentLang = lang;
  document.documentElement.setAttribute('data-lang', lang);

  document.querySelectorAll('[data-en]').forEach(el => {
    const text = el.getAttribute(`data-${lang}`);
    if (text) el.textContent = text;
  });

  document.querySelectorAll('.lang-option').forEach(opt => {
    opt.classList.toggle('active', opt.dataset.lang === lang);
  });
}
```

The DOMContentLoaded handler should be:

```js
document.addEventListener('DOMContentLoaded', () => {
  initScrollAnimations();
  initSkillBars();
  initActiveNav();
});
```

- [ ] **Step 3: Update hero-subtitle CSS**

In `style.css`, replace the hero-subtitle and typed-cursor styles (lines 215-227):

```css
.hero-subtitle {
  font-family: var(--font-mono);
  font-size: 15px;
  color: var(--accent-soft);
  margin-bottom: 36px;
}
```

Remove the `.typed-cursor` rule and the `@keyframes blink` rule entirely.

- [ ] **Step 4: Verify in browser**

Open `index.html`. Confirm:
- No typing animation
- Static status line `> building microservices @ Cencosud · Montevideo, UY` visible
- Language toggle switches the subtitle text
- No JS console errors

- [ ] **Step 5: Commit**

```bash
git add index.html script.js style.css
git commit -m "feat: replace typing effect with static status line subtitle"
```

---

### Task 3: About Section — Restructure Highlight Cards

**Files:**
- Modify: `index.html:121-138` (highlight cards HTML)
- Modify: `style.css:347-372` (highlight card styles)

- [ ] **Step 1: Update highlight card HTML structure**

In `index.html`, replace the `about-highlights` div (lines 121-138):

```html
        <div class="about-highlights">
          <div class="highlight-card">
            <span class="highlight-tag" data-en="services" data-es="servicios">services</span>
            <span class="highlight-number">14+</span>
            <span class="highlight-label" data-en="in production" data-es="en produccion">in production</span>
          </div>
          <div class="highlight-card">
            <span class="highlight-tag" data-en="tenure" data-es="antiguedad">tenure</span>
            <span class="highlight-number">3+</span>
            <span class="highlight-label" data-en="years at Cencosud" data-es="anos en Cencosud">years at Cencosud</span>
          </div>
          <div class="highlight-card">
            <span class="highlight-tag" data-en="reach" data-es="alcance">reach</span>
            <span class="highlight-number">4</span>
            <span class="highlight-label" data-en="countries served" data-es="paises servidos">countries served</span>
          </div>
          <div class="highlight-card">
            <span class="highlight-tag" data-en="open source" data-es="codigo abierto">open source</span>
            <span class="highlight-number">28+</span>
            <span class="highlight-label" data-en="public repos" data-es="repos publicos">public repos</span>
          </div>
        </div>
```

- [ ] **Step 2: Update highlight card CSS**

In `style.css`, replace the highlight card styles (lines 347-372):

```css
.about-highlights {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}
.highlight-card {
  background: var(--surface);
  border: 1px solid var(--border);
  border-left: 3px solid var(--accent);
  border-radius: var(--radius);
  padding: 20px;
  text-align: center;
  transition: border-color var(--transition);
}
.highlight-card:hover {
  border-color: var(--accent);
}
.highlight-tag {
  display: block;
  font-family: var(--font-mono);
  font-size: 11px;
  color: var(--text-muted);
  text-transform: lowercase;
  letter-spacing: 1px;
  margin-bottom: 4px;
}
.highlight-number {
  display: block;
  font-size: 28px;
  font-weight: 800;
  color: var(--accent);
  font-family: var(--font-mono);
}
.highlight-label {
  font-size: 12px;
  color: var(--text-muted);
  margin-top: 4px;
  display: block;
}
```

- [ ] **Step 3: Verify in browser**

Confirm:
- Cards show 3-line structure: tag / number / label
- Amber left border on each card
- No lift-on-hover, just border color change
- Language toggle updates tags and labels

- [ ] **Step 4: Commit**

```bash
git add index.html style.css
git commit -m "style: restructure about highlight cards with metric readout layout"
```

---

### Task 4: Experience Timeline — Version Markers and Amber Current Border

**Files:**
- Modify: `index.html:148-150,182-183,216-217` (timeline dots → version labels)
- Modify: `style.css:375-477` (timeline styles)

- [ ] **Step 1: Replace timeline dots with version markers in HTML**

In `index.html`, replace each `<div class="timeline-dot">` with version label markup:

First timeline item (Backend Engineer), replace line 149:
```html
          <div class="timeline-version current">v3.0</div>
```

Second timeline item (QA Engineer), replace line 183:
```html
          <div class="timeline-version">v2.0</div>
```

Third timeline item (University), replace line 217:
```html
          <div class="timeline-version edu">v1.0</div>
```

- [ ] **Step 2: Update timeline CSS**

In `style.css`, replace `.timeline-dot` and `.dot-edu` rules (lines 393-407):

```css
.timeline-version {
  position: absolute;
  left: -40px;
  top: 8px;
  font-family: var(--font-mono);
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  white-space: nowrap;
}
.timeline-version.current {
  color: var(--accent);
}
.timeline-version.edu {
  color: var(--accent-soft);
}
```

Update `.timeline` padding-left to accommodate wider labels (line 377):

```css
.timeline {
  position: relative;
  padding-left: 48px;
}
```

Update `.timeline::before` left position (line 382):

```css
  left: 16px;
```

Add amber left border to current timeline card. Replace `.timeline-content:hover` rule (line 415):

```css
.timeline-item:first-child .timeline-content {
  border-left: 3px solid var(--accent);
}
.timeline-content:hover {
  border-color: var(--accent);
}
```

- [ ] **Step 3: Verify in browser**

Confirm:
- `v3.0` / `v2.0` / `v1.0` labels appear instead of dots
- `v3.0` is amber-colored, others are muted
- First (current) card has amber left border
- Vertical timeline line still connects properly

- [ ] **Step 4: Commit**

```bash
git add index.html style.css
git commit -m "style: replace timeline dots with version markers, amber current indicator"
```

---

### Task 5: Projects Section — Asymmetric Layout with Spec Sheet

**Files:**
- Modify: `index.html:237-303` (projects section HTML)
- Modify: `style.css:479-565` (project styles)

- [ ] **Step 1: Update projects HTML — featured card with spec sheet**

In `index.html`, replace the entire `projects-grid` div (lines 240-301):

```html
      <div class="projects-grid">

        <a href="https://github.com/diegoQuinas/bloomia-backend" target="_blank" rel="noopener" class="project-card featured">
          <div class="project-badge" data-en="Featured" data-es="Destacado">Featured</div>
          <div class="project-featured-inner">
            <div class="project-featured-left">
              <h3>Bloomia</h3>
              <p data-en="Full-stack flower e-commerce application. Rust backend built with Axum, PostgreSQL/sqlx for async compile-time checked queries, and Dioxus reactive frontend."
                 data-es="Aplicacion e-commerce de flores full-stack. Backend en Rust con Axum, PostgreSQL/sqlx para queries async verificadas en compilacion, y frontend reactivo en Dioxus.">
                Full-stack flower e-commerce application. Rust backend built with Axum, PostgreSQL/sqlx for async compile-time checked queries, and Dioxus reactive frontend.
              </p>
              <div class="project-tags">
                <span>Rust</span><span>Axum</span><span>Dioxus</span><span>PostgreSQL</span><span>sqlx</span>
              </div>
            </div>
            <div class="project-spec">
              <div class="spec-row"><span class="spec-key">lang</span><span class="spec-val">Rust</span></div>
              <div class="spec-row"><span class="spec-key">stack</span><span class="spec-val">Axum · sqlx · Dioxus</span></div>
              <div class="spec-row"><span class="spec-key">db</span><span class="spec-val">PostgreSQL</span></div>
              <div class="spec-row"><span class="spec-key">status</span><span class="spec-val">active</span></div>
            </div>
          </div>
        </a>

        <a href="https://github.com/diegoQuinas" target="_blank" rel="noopener" class="project-card">
          <span class="project-lang">rs</span>
          <h3 data-en="Bevy Game Projects" data-es="Juegos con Bevy">Bevy Game Projects</h3>
          <p data-en="Mining Simulator, Orkish Defense, Tower Defense — multiple games built with Bevy ECS: state-driven logic, custom UI systems, entity management. Inspired by a lifelong passion for retro MMORPGs."
             data-es="Mining Simulator, Orkish Defense, Tower Defense — multiples juegos con Bevy ECS: logica por estados, sistemas de UI custom, manejo de entidades. Inspirados por una pasion de toda la vida por los MMORPGs retro.">
            Mining Simulator, Orkish Defense, Tower Defense — multiple games built with Bevy ECS: state-driven logic, custom UI systems, entity management. Inspired by a lifelong passion for retro MMORPGs.
          </p>
          <div class="project-tags">
            <span>Rust</span><span>Bevy</span><span>ECS</span><span>Game Dev</span>
          </div>
        </a>

        <a href="https://github.com/diegoQuinas/sudoku-solver-rs" target="_blank" rel="noopener" class="project-card">
          <span class="project-lang">rs</span>
          <h3>Sudoku Solver</h3>
          <p data-en="Algorithmic puzzle solver built in Rust as a systems programming exercise. Backtracking algorithm with constraint propagation."
             data-es="Solucionador de sudoku algoritmico en Rust como ejercicio de programacion de sistemas. Algoritmo de backtracking con propagacion de restricciones.">
            Algorithmic puzzle solver built in Rust as a systems programming exercise. Backtracking algorithm with constraint propagation.
          </p>
          <div class="project-tags">
            <span>Rust</span><span>Algorithms</span>
          </div>
        </a>

        <div class="project-card">
          <div class="project-badge" data-en="Contribution" data-es="Contribucion">Contribution</div>
          <span class="project-lang">rs</span>
          <div class="project-header">
            <div class="project-stars">128</div>
          </div>
          <h3>Egor</h3>
          <p data-en="Contributor to a cross-platform 2D graphics engine in Rust. Supports DirectX 12, Vulkan, Metal, OpenGL, and WebGPU/WASM with a plugin architecture."
             data-es="Contribuidor a un motor grafico 2D cross-platform en Rust. Soporta DirectX 12, Vulkan, Metal, OpenGL y WebGPU/WASM con arquitectura de plugins.">
            Contributor to a cross-platform 2D graphics engine in Rust. Supports DirectX 12, Vulkan, Metal, OpenGL, and WebGPU/WASM with a plugin architecture.
          </p>
          <div class="project-tags">
            <span>Rust</span><span>wgpu</span><span>WGSL</span><span>Open Source</span>
          </div>
        </div>

      </div>
```

- [ ] **Step 2: Update project CSS**

In `style.css`, replace the entire projects CSS section (lines 479-565):

```css
/* ===== Projects ===== */
.projects-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
}
.project-card {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 24px;
  transition: all var(--transition);
  position: relative;
  display: flex;
  flex-direction: column;
}
.project-card:hover {
  border-color: var(--accent);
  box-shadow: 0 0 20px rgba(232, 118, 42, 0.08) inset;
}
.project-card.featured {
  grid-column: 1 / -1;
  border-color: var(--accent);
  background: linear-gradient(135deg, var(--surface) 0%, rgba(232, 118, 42, 0.05) 100%);
}
.project-featured-inner {
  display: grid;
  grid-template-columns: 1fr 200px;
  gap: 24px;
  align-items: start;
}
.project-featured-left h3 {
  font-size: 20px;
  font-weight: 700;
  margin-bottom: 10px;
}
.project-spec {
  font-family: var(--font-mono);
  font-size: 13px;
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 16px;
  background: var(--bg);
  border-radius: var(--radius-sm);
  border: 1px solid var(--border);
}
.spec-row {
  display: flex;
  gap: 12px;
}
.spec-key {
  color: var(--text-muted);
  min-width: 48px;
}
.spec-val {
  color: var(--accent-soft);
}
.project-lang {
  font-family: var(--font-mono);
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: lowercase;
  margin-bottom: 12px;
  display: block;
}
.project-badge {
  position: absolute;
  top: 12px;
  right: 12px;
  background: var(--accent);
  color: #fff;
  font-size: 10px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 1px;
  padding: 3px 10px;
  border-radius: 20px;
}
.project-header {
  display: flex;
  justify-content: flex-end;
  align-items: center;
  margin-bottom: 8px;
  color: var(--text-muted);
}
.project-stars {
  font-family: var(--font-mono);
  font-size: 13px;
  color: var(--accent-secondary);
  display: flex;
  align-items: center;
  gap: 4px;
}
.project-stars::before {
  content: '';
  display: inline-block;
  width: 14px;
  height: 14px;
  background: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' fill='%23f59e0b' viewBox='0 0 24 24'%3E%3Cpath d='M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z'/%3E%3C/svg%3E") no-repeat center;
  background-size: contain;
}
.project-card h3 {
  font-size: 18px;
  font-weight: 700;
  margin-bottom: 8px;
}
.project-card p {
  font-size: 14px;
  color: var(--text-muted);
  line-height: 1.6;
  flex: 1;
}
.project-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-top: 16px;
}
.project-tags span {
  font-size: 11px;
  font-weight: 500;
  padding: 3px 10px;
  border-radius: 20px;
  background: var(--accent-glow);
  color: var(--accent-soft);
  border: 1px solid rgba(232, 118, 42, 0.2);
}
```

- [ ] **Step 3: Update responsive CSS for projects**

In `style.css`, inside the `@media (max-width: 768px)` block, update the projects-grid rule:

```css
  .projects-grid { grid-template-columns: 1fr; }
  .project-featured-inner { grid-template-columns: 1fr; }
```

- [ ] **Step 4: Verify in browser**

Confirm:
- Bloomia spans full width with spec sheet on the right
- Other 3 cards in 2-column grid below
- `rs` language tag in top-left of non-featured cards
- Hover shows amber border + subtle inner glow, no vertical lift
- On mobile, everything stacks to single column

- [ ] **Step 5: Commit**

```bash
git add index.html style.css
git commit -m "feat: asymmetric project layout with spec sheet for featured project"
```

---

### Task 6: Skills Section — Dependency List Style and Labels

**Files:**
- Modify: `index.html:306-368` (skills section HTML)
- Modify: `style.css:567-633` (skills styles)

- [ ] **Step 1: Update skill level labels and list style in HTML**

In `index.html`, update the skill bar labels:

```html
        <div class="skill-category">
          <h3 data-en="Languages" data-es="Lenguajes">Languages</h3>
          <div class="skill-bars">
            <div class="skill-bar-item">
              <div class="skill-bar-header"><span>Go</span><span>daily</span></div>
              <div class="skill-bar"><div class="skill-bar-fill" style="width:90%"></div></div>
            </div>
            <div class="skill-bar-item">
              <div class="skill-bar-header"><span>Rust</span><span>daily</span></div>
              <div class="skill-bar"><div class="skill-bar-fill" style="width:88%"></div></div>
            </div>
            <div class="skill-bar-item">
              <div class="skill-bar-header"><span>TypeScript</span><span>frequent</span></div>
              <div class="skill-bar"><div class="skill-bar-fill" style="width:70%"></div></div>
            </div>
            <div class="skill-bar-item">
              <div class="skill-bar-header"><span>SQL</span><span>frequent</span></div>
              <div class="skill-bar"><div class="skill-bar-fill" style="width:75%"></div></div>
            </div>
          </div>
        </div>
```

Replace all other skill categories with `skill-list` class instead of `skill-tags`:

```html
        <div class="skill-category">
          <h3>Go Ecosystem</h3>
          <ul class="skill-list">
            <li>gRPC / Protobuf</li><li>sqlc</li><li>golangci-lint</li><li>buf</li><li>Tilt</li><li>net/http</li>
          </ul>
        </div>

        <div class="skill-category">
          <h3>Rust Ecosystem</h3>
          <ul class="skill-list">
            <li>Axum</li><li>Tokio</li><li>sqlx</li><li>reqwest</li><li>serde</li><li>Bevy</li><li>wgpu</li><li>Ratatui</li>
          </ul>
        </div>

        <div class="skill-category">
          <h3 data-en="Infrastructure" data-es="Infraestructura">Infrastructure</h3>
          <ul class="skill-list">
            <li>Kubernetes</li><li>Docker</li><li>GCP</li><li>AWS</li><li>Rancher</li><li>PostgreSQL</li><li>SQLite</li>
          </ul>
        </div>

        <div class="skill-category">
          <h3>AI & LLM</h3>
          <ul class="skill-list highlight">
            <li>Claude Code</li><li>GitHub Copilot</li><li>MCP Servers</li><li>LLM APIs</li><li>Prompt Engineering</li><li>AI Agents</li>
          </ul>
        </div>

        <div class="skill-category">
          <h3 data-en="Practices" data-es="Practicas">Practices</h3>
          <ul class="skill-list">
            <li>TDD</li><li>Clean Architecture</li><li>Microservices</li><li>CI/CD</li><li>Code Review</li><li>ECS Pattern</li>
          </ul>
        </div>
```

- [ ] **Step 2: Update skills CSS**

In `style.css`, update the skills-grid to fixed 3 columns:

```css
.skills-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 24px;
}
```

Add skill-list styles after the existing `.skill-tags` rules. Replace the `.skill-tags` block and add `.skill-list`:

```css
.skill-list {
  list-style: none;
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.skill-list li {
  font-family: var(--font-mono);
  font-size: 13px;
  color: var(--text-muted);
  padding-left: 16px;
  position: relative;
}
.skill-list li::before {
  content: '—';
  position: absolute;
  left: 0;
  color: var(--border);
}
.skill-list.highlight li {
  color: var(--accent-soft);
}
.skill-list.highlight li::before {
  color: var(--accent);
}
```

Remove the old `.skill-tags` rules (`.skill-tags`, `.skill-tags span`, `.skill-tags span:hover`, `.skill-tags.highlight span`) since they are no longer used.

Update the `skill-bar-header` last child to use monospace:

```css
.skill-bar-header span:last-child { color: var(--text-muted); font-size: 11px; font-family: var(--font-mono); }
```

- [ ] **Step 3: Update responsive CSS**

In the `@media (max-width: 768px)` block:

```css
  .skills-grid { grid-template-columns: 1fr; }
```

(This already exists, just confirm it's still there after changes.)

- [ ] **Step 4: Verify in browser**

Confirm:
- Languages card shows `daily` / `frequent` labels in monospace
- Other categories show vertical list with `—` prefix
- AI & LLM list items are amber-colored
- 3-column grid on desktop, 1-column on mobile

- [ ] **Step 5: Commit**

```bash
git add index.html style.css
git commit -m "style: skill categories as dependency lists, usage-based level labels"
```

---

### Task 7: Contact and Footer

**Files:**
- Modify: `index.html:398-404` (footer HTML)
- Modify: `style.css:651-666` (contact-card hover)

- [ ] **Step 1: Update contact card hover**

In `style.css`, replace the `.contact-card:hover` rule:

```css
.contact-card:hover {
  border-left: 3px solid var(--accent);
  color: var(--accent-soft);
}
```

Remove the `transform: translateX(8px)` — replaced by the left border effect.

- [ ] **Step 2: Update footer text**

In `index.html`, replace the footer paragraph (lines 400-403):

```html
    <p data-en="handcrafted · no frameworks · no dependencies"
       data-es="hecho a mano · sin frameworks · sin dependencias">
      handcrafted · no frameworks · no dependencies
    </p>
```

- [ ] **Step 3: Verify in browser**

Confirm:
- Contact cards show amber left border on hover (no slide)
- Footer shows new monospace text
- Language toggle updates footer

- [ ] **Step 4: Commit**

```bash
git add index.html style.css
git commit -m "style: contact hover border, update footer text"
```

---

### Task 8: Final Polish — Inline Code Treatment and Cleanup

**Files:**
- Modify: `style.css` (add inline code class)
- Modify: `index.html` (add inline code spans to about text)
- Modify: `script.js` (remove unused `translations` object)

- [ ] **Step 1: Add inline code CSS class**

In `style.css`, after the reset section (after line 34), add:

```css
.code {
  font-family: var(--font-mono);
  font-size: 0.9em;
  background: var(--accent-glow);
  padding: 2px 6px;
  border-radius: 4px;
  color: var(--accent-soft);
}
```

- [ ] **Step 2: Add inline code treatment to about text**

In `index.html`, update the about paragraphs to wrap tech names in `<span class="code">`. Update both `data-en` and `data-es` attributes and the visible text.

For the first paragraph, the visible text becomes:
```
Self-taught programmer since age 12 and backend engineer based in Montevideo, Uruguay. After 3+ years as QA Engineer at Cencosud, I transitioned into <span class="code">Go</span> backend development — building microservices for a multi-country e-commerce platform serving millions of users across Latin America.
```

For the second paragraph, the visible text becomes:
```
In my own time I build games, tools, and systems — all in <span class="code">Rust</span>. From <span class="code">Bevy ECS</span> game projects inspired by retro MMORPGs to full-stack web apps with <span class="code">Axum</span>. I'm also deeply invested in AI-augmented development, using LLMs daily to accelerate architecture design, code review, and delivery.
```

Note: Since the `data-en`/`data-es` attributes use `textContent` for swapping, the `<span class="code">` tags only apply to the initial render. To make them work with language switching, the `setLanguage` function would need to use `innerHTML` instead of `textContent`. However, this is a security concern (XSS via data attributes). Instead, mark these paragraphs with a `data-has-code` attribute and skip them in the language switcher — they don't change between EN/ES for the tech names.

Actually, the simpler approach: leave the `data-en`/`data-es` attributes as plain text (they already work). The inline code spans only show on initial load in the default language. When switching language, `textContent` will strip the spans. This is acceptable since the tech names are the same in both languages. But a cleaner fix: use `innerHTML` only for elements with class `has-code`:

In `script.js`, update the `setLanguage` function:

```js
function setLanguage(lang) {
  currentLang = lang;
  document.documentElement.setAttribute('data-lang', lang);

  document.querySelectorAll('[data-en]').forEach(el => {
    const text = el.getAttribute(`data-${lang}`);
    if (text) {
      if (el.classList.contains('has-code')) {
        el.innerHTML = text;
      } else {
        el.textContent = text;
      }
    }
  });

  document.querySelectorAll('.lang-option').forEach(opt => {
    opt.classList.toggle('active', opt.dataset.lang === lang);
  });
}
```

Then in the about paragraphs, include `<span class="code">` in the `data-en` and `data-es` attributes too, and add `class="has-code"`:

First paragraph:
```html
          <p class="has-code"
             data-en='Self-taught programmer since age 12 and backend engineer based in Montevideo, Uruguay. After 3+ years as QA Engineer at Cencosud, I transitioned into <span class="code">Go</span> backend development — building microservices for a multi-country e-commerce platform serving millions of users across Latin America.'
             data-es='Programador autodidacta desde los 12 anos e ingeniero backend en Montevideo, Uruguay. Despues de 3+ anos como QA Engineer en Cencosud, hice la transicion a desarrollo backend en <span class="code">Go</span> — construyendo microservicios para una plataforma e-commerce multi-pais que sirve a millones de usuarios en Latinoamerica.'>
            Self-taught programmer since age 12 and backend engineer based in Montevideo, Uruguay. After 3+ years as QA Engineer at Cencosud, I transitioned into <span class="code">Go</span> backend development — building microservices for a multi-country e-commerce platform serving millions of users across Latin America.
          </p>
```

Second paragraph:
```html
          <p class="has-code"
             data-en='In my own time I build games, tools, and systems — all in <span class="code">Rust</span>. From <span class="code">Bevy ECS</span> game projects inspired by retro MMORPGs to full-stack web apps with <span class="code">Axum</span>. I&apos;m also deeply invested in AI-augmented development, using LLMs daily to accelerate architecture design, code review, and delivery.'
             data-es='En mi tiempo libre construyo juegos, herramientas y sistemas — todo en <span class="code">Rust</span>. Desde proyectos de juegos con <span class="code">Bevy ECS</span> inspirados en MMORPGs retro hasta aplicaciones web full-stack con <span class="code">Axum</span>. Tambien estoy profundamente involucrado en el desarrollo asistido por IA, usando LLMs diariamente para acelerar el diseno de arquitectura, revision de codigo y entregas.'>
            In my own time I build games, tools, and systems — all in <span class="code">Rust</span>. From <span class="code">Bevy ECS</span> game projects inspired by retro MMORPGs to full-stack web apps with <span class="code">Axum</span>. I'm also deeply invested in AI-augmented development, using LLMs daily to accelerate architecture design, code review, and delivery.
          </p>
```

Third paragraph stays unchanged (no tech names worth highlighting).

- [ ] **Step 3: Remove unused translations object from script.js**

In `script.js`, remove line 2:

```js
const translations = { en: {}, es: {} };
```

- [ ] **Step 4: Full verification in browser**

Open `index.html` and check every section:
- Warm amber color palette throughout
- Hero: static status line, no typing
- About: metric-style cards, inline code highlights on tech names
- Experience: version markers, amber current card
- Projects: featured full-width with spec sheet, 2-col grid for rest
- Skills: dependency list style, `daily`/`frequent` labels
- Contact: left-border hover
- Footer: new monospace tagline
- Language toggle works on all sections including inline code spans
- Mobile responsive layout works
- No JS console errors

- [ ] **Step 5: Commit**

```bash
git add index.html style.css script.js
git commit -m "feat: inline code highlights for tech names, cleanup unused code"
```
