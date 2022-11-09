FROM rust:1.62.1 AS rust-builder

WORKDIR /workspace
COPY . .
RUN cargo build --release && \
    ls -Rtlh target/release

FROM registry.access.redhat.com/ubi8/ubi:latest

WORKDIR /
COPY --from=rust-builder /workspace/target/release/tterm .
USER 65532:65532

ENTRYPOINT ["/tterm"]
