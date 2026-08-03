ARG RUST_TARGET=x86_64-unknown-linux-gnu

## CARGO CHEF ##
FROM lukemathwalker/cargo-chef:latest-rust-slim AS chef
ARG RUST_TARGET
ENV SQLX_OFFLINE=true
ENV CARGO_BUILD_TARGET=${RUST_TARGET}
WORKDIR /work


## PLAN ##
FROM chef AS plan
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


## BUILD ##
FROM chef AS build
COPY --from=plan /work/recipe.json recipe.json

# Build dependencies
RUN cargo chef cook --release --locked -p api -p update-agent --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release --locked -p api -p update-agent


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
ARG RUST_TARGET
WORKDIR /app

COPY --from=tools /tools/ /bin/
COPY --from=build /work/target/${RUST_TARGET}/release/api .
COPY --from=build /work/target/${RUST_TARGET}/release/update-agent .

EXPOSE 7878
ENTRYPOINT ["/app/api"]
