module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,html,rs}",
    "./api/**/*.rs"
  ],
  darkMode: 'selector',
  theme: {
    screens: {
      sm: '480px',
      md: '768px',
      lg: '976px',
      xl: '1440px',
    },
    fontFamily: {
      sans: ['Lexend', 'sans-serif'],
      mono: ['firacode', 'monospace'],
    },
    extend: {
      colors: {
        transparent: 'transparent',
        white: {
          DEFAULT: '#FCFCFC',
          dark: '#BFBDB6'
        },
        black: {
          DEFAULT: '#5C6166',
          dark: '#0D1017'
        },
        blue: {
          DEFAULT: '#4CBF99',
          dark: '#95E6CB'
        },
        purple: {
          DEFAULT: '#A37ACC',
          dark: '#D2A6FF'
        },
        red: {
          DEFAULT: '#F07171',
          dark: '#F26D78'
        },
        pink: {
          DEFAULT: '#F07171',
          dark: '#F07178'
        },
        orange: {
          DEFAULT: '#ED9366',
          dark: '#FF8F40'
        },
        green: {
          DEFAULT: '#13ce66',
          dark: '#7FD962'
        },
        yellow: {
          DEFAULT: '#86B300',
          dark: '#E6B450'
        },
        gray: {
          DEFAULT: '#8A9199',
          dark: '#565B66',
          'dark-dark': '#273444',
          'light-dark': '#ACB6BF'
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
