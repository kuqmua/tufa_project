FROM postgres:16 AS builder
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential \
    libpq-dev \
    postgresql-server-dev-16 \
    curl \
    pkg-config \
    cmake \
    git \
    ca-certificates \
    libreadline6-dev \
    zlib1g-dev
ENV CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup \
    PATH=/usr/local/cargo/bin:$PATH
USER root
#start this need coz curl rust install command not work in my region
COPY rustup-init.sh /rustup-init.sh
RUN chmod +x /rustup-init.sh
RUN \
  /rustup-init.sh -y --no-modify-path --profile minimal --default-toolchain stable
#end this need coz curl rust install command not work in my region
WORKDIR /home/supa
COPY . .
RUN cargo install cargo-pgrx --version 0.16.1 --locked
RUN cargo pgrx init --pg16 $(which pg_config)
RUN cargo pgrx install

FROM postgres:16
COPY --from=builder /usr/lib/postgresql/16/lib /usr/lib/postgresql/16/lib
COPY --from=builder /usr/share/postgresql/16/extension /usr/share/postgresql/16/extension
USER postgres
