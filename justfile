generate:
  tailwindcss -i ./styles/input.css -o ./styles/output.css --watch

serve:
  trunk serve --port 3000

serve-open:
  trunk serve --port 3000 --open

dev:
  just generate | just serve