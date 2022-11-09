# This is the build stage for appchain. Here we create the binary in a temporary image.
FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /appchain
COPY . /appchain

RUN cargo build --locked --release

# This is the 2nd stage: a very small image where we copy the appchain binary."
FROM docker.io/library/ubuntu:20.04

COPY --from=builder /appchain/target/release/appchain-barnacle /usr/local/bin

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates

RUN useradd -m -u 1000 -U -s /bin/sh -d /appchain appchain && \
	mkdir -p /data /appchain/.local/share && \
	chown -R appchain:appchain /data && \
	ln -s /data /appchain/.local/share/appchain && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/appchain-barnacle --version

USER appchain

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/appchain-barnacle"]
