# PROPOSAL 2: "THE TERMINAL CARET & HYPHEN"

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
- **Display / Wordmark:** *Geist* (Bold, uppercase `LIDA` tracking `+0.05em`)
- **Body & Docs:** *Geist* (Regular 400)
- **Monospace CLI:** *Geist Mono* (Regular & SemiBold)

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
