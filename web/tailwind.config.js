/** @type {import('tailwindcss').Config} */
export default {
  darkMode: "class",
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        "surface-base": "#0A0E17",
        "surface-card": "#131A29",
        "surface-hover": "#1C263A",
        "surface-container-lowest": "#0A0E17",
        "surface-container-low": "#181B25",
        "surface-container": "#1C1F29",
        "surface-container-high": "#262A34",
        "surface-container-highest": "#31353F",
        "border-subtle": "#222D42",
        "border-active": "#3E4C6D",
        primary: "#C3C0FF",
        "primary-container": "#4F46E5",
        "on-primary-container": "#DAD7FF",
        secondary: "#ADC6FF",
        "secondary-container": "#0566D9",
        tertiary: "#10B981",
        "vote-yay": "#10B981",
        "vote-nay": "#EF4444",
        "vote-abstain": "#F59E0B",
        "text-primary": "#F8FAFC",
        "text-secondary": "#94A3B8",
        "text-muted": "#64748B",
      },
      fontFamily: {
        sans: ["Plus Jakarta Sans", "Inter Variable", "system-ui", "sans-serif"],
        mono: ["IBM Plex Mono", "monospace"],
      },
      keyframes: {
        fadeIn: {
          "0%": { opacity: "0", transform: "scale(0.98)" },
          "100%": { opacity: "1", transform: "scale(1)" },
        },
      },
      animation: {
        fadeIn: "fadeIn 0.2s ease-out forwards",
      },
    },
  },
  plugins: [],
};
