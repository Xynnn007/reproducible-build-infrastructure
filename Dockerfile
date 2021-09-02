FROM rust:1.54.0 as builder

LABEL maintainer="Ding Ma <ding.ma@linux.alibaba.com>"

WORKDIR /root

COPY ./ /root

RUN rustup component add rustfmt && \
    cargo build --release

FROM centos:8.3.2011 as prod

WORKDIR /root/

COPY --from=0 /root/target/release/reproducible-build-infrastructure .

COPY --from=0 /root/config/config.toml /etc/rbi/config.toml

CMD ["./reproducible-build-infrastructure", "-c", "/etc/rbi/config.toml"]
