FROM rust:1.48

ARG user=vscode
ARG uid=1000
ARG gid=${uid}

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    git wget sudo 

ENV USERNAME ${user}
ENV UID ${uid}

RUN useradd -m $USERNAME && \
    echo "$USERNAME:$USERNAME"  | chpasswd &&\
    usermod --shell /bin/bash $USERNAME &&\
    usermod -aG sudo $USERNAME &&\
    echo "$USERNAME ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers.d/$USERNAME &&\
    chmod 0440 /etc/sudoers.d/$USERNAME &&\
    usermod --uid $UID $USERNAME &&\
    groupmod --gid $UID $USERNAME

USER $USERNAME

RUN rustup update 2>&1 && \
  rustup component add rls rust-analysis rust-src rustfmt clippy 2>&1

ENV DEBIAN_FRONTEND=dialog
