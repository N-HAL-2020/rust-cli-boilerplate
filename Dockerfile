ARG RUST_VERSION=1.72.1
FROM rust:${RUST_VERSION}-bullseye

ARG USERNAME=app
ARG USER_UID=1000
ARG USER_GID=$USER_UID

WORKDIR /workspace

RUN \
    apt-get update \
    && apt-get install -y --no-install-recommends \
    git \
    sudo \
    vim \
    less \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && rustup component add rustfmt clippy \
    && groupadd --gid $USER_GID $USERNAME \
    && useradd -s /bin/bash --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && mkdir /workspace/target; chown -R $USERNAME:$USERNAME /workspace/target

USER $USERNAME
