# PROPOSAL 1: "THE DUAL FLUX"

### 1. Concept & Geometry

- **Metaphor:** Two interlocking chevron brackets forming a continuous square loop (`{ }` meets `[x]`). Represents the frictionless conversion of unstructured Markdown list syntax into structured YAML key-value dictionaries.
- **Aesthetic:** High-precision Swiss modernist geometry with 45° chamfers and optical balance.

### 2. Primary Mark (SVG Specification)

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="100" height="100" fill="none">
  <rect width="100" height="100" rx="20" fill="#0D1117"/>
  <!-- Left Bracket / Markdown side -->
  <path d="M42 28H28V72H42" stroke="#E6EDF3" stroke-width="8" stroke-linecap="round" stroke-linejoin="round"/>
  <!-- Right Key-Value / YAML colon glyph -->
  <path d="M58 28H72V72H58" stroke="#F59E0B" stroke-width="8" stroke-linecap="round" stroke-linejoin="round"/>
  <!-- Center Task Check Bar -->
  <line x1="36" y1="50" x2="64" y2="50" stroke="#F59E0B" stroke-width="6" stroke-linecap="round"/>
</svg>
```

### 3. Calibrated Color Palette

- `Base Neutral Dark`: `#0D1117` (Obsidian Charcoal)
- `Base Neutral Light`: `#FAFAFA` (Pristine Snow)
- `Surface / Border`: `#21262D` (Subtle boundary gray)
- `Signature Accent`: `#F59E0B` (Safety Amber / Electric Ochre)
- `Status Hues`:
  - `Todo`: `#8B949E` (Slate Gray)
  - `InProgress`: `#F59E0B` (Safety Amber)
  - `Done`: `#10B981` (Nordic Spruce)
  - `Blocked`: `#F43F5E` (Signal Crimson)

### 4. Typography Hierarchy

- **Display / Wordmark:** _Cabinet Grotesk_ (Bold, lowercase tracking `-0.04em`)
- **Body & Docs:** _Instrument Sans_ (Regular 400 & Medium 500)
- **Monospace CLI:** _Geist Mono_ (Tabular numbers, clean code punctuation)

### 5. Terminal ASCII Art (`lida --help`)

```text
 _ _     _
| (_) __| | __ _
| | |/ _` |/ _` |
|_|_|\__,_|\__,_| [YAML · MD]
```

### 6. GitHub README Banner Mockup (1200x380)

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 380" width="1200" height="380">
  <rect width="1200" height="380" fill="#0D1117"/>
  <rect x="24" y="24" width="1152" height="332" rx="16" fill="#161B22" stroke="#21262D" stroke-width="1.5"/>
  <!-- Mark -->
  <g transform="translate(64, 80)">
    <rect width="90" height="90" rx="18" fill="#21262D"/>
    <path d="M38 25H25V65H38" stroke="#F0F6FC" stroke-width="7" stroke-linecap="round"/>
    <path d="M52 25H65V65H52" stroke="#F59E0B" stroke-width="7" stroke-linecap="round"/>
    <line x1="32" y1="45" x2="58" y2="45" stroke="#F59E0B" stroke-width="5" stroke-linecap="round"/>
  </g>
  <!-- Wordmark & Tagline -->
  <text x="178" y="130" font-family="'Cabinet Grotesk', system-ui" font-size="52" font-weight="800" fill="#F0F6FC" letter-spacing="-2">lida</text>
  <rect x="286" y="98" width="86" height="24" rx="12" fill="#F59E0B" fill-opacity="0.15" stroke="#F59E0B" stroke-width="1"/>
  <text x="329" y="114" font-family="'Geist Mono', monospace" font-size="12" font-weight="600" fill="#F59E0B" text-anchor="middle">v0.4.2</text>
  <text x="180" y="162" font-family="'Instrument Sans', system-ui" font-size="20" fill="#8B949E">Dual-format project task engine (YAML · MD)</text>

  <!-- Terminal Showcase Card -->
  <g transform="translate(680, 60)">
    <rect width="450" height="260" rx="10" fill="#0D1117" stroke="#30363D" stroke-width="1"/>
    <!-- Terminal Header -->
    <circle cx="20" cy="20" r="5" fill="#30363D"/>
    <circle cx="36" cy="20" r="5" fill="#30363D"/>
    <circle cx="52" cy="20" r="5" fill="#30363D"/>
    <text x="225" y="24" font-family="'Geist Mono', monospace" font-size="11" fill="#6E7681" text-anchor="middle">fish ~/repo (main)</text>
    <!-- Commands -->
    <text x="24" y="65" font-family="'Geist Mono', monospace" font-size="13" fill="#F59E0B">$ lida add "Sync dual schemas" -t urgent</text>
    <text x="24" y="90" font-family="'Geist Mono', monospace" font-size="12" fill="#3FB950">✓ Added #4a19 [TODO.md + TODO.yaml] (0.8ms)</text>
    <text x="24" y="125" font-family="'Geist Mono', monospace" font-size="13" fill="#F59E0B">$ lida list --status in-progress</text>
    <text x="24" y="155" font-family="'Geist Mono', monospace" font-size="12" fill="#8B949E">ID    STATUS  TITLE</text>
    <text x="24" y="180" font-family="'Geist Mono', monospace" font-size="12" fill="#F0F6FC">#4a19 <tspan fill="#F59E0B">PROG</tspan>    Sync dual schemas</text>
    <text x="24" y="205" font-family="'Geist Mono', monospace" font-size="12" fill="#F0F6FC">#2b88 <tspan fill="#3FB950">DONE</tspan>    Rust engine SIMD parser</text>
  </g>
</svg>
```

---
