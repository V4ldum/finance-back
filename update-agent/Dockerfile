FROM rust:slim AS build
ENV SQLX_OFFLINE=true
WORKDIR /work
COPY . .

# Build
RUN cargo build --release --locked --target x86_64-unknown-linux-gnu

FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app

COPY --from=build /work/target/x86_64-unknown-linux-gnu/release/update_agent .
ENTRYPOINT ["/app/update_agent"]
