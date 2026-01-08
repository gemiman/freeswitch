# FreeSWITCH Docker 生产级部署与使用手册

## 1. 镜像信息
*   **镜像**: `gemiman/freeswitch:0.0.3-amd64`
*   **功能**: 全功能 FreeSWITCH + Nginx 录音服务器
*   **架构**: x86_64 (Linux)

## 2. 生产环境部署 (Docker Compose)

我们提供了一个生产级别的 `docker-compose.yml` 文件。

### 2.1 初始化配置目录
在启动之前，建议先将镜像内的默认配置文件复制到宿主机，以便持久化和修改。

```bash
# 创建目录
mkdir -p conf recordings log db

# 从镜像中提取默认配置
docker run --rm --entrypoint tar gemiman/freeswitch:0.0.3-amd64 -cC /usr/local/freeswitch conf | tar xC ./
```

执行完上述命令后，当前目录下应出现 `conf/` 文件夹。

### 2.2 docker-compose.yml 示例内容

您可以直接创建该文件并根据需要取消 `host` 模式的注释。

```yaml
version: '3.8'

services:
  freeswitch:
    image: gemiman/freeswitch:0.0.3-amd64
    container_name: freeswitch_prod
    restart: unless-stopped
    
    # Linux 生产环境强烈建议开启 'host' 模式
    # network_mode: host
    
    ports:
      - "5060:5060/tcp"
      - "5060:5060/udp"
      - "5080:5080/tcp"
      - "5080:5080/udp"
      - "5061:5061/tcp"
      - "5061:5061/udp"
      - "8021:8021/tcp"
      - "7443:7443/tcp"
      - "5066:5066/tcp"
      - "8080:80/tcp"
      - "16384-32768:16384-32768/udp"

    volumes:
      - ./conf:/usr/local/freeswitch/conf
      - ./recordings:/usr/local/freeswitch/recordings
      - ./log:/usr/local/freeswitch/log
      - ./db:/usr/local/freeswitch/db
      - /etc/localtime:/etc/localtime:ro
      - /etc/timezone:/etc/timezone:ro

    environment:
      - TZ=Asia/Shanghai

    ulimits:
      core: -1
      nofile:
        soft: 65536
        hard: 65536
    
    deploy:
      resources:
        limits:
          cpus: '4'
          memory: 4G
        reservations:
          memory: 1G

    healthcheck:
      test: ["CMD", "fs_cli", "-x", "status"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 20s

    logging:
      driver: "json-file"
      options:
        max-size: "100m"
        max-file: "5"
```

### 2.3 启动服务
确保 `docker-compose.yml` 在当前目录。

```bash
docker compose up -d
```

### 2.3 进阶：WebRTC 场景部署 (集成 Coturn)

针对 WebRTC 场景，我们在 `examples/WebRTC_FullStack/` 目录下提供了一套完整的编排方案，集成了 **Coturn (TURN/STUN)** 服务器。

#### 启动 WebRTC 组合方案：
```bash
# 请将 1.2.3.4 替换为你的服务器公网 IP
cd examples/WebRTC_FullStack
PUBLIC_IP=1.2.3.4 docker compose up -d
```

#### 配置 FreeSWITCH 对接 Coturn：
修改宿主机挂载的 `conf/sip_profiles/internal.xml`，在 `<settings>` 节点添加：
```xml
<param name="stun-server" value="YOUR_PUBLIC_IP:3478"/>
<param name="turn-server" value="YOUR_PUBLIC_IP:3478:udp"/>
<param name="credential-mechanisms" value="turn-rest-auth"/>
<param name="turn-rest-auth-shared-secret" value="MySuperSecretKey123"/>
```
*注：`turn-rest-auth-shared-secret` 必须与 docker-compose-webrtc.yml 中的 `static-auth-secret` 一致。*

### 2.4 验证状态
```bash
# 查看容器日志
docker compose logs -f

# 进入 FS 控制台
docker compose exec freeswitch fs_cli
```

