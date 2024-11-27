#!/bin/bash

# Create dist directory if it doesn't exist
mkdir -p dist

# Clean dist directory
rm -rf dist/*

# Build Tailwind CSS
npx tailwindcss -i src/css/input.css -o dist/css/style.css --minify

# Copy HTML files
cp src/*.html dist/

# Copy JavaScript files
cp -r src/js dist/

# If you have org-mode files, convert them
# TODO

# Copy HTMX
cp node_modules/htmx.org/dist/htmx.min.js dist/js/
