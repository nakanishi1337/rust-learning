FROM ubuntu:24.04
ENV DEBIAN_FRONTEND=noninteractive


RUN apt-get update -qq && \
    apt-get upgrade -y && \
    apt-get autoremove -y --purge
RUN apt update -y \
    && apt upgrade -y \
    && apt autoremove -y

RUN apt-get update -qq && \
    apt-get install -y --no-install-recommends curl build-essential cmake libclang-dev iproute2 ca-certificates git openssh-client iputils-ping&& \
    apt-get autoremove -y --purge && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Enable sudo for the ubuntu user
RUN apt update -y\
    && apt install -y sudo
RUN echo "ubuntu ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

# Create a rustcean user
USER ubuntu
WORKDIR /home/ubuntu

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/ubuntu/.cargo/bin:${PATH}"
RUN echo 'source "$HOME/.cargo/env"' >> /home/ubuntu/.bashrc

# CMD ["bash"]