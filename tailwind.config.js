/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./web/src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
    },
  },
  theme: {
    extend: {},
  },
  plugins: [],
}
