#!/bin/bash
set -e

# ==============================================================================
# FreeSWITCH WebRTC 环境初始化脚本 (生产增强版)
# ==============================================================================

# 1. 检查并设置环境变量
if [ -z "$PUBLIC_IP" ]; then
    echo "错误: 请先设置 PUBLIC_IP 环境变量。"
    echo "示例: export PUBLIC_IP=1.2.3.4"
    exit 1
fi

DOMAIN=${DOMAIN:-"rtc.example.com"}
TURN_SECRET=${TURN_SECRET:-"MySuperSecretKey123"}

# 增加交互式输入
if [ -z "$FS_PASSWORD" ]; then
    read -p "请输入 FreeSWITCH 注册密码 (默认: StrongPassword123!): " FS_PASSWORD
    FS_PASSWORD=${FS_PASSWORD:-"StrongPassword123!"}
fi

echo "============================================================"
echo "正在初始化 FreeSWITCH 配置..."
echo "公网 IP: $PUBLIC_IP"
echo "域名: $DOMAIN"
echo "TURN 密钥: $TURN_SECRET"
echo "FreeSWITCH 密码: $FS_PASSWORD"
echo "============================================================"

# 2. 创建必要目录
mkdir -p conf recordings log db certs

# 3. 从 Docker 镜像提取默认配置 (如果 conf 为空)
# 注意：我们必须确保提取的是完整的配置
# 如果之前提取过但不完整（比如只有 overlay），会导致 conf 不为空从而跳过。
# 这里增加一个检测：如果缺 freeswitch.xml，就强制重新提取
if [ ! -f conf/freeswitch.xml ]; then
    echo "检测到 conf 目录不完整（缺少 freeswitch.xml），正在从 Docker 镜像重新提取..."
    # 镜像内路径变更: /usr/local/freeswitch/etc/freeswitch
    docker run --rm --entrypoint tar gemiman/freeswitch:0.0.9-amd64 -cC /usr/local/freeswitch/etc/freeswitch . | tar xC ./conf
    echo "默认配置提取完成。"
else
    echo "检测到 conf 目录完整，跳过默认配置提取。"
fi

# 4. 应用 Overlay 配置 (覆盖默认文件)
echo "正在应用 WebRTC 和安全加固配置..."
cp -r conf_overlay/* conf/

# 5. 复制并链接证书
# FreeSWITCH 默认在 conf/ssl 中查找证书
echo "正在配置 SSL 证书..."
mkdir -p conf/ssl
cp certs/* conf/ssl/ 2>/dev/null || echo "警告: certs 目录中没有发现证书文件，请稍后手动放置。"

# 6. 替换配置文件中的占位符
echo "正在替换变量..."

# 替换 vars.xml
# 兼容替换 CHANGE_ME 和 默认值 1234
sed -i "s/YOUR_PUBLIC_IP/$PUBLIC_IP/g" conf/vars.xml
sed -i "s/YOUR_TURN_SECRET/$TURN_SECRET/g" conf/vars.xml
sed -i "s/CHANGE_ME/$FS_PASSWORD/g" conf/vars.xml
sed -i "s/default_password=1234/default_password=$FS_PASSWORD/g" conf/vars.xml

# 替换默认域名
sed -i "s/rtc.example.com/$DOMAIN/g" conf/vars.xml
sed -i "s/domain=\$\${local_ip_v4}/domain=$DOMAIN/g" conf/vars.xml

# 替换 event_socket.conf.xml
sed -i "s/CHANGE_ME/$FS_PASSWORD/g" conf/autoload_configs/event_socket.conf.xml
sed -i "s/value=\"ClueCon\"/value=\"$FS_PASSWORD\"/g" conf/autoload_configs/event_socket.conf.xml

# 替换 turnserver.conf
if [ -f turnserver.conf.example ]; then
    cp turnserver.conf.example turnserver.conf
    sed -i "s/# external-ip=1.2.3.4/external-ip=$PUBLIC_IP/g" turnserver.conf
    sed -i "s/static-auth-secret=MySuperSecretKey123/static-auth-secret=$TURN_SECRET/g" turnserver.conf
    sed -i "s/rtc.example.com/$DOMAIN/g" turnserver.conf
fi

echo "============================================================"
echo "配置初始化完成！"
echo "生产部署建议："
echo "1. 请确保 certs/ 目录下已放置真实的域名证书 (wss.pem, cafile.pem)"
echo "2. 运行: docker compose up -d"
echo "============================================================"
