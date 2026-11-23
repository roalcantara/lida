# BRAND DESIGN IDENTITY & DESIGN SYSTEM SPECIFICATION: `lida`

**Companion to `toma` developer CLI**\
_Core Philosophy: "Zero ceremony. Dual-format. Git-native task management for developers and AI coding agents."_

---

# SECTION A: MASTER BRAND ARCHITECTURE & COPYWRITING DEFINITION

### 1. Elevator Pitch

> **`lida` is a blazingly fast Rust CLI that bridges the human-agent gap in project task tracking by maintaining bidirectional zero-loss sync between Markdown (`TODO.md`) and structured YAML (`TODO.yaml`). It turns Git repositories into self-documenting task engines with zero external servers, conflict-resilient short IDs, and instant CLI ergonomics.**

### 2. Tagline Alternatives

1. _"Zero ceremony. Dual-format. Tasks in Git."_
2. _"The deterministic bridge between `TODO.md` and `TODO.yaml`."_
3. _"Engineered for developers. Typed for AI coding agents."_

### 3. Key Value Pillars

1. **Dual Native (Human & Agent Synchrony):** Developers review clean, readable Markdown check-lists in PRs; LLMs and background automation query and mutate structured, schema-validated YAML with typed keys (`--meta key=val`) and instant `--json` output.
2. **Git-First & Local-Only:** Single-file or dual-file repo tracking. Conflict-resilient 4-character base36 hash IDs (`#a8f2`) minimize Git rebase collisions and enable painless multi-branch work without centralized database locks.
3. **Rust-Engineered Performance:** Sub-millisecond cold start execution (<4ms), zero runtime dependencies, instant AST parsing of GFM task lists, and rock-solid deterministic formatting.

### 4. Voice, Tone & Persona Guidelines

- **Persona:** Precision toolmaker, tactical UNIX utility, silent collaborator.
- **Attributes:** Concise, humble, zero-bullshit, deterministic, dev-first.
- **CLI Message UX Design:**
  - _Success:_ Subtle dim green status glyph with terse summary: `✓ added #8f1b "Implement tree shaking" [TODO.md, TODO.yaml]`
  - _Error:_ Clear error code, line/col diagnosis, exact suggested resolution:
    ```text
    error[E042]: Malformed YAML block in TODO.yaml:14:6
      --> TODO.yaml:14:6
       |
    14 |   tags: [frontend,
       |         ^ unclosed bracket
       = help: run `lida fix --lint` to auto-repair schema syntax
    ```

---

# 10 CONCEPTUALLY DIVERSE DESIGN PROPOSALS

Each proposal includes:

- Concept & Metaphor
- Inline Vector SVG Primary Logo
- Dark & Light Color Values & functional roles
- Typography stack (Display, Body, Monospace)
- ASCII Terminal Glyph
- GitHub README Header Banner Mockup
- Favicon / App Icon spec

---

## PROPOSAL 01: "THE DUAL FLUX" (Nordic Functionalism)

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

## PROPOSAL 02: "THE TERMINAL CARET & HYPHEN" (Pure CLI Minimal)

### 1. Concept & Geometry

- **Metaphor:** The terminal input prompt `>` transforming into a Markdown list bullet `- [ ]` and YAML sequence hyphen `- `. It captures `lida`'s command-line roots and instant keyboard speed.
- **Aesthetic:** Utilitarian, typewriter-inspired monospace geometry.

### 2. Primary Mark (SVG Specification)

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="100" height="100" fill="none">
  <rect width="100" height="100" rx="16" fill="#09090B"/>
  <!-- Terminal Prompt Caret -->
  <path d="M26 32L46 50L26 68" stroke="#06B6D4" stroke-width="8" stroke-linecap="round" stroke-linejoin="round"/>
  <!-- Sequence Hyphen / List Bar -->
  <line x1="56" y1="50" x2="76" y2="50" stroke="#FAFAFA" stroke-width="8" stroke-linecap="round"/>
  <!-- Dot Matrix anchor -->
  <circle cx="76" cy="32" r="4" fill="#06B6D4"/>
