#!/bin/bash
set -e

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
    # 使用 su -s /bin/bash -c 确保 shell 环境
    exec su -s /bin/bash freeswitch -c "freeswitch -nf -ncwait"
else
    exec su -s /bin/bash freeswitch -c "$*"
fi
