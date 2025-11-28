FROM ubuntu:noble

RUN apt-get update
RUN DEBIAN_FRONTEND=noninteractive \
    apt install -y --no-install-recommends software-properties-common

RUN add-apt-repository ppa:vbernat/haproxy-3.2 -y
RUN DEBIAN_FRONTEND=noninteractive \
    apt install -y --no-install-recommends curl git haproxy ca-certificates build-essential libclang-dev

COPY . /root/haproxy
WORKDIR /root/haproxy

# Build plugin
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    . "$HOME/.cargo/env" && \
    sed -rie 's/^(haproxy-api)\s=\s.+$/\1 = "*"/g' Cargo.toml && \
    cargo build --release

CMD ["haproxy", "-d", "-f", "haproxy.cfg"]