## 3. 关键配置文件详解

所有配置文件均位于 `./conf` 目录下。修改后需重启容器或在 `fs_cli` 执行 `reloadxml` 生效。

### 3.1 `conf/vars.xml` (全局变量)
这是最重要的地方，定义了全局的 IP、密码和域名。

*   **默认密码**:
    ```xml
    <X-PRE-PROCESS cmd="set" data="default_password=1234"/>
    ```
    **生产环境必须修改**为强密码。这是分机注册密码。

*   **本机 IP**:
    ```xml
    <X-PRE-PROCESS cmd="set" data="local_ip_v4=$${local_ip_v4}"/>
    ```
    在 Docker NAT 模式下，FreeSWITCH 自动检测到的可能是容器内 IP (172.x.x.x)。
    如果遇到单通问题，可能需要手动将其改为宿主机的公网 IP：
    `<X-PRE-PROCESS cmd="set" data="local_ip_v4=1.2.3.4"/>`
    或者设置外部 RTP/SIP IP：
    `<X-PRE-PROCESS cmd="set" data="external_rtp_ip=1.2.3.4"/>`
    `<X-PRE-PROCESS cmd="set" data="external_sip_ip=1.2.3.4"/>`

### 3.2 `conf/autoload_configs/event_socket.conf.xml` (ESL 接口)
外部程序（如 Java/Python/Go 客户端）控制 FS 的接口。

```xml
<configuration name="event_socket.conf" description="Socket Client">
  <settings>
    <param name="nat-map" value="false"/>
    <param name="listen-ip" value="0.0.0.0"/>
    <param name="listen-port" value="8021"/>
    <param name="password" value="ClueCon"/> <!-- 生产环境必须修改 -->
  </settings>
</configuration>
```

### 3.3 `conf/autoload_configs/acl.conf.xml` (访问控制)
限制谁可以连接 ESL 或发起 SIP 呼叫。

```xml
<list name="domains" default="deny">
  <!-- 允许本地网络 -->
  <node type="allow" cidr="192.168.0.0/16"/>
  <!-- 建议添加你的业务服务器 IP -->
  <node type="allow" cidr="10.0.0.5/32"/>
</list>
```

### 3.4 `conf/sip_profiles/internal.xml` (内网 SIP 配置文件)
处理分机注册和内部呼叫。

*   **启用 WebSocket (WebRTC)**:
    确保以下配置存在且未被注释：
    ```xml
    <param name="ws-binding" value=":5066"/>
    <param name="wss-binding" value=":7443"/>
    ```

## 4. 常见问题排查

### 4.1 只有单向声音 (One-way Audio)
通常是因为 NAT 问题，SIP 协商的 SDP 包中包含了容器内部 IP (172.x)，导致外部音频发不进来。

**解决方案**:
1.  **推荐**: 使用 `docker-compose.yml` 中的 `network_mode: host` (仅 Linux)。
2.  **备选**: 修改 `conf/vars.xml`，显式指定公网 IP：
    ```xml
    <X-PRE-PROCESS cmd="set" data="external_rtp_ip=YOUR_HOST_PUBLIC_IP"/>
    <X-PRE-PROCESS cmd="set" data="external_sip_ip=YOUR_HOST_PUBLIC_IP"/>
    ```

### 4.2 Nginx 无法访问录音
检查：
1.  容器端口映射是否正确 (`-p 8080:80`)。
2.  `recordings` 目录权限是否正确 (应属于 freeswitch 用户)。
3.  是否有录音文件生成。

## 5. 常用命令速查

| 目标 | 命令 |
| :--- | :--- |
| **重载 XML 配置** | `fs_cli -x "reloadxml"` |
| **查看 SIP 状态** | `fs_cli -x "sofia status"` |
| **开启 SIP 抓包** | `fs_cli -x "sofia global siptrace on"` |
| **查看当前通话** | `fs_cli -x "show channels"` |
| **注册分机状态** | `fs_cli -x "show registrations"` |

