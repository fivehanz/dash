/** @type {import('tailwindcss').Config} */
// tailwind.config.js
const { nextui } = require('@nextui-org/react');

const { createGlobPatternsForDependencies } = require('@nx/react/tailwind');
const { join } = require('path');

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    join(__dirname, '{components, app}/**/*!(*.stories|*.spec).{ts,tsx,html}'),
    join(__dirname, '{app, components}/**/*.{js,ts,jsx,tsx,mdx}'),
    ...createGlobPatternsForDependencies(__dirname),
    '../../node_modules/@nextui-org/theme/dist/**/*.{js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {},
  },
  darkMode: 'class',
  plugins: [nextui()],
};
