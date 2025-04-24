FROM rust:1.86-slim

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    pkg-config \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

WORKDIR /app

# Install Node.js for the web part
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs

# Copy package files
COPY package*.json ./
RUN npm install

# Copy the rest of the application
COPY . .

CMD ["bash"] 