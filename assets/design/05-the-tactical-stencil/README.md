# PROPOSAL 5: "THE TACTICAL STENCIL"

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
