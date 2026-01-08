#!/bin/bash
set -e

# --- 动态配置 ---
# 如果设置了 FS_DEFAULT_PASSWORD 环境变量，则修改 vars.xml
if [ -n "$FS_DEFAULT_PASSWORD" ]; then
    echo "Setting default_password to provided value..."
    sed -i "s/default_password=1234/default_password=$FS_DEFAULT_PASSWORD/g" /usr/local/freeswitch/conf/vars.xml
fi

# 如果设置了 FS_ESL_PASSWORD 环境变量，则修改 event_socket.conf.xml
if [ -n "$FS_ESL_PASSWORD" ]; then
    echo "Setting ESL password to provided value..."
    sed -i "s/value=\"ClueCon\"/value=\"$FS_ESL_PASSWORD\"/g" /usr/local/freeswitch/conf/autoload_configs/event_socket.conf.xml
fi
# ----------------

# 启动 Nginx (后台运行)
echo "Starting Nginx..."
service nginx start

# 确保录音目录存在并有权限，以便 Nginx 可以读取
mkdir -p /usr/local/freeswitch/recordings
chown -R freeswitch:freeswitch /usr/local/freeswitch/recordings
chmod -R 755 /usr/local/freeswitch/recordings

# 启动 FreeSWITCH (前台运行，切换到 freeswitch 用户)
echo "Starting FreeSWITCH..."
# 如果传入了参数，则执行参数，否则默认启动 freeswitch
if [ "$#" -eq 0 ]; then
    # 以 root 启动，由 FreeSWITCH 内部降级用户 (从而保留设置优先级的权限)
    # -u freeswitch -g freeswitch: 指定运行用户和组
    # -nf: 不后台运行 (No Fork)
    # -c: 控制台模式 (Console)，日志输出到 stdout
    exec freeswitch -u freeswitch -g freeswitch -nf -c
else
    # 如果有自定义参数，直接执行 (保持 root 权限，用户需自行处理 -u/-g)
    exec freeswitch "$@"
fi
