module.exports = {
  content: ['./templates/**/*.html.tera'],
  plugins: [
    require("@tailwindcss/forms")({
      strategy: 'base',
    }),
  ],
}