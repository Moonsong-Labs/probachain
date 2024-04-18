# Node for Probachain
#
# Requires to run from repository root and to copy the binary in the build folder (part of the release workflow)

FROM docker.io/library/ubuntu:20.04 AS builder

# Branch or tag to build probachain from
ARG COMMIT="main"
ARG RUSTFLAGS=""
ENV RUSTFLAGS=$RUSTFLAGS
ENV DEBIAN_FRONTEND=noninteractive

WORKDIR /

RUN echo "*** Installing Basic dependencies ***"
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates
RUN apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler pkg-config

RUN set -e

RUN echo "*** Installing Rust environment ***"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"
RUN rustup default stable
# rustup version are pinned in the rust-toolchain file

RUN echo "*** Cloning Probachain ***"
RUN git clone --depth=1 --branch $COMMIT https://github.com/moonsong-labs/probachain.git

WORKDIR /probachain

RUN echo "*** Building Probachain ***"
RUN cargo build --profile=release --all

FROM debian:bookworm-slim
LABEL maintainer "alan@moonsonglabs.com"
LABEL description="Binary for Probachain Nodes"

RUN useradd -m -u 1000 -U -s /bin/sh -d /probachain probachain && \
	mkdir -p /probachain/.local/share && \
	mkdir /data && \
	chown -R probachain:probachain /data && \
	ln -s /data /probachain/.local/share/probachain && \
	rm -rf /usr/sbin

USER probachain

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder --chown=probachain /probachain/target/release/node-template /probachain/node-template

RUN chmod uog+x /probachain/node-template

# 30333 for blockchain p2p
# 9944 for Websocket & RPC call
# 9615 for Prometheus (metrics)
EXPOSE 30333 9944 9615

VOLUME ["/data"]

ENTRYPOINT ["/probachain/node-template"]
