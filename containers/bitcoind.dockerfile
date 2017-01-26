FROM ubuntu:xenial
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
  wget \
  build-essential \
  make \
  cmake \
  g++ \
  python-leveldb \
  libboost-all-dev \
  libssl-dev \
  libdb++-dev \
  libtool \
  autotools-dev \
  autoconf \
  bsdmainutils \
  pkg-config \
  software-properties-common

RUN add-apt-repository ppa:bitcoin/bitcoin \
  && apt-get update \
  && apt-get install -y bitcoind \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
  
# Install python
RUN apt-get update && apt-get install -y \
  python \
  python-dev \
  python-distribute \
  python-pip

# Download and install required packages:
RUN pip install --upgrade pip
RUN pip install redis

VOLUME ["/var/lib/bitcoind"]
VOLUME ["/var/www/bitcoind"]
EXPOSE 8332 8333

WORKDIR /var/lib/bitcoind
CMD /bin/sh -c "/usr/bin/bitcoind -conf=/var/www/bitcoind/bitcoin.conf -datadir=/var/www/bitcoind/data -sysperms -disablewallet -printtoconsole -txindex=1 -rest"
