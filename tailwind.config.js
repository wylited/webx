module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts}",
  ],
  darkMode: 'selector',
  theme: {
    screens: {
      sm: '480px',
      md: '768px',
      lg: '976px',
      xl: '1440px',
    },
    colors: {
      'transparent': 'transparent',
      'white': '#FCFCFC',
      'black': '#5C6166',
      'blue': '#4CBF99',
      'purple': '#A37ACC',
      'red': '#F07171',
      'pink': '#F07171',
      'orange': '#ED9366',
      'green': '#13ce66',
      'yellow': '#86B300',
      'gray-dark': '#273444',
      'gray': '#8A9199',
      'gray-light': '#F8F9FA'
    },
    fontFamily: {
      sans: ['Graphik', 'sans-serif'],
      serif: ['Merriweather', 'serif'],
    },
    extend: {
      colors: {
        white: {
          DEFAULT: '#FCFCFC',
          dark: '#BFBDB6' // editor.fg
        },
        black: {
          DEFAULT: '#5C6166',
          dark: '#0D1017' // editor.bg
        },
        blue: {
          DEFAULT: '#4CBF99',
          dark: '#39BAE6' // syntax.tag
        },
        purple: {
          DEFAULT: '#A37ACC',
          dark: '#D2A6FF' // syntax.constant
        },
        red: {
          DEFAULT: '#F07171',
          dark: '#F26D78' // vcs.removed
        },
        pink: {
          DEFAULT: '#F07171',
          dark: '#F07178' // syntax.markup
        },
        orange: {
          DEFAULT: '#ED9366',
          dark: '#FF8F40' // syntax.keyword
        },
        green: {
          DEFAULT: '#13ce66',
          dark: '#7FD962' // vcs.added
        },
        yellow: {
          DEFAULT: '#86B300',
          dark: '#E6B450' // common.accent
        },
        gray: {
          DEFAULT: '#8A9199',
          dark: '#565B66', // ui.fg
          'dark-dark': '#273444',
          'light-dark': '#ACB6BF' // syntax.comment base
        }
      },
      spacing: {
        '128': '32rem',
        '144': '36rem',
      },
      borderRadius: {
        '4xl': '2rem',
      }
    }
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
}
