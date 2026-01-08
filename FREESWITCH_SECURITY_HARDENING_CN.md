# FreeSWITCH 安全加固配置手册

本文档提供了一套完整的 FreeSWITCH 生产环境安全加固方案，涵盖账户安全、网络安全、信令加密和防盗打策略。

---

## 1. 账户与认证安全 (基础必做)

### 1.1 修改默认密码
FreeSWITCH 默认密码极为知名，**必须**在部署第一天修改。

**目标文件**: `conf/vars.xml`

```xml
<!-- 将 1234 改为强密码 (用于分机注册) -->
<X-PRE-PROCESS cmd="set" data="default_password=YourStrongPassword!@#"/>

<!-- 将 ClueCon 改为强密码 (用于 ESL 管理接口) -->
<!-- 注意: 同时也需要修改你的 ESL 客户端代码中的连接密码 -->
<!-- 实际上 ESL 密码通常在 autoload_configs/event_socket.conf.xml 中定义，但有些版本引用了 vars -->
```

**目标文件**: `conf/autoload_configs/event_socket.conf.xml`

```xml
<configuration name="event_socket.conf" description="Socket Client">
  <settings>
    <param name="nat-map" value="false"/>
    <param name="listen-ip" value="127.0.0.1"/> <!-- 强烈建议仅监听本地 -->
    <param name="listen-port" value="8021"/>
    <param name="password" value="YourStrongESLPassword!@#"/> <!-- 修改此处 -->
  </settings>
</configuration>
```

### 1.2 禁用默认分机 (1000-1019)
默认配置包含预设的 20 个分机，这是扫描器的首选目标。
建议：**删除** `conf/directory/default/` 下的所有 `.xml` 文件，仅创建实际需要的业务分机。

---

## 2. 网络访问控制 (ACL)

使用 FreeSWITCH 内置的 ACL (Access Control List) 限制 SIP 和 ESL 的访问来源。

**目标文件**: `conf/autoload_configs/acl.conf.xml`

```xml
<list name="domains" default="deny">
  <!-- 允许本地回环 -->
  <node type="allow" cidr="127.0.0.1/32"/>
  <node type="allow" cidr="::1/128"/>
  
  <!-- 允许内网段 (根据实际情况调整) -->
  <node type="allow" cidr="192.168.1.0/24"/>
  
  <!-- 允许特定的 SIP 运营商 IP -->
  <node type="allow" cidr="202.106.0.20/32"/>
</list>
```

在 `sip_profiles` 中应用 ACL：
**目标文件**: `conf/sip_profiles/internal.xml`

```xml
<!-- 拒绝不在 ACL 列表中的 IP 发起呼叫或注册 -->
<param name="apply-inbound-acl" value="domains"/>
<param name="apply-register-acl" value="domains"/>
```

---

## 3. 通信加密 (TLS & SRTP)

防止信令和语音流在公网被窃听。

