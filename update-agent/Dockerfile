FROM messense/rust-musl-cross:aarch64-musl AS build
ENV SQLX_OFFLINE=true
WORKDIR /update-agent
COPY . .
RUN cargo build --release --locked -q --target aarch64-unknown-linux-musl

FROM scratch AS export
COPY --from=build /update-agent/target/aarch64-unknown-linux-musl/release/update_agent .