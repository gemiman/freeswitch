# syntax=docker/dockerfile:1
FROM debian:bookworm AS builder

# 设置非交互式前端，避免安装过程中的提示
ENV DEBIAN_FRONTEND=noninteractive

# 1. 安装基础构建依赖
# 包含编译工具、核心库依赖、多媒体库依赖、数据库依赖等
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential cmake git automake autoconf libtool libtool-bin pkg-config \
    libssl-dev zlib1g-dev libcurl4-openssl-dev libjpeg-dev \
    libsqlite3-dev libpcre2-dev libspeex-dev libspeexdsp-dev \
    libldns-dev libedit-dev libtiff-dev yasm \
    libopus-dev libsndfile1-dev liblua5.2-dev \
    libavformat-dev libswscale-dev libswresample-dev \
    uuid-dev libexpat1-dev libgdbm-dev libdb-dev \
    libshout3-dev libmpg123-dev libmp3lame-dev \
    libpq-dev unixodbc-dev \
    python3 python3-dev \
    ca-certificates wget \
    && rm -rf /var/lib/apt/lists/*

# 2. 编译 libks (SignalWire 基础库)
WORKDIR /usr/src
RUN git clone https://github.com/signalwire/libks.git \
    && cd libks \
    && cmake . -DCMAKE_INSTALL_PREFIX=/usr -DWITH_LIBBACKTRACE=1 \
    && make && make install

# 3. 编译 signalwire-c (SignalWire 客户端库)
WORKDIR /usr/src
RUN git clone https://github.com/signalwire/signalwire-c.git \
    && cd signalwire-c \
    && cmake . -DCMAKE_INSTALL_PREFIX=/usr \
    && make -j$(nproc) && make install

# 4. 编译 sofia-sip (SIP 协议栈)
WORKDIR /usr/src
RUN git clone https://github.com/freeswitch/sofia-sip.git \
    && cd sofia-sip \
    && ./bootstrap.sh && ./configure --prefix=/usr \
    && make -j$(nproc) && make install

# 5. 编译 spandsp (DSP 信号处理库)
WORKDIR /usr/src
RUN git clone https://github.com/freeswitch/spandsp.git \
    && cd spandsp \
    && ./bootstrap.sh && ./configure --prefix=/usr \
    && make -j$(nproc) && make install

# 6. 编译 FreeSWITCH
# 复制当前源码到容器
COPY . /usr/src/freeswitch
WORKDIR /usr/src/freeswitch

# 预处理：生成 modules.conf 并启用常用模块
RUN cp build/modules.conf.in build/modules.conf.in.bak \
    && sed -i 's|#applications/mod_callcenter|applications/mod_callcenter|' build/modules.conf.in \
    && sed -i 's|#applications/mod_av |applications/mod_av |' build/modules.conf.in \
    && sed -i 's|#applications/mod_xml_curl|applications/mod_xml_curl|' build/modules.conf.in \
    && sed -i 's|#applications/mod_xml_cdr|applications/mod_xml_cdr|' build/modules.conf.in \
    && sed -i 's|#applications/mod_http_cache|applications/mod_http_cache|' build/modules.conf.in \
    && sed -i 's|#formats/mod_shout|formats/mod_shout|' build/modules.conf.in \
    # 禁用可能会导致编译问题的模块 (视情况而定)
    && sed -i 's|languages/mod_java|#languages/mod_java|' build/modules.conf.in

# 编译安装
# 运行 bootstrap.sh，如果未生成 Makefile.in 则尝试手动 autoreconf 以暴露错误
RUN ./bootstrap.sh && ([ -f Makefile.in ] || autoreconf -fisv)
RUN ./configure --prefix=/usr/local/freeswitch \
    --enable-core-odbc-support \
    --enable-core-pgsql-support \
    --with-openssl \
    --with-lws=no 
    # 注意：--with-lws=no 是为了避免如果系统没有 libwebsockets 导致的潜在问题，FreeSWITCH 内部有自带的或者使用其他的 WebSocket 实现 (mod_sofia 自带)
    
RUN make -j$(nproc)
RUN make install
RUN make samples

# 7. 运行时镜像 (减小体积)
FROM debian:bookworm

ENV DEBIAN_FRONTEND=noninteractive

# 安装运行时依赖
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 zlib1g libcurl4 libjpeg62-turbo \
    libsqlite3-0 libpcre2-8-0 libspeex1 libspeexdsp1 \
    libldns3 libedit2 libtiff6 \
    libopus0 libsndfile1 liblua5.2-0 \
    libavformat59 libswscale6 \
    libshout3 libmpg123-0 libmp3lame0 \
    libpq5 libodbc2 \
    ca-certificates \
    net-tools iproute2 \
    nginx \
    && rm -rf /var/lib/apt/lists/*

# 配置 Nginx
COPY nginx_recordings.conf /etc/nginx/sites-available/default
# 确保 Nginx 以前台模式运行不需要 daemon off，因为我们会用脚本控制，或者保持默认后台服务模式
# 这里直接覆盖 default 站点即可

# 复制启动脚本
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# 从 builder 复制 FreeSWITCH 安装目录
COPY --from=builder /usr/local/freeswitch /usr/local/freeswitch

# 从 builder 复制手动编译的库
# 注意：这些库被安装到了 /usr/lib 或 /usr/lib/x86_64-linux-gnu，取决于 debian 的配置
# 我们尝试通配符复制，并处理可能的路径差异
COPY --from=builder /usr/lib/libks* /usr/lib/
COPY --from=builder /usr/lib/libsignalwire* /usr/lib/
COPY --from=builder /usr/lib/libsofia* /usr/lib/
COPY --from=builder /usr/lib/libspandsp* /usr/lib/
# 以防万一安装到了 lib64 (通常 debian 不会，但 cmake 有时会)
# COPY --from=builder /usr/lib64/* /usr/lib/ || true

# 配置环境变量
ENV PATH="/usr/local/freeswitch/bin:${PATH}"
ENV LD_LIBRARY_PATH="/usr/local/freeswitch/lib:/usr/lib:/usr/local/lib"

# 创建用户和必要的目录权限
RUN groupadd -r freeswitch && useradd -r -g freeswitch freeswitch \
    && chown -R freeswitch:freeswitch /usr/local/freeswitch \
    && chmod -R 770 /usr/local/freeswitch \
    && mkdir -p /var/run/freeswitch \
    && chown -R freeswitch:freeswitch /var/run/freeswitch

# 端口暴露
# SIP
EXPOSE 5060/tcp 5060/udp 5080/tcp 5080/udp
# SIP TLS
EXPOSE 5061/tcp 5061/udp 5081/tcp 5081/udp
# ESL
EXPOSE 8021/tcp
# WebSocket (WebRTC)
EXPOSE 5066/tcp 7443/tcp
# Verto
EXPOSE 8081/tcp 8082/tcp
# Nginx 录音下载
EXPOSE 80/tcp
# RTP (范围可根据 freeswitch.xml 配置调整)
EXPOSE 16384-32768/udp

# 健康检查
HEALTHCHECK --interval=30s --timeout=5s \
  CMD fs_cli -x status || exit 1

# 启动
WORKDIR /usr/local/freeswitch
# 注意：Nginx 需要 root 权限启动 (虽然 worker 可以是非 root)，但 service nginx start 需要 root。
# FreeSWITCH 建议非 root 运行，但为了简化双进程启动，我们在 entrypoint 脚本里处理权限降级或直接以 root 启动但切换用户。
# 简单起见，这里保持 USER 为 root，在脚本中可以 su 切换 freeswitch，或者我们暂时让 FS 以 root 运行 (虽然不推荐但方便)。
# 为了更安全，entrypoint.sh 可以用 gosu 或 su-exec 切换用户运行 freeswitch。
# 鉴于之前配置了 USER freeswitch，我们尽量保持。
# 但 service nginx start 需要 root。所以我们必须切回 USER root。
USER root
ENTRYPOINT ["/entrypoint.sh"]
CMD []
