FROM rustembedded/cross:arm-unknown-linux-gnueabihf-0.2.1

# RUN dpkg --add-architecture armhf && \
# 	    apt-get update && \
# 	    apt-get install --assume-yes libdbus-1-dev libdbus-1-dev:armhf pkg-config libsystemd-dev:armhf

RUN apt-get update
RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install --assume-yes \
      libssl-dev:armhf \
      libasound2-dev:armhf \
      libdbus-1-dev:armhf \
      libsystemd-dev:armhf
RUN ln -s /lib/arm-linux-gnueabihf/libsystemd.so.0 /usr/lib/arm-linux-gnueabihf/libsystemd.so.0
ENV PKG_CONFIG_LIBDIR=/usr/local/lib/arm-linux-gnueabihf/pkgconfig:/usr/lib/arm-linux-gnueabihf/pkgconfig