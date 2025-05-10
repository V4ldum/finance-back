FROM rust:alpine AS build
ENV SQLX_OFFLINE=true
WORKDIR /api
COPY . .

RUN apk update
RUN apk upgrade --no-cache
RUN apk add --no-cache curl lld mold musl musl-dev libc-dev cmake clang clang-dev openssl file \
        libressl-dev git make build-base bash curl wget zip gnupg coreutils gcc g++ zstd binutils ca-certificates upx

RUN cargo build --release --locked -q

FROM alpine AS files

# mailcap is used for content type (MIME type) detection
# tzdata is used for timezones info
RUN apk update && \
    apk upgrade --no-cache && \
    apk add --no-cache ca-certificates mailcap tzdata

RUN update-ca-certificates

# Don't forget to chown files
ENV UID=10001
ENV USER=finance
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


FROM scratch
# the scratch image doesn't have a /tmp folder
WORKDIR /tmp
WORKDIR /api

COPY --from=build /api/target/release/finance-api finance-api

# /etc/nsswitch.conf may be used by some DNS resolvers
# /etc/mime.types may be used to detect the MIME type of files
COPY --from=files /etc/passwd /etc/group /etc/nsswitch.conf /etc/mime.types /etc/
COPY --from=files /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=files /usr/share/zoneinfo /usr/share/zoneinfo

USER finance:finance
EXPOSE 7878

ENTRYPOINT ["./finance-api"]