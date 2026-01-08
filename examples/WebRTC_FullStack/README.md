# WebRTC 全栈部署方案 (FreeSWITCH + Coturn)

本目录包含了一套开箱即用的 Docker 编排配置，用于部署支持 WebRTC 的 FreeSWITCH 系统。

## 包含组件
1.  **FreeSWITCH**: 核心软交换，配置为 Host 网络模式。
2.  **Coturn**: STUN/TURN 服务器，解决 NAT 穿透。

## 快速开始

1.  **设置公网 IP**:
    ```bash
    export PUBLIC_IP=1.2.3.4
    ```

2.  **启动服务**:
    ```bash
    docker compose up -d
    ```

3.  **配置 FreeSWITCH 对接**:
    修改 FreeSWITCH 的 `sip_profiles/internal.xml`，添加 Coturn 的连接信息（参考 `FREESWITCH_DOCKER_MANUAL_CN.md`）。

## 文件说明
- `docker-compose.yml`: 完整的服务编排文件。
- `turnserver.conf.example`: Coturn 配置文件参考（可选，如果不想用命令行参数配置）。