</svg>
```

### 3. Calibrated Color Palette

- `Base Neutral Dark`: `#09090B` (Zinc-950)
- `Base Neutral Light`: `#FAFAFA` (Zinc-50)
- `Surface / Border`: `#27272A` (Zinc-800)
- `Signature Accent`: `#06B6D4` (Hyper Cyan / Terminal Phosphor)
- `Status Hues`:
  - `Todo`: `#71717A` (Zinc-500)
  - `InProgress`: `#06B6D4` (Hyper Cyan)
  - `Done`: `#22C55E` (Matrix Green)
  - `Blocked`: `#EF4444` (Laser Red)

### 4. Typography Hierarchy

- **Display / Wordmark:** _Geist_ (Bold, uppercase `LIDA` tracking `+0.05em`)
- **Body & Docs:** _Geist_ (Regular 400)
- **Monospace CLI:** _Geist Mono_ (Regular & SemiBold)

### 5. Terminal ASCII Art (`lida --help`)

```text
 _     ___ ____    _
| |   |_ _|  _ \  / \
| |    | || | | |/ _ \
| |___ | || |_| / ___ \
|_____|___|____/_/   \_\ [GIT-NATIVE ENGINE]
```

### 6. GitHub README Banner Mockup Spec

- **Dimensions:** 1200x380px.
- **Palette:** Zinc-950 `#09090B` background with subtle 1px Zinc-800 grid pattern (`size 24px`).
- **Composition:** Centered bold cyan prompt icon `> -`, crisp `lida` wordmark with subline `"Dual-format project task engine (YAML · MD)"`, flanked by minimal benchmark metric pill `"Cold start: 1.2ms in Rust"`.

---

## PROPOSAL 03: "THE BINARY ANCHOR / COMPILER" (Industrial Rust Engine)

### 1. Concept & Geometry

- **Metaphor:** A stylized lower-case letter `l` forged with a check-box node `[✓]`, evoking native machine compilation and deterministic file state.
- **Aesthetic:** German Bauhaus industrial engineering, precision line weight, mechanical sharpness.

### 2. Primary Mark (SVG Specification)

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="100" height="100" fill="none">
  <rect width="100" height="100" rx="24" fill="#111318"/>
  <!-- Outer precision bounding bracket -->
  <rect x="22" y="22" width="56" height="56" rx="8" stroke="#323842" stroke-width="4"/>
  <!-- The 'l' and check stem -->
  <path d="M36 28V68H64" stroke="#FF5722" stroke-width="8" stroke-linecap="square"/>
  <circle cx="58" cy="42" r="5" fill="#E2E8F0"/>
</svg>
```

### 3. Calibrated Color Palette

- `Base Neutral Dark`: `#111318` (Carbon Slate)
- `Base Neutral Light`: `#F8FAFC` (Slate-50)
- `Surface / Border`: `#242A35` (Alloy Boundary)
- `Signature Accent`: `#FF5722` (Rust Oxide / Forge Orange)
- `Status Hues`:
  - `Todo`: `#64748B` (Slate-500)
  - `InProgress`: `#FF5722` (Forge Orange)
  - `Done`: `#10B981` (Spruce Emerald)
  - `Blocked`: `#E11D48` (Rose-600)

### 4. Typography Hierarchy

- **Display / Wordmark:** _Outfit_ (SemiBold 600, tight kerning)
- **Body & Docs:** _Outfit_ (Regular 400)
- **Monospace CLI:** _JetBrains Mono_ (Engineered code readability)

### 5. Terminal ASCII Art (`lida --help`)

```text
  ___   __  ___   ____
 / / / / / / _ \ / __ \
/ / / / / / // // /_/ /
\_\_\/_/ /____/ \__,_/  [FAST RUST CLI]
```

---

## PROPOSAL 04: "THE GIT DAG CONVERGENCE" (Version Control Native)

### 1. Concept & Geometry

