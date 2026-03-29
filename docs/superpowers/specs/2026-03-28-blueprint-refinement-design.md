# Blueprint Refinement — Portfolio Redesign

## Goal

Make the portfolio feel distinctly "Diego" — a self-taught systems engineer who builds things — rather than a generic dark dev portfolio template. The approach: keep the existing structure (it works), replace the visual identity.

## Design Principles

- **Systems craft + builder personality**: precision without corporate sterility
- **Warm, not cold**: amber/stone tones instead of indigo/blue-gray
- **Monospace as structure, not decoration**: used for labels, metadata, version markers — not for body text
- **Purposeful motion**: no uniform hover effects; each interaction has a reason
- **Engineering document feel**: spec sheets, version numbers, status indicators

## Color Palette

| Token | Value | Usage |
|---|---|---|
| `--bg` | `#0c0a09` | Page background (warm black) |
| `--bg-alt` | `#1c1917` | Alternate section background |
| `--surface` | `#292524` | Cards, containers |
| `--surface-hover` | `#33302e` | Card hover state |
| `--border` | `#44403c` | Default borders |
| `--text` | `#e7e5e4` | Primary text |
| `--text-muted` | `#a8a29e` | Secondary text |
| `--accent` | `#e8762a` | Primary accent (rust orange) |
| `--accent-soft` | `#f59e0b` | Secondary accent (amber) |
| `--accent-glow` | `rgba(232, 118, 42, 0.12)` | Glow/highlight backgrounds |

All blue-gray tones replaced with warm stone tones throughout.

## Typography

- **Inter**: body text, headings (unchanged)
- **JetBrains Mono**: increased usage — section title prefixes, inline tech names, metadata blocks, labels, footer
- **Inline code treatment**: tech names in body text (Go, Axum, Bevy ECS) rendered in JetBrains Mono with a subtle `--accent-glow` background and rounded corners, like markdown inline code

## Section-by-Section Changes

### Hero

- **Remove** the typing effect and its JS code
- **Subtitle**: static monospace one-liner styled like a status line:
  `> building microservices @ Cencosud · Montevideo, UY`
  (bilingual: `> construyendo microservicios @ Cencosud · Montevideo, UY`)
- **Glow background**: shift radial gradient from indigo to warm amber
- **CTAs and socials**: recolor to amber palette
- **Scroll indicator**: recolor to amber

### About

- **Layout**: keep 2-column (text left, cards right)
- **Highlight cards**: restructured content layout — small monospace label on top, large number in the middle, description below. Example:
  ```
  services
  14+
  in production
  ```
- **Card style**: left amber border accent instead of full-border hover glow. No lift-on-hover.
- **Body text**: tech names get the inline monospace code treatment
- **Spacing**: slightly more breathing room between paragraphs

### Experience Timeline

- **Timeline dots replaced with version markers**: monospace labels `v3.0` (Backend Engineer), `v2.0` (QA Engineer), `v1.0` (University)
- **Most recent card**: solid amber left border to indicate "current"
- **Past role cards**: default stone border
- **Remove** the hover border-glow effect
- **Tags**: recolor from indigo to amber-tinted pills

### Projects

- **Layout change**: featured project spans full width (horizontal card), remaining projects in 2-column grid below
- **Featured card (Bloomia)**: horizontal layout — left side has title/description/tags, right side has a monospace metadata block:
  ```
  lang     Rust
  stack    Axum · sqlx · Dioxus
  db       PostgreSQL
  status   active
  ```
- **Regular cards**: remove the folder SVG icon, replace with a small monospace language tag in top-left (`rs`, `go`). Keep "Contribution" badge.
- **Hover**: border transitions to amber + subtle warm inner glow. No vertical lift movement.

### Skills

- **Languages card**: keep skill bars, recolor to amber gradient. Replace level labels ("Production", "Advanced", "Proficient") with usage descriptors: `daily`, `frequent`, `familiar`
- **Other categories**: replace tag pill clouds with compact list style — each skill on its own line with `·` or monospace dash prefix. Reads like a dependency list.
- **AI & LLM**: keep highlighted treatment, recolor to amber
- **Grid**: fixed 3 columns on desktop instead of auto-fill

### Contact

- Recolor links and hover states to amber
- Replace "slide right" hover with amber left-border appearing on hover (consistent with about cards)

### Footer

- Replace text with: `handcrafted · no frameworks · no dependencies`
- Monospace, muted color
- Bilingual: `hecho a mano · sin frameworks · sin dependencias`

## Favicon

Already exists as `favicon.svg` — update the accent color from indigo circle to amber circle.

## What Stays Unchanged

- Overall page structure and section order
- Bilingual EN/ES toggle system and all text content
- Mobile responsive breakpoints and hamburger menu
- Scroll-triggered fade-in animations (just recolored)
- Active nav link highlighting
- All links, URLs, and social references
- OG/Twitter meta tags (just update theme-color if present)

## What Gets Removed

- Typing effect JS code and related HTML (`#typedText`, `.typed-cursor`, `phrases` object)
- Uniform "lift on hover" card effects
- Folder SVG icons on project cards
- Indigo color references throughout

## Files Modified

- `style.css` — color variables, card styles, layout changes, hover effects
- `index.html` — hero subtitle, highlight card markup, featured project layout, project metadata blocks, skills lists, footer text, favicon color
- `script.js` — remove typing effect code, remove unused phrases object
- `favicon.svg` — accent color update
