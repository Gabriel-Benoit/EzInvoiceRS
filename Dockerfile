FROM debian:buster

RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    ca-certificates curl file \
    #chromium \
    #chromium-sandbox \
    build-essential \
    autoconf automake autotools-dev libtool xutils-dev && \
    rm -rf /var/lib/apt/lists/* 

# Install Google Chrome Stable and fonts
# Note: this installs the necessary libs to make the browser work with Puppeteer.
RUN apt-get update && apt-get install gnupg wget -y && \
    wget --quiet --output-document=- https://dl-ssl.google.com/linux/linux_signing_key.pub | gpg --dearmor > /etc/apt/trusted.gpg.d/google-archive.gpg && \
    sh -c 'echo "deb [arch=amd64] http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list' && \
    apt-get update && \
    apt-get install google-chrome-stable -y --no-install-recommends && \
    rm -rf /var/lib/apt/lists/*

RUN useradd -m app
USER app
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y 

ENV PATH=/home/app/.cargo/bin:$PATH
RUN rustup default nightly

WORKDIR /app

COPY Cargo.toml Cargo.lock  ./
RUN echo 'fn main() {panic!("DUMMY BUILD")}' > dummy.rs
RUN echo 'fn dummy() {panic!("DUMMY BUILD")}' > dummy_lib.rs

RUN sed -i 's#src/bin.rs#dummy.rs#' Cargo.toml
RUN sed -i 's#src/lib.rs#dummy_lib.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/bin.rs#' Cargo.toml
RUN sed -i 's#dummy_lib.rs#src/lib.rs#' Cargo.toml

COPY Rocket.toml ./
COPY src/ ./src/

RUN cargo build --release

