# -----------------------------------------------------
# Counsel AI — Desktop UI Dockerfile
# -----------------------------------------------------
# React + Tauri app for local legal reasoning.
# Uses Vite for hot reload and local development.

FROM node:22-slim as build
WORKDIR /app

# Install dependencies
COPY package*.json ./
RUN npm install --legacy-peer-deps

# Copy source code
COPY . .

# Build React front-end
RUN npm run build

# -----------------------------------------------------
# Final lightweight Nginx runtime
# -----------------------------------------------------
FROM nginx:stable-alpine
WORKDIR /usr/share/nginx/html

# Copy built assets
COPY --from=build /app/dist .

# Replace default Nginx config
RUN rm /etc/nginx/conf.d/default.conf
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Expose UI port
EXPOSE 5173

CMD ["nginx", "-g", "daemon off;"]
