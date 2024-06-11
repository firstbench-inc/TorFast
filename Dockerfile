FROM rust:1-slim-bookworm

COPY . .

# syntax=docker/dockerfile:1
RUN rm -f /etc/apt/apt.conf.d/docker-clean; echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
  --mount=type=cache,target=/var/lib/apt,sharing=locked \
  apt-get --no-install-recommends install -y libssl-dev ca-certificates wget gnupg2 lsb-release pkg-config

# RUN apt-get update && apt-get install -y \
#     libssl-dev \
#     ca-certificates \
#     wget \
#     gnupg2 \
#     lsb-release \
#     pkg-config \
#     && rm -rf /var/lib/apt/lists/*

RUN wget -qO - https://artifacts.elastic.co/GPG-KEY-elasticsearch | gpg --dearmor -o /usr/share/keyrings/elasticsearch-keyring.gpg \
    && echo "deb [signed-by=/usr/share/keyrings/elasticsearch-keyring.gpg] https://artifacts.elastic.co/packages/8.x/apt stable main" | tee /etc/apt/sources.list.d/elastic-8.x.list
#
RUN touch /etc/apt/sources.list.d/tor.list
RUN echo "deb     [signed-by=/usr/share/keyrings/tor-archive-keyring.gpg] https://deb.torproject.org/torproject.org bookworm main" >> /etc/apt/sources.list.d/tor.list
RUN echo "deb-src [signed-by=/usr/share/keyrings/tor-archive-keyring.gpg] https://deb.torproject.org/torproject.org bookworm main" >> /etc/apt/sources.list.d/tor.list
RUN wget -qO- https://deb.torproject.org/torproject.org/A3C4F0F979CAA22CDBA8F512EE8CBC9E886DDD89.asc | gpg --dearmor | tee /usr/share/keyrings/tor-archive-keyring.gpg >/dev/null

# RUN apt update
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
  --mount=type=cache,target=/var/lib/apt,sharing=locked \
  apt-get --no-install-recommends install -y tor deb.torproject.org-keyring elasticsearch
# RUN apt install tor deb.torproject.org-keyring -y
#
# RUN apt-get install -y elasticsearch
RUN echo "network.host: 0.0.0.0" >> /etc/elasticsearch/elasticsearch.yml \
    && echo "http.port: 9200" >> /etc/elasticsearch/elasticsearch.yml \
    && echo "xpack.security.enabled: false"  >> /etc/elasticsearch/elasticsearch.yml \
    && echo "xpack.security.http.ssl.enabled: false"  >> /etc/elasticsearch/elasticsearch.yml \
    && echo "xpack.security.transport.ssl.enabled: false"  >> /etc/elasticsearch/elasticsearch.yml

RUN touch prog.sh
RUN touch tor.sh
RUN echo "tor &" >> "tor.sh"
RUN echo "sh tor.sh > /dev/null" >> prog.sh
RUN echo "/bin/systemctl start elasticsearch.service" >> prog.sh
RUN echo "cargo run" >> prog.sh

CMD ["sh", "prog.sh"]
# CMD RUN systemctl start elasticsearch.service

# CMD ["/bin/sh", "tor.sh"]
