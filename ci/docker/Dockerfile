FROM harbor.leryn.top/debian:buster-slim AS build

WORKDIR /opt/project

RUN { \
        echo "deb https://mirrors.tuna.tsinghua.edu.cn/debian/ bullseye main contrib non-free"; \
        echo "deb https://mirrors.tuna.tsinghua.edu.cn/debian/ bullseye-updates main contrib non-free"; \
        echo "deb https://mirrors.tuna.tsinghua.edu.cn/debian/ bullseye-backports main contrib non-free"; \
        echo "deb https://mirrors.tuna.tsinghua.edu.cn/debian-security bullseye-security main contrib non-free"; \
        echo "deb https://mirrors.bfsu.edu.cn/debian/ bullseye main contrib non-free"; \
        echo "deb https://mirrors.bfsu.edu.cn/debian/ bullseye-updates main contrib non-free"; \
        echo "deb https://mirrors.bfsu.edu.cn/debian/ bullseye-backports main contrib non-free"; \
        echo "deb https://mirrors.bfsu.edu.cn/debian-security bullseye-security main contrib non-free"; \
    } > /etc/apt/sources.list \
      && \ 
    apt update && apt install -f -y \
        apt-transport-https ca-certificates \
        gcc g++ cmake make \
      && \
    rm -rf /var/lib/apt/lists/*

COPY . /opt/project

RUN bash build.sh \
      && \
    cp -ar build/release-0.0.1-linux-x64 release

FROM harbor.leryn.top/debian:buster-slim AS publish

WORKDIR /root

COPY entrypoint.sh  /
COPY --from=build release /usr/lib/vsp

CMD /usr/lib/vsp/bin/vspr

ENTRYPOINT docker-entrypoint.sh

# TODO