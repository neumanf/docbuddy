const resolve = require('path').resolve

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [resolve(__dirname, 'index.html'), resolve(__dirname, 'src/**/*.{vue,ts}')],
    theme: {
        extend: {},
    },
    plugins: [],
}