### 3.1 启用 SIP over TLS (SIPS)
1.  **生成证书**: 使用 Certbot (Let's Encrypt) 或自签名证书。
    证书文件需放置在 `conf/ssl/` 目录下：
    *   `agent.pem`: 包含 证书 + 私钥
    *   `cafile.pem`: CA 根证书链

2.  **配置 Profile**:
    **目标文件**: `conf/sip_profiles/internal.xml`

    ```xml
    <param name="tls" value="true"/>
    <param name="tls-only" value="false"/> <!-- 若设为 true 则仅允许加密连接 -->
    <param name="tls-bind-params" value="transport=tls"/>
    <param name="tls-sip-port" value="5061"/>
    <param name="tls-cert-dir" value="$${conf_dir}/ssl"/>
    <param name="tls-version" value="tlsv1.2,tlsv1.3"/>
    ```

### 3.2 强制 SRTP (加密语音)
强制所有通过该 Profile 的通话使用加密 RTP。

**目标文件**: `conf/dialplan/public.xml` 或 `default.xml`

```xml
<extension name="enforce_srtp">
  <condition field="destination_number" expression="^.*$">
    <action application="set" data="rtp_secure_media=true"/>
    <!-- 可选: 强制要求对端也支持 SRTP，否则挂断 -->
    <!-- <action application="set" data="rtp_secure_media_inbound=true"/> -->
    <!-- <action application="set" data="rtp_secure_media_outbound=true"/> -->
  </condition>
</extension>
```

### 3.3 WebRTC 特殊安全要求
WebRTC 的安全模型是现代浏览器强制执行的，比传统 SIP 更严格。

1.  **强制 WSS (WebSocket Secure)**
    *   **原理**: 浏览器禁止在 HTTPS 页面发起非加密的 WebSocket (WS) 连接。因此，WebRTC 坐席端必须通过 WSS (TCP/7443) 连接 FreeSWITCH。
    *   **配置**: 在 SIP Profile 中必须开启 `wss-binding`。
        ```xml
        <!-- conf/sip_profiles/internal.xml -->
        <param name="wss-binding" value=":7443"/>
        ```

2.  **强制 DTLS-SRTP (媒体加密)**
    *   **原理**: 浏览器强制所有媒体流 (RTP) 必须通过 DTLS 握手并使用 SRTP 加密。
    *   **配置**: FreeSWITCH 默认开启 DTLS，但需确保 Profile 中包含 `rtp_secure_media` 相关设置，且防火墙不能拦截 DTLS 的握手包。

3.  **TURN 服务器认证安全 (防滥用)**
    *   **风险**: 公开的 TURN 服务器会被黑客滥用，作为流量中转站进行 DDoS 攻击或隐藏真实 IP。
    *   **解决方案**: 必须启用 `credential-mechanisms`。
    *   **原理**: FreeSWITCH 在 ICE 协商时，动态为每个 WebRTC 客户端生成一个临时的、有时效的用户名和密码。这样，只有合法的客户端才能使用你的 TURN 服务。
    *   **配置**: 
        ```xml
        <!-- conf/sip_profiles/internal.xml -->
        <param name="stun-server" value="your.turn.server:3478"/>
        <param name="turn-server" value="your.turn.server:3478:udp"/>
        
        <!-- 开启临时认证 -->
        <param name="credential-mechanisms" value="turn-rest-auth"/>
        
        <!-- 认证服务器地址和共享密钥 -->
        <param name="turn-rest-auth-server" value="https://your.auth.server/api/turn"/>
        <param name="turn-rest-auth-shared-secret" value="YourSecretKey"/>
        ```
        *注: `turn-rest-auth` 通常需要一个简单的 Web 服务来验证 FreeSWITCH 发来的请求。*

4.  **公网信任证书**
    *   **风险**: 浏览器会拒绝连接使用自签名证书的 WSS 服务器。
    *   **解决方案**: 必须使用 Let's Encrypt 或其他商业 CA 签发的、受浏览器信任的 TLS 证书。

---

## 4. 防攻击与防盗打 (Anti-Fraud)

### 4.1 Fail2Ban 集成
自动封禁频繁试探密码的 IP。

1.  **安装**: `apt-get install fail2ban`
2.  **配置 Jail**: `/etc/fail2ban/jail.local`

    ```ini
    [freeswitch]
    enabled  = true
    port     = 5060,5061,5080,5081
    protocol = all
    filter   = freeswitch
    logpath  = /usr/local/freeswitch/log/freeswitch.log
    maxretry = 5
    findtime = 600
    bantime  = 3600
    action   = iptables-allports[name=freeswitch]
    ```

3.  **配置 Filter**: `/etc/fail2ban/filter.d/freeswitch.conf`
    FreeSWITCH 默认日志通常已经包含 `[WARNING] sofia_reg.c:1792 SIP auth failure (REGISTER)`，Fail2Ban 默认规则可识别。

### 4.2 拨号计划安全 (防盗打核心)
黑客最喜欢利用配置错误的 Dialplan 盗打国际长途。

**原则**:
*   **默认拒绝**: `public` context (外部来电) 默认不应包含任何 `bridge` 到外线的动作，只能转入 IVR 或本地分机。
*   **严格正则**: 避免使用 `^.*$` 这种通配符匹配被叫号码。

**示例安全配置**:
**目标文件**: `conf/dialplan/public.xml`

```xml
<context name="public">
  <!-- 仅允许呼入特定的 DID 号码 -->
  <extension name="inbound_did">
    <condition field="destination_number" expression="^(861088888888)$">
      <action application="transfer" data="1000 XML default"/>
    </condition>
  </extension>
  
  <!-- 拦截所有其他呼叫 -->
  <extension name="block_others">
    <condition field="destination_number" expression="^.*$">
      <action application="answer"/>
      <action application="sleep" data="1000"/>
      <action application="hangup" data="CALL_REJECTED"/>
    </condition>
  </extension>
</context>
```

### 4.3 限制并发与时长
防止被攻破后造成天价话费。

```xml
<extension name="limit_calls">
  <condition>
    <!-- 限制单通电话最大 1 小时 (3600秒) -->
    <action application="sched_hangup" data="+3600 allocated_time"/>
    
    <!-- 限制全系统最大并发为 100 -->
    <action application="limit" data="hash system max_calls 100 !CALL_REJECTED"/>
  </condition>
</extension>
```

---

## 5. 操作系统级防护

### 5.1 防火墙 (Iptables/UFW)
仅开放白名单 IP 访问 5060/5080。

```bash
# 允许特定运营商 IP
iptables -A INPUT -p udp -m udp --dport 5080 -s 202.106.0.20 -j ACCEPT
# 拒绝其他所有 5080 访问
iptables -A INPUT -p udp -m udp --dport 5080 -j DROP
```

### 5.2 隐藏版本号
防止攻击者根据版本号利用已知漏洞。

**目标文件**: `conf/vars.xml`

```xml
<!-- 自定义 User-Agent 头 -->
<X-PRE-PROCESS cmd="set" data="user_agent_string=Generic SIP Agent"/>
```

---

## 6. 检查清单 (上线前必核对)

- [ ] `default_password` 已修改。
- [ ] ESL `password` 已修改且监听在 `127.0.0.1` (或配置了 ACL)。
- [ ] 默认的 20 个分机已删除或修改密码。
- [ ] `public` dialplan 中没有开放任意外呼的路由。
- [ ] Fail2Ban 服务运行正常 (尝试输错密码测试是否被封)。
- [ ] 防火墙仅开放了必要的 UDP/TCP 端口。
- [ ] 不必要的模块 (如 `mod_xml_rpc` 若不用) 已禁用。
