FROM messense/rust-musl-cross:aarch64-musl AS build
WORKDIR /update-agent
COPY . .
RUN cargo build --release --target aarch64-unknown-linux-musl

FROM scratch AS export
COPY --from=build /update-agent/target/aarch64-unknown-linux-musl/release/update_agent .