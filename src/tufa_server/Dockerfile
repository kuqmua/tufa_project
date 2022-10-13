FROM rust:latest as rust-latest-base-image
RUN apt-get update
RUN apt install musl-tools -y
RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup install nightly
RUN ln -s /usr/include/x86_64-linux-gnu/asm /usr/include/x86_64-linux-musl/asm && \    
    ln -s /usr/include/asm-generic /usr/include/x86_64-linux-musl/asm-generic && \
    ln -s /usr/include/linux /usr/include/x86_64-linux-musl/linux && \
    mkdir /musl && \
    wget https://github.com/openssl/openssl/archive/OpenSSL_1_1_1f.tar.gz && \
    tar zxvf OpenSSL_1_1_1f.tar.gz  && \
    cd openssl-OpenSSL_1_1_1f/ && \
    CC="musl-gcc -fPIE -pie" ./Configure no-shared no-async --prefix=/musl --openssldir=/musl/ssl linux-x86_64 && \
    make depend && \
    make -j$(nproc) && \
    make install 
ARG PKG_CONFIG_ALLOW_CROSS=1
ARG OPENSSL_STATIC=true
ARG OPENSSL_DIR=/musl
WORKDIR /usr/src/tufa_server
COPY . .
RUN cargo +nightly build --target=x86_64-unknown-linux-musl --release

FROM alpine:latest
RUN addgroup -g 1000 tufa_server
RUN adduser -D -s /bin/sh -u 1000 -G tufa_server tufa_server
WORKDIR /home/rust_rest/bin/
COPY --from=rust-latest-base-image /usr/src/tufa_server/target/x86_64-unknown-linux-musl/release/tufa_server . 
COPY .env .env
# maybe later add default ENV instead of COPY .env .env
COPY providers_link_parts providers_link_parts
# remove providers_link_parts later coz cannot create file without sudo and creating files in docker is not a good idea
RUN chown tufa_server:tufa_server tufa_server
USER tufa_server
EXPOSE 8080
CMD [ "./tufa_server" ]

# maybe add install diesel cli for postgres(diesel dependency)
# coz maybe would be linking error
# sudo apt install libpq-dev <br/>
# cargo install diesel_cli --no-default-features --features postgres

##################################
# custom linker dependencies
# from https://www.lpalmieri.com/posts/session-based-authentication-in-rust/
# FROM lukemathwalker/cargo-chef:latest-rust-1.56.0 as chef
# WORKDIR /app
# RUN apt update && apt install lld clang -y