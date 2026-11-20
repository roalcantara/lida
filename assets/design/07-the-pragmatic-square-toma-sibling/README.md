# PROPOSAL 7: "THE PRAGMATIC SQUARE / TOMA SIBLING"

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
- **Display / Wordmark:** *Satoshi* (Medium 500, lowercase `lida`)
- **Body & Docs:** *Satoshi* (Regular 400)
- **Monospace CLI:** *JetBrains Mono*

---
