module.exports = {
    mode: "jit",
    content: {
        files: ["src/**/*.rs", "**/*.html", "./node_modules/flowbite/**/*.js"],
    },
    darkMode: "media", // 'media' or 'class'
    theme: {
        extend: {},
    },
    variants: {
        extend: {},
    },
    plugins: [
        require('flowbite/plugin')
    ],
};