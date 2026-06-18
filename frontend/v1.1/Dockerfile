# Stage 1: Build the client assets
FROM oven/bun:1-alpine AS builder

WORKDIR /app

# Copy lock and package files
COPY package.json bun.lock /app/

# Install dependencies using bun
RUN bun install --frozen-lockfile

# Copy source code and build
COPY . /app/
RUN bun run build

# Stage 2: Minimal runtime image
FROM oven/bun:1-alpine

WORKDIR /app

# Copy dependencies and built output
COPY package.json bun.lock /app/
RUN bun install --frozen-lockfile --production

COPY --from=builder /app/build /app/build

EXPOSE 3000
ENV PORT=3000
CMD ["bun", "run", "start"]