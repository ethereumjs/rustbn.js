FROM ubuntu:18.04

# System deps
RUN apt-get update
RUN apt-get install -y build-essential curl git python cmake
RUN apt-get install -y nodejs npm


# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y && . $HOME/.cargo/env
ENV PATH=/root/.cargo/bin:$PATH
RUN rustup default stable

RUN rustup target add wasm32-unknown-emscripten
RUN rustup target add asmjs-unknown-emscripten

# Install Emscripten
RUN git clone --depth 1 https://github.com/emscripten-core/emsdk.git
RUN cd emsdk && ./emsdk install latest
RUN cd emsdk && ./emsdk activate latest 

ENV PATH=/emsdk:/emsdk/upstream/emscripten:/emsdk/node/12.9.1_64bit/bin:$PATH
ENV EMSDK=/emsdk
ENV EM_CONFIG=/root/.emscripten
ENV EMSDK_NODE=/emsdk/node/12.9.1_64bit/bin/node

RUN cd emsdk && ./emsdk_env.sh

WORKDIR /rustbn.js/

CMD /bin/bash

