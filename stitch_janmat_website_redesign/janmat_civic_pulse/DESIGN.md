---
name: JanMat Civic Pulse
colors:
  surface: '#0f131c'
  surface-dim: '#0f131c'
  surface-bright: '#353943'
  surface-container-lowest: '#0a0e17'
  surface-container-low: '#181b25'
  surface-container: '#1c1f29'
  surface-container-high: '#262a34'
  surface-container-highest: '#31353f'
  on-surface: '#dfe2ef'
  on-surface-variant: '#c7c4d8'
  inverse-surface: '#dfe2ef'
  inverse-on-surface: '#2c303a'
  outline: '#918fa1'
  outline-variant: '#464555'
  surface-tint: '#c3c0ff'
  primary: '#c3c0ff'
  on-primary: '#1d00a5'
  primary-container: '#4f46e5'
  on-primary-container: '#dad7ff'
  inverse-primary: '#4d44e3'
  secondary: '#adc6ff'
  on-secondary: '#002e6a'
  secondary-container: '#0566d9'
  on-secondary-container: '#e6ecff'
  tertiary: '#4edea3'
  on-tertiary: '#003824'
  tertiary-container: '#006e4b'
  on-tertiary-container: '#67f4b7'
  error: '#ffb4ab'
  on-error: '#690005'
  error-container: '#93000a'
  on-error-container: '#ffdad6'
  primary-fixed: '#e2dfff'
  primary-fixed-dim: '#c3c0ff'
  on-primary-fixed: '#0f0069'
  on-primary-fixed-variant: '#3323cc'
  secondary-fixed: '#d8e2ff'
  secondary-fixed-dim: '#adc6ff'
  on-secondary-fixed: '#001a42'
  on-secondary-fixed-variant: '#004395'
  tertiary-fixed: '#6ffbbe'
  tertiary-fixed-dim: '#4edea3'
  on-tertiary-fixed: '#002113'
  on-tertiary-fixed-variant: '#005236'
  background: '#0f131c'
  on-background: '#dfe2ef'
  surface-variant: '#31353f'
  surface-base: '#0A0E17'
  surface-card: '#131A29'
  surface-hover: '#1C263A'
  border-subtle: '#222D42'
  border-active: '#3E4C6D'
  text-primary: '#F8FAFC'
  text-secondary: '#94A3B8'
  text-muted: '#64748B'
  vote-yay: '#10B981'
  vote-nay: '#EF4444'
  vote-abstain: '#F59E0B'
typography:
  display-hero:
    fontFamily: Plus Jakarta Sans
    fontSize: 40px
    fontWeight: '800'
    lineHeight: 48px
    letterSpacing: -0.03em
  headline-lg:
    fontFamily: Plus Jakarta Sans
    fontSize: 32px
    fontWeight: '700'
    lineHeight: 40px
    letterSpacing: -0.02em
  headline-lg-mobile:
    fontFamily: Plus Jakarta Sans
    fontSize: 26px
    fontWeight: '700'
    lineHeight: 34px
    letterSpacing: -0.015em
  headline-md:
    fontFamily: Plus Jakarta Sans
    fontSize: 22px
    fontWeight: '600'
    lineHeight: 28px
    letterSpacing: -0.01em
  headline-sm:
    fontFamily: Plus Jakarta Sans
    fontSize: 18px
    fontWeight: '600'
    lineHeight: 24px
  body-lg:
    fontFamily: Plus Jakarta Sans
    fontSize: 16px
    fontWeight: '400'
    lineHeight: 26px
  body-md:
    fontFamily: Plus Jakarta Sans
    fontSize: 14px
    fontWeight: '400'
    lineHeight: 22px
  body-sm:
    fontFamily: Plus Jakarta Sans
    fontSize: 12px
    fontWeight: '400'
    lineHeight: 18px
  label-md:
    fontFamily: Plus Jakarta Sans
    fontSize: 13px
    fontWeight: '600'
    lineHeight: 16px
    letterSpacing: 0.01em
  label-sm:
    fontFamily: Plus Jakarta Sans
    fontSize: 11px
    fontWeight: '700'
    lineHeight: 14px
    letterSpacing: 0.04em
  metric-bold:
    fontFamily: Plus Jakarta Sans
    fontSize: 20px
    fontWeight: '800'
    lineHeight: 24px
    letterSpacing: -0.02em
rounded:
  sm: 0.5rem
  DEFAULT: 1rem
  md: 1.5rem
  lg: 2rem
  xl: 3rem
  full: 9999px
spacing:
  gutter: 1rem
  margin: 1rem
  space-2xs: 0.25rem
  space-xs: 0.5rem
  space-sm: 0.75rem
  space-md: 1rem
  space-lg: 1.5rem
  space-xl: 2rem
  space-2xl: 3rem
---

## Brand & Style

This design system is tailored for an elevated civic opinion and community consensus platform. The aesthetic merges the authority and credibility of democratic governance with the kinetic, responsive feel of modern consumer social products. 

The personality is objective, articulate, transparent, and quietly confident. It rejects both the clinical sterility of legacy bureaucracy and the chaotic noise of traditional algorithmic social feeds. Visual cues emphasize verified discourse, measurable consensus, and democratic participation. 

