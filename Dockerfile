## CARGO CHEF ##
FROM lukemathwalker/cargo-chef:latest-rust-slim AS chef
WORKDIR /work


## PLANNER ##
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


## BUILDER ##
FROM chef AS builder
ENV SQLX_OFFLINE=true
COPY --from=planner /work/recipe.json recipe.json

# Build dependencies
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release --locked --target x86_64-unknown-linux-gnu -p api -p update-agent


## TOOLS ##
FROM busybox:musl AS tools
RUN mkdir -p /tools && \
    cp /bin/busybox /tools/ && \
    cd /tools && \
    ln -s busybox sh && \
    ln -s busybox test && \
    ln -s busybox touch


## RUN ##
FROM gcr.io/distroless/cc-debian13:nonroot
WORKDIR /app

COPY --from=tools /tools/ /bin/
COPY --from=build /work/target/x86_64-unknown-linux-gnu/release/api .
COPY --from=build /work/target/x86_64-unknown-linux-gnu/release/update-agent .

EXPOSE 7878
ENTRYPOINT ["/app/api"]
