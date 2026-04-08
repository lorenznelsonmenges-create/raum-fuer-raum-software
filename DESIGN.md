```markdown
# Design System Specification: The Curated Sanctuary

## 1. Overview & Creative North Star
The Creative North Star for this design system is **"The Curated Sanctuary."** 

This is not a utility-first interface; it is a digital architectural space designed to evoke the psychological relief of a cleared room. We move beyond the "template" look by rejecting rigid, crowded grids in favor of **Intentional Asymmetry** and **Editorial Breathing Room**. The layout should feel like a high-end interior design journal—sophisticated, quiet, and deeply empathetic. 

By utilizing overlapping elements, varying typographic scales, and a rejection of traditional containment (borders), we create an experience that feels "unfolded" rather than "packaged."

---

## 2. Colors
Our palette is rooted in biophilic design—mimicking the soft, filtered light of a garden studio.

*   **The "No-Line" Rule:** To maintain the "Atemzug" (breath) quality, **1px solid borders are strictly prohibited** for sectioning or containment. Boundaries are defined exclusively through background shifts. For example, a content block using `surface_container_low` (#f5f4ec) should sit directly on a `surface` (#fbf9f3) background.
*   **Surface Hierarchy & Nesting:** Treat the UI as physical layers of fine paper. 
    *   **Level 0 (Base):** `surface` (#fbf9f3)
    *   **Level 1 (Sections):** `surface_container_low` (#f5f4ec)
    *   **Level 2 (Interactive Elements/Cards):** `surface_container_lowest` (#ffffff) for maximum "lift" and clarity.
*   **Signature Textures:** For Hero sections or Primary CTAs, use a soft linear gradient transitioning from `primary` (#526447) to `primary_container` (#d4e9c4) at a 135-degree angle. This introduces a "visual soul" that flat colors lack.
*   **Glassmorphism:** For floating navigation or modals, use `surface` at 80% opacity with a `20px` backdrop blur. This allows the calming sage and beige tones to bleed through, maintaining a sense of spatial continuity.

---

## 3. Typography
The typography is a dialogue between the classical authority of the Serif and the modern clarity of the Sans-Serif.

*   **The Display Voice (Noto Serif):** Used for `display` and `headline` roles. This font carries the emotional weight. It should be typeset with slightly tighter letter-spacing (-0.02em) to feel "bound" and trustworthy. Use `display-lg` (3.5rem) sparingly to introduce major themes.
*   **The Functional Voice (Manrope):** Used for `title`, `body`, and `label` roles. Manrope provides a clean, neutral counterpoint to the Serif. 
*   **Hierarchy as Empathy:** Use extreme scale contrast. A large `headline-lg` (2rem) paired with a much smaller, widely-spaced `label-md` (0.75rem, tracking +0.1em) creates an editorial look that feels expensive and intentional.

---

## 4. Elevation & Depth
Depth in this system is achieved through **Tonal Layering**, not structural artifice.

*   **The Layering Principle:** Instead of shadows, use the `surface_container` tiers. A `surface_container_highest` (#e3e3d9) element placed on a `surface_dim` (#dadbd0) background creates a natural, recessed "carved" effect.
*   **Ambient Shadows:** If a floating effect is required (e.g., a primary action button), use an ultra-diffused shadow: `box-shadow: 0 20px 40px rgba(49, 51, 44, 0.06);`. Note the color: we use a low-opacity version of `on_surface` (#31332c), never pure black, to mimic natural soft-box lighting.
*   **The Ghost Border:** If accessibility requires a border, use `outline_variant` (#b2b3a9) at 15% opacity. It should be felt, not seen.

---

## 5. Components

### Buttons
*   **Primary:** Filled with `primary` (#526447), text in `on_primary` (#ecffdd). Shape: `Rounded-lg` (0.5rem).
*   **Secondary:** Ghost style. No background, no border. Use `title-sm` (Manrope) with a subtle `primary` underline (2px offset).
*   **States:** On hover, primary buttons should shift to `primary_dim` (#46583c) with a slight "lift" (Ambient Shadow).

### Cards & Containers
*   **Constraint:** Zero borders. 
*   **Styling:** Use `surface_container_lowest` (#ffffff) and the `16` (5.5rem) spacing token for internal padding. This "wasteful" use of space is a signal of luxury and mindfulness.

### Input Fields
*   **Visual Style:** Do not use outlined boxes. Use a subtle background fill of `surface_container` (#efeee6) with a `Rounded-sm` (0.125rem) corner. 
*   **Focus State:** Transition the background to `surface_container_highest` (#e3e3d9) with a 1px `primary` bottom-only stroke.

### Lists
*   **Spacing:** Prohibit horizontal dividers. Use the `6` (2rem) spacing scale to separate list items. 
*   **Leading Elements:** Icons should be wrapped in a `primary_container` (#d4e9c4) circle with 40% opacity.

### The "Atemzug" (Breath) Component
A unique layout pattern: A large, empty container (using `24` or 8.5rem spacing) containing only a single centered quote in `headline-sm` (Noto Serif). This reinforces the "Platz schaffen" (creating space) philosophy.

---

## 6. Do's and Don'ts

### Do:
*   **Embrace Whitespace:** Use the `20` (7rem) and `24` (8.5rem) spacing tokens between sections. If it feels "too empty," you are doing it right.
*   **Use Soft Grids:** Align text to a 12-column grid, but allow images and decorative "sage" containers to break the grid and bleed off-screen.
*   **Prioritize Readability:** Ensure `body-lg` has a line-height of at least 1.6 to allow the text to breathe.

### Don't:
*   **Don't use hard corners:** Avoid `none` (0px) roundedness unless for full-bleed images. Use `md` (0.375rem) as the standard for a soft, approachable feel.
*   **Don't use pure black:** Use `on_surface` (#31332c) for all "black" text to maintain warmth.
*   **Don't rush the user:** Avoid aggressive "Buy Now" or high-pressure microcopy. Use empathetic labels like "Begin your journey" or "Explore the space."
*   **No hard dividers:** Never use `#000` or high-contrast lines to separate content. Let the colors do the work.```