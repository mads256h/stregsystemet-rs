FROM rust:alpine AS build
WORKDIR /build
COPY . .
ENV SQLX_OFFLINE=true
RUN apk add libc-dev
RUN cargo build --release
RUN mkdir -p /opt/stregsystemet/
RUN cp target/release/stregsystemet-rs /opt/stregsystemet
RUN cp -r static/ /opt/stregsystemet/
FROM alpine
LABEL org.opencontainers.image.source="https://github.com/mads256h/stregsystemet-rs"
COPY --from=build /opt/stregsystemet/ /opt/stregsystemet
WORKDIR /opt/stregsystemet
CMD ["/opt/stregsystemet/stregsystemet-rs"]
EXPOSE 8080