- **Metaphor:** Two Git commit branches (representing `TODO.md` and `TODO.yaml`) diverging from a common origin and merging cleanly without merge conflicts.
- **Aesthetic:** Graph theory visualization, nodal connections, sleek data topology.

### 2. Primary Mark (SVG Specification)

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="100" height="100" fill="none">
  <rect width="100" height="100" rx="20" fill="#0A0C10"/>
  <!-- Git Branch Paths -->
  <path d="M30 74V26" stroke="#2563EB" stroke-width="6" stroke-linecap="round"/>
  <path d="M30 50C48 50 56 36 70 36" stroke="#38BDF8" stroke-width="6" stroke-linecap="round"/>
  <!-- Commit Nodes -->
  <circle cx="30" cy="26" r="7" fill="#0A0C10" stroke="#2563EB" stroke-width="5"/>
  <circle cx="30" cy="74" r="7" fill="#0A0C10" stroke="#2563EB" stroke-width="5"/>
  <circle cx="70" cy="36" r="7" fill="#0A0C10" stroke="#38BDF8" stroke-width="5"/>
</svg>
```

### 3. Calibrated Color Palette

- `Base Neutral Dark`: `#0A0C10` (Abyssal Navy-Black)
- `Base Neutral Light`: `#F1F5F9` (Slate-100)
- `Surface / Border`: `#1E293B` (Slate-800)
- `Signature Accent`: `#2563EB` (Cobalt Precision Blue) & `#38BDF8` (Sky Tint)
- `Status Hues`:
  - `Todo`: `#64748B` (Neutral Branch)
  - `InProgress`: `#2563EB` (Cobalt Blue)
  - `Done`: `#059669` (Emerald Git Tag)
  - `Blocked`: `#DC2626` (Merge Conflict Crimson)

### 4. Typography Hierarchy

- **Display / Wordmark:** _Satoshi_ (Bold 700)
- **Body & Docs:** _Instrument Sans_
- **Monospace CLI:** _Cascadia Code_

### 5. Terminal ASCII Art (`lida --help`)

```text
*-- lida [v0.4]
|   |-- TODO.md   (human)
|   \-- TODO.yaml (agent)
```

---

## PROPOSAL 05: "THE TACTICAL STENCIL" (Monochrome Brutalism)

### 1. Concept & Geometry

- **Metaphor:** High-density military stencil lettering meets military avionics HUD displays. Uncompromising, stark contrast, zero ornamental fluff.
- **Aesthetic:** Swiss graphic neo-brutalism, monospaced grid alignment, pure black & stark white.

### 2. Primary Mark (SVG Specification)

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="100" height="100" fill="none">
  <rect width="100" height="100" rx="4" fill="#000000"/>
  <!-- Razor Stencil Letter L & I -->
  <rect x="22" y="24" width="8" height="52" fill="#FFFFFF"/>
  <rect x="22" y="68" width="28" height="8" fill="#FFFFFF"/>
  <!-- Dot / Caret -->
  <rect x="42" y="24" width="8" height="8" fill="#FFFFFF"/>
  <rect x="42" y="38" width="8" height="22" fill="#FFFFFF"/>
  <!-- Status Matrix corner ticks -->
  <path d="M12 18V12H18M82 12H88V18M88 82V88H82M18 88H12V82" stroke="#FFFFFF" stroke-width="2"/>
</svg>
```

### 3. Calibrated Color Palette

- `Base Neutral Dark`: `#000000` (Void Pitch)
- `Base Neutral Light`: `#FFFFFF` (Pure Specular)
- `Surface / Border`: `#262626` (Neutral-800)
- `Signature Accent`: `#FFFFFF` (Stark White Monochrome) with `#E11D48` (Sole Tactical Alert Red)
- `Status Hues`:
  - `Todo`: `#737373` (50% Gray)
  - `InProgress`: `#FFFFFF` (Solid Inverted)
  - `Done`: `#A3A3A3` (Check Marked)
  - `Blocked`: `#E11D48` (Alert Red)

### 4. Typography Hierarchy

- **Display / Wordmark:** _Geist Mono_ (Heavy, uppercase `LIDA`, stencil track `0.2em`)
- **Body & Docs:** _Geist_
- **Monospace CLI:** _Geist Mono_

