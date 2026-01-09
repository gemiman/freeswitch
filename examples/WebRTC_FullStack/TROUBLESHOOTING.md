# FreeSWITCH WebRTC 环境故障排查手册

本文档记录了 `examples/WebRTC_FullStack` 部署过程中可能遇到的常见问题及其解决方案。

## 1. 容器启动失败：数据库/日志权限错误

### 现象
容器启动后立即退出，日志显示：
```log
Cannot Initialize [Cannot Open log directory or XML Root!]
```
或者：
```log
[ERR] switch_core_db.c:246 SQL ERR [unable to open database file]
[CRIT] switch_core_sqldb.c:3588 CORE DATABASE INITIALIZATION FAILURE!
```

### 原因
FreeSWITCH 容器内部以 `freeswitch` 用户 (UID 1000) 运行，而宿主机挂载的 `log/` 或 `db/` 目录可能归属于 `root` 用户，或者包含旧的 `root` 权限文件，导致容器无权写入。

### 解决方案
在宿主机项目目录下执行：
```bash
# 1. 清理可能被 root 锁定的旧数据库文件
sudo rm -rf db/*

# 2. 赋予宽松的写入权限 (开发环境推荐)
sudo chmod -R 777 log db recordings

# 3. 确保配置文件可读
sudo chmod -R 755 conf certs

# 4. 重启容器
docker compose up -d --force-recreate freeswitch
```

---

## 2. 自动录音未生效 / 找不到录音文件

### 现象
*   通话结束，`recordings` 目录为空。
*   FreeSWITCH 日志中没有出现 `EXECUTE ... record_session`。
*   或者日志提示录音路径为 `/usr/local/freeswitch/var/lib/...` 但宿主机找不到。

### 原因
1.  **路径映射错误**：`docker-compose.yml` 曾错误地将宿主机目录挂载到容器的深层路径，而 `vars.xml` 配置的是 `/usr/local/freeswitch/recordings`。
2.  **Dialplan 优先级**：默认的 `default.xml` 优先级高于自定义的 `00_auto_record.xml`，导致呼叫在命中录音规则前就被处理并终止了。

### 解决方案
1.  **修正挂载路径** (`docker-compose.yml`)：
    ```yaml
    volumes:
      - ./recordings:/usr/local/freeswitch/recordings
    ```
2.  **调整 Dialplan**：
    确保在 `conf_overlay/dialplan/default.xml` 或生成的 `conf/dialplan/default.xml` 中，自定义包含 (`<X-PRE-PROCESS cmd="include" data="default/*.xml"/>`) 位于其他 extension 之前。
    *本项目已通过 setup.sh 自动处理此逻辑。*

---

## 3. 录音下载 403 Forbidden

### 现象
访问 `http://IP:8082/` 时，浏览器显示 `403 Forbidden`。

### 原因
1.  **权限不一致**：Nginx 默认以 `www-data` 运行，而录音文件由 `freeswitch` 用户生成，Nginx 无权读取。
2.  **配置缺失**：未开启 `autoindex on`。

### 解决方案
1.  **强制 Nginx 使用 root 运行**：
    挂载自定义的主配置文件 `nginx.conf`，将 `user www-data;` 修改为 `user root;`。
2.  **挂载配置** (`docker-compose.yml`)：
    ```yaml
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./nginx_recordings.conf:/etc/nginx/sites-enabled/default
    ```

---

## 4. Nginx 认证密码无效 (500 Internal Server Error)

### 现象
配置了 Basic Auth，但输入密码后一直提示错误，或者 Nginx 报错。

### 原因
`openssl passwd` 命令在不同系统（如 macOS vs Linux）上默认算法不同。Nginx 对 `SHA-512` (`-6`) 支持可能受限，最通用的格式是 `APR1` (Apache MD5)。

### 解决方案
生成密码时使用 `-apr1` 参数：
```bash
echo "admin:$(openssl passwd -apr1 你的密码)" > .htpasswd
```

---

## 5. SIP 暴力破解攻击 (SIP Auth Challenge)

### 现象
日志大量刷屏：
```log
[WARNING] sofia_reg.c:1842 SIP auth challenge (REGISTER) ... from ip 151.x.x.x
```

### 原因
公网开放了 5060 端口，遭遇脚本扫描撞库。

### 解决方案
**部署 Fail2Ban (宿主机)**：
由于容器权限限制，建议在宿主机安装 Fail2Ban 监控映射出来的日志文件。
详细配置请参考 `README.md` 第 5 节。

---

## 6. 其他常见错误

### `Failed to set SCHED_FIFO scheduler`
*   **级别**：警告 (Warning)，不影响运行。
*   **原因**：容器缺少实时调度权限。
*   **解法**：在 `docker-compose.yml` 中添加 `cap_add: - SYS_NICE` (已添加，若宿主机不支持可忽略)。
