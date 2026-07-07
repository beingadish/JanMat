# syntax=docker/dockerfile:1

# ─────────────────────────────────────────────────────────────
# Build the Vite + React app to static assets.
# ─────────────────────────────────────────────────────────────
FROM node:20-alpine AS build
WORKDIR /app

# Install deps first for better layer caching (lockfile optional).
COPY package.json package-lock.json* ./
RUN npm install

COPY . .
# VITE_API_BASE_URL defaults to "/api", which nginx proxies to the backend.
RUN npm run build

# ─────────────────────────────────────────────────────────────
# Serve the static bundle with nginx + API reverse proxy.
# alpine-slim carries 0 known critical/high CVEs (docker scout).
# ─────────────────────────────────────────────────────────────
FROM nginx:1-alpine-slim AS runtime
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/dist /usr/share/nginx/html
EXPOSE 80
