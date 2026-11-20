# PROPOSAL 4: "THE GIT DAG CONVERGENCE"

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
