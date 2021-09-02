FROM rust:1.54.0 as builder

LABEL maintainer="Ding Ma <ding.ma@linux.alibaba.com>"

WORKDIR /root

COPY ./ /root

RUN rustup component add rustfmt && \
    cargo build --release

FROM python:3.6 as prod

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path

ENV PATH "/root/.cargo/bin:${PATH}"

RUN rustup target add x86_64-unknown-linux-musl

RUN pip install "setuptools_rust>=0.12.1" \
        "cffi>=1.14.6" \
        "in-toto==1.0.1" \
        "pathlib==1.0.1"

WORKDIR /root/

COPY --from=0 /root/target/release/reproducible-build-infrastructure .

COPY --from=0 /root/config/config.toml /etc/rbi/config.toml

CMD ["./reproducible-build-infrastructure", "-c", "/etc/rbi/config.toml"]