### 5. Terminal ASCII Art (`lida --help`)

```text
[L] [I] [D] [A] :: DUAL_FORMAT_ENGINE
```

---

## PROPOSAL 06: "THE AGENT PROTOCOL" (Cybernetic Cyber-Teal)

### 1. Concept & Geometry

- **Metaphor:** A cybernetic socket connecting an autonomous AI Coding Agent (Cursor, Claude, Copilot) to raw local repo storage.
- **Aesthetic:** Sci-fi telemetry, micro-circuitry, luminescent teal on deep obsidian.

### 2. Primary Mark (SVG Specification)

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="100" height="100" fill="none">
  <rect width="100" height="100" rx="18" fill="#06090E"/>
  <!-- Agent Sensor Aperture -->
  <circle cx="50" cy="50" r="30" stroke="#132B32" stroke-width="4"/>
  <circle cx="50" cy="50" r="20" stroke="#0D9488" stroke-width="3" stroke-dasharray="4 4"/>
  <!-- Core Data Spark -->
  <polygon points="50,34 62,50 50,66 38,50" fill="#14B8A6"/>
  <!-- Status Pin -->
  <circle cx="50" cy="50" r="4" fill="#F0FDFA"/>
</svg>
```

### 3. Calibrated Color Palette

- `Base Neutral Dark`: `#06090E` (Deep Space Obsidian)
- `Base Neutral Light`: `#F0FDFA` (Teal-50)
- `Surface / Border`: `#132B32` (Sub-aquatic Boundary)
- `Signature Accent`: `#14B8A6` (Hyper Teal / Bio-Phosphor)
- `Status Hues`:
  - `Todo`: `#5EEAD4` (Dim Teal)
  - `InProgress`: `#14B8A6` (Active Signal Teal)
  - `Done`: `#10B981` (Spruce Green)
  - `Blocked`: `#F43F5E` (Laser Crimson)

### 4. Typography Hierarchy

- **Display / Wordmark:** _Cabinet Grotesk_ (Medium 500)
- **Body & Docs:** _Instrument Sans_
- **Monospace CLI:** _Geist Mono_

---

## PROPOSAL 07: "THE PRAGMATIC SQUARE / TOMA SIBLING" (CLI Sister Suite)

### 1. Concept & Geometry

- **Metaphor:** Designed to look immediately continuous alongside the `toma` CLI family. A pair of balanced modular task squares sharing identical edge radii, symbolizing paired terminal tools.
- **Aesthetic:** Clean Japanese visual ergonomics, modular grids, friendly precision.

### 2. Primary Mark (SVG Specification)

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="100" height="100" fill="none">
  <rect width="100" height="100" rx="22" fill="#18181B"/>
  <!-- Top Left Square (MD) -->
  <rect x="26" y="26" width="20" height="20" rx="6" fill="#F43F5E"/>
  <!-- Bottom Right Square (YAML) -->
  <rect x="54" y="54" width="20" height="20" rx="6" fill="#FAFAFA"/>
  <!-- Synchronizing connecting diagonal -->
  <line x1="36" y1="36" x2="64" y2="64" stroke="#F43F5E" stroke-width="4" stroke-dasharray="2 3"/>
</svg>
```

### 3. Calibrated Color Palette

- `Base Neutral Dark`: `#18181B` (Zinc-900)
- `Base Neutral Light`: `#FAFAFA` (Zinc-50)
- `Surface / Border`: `#27272A` (Zinc-800)
- `Signature Accent`: `#F43F5E` (Rose Quartz / Electric Coral)
- `Status Hues`:
  - `Todo`: `#A1A1AA` (Zinc-400)
  - `InProgress`: `#F43F5E` (Rose Coral)
  - `Done`: `#22C55E` (Emerald)
  - `Blocked`: `#E11D48` (Crimson)

### 4. Typography Hierarchy

