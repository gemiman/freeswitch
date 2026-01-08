# WebRTC 全栈生产部署方案 (FreeSWITCH + Coturn)

本目录包含了一套专为生产环境优化的 Docker 编排配置，集成了 FreeSWITCH (WSS/DTLS) 和 Coturn (中继服务)。

## 1. 包含组件
*   **FreeSWITCH**: 核心通信引擎（已集成 Nginx 录音下载）。
*   **Coturn**: 解决 NAT 穿透的关键中继服务器。
*   **Configuration Overlay**: 预设了 WebRTC 和安全加固的配置模板。

## 2. 生产环境部署清单 (必读)

在正式部署到生产服务器之前，请务必完成以下替换：

### 2.1 域名与证书 (核心)
WebRTC 必须通过 HTTPS/WSS 访问，因此需要正式的域名证书。

1.  **准备证书**: 获取您的域名证书（如通过 Let's Encrypt、阿里云等）。通常您会得到一个 `.crt` (或 `.pem`) 公钥文件和一个 `.key` 私钥文件。
2.  **生成 wss.pem**: FreeSWITCH 需要将私钥和公钥合并在同一个文件中。
    ```bash
    # 语法: cat 公钥文件 私钥文件 > wss.pem
    
    # 示例 (Nginx/Apache 格式):
    cat your_domain.crt your_domain.key > wss.pem
    
    # 示例 (Let's Encrypt):
    cat fullchain.pem privkey.pem > wss.pem
    ```
3.  **替换文件**: 将生成的 `wss.pem` 放入本目录的 `certs/` 文件夹中，覆盖默认文件。
    *   `wss.pem`: (必须) 包含 **[证书 + 中间链 + 私钥]**。这是 FreeSWITCH WSS 服务使用的核心文件。
    *   `cafile.pem`: (可选) 包含 **[证书 + 中间链]**，用于验证客户端证书（通常不需要改动）。
4.  **注意**: 默认提供的 `certs/` 下的文件是自签名的，仅供启动测试，浏览器会严格拦截自签名 WSS 连接。

### 2.2 环境变量替换
执行 `setup.sh` 之前，请设置以下环境变量：
```bash
export PUBLIC_IP=你的公网IP
export DOMAIN=你的通信域名 (如 rtc.yourcompany.com)
export FS_PASSWORD=你的强密码 (用于分机注册和控制台)
export TURN_SECRET=你的TURN共享密钥 (建议随机长字符串)
```

### 2.3 执行初始化
```bash
chmod +x setup.sh
./setup.sh
```
该脚本会自动：
1. 从镜像中提取官方默认配置。
2. 将 `conf_overlay/` 中的生产模板覆盖进去。
3. 自动将您的 IP、域名和密码注入到所有 XML 配置文件中。

## 3. 启动服务
```bash
docker compose up -d
```

## 4. 端口开放说明 (防火墙)
请确保您的服务器防火墙（安全组）放行以下端口：
*   **TCP 8082**: Nginx 录音下载 (从默认 80 改为 8082)。
*   **TCP 5060, 5061, 5080, 5081**: SIP 信令。
*   **TCP 5066, 7443**: WebRTC (WS/WSS)。
*   **TCP 8021**: ESL (建议仅限内网)。
*   **UDP 3478**: Coturn 信令。
*   **UDP 16384-32768**: FreeSWITCH 媒体流 (RTP)。
*   **UDP 49152-65535**: Coturn 媒体中继流。

## 5. 文件说明
*   `conf_overlay/`: 生产环境配置模板（安全加固、WebRTC 适配）。
*   `certs/`: 证书存放地。
*   `setup.sh`: 自动化部署粘合脚本。
*   `docker-compose.yml`: 生产级容器编排。