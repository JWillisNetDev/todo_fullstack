/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["index.html", "./src/*.rs"],
  theme: {
    colors: {
      black: '#000',
      white: '#fff',
      green: {
        light: '#1fe9c7',
        DEFAULT: '#0b7261',
        dark: '#074b3f',
      },
      purple: {
        light: '#6350b8',
        DEFAULT: '#2e2459',
        dark: '#1e183a',
      },
      yellow: '#ffc832',
    },
    extend: {
      fontFamily: {
        'alfa': ['Alfa Slab One', 'serif'],
      }
    },
  },
  plugins: [],
}