- **Display / Wordmark:** _Satoshi_ (Medium 500, lowercase `lida`)
- **Body & Docs:** _Satoshi_ (Regular 400)
- **Monospace CLI:** _JetBrains Mono_

---

## PROPOSAL 08: "THE ISO-CUBE / COMPONENT CHIP" (Hardware Logic)

### 1. Concept & Geometry

- **Metaphor:** An isometric micro-chip whose 3 visible facets correspond to: 1. `TODO.md` (Human face), 2. `TODO.yaml` (Machine face), and 3. `Git HEAD` (Foundation).
- **Aesthetic:** Clean isometric projection without gradients, sharp razor edges.

### 2. Primary Mark (SVG Specification)

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="100" height="100" fill="none">
  <rect width="100" height="100" rx="20" fill="#0C0E12"/>
  <!-- Top Facet -->
  <polygon points="50,22 76,36 50,50 24,36" fill="#6366F1" fill-opacity="0.4" stroke="#6366F1" stroke-width="3"/>
  <!-- Left Facet -->
  <polygon points="24,36 50,50 50,78 24,64" fill="#1E1B4B" stroke="#6366F1" stroke-width="3"/>
  <!-- Right Facet -->
  <polygon points="50,50 76,36 76,64 50,78" fill="#312E81" stroke="#818CF8" stroke-width="3"/>
  <!-- Center Mark Point -->
  <circle cx="50" cy="50" r="4" fill="#FFFFFF"/>
</svg>
```

### 3. Calibrated Color Palette

- `Base Neutral Dark`: `#0C0E12` (Onyx Slate)
- `Base Neutral Light`: `#F5F3FF` (Indigo-50)
- `Surface / Border`: `#1E222B` (Boundary Slate)
- `Signature Accent`: `#6366F1` (Electric Indigo / Deep Violet)
- `Status Hues`:
  - `Todo`: `#64748B` (Slate)
  - `InProgress`: `#6366F1` (Electric Indigo)
  - `Done`: `#10B981` (Spruce)
  - `Blocked`: `#F43F5E` (Rose)

### 4. Typography Hierarchy

- **Display / Wordmark:** _Instrument Sans_ (SemiBold 600)
- **Body & Docs:** _Instrument Sans_
- **Monospace CLI:** _Geist Mono_

---

## PROPOSAL 09: "THE RAW TOKEN / LEXER" (Compiler AST)

### 1. Concept & Geometry

- **Metaphor:** The tokenization stream of a parser engine: raw punctuation symbols `[ ]`, `:`, and `#` colliding to form the lowercase word `lida`.
- **Aesthetic:** Deep developer inside joke, compiler theory elegance, ASCII purism.

