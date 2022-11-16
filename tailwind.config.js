module.exports = {
  content: ['./templates/**/*.html.tera'],
  plugins: [
    require('@tailwindcss/forms')({
      strategy: 'base',
    }),
  ],
  theme: {
    extend: {
      colors: {
        'sprout': '#95f9b0',
        'tie': '#ff4355',
        'dark': {
          DEFAULT: '#1c1c1c',
          light: '#262626',
        },
      },
    },
  },
}