/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./templates/**/*.html",
    "./assets/js/**/*.js"
  ],
  theme: {
    extend: {
      colors: {
        blue: {
          theme: '#61afef',
        }
      },
    },
    fontFamily: {
      'mono': ['Fira Code'],
    },
  },
  plugins: [],
  darkMode: 'selector',
};