The visual style blends **Contemporary Dark Tech Minimalist** with **Subtle Glassmorphic Depth**:
- Immersive, pitch-depth backgrounds that minimize eye strain during intensive reading and live ballot tracking.
- Electric indigo and royal blue focal accents that guide attention directly to key civic actions (voting, debating, certifying).
- Soft pillular geometry paired with surgical 1px micro-borders to maintain structural order without harsh visual tension.

## Colors

The color palette is engineered specifically for deep-contrast civic workflows in low-light environments.

- **Primary (`#4F46E5`):** The signature electric indigo anchors brand identity, primary navigation tabs, and committed user submissions.
- **Secondary (`#3B82F6`):** Radiant cobalt serves as an interactive highlight for state transitions, active filter pills, and live voter percentage markers.
- **Tertiary (`#10B981`):** Verified green represents affirmative consensus, passing thresholds, verified badge verification, and constructive metrics.
- **Neutral Base (`#0A0E17`):** An obsidian navy tone that recedes backward, avoiding the harshness of pure `#000000` while granting depth.

### Named Civic Accents
- `vote-yay` (`#10B981`), `vote-nay` (`#EF4444`), and `vote-abstain` (`#F59E0B`) provide neutral, immediate triage for voting ratios.
- Subtle borders utilize `#222D42` to delineate content boundaries cleanly against dark surfaces.

## Typography

The typographic hierarchy relies on **Plus Jakarta Sans** across all levels. Its balanced geometric proportions offer clarity at small data points alongside friendly, modern presence on civic headlines.

- **Display & Headings:** Expressed with negative letter tracking and heavy weights (`700` and `800`) to evoke conviction and focal structure on mobile viewpoints.
- **Editorial & Body:** Set with open line heights (`1.5` to `1.6`) to maintain readability across sustained political and civic deliberation texts.
- **Labels & Metrics:** Applied with elevated weights (`600` to `800`) and slight uppercase tracking for badges, voting splits, and community categories.

## Layout & Spacing

This design system uses a strict mobile-first fluid layout configured to optimize thumb zones and one-handed reading.

- **Mobile (<640px):** Single-column stack with `1rem` outer canvas margin and `1rem` vertical gaps. All critical voting triggers reside in the bottom 40% of the screen.
- **Tablet (640px–1024px):** 6-column fluid structure with `1.5rem` margins and nested split views for topic overview and opinion threads.
- **Desktop (>1024px):** 12-column centered layout constrained to a maximum content container of `1180px`, separating community navigation, active voting streams, and trending analytic sidebars.

## Elevation & Depth

Visual hierarchy does not use high-drop muddy drop shadows. Instead, it relies on tonal stacking and subtle ambient luminescence:

- **Level 0 (Canvas):** Base background `#0A0E17`.
- **Level 1 (Cards & Modules):** Surface `#131A29` framed with a 1px solid micro-border (`#222D42`).
- **Level 2 (Interactive Modals & Menus):** Surface `#1C263A` complemented by a 12px outer blur with 15% opacity primary glow (`rgba(79, 70, 229, 0.15)`).
- **Level 3 (Floating Voting Bar / Action CTA):** Full pill boundary with a subtle top edge highlight (`border-t: 1px solid rgba(255, 255, 255, 0.1)`) and diffuse shadow `0 8px 32px rgba(0, 0, 0, 0.45)`.

## Shapes

The interface embraces a **pill-shaped** (`3`) design philosophy that softens the gravity of civic debates, making the platform feel welcoming and modern.

- **Buttons & Tabs:** Fully pillular (`rounded-full` / `9999px`) for touch targets, secondary chips, and filter toggles.
- **Cards & Modals:** Large rounded styling (`2rem` / `32px` on desktop, `1.5rem` / `24px` on mobile) ensures cards feel touchable and self-contained.
- **Inputs & Sliders:** Full pill boundaries for search, single-line response inputs, and segmented voting selectors.

## Components

### Buttons & CTAs
- **Primary Action Button:** Gradient or solid `#4F46E5` with `#F8FAFC` label, full pill radius, `12px 24px` padding, displaying a gentle blue ambient rim glow on hover/focus.
- **Secondary / Ghost Button:** Transparent background with a `1px` solid `#222D42` border, shifting to `#1C263A` on touch.

### Opinion & Poll Cards
- Contained within `#131A29` with `1.25rem` padding and `1.25rem` radius.
- Embedded progress bars use a segmented dual-color track (Indigo for Option A, Emerald for Option B, Slate for neutral).
- Interactive voting chips show percentage changes dynamically upon click with a haptic scale feedback (`scale(0.98)`).

### Community Navigation Chips
- Horizontal scrolling pill lists with low-contrast background `#131A29` when inactive and solid `#4F46E5` with white text when active.

### Live Status Badges
- Encapsulated pills featuring an inner pulsing green or blue status dot (`6px`), a subtle semi-transparent background (`rgba(59, 130, 246, 0.1)`), and text in `label-sm`.

### Form Fields & Inputs
- Dark input containers (`#0A0E17`) set into `#131A29` cards, outlined with `1px` `#222D42`. Focused state transitions immediately to `#3B82F6` with an inner 2px glow ring.