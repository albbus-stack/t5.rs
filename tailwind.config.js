const withMT = require("@material-tailwind/html/utils/withMT");

/** @type {import('tailwindcss').Config} */
module.exports = withMT({
  content: ["./src/**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [],
});
