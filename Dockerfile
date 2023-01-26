FROM debian:buster

# Install utilities
RUN apt-get update  --fix-missing && apt-get -y upgrade && \ 
    apt-get install --no-install-recommends -y \
    ca-certificates curl file \
    gnupg2 \
    wget \
    build-essential \
    autoconf automake autotools-dev libtool xutils-dev && \
    rm -rf /var/lib/apt/lists/* 

# Install latest chrome dev package.
RUN wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add - \
    && sh -c 'echo "deb [arch=amd64] http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list' \
    && apt-get update \
    && apt-get install -y google-chrome-stable --no-install-recommends \
    && rm -rf /var/lib/apt/lists/* \
    && rm -rf /src/*.deb

# Add a chrome user and setup home dir.
RUN groupadd --system app && \
    useradd --system --create-home --gid app --groups audio,video app && \
    mkdir --parents /home/app/reports && \
    chown --recursive app:app /home/app

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
