# PROPOSAL 3: "THE BINARY ANCHOR / COMPILER"

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
