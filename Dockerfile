FROM ghcr.io/cross-rs/armv7-unknown-linux-gnueabi:latest

# Ensure the correct library paths are set
ENV PKG_CONFIG_ALLOW_CROSS=1 \
    PKG_CONFIG_LIBDIR=/usr/lib/arm-linux-gnueabihf/pkgconfig

RUN apt-get update && \
    apt-get install -y libusb-1.0-0-dev && \
    rm -rf /var/lib/apt/lists/*