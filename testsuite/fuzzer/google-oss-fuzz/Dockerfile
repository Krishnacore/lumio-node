FROM gcr.io/oss-fuzz-base/base-builder-rust
RUN apt-get update && apt-get install -y make autoconf automake libclang-dev libtool pkg-config
RUN git clone --depth=1 ssh://git@github.com/pontem-network/lumio-node.git
WORKDIR aptos-core
COPY build.sh $SRC
