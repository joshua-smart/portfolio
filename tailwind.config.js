/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./templates/**/*.html",
    "./assets/js/**/*.js"
  ],
  theme: {
    extend: {
      colors: {
        theme: {
          blue: '#78dce8',
          red: '#ff6188',
          orange: '#fc9867',
          yellow: '#ffd866',
          green: '#a9dc76',
          purple: '#ab9df2',
        },
        background: '#18222e',
        foreground: '#ffffff',
        secondary: '#71717a',
      },
    },
    fontFamily: {
      'sans': ['Inter'],
      'mono': ['Fira Code'],
    },
  },
  plugins: [],
  darkMode: 'selector',
};