### 2. Primary Mark (SVG Specification)

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="100" height="100" fill="none">
  <rect width="100" height="100" rx="16" fill="#0A0A0C"/>
  <!-- The Hash Symbol (#) + Colon (:) fusion -->
  <line x1="32" y1="28" x2="32" y2="72" stroke="#E2E8F0" stroke-width="6" stroke-linecap="round"/>
  <circle cx="56" cy="38" r="5" fill="#EAB308"/>
  <circle cx="56" cy="62" r="5" fill="#EAB308"/>
  <line x1="68" y1="28" x2="68" y2="72" stroke="#334155" stroke-width="6" stroke-linecap="round"/>
</svg>
```

### 3. Calibrated Color Palette

- `Base Neutral Dark`: `#0A0A0C` (Carbon Black)
- `Base Neutral Light`: `#FEFCE8` (Warm Ivory)
- `Surface / Border`: `#1E1E24` (Subtle boundary)
- `Signature Accent`: `#EAB308` (Ochre Gold / Compiler Yellow)
- `Status Hues`:
  - `Todo`: `#64748B` (Slate)
  - `InProgress`: `#EAB308` (Ochre Gold)
  - `Done`: `#22C55E` (Forest)
  - `Blocked`: `#EF4444` (Amber Red)

### 4. Typography Hierarchy

- **Display / Wordmark:** _Cabinet Grotesk_ (Bold 700)
- **Body & Docs:** _Instrument Sans_
- **Monospace CLI:** _JetBrains Mono_

---

## PROPOSAL 10: "THE SPEED RUNNER / HYPER-THREAD" (Zero-Latency Rust)

### 1. Concept & Geometry

- **Metaphor:** Sub-millisecond execution threads cutting through file I/O latency. A dynamic speed-line slash cutting across an origami task card.
- **Aesthetic:** Kinetic minimalism, aerodynamic angles (18° italic slant), ultra-fast CLI feel.

### 2. Primary Mark (SVG Specification)

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="100" height="100" fill="none">
  <rect width="100" height="100" rx="20" fill="#0F1115"/>
  <!-- High velocity slanted glyphs -->
  <path d="M34 76L46 24H56L44 76H34Z" fill="#10B981"/>
  <path d="M54 48L64 24H74L64 48H54Z" fill="#E2E8F0"/>
  <!-- Zero-latency dot -->
  <circle cx="68" cy="68" r="6" fill="#10B981"/>
</svg>
```

### 3. Calibrated Color Palette

- `Base Neutral Dark`: `#0F1115` (Velocity Charcoal)
- `Base Neutral Light`: `#F0FDF4` (Mint Cream)
- `Surface / Border`: `#222731` (Track Line)
- `Signature Accent`: `#10B981` (Nordic Spruce / Hyper Speed Emerald)
- `Status Hues`:
  - `Todo`: `#64748B` (Track Slate)
  - `InProgress`: `#38BDF8` (Aero Blue)
  - `Done`: `#10B981` (Speed Emerald)
  - `Blocked`: `#F43F5E` (Velocity Red)

### 4. Typography Hierarchy

- **Display / Wordmark:** _Outfit_ (Black 900, Italic slant `-8deg`)
- **Body & Docs:** _Outfit_ (Regular 400)
- **Monospace CLI:** _Geist Mono_

---

# COMPARATIVE SUMMARY MATRIX

| Proposal                 | Metaphor & Archetype             | Signature Accent          | Key Font Stack               | Ideal Resonance                      |
| :----------------------- | :------------------------------- | :------------------------ | :--------------------------- | :----------------------------------- |
| **01. Dual Flux**        | Chevrons & colon transition      | Safety Amber `#F59E0B`    | Cabinet Grotesk / Geist Mono | Most balanced, canonical dev-tool    |
| **02. Terminal Caret**   | Command prompt `>` meets `-`     | Hyper Cyan `#06B6D4`      | Geist / Geist Mono           | Pure terminal purists & minimalists  |
| **03. Binary Anchor**    | Rust compiler industrial stamp   | Forge Orange `#FF5722`    | Outfit / JetBrains Mono      | Systems programming, blazing speed   |
| **04. Git DAG**          | Dual commit convergence          | Cobalt Blue `#2563EB`     | Satoshi / Cascadia Code      | Git-first workflows & PR automations |
| **05. Tactical Stencil** | Monochrome brutalist HUD         | Stark White `#FFFFFF`     | Geist Mono (all)             | Hardcore hacker aesthetic            |
| **06. Agent Protocol**   | Cybernetic socket & aperture     | Hyper Teal `#14B8A6`      | Cabinet Grotesk / Geist Mono | AI coding agent integration          |
| **07. Pragmatic Square** | Sister tool to `toma` CLI        | Rose Coral `#F43F5E`      | Satoshi / JetBrains Mono     | Immediate family tool continuity     |
| **08. Iso-Cube**         | 3-facet data container chip      | Electric Indigo `#6366F1` | Instrument Sans / Geist Mono | Schema-first, high structure         |
| **09. Raw Token**        | AST Lexer punctuation (`#`, `:`) | Ochre Gold `#EAB308`      | Cabinet Grotesk / JetBrains  | Compiler engineers, syntax junkies   |
| **10. Speed Runner**     | Sub-millisecond execution lines  | Speed Emerald `#10B981`   | Outfit / Geist Mono          | Performance zealots, Rust SIMD       |

---

_Generated for `lida` CLI project suite — companion to `toma`._
