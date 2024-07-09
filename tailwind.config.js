/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./templates/**/*.html",
    "./assets/js/**/*.js"
  ],
  theme: {
    extend: {},
    fontFamily: {
      'mono': ['Fira Code'],
    },
  },
  plugins: [],
  darkMode: 'selector',
};
