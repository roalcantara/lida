# PROPOSAL 8: "THE ISO-CUBE / COMPONENT CHIP"

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
