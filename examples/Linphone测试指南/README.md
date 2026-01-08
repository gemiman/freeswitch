# Linphone 测试 FreeSWITCH 指南

本指南介绍如何使用 Linphone 客户端连接到 FreeSWITCH 进行基本功能测试。

## 1. 简介
Linphone 是一款开源的 SIP 客户端，支持语音、视频通话、即时消息等。它是测试 FreeSWITCH SIP 注册、通话、编解码协商等功能的理想工具。

## 2. 环境准备
* **FreeSWITCH**: 已安装并正常运行。
* **Linphone**: 可以在官网 [linphone.org](https://www.linphone.org/) 下载安装包（支持 Windows, macOS, Linux, iOS, Android）。

## 3. 账户配置
假设您的 FreeSWITCH 使用默认配置，分机号通常为 `1000` 到 `1019`。

> **注意：** 默认密码通常为 `1234`。如果您修改了配置中的密码，请使用您设置的新密码。

### 3.1 密码说明
FreeSWITCH 的默认密码在 `conf/vars.xml` 文件中定义：
```xml
<X-PRE-PROCESS cmd="set" data="default_password=1234"/>
```
分机配置（如 `conf/directory/default/1000.xml`）通常引用此变量：
```xml
<param name="password" value="$${default_password}"/>
```
**如果您修改了密码**：
1. 修改 `conf/vars.xml` 中的 `default_password` 值。
2. 进入 FreeSWITCH 控制台 (`fs_cli`) 执行 `reloadxml` 命令使配置生效。
3. 如果未执行 `reloadxml`，旧密码 `1234` 将继续有效。

### 3.2 打开 Linphone 账户设置
1. 启动 Linphone。
2. 点击左上角的侧边栏按钮，选择 **助手 (Assistant)**。
3. 选择 **使用 SIP 账户 (Use a SIP Account)**。

![Linphone 助手界面](images/linphone_assistant.png)

### 3.3 填写账户信息
按照以下参数填写：
* **用户名 (Username)**: `1000`
* **域名 (Domain)**: `[FreeSWITCH 服务器 IP]` (例如 `192.168.1.100`)
* **密码 (Password)**: `1234` (或您修改后的密码)
* **传输方式 (Transport)**: `UDP` (根据您的 FreeSWITCH 配置，通常默认为 UDP)

![填写账户信息](images/linphone_config.png)

点击 **确认 (Confirm)**。

### 3.4 检查注册状态
如果配置正确，Linphone 顶部会显示一个绿色的圆点，表示已成功注册到 FreeSWITCH。

![注册成功状态](images/linphone_registered.png)

## 4. 拨打测试电话
成功注册后，您可以拨打 FreeSWITCH 默认配置中的一些特殊号码进行测试。

### 4.1 回声测试 (Echo Test)
拨打 `9196`。
* **现象**: 您说话后应该能立刻听到自己的声音。
* **目的**: 验证双向音频流是否正常。

### 4.2 播放音乐 (Music on Hold)
拨打 `9198`。
* **现象**: 您应该能听到 FreeSWITCH 默认的等待音乐。
* **目的**: 验证单向音频下载流和编解码（通常是 G.711 或 L16）是否正常。

![拨打电话界面](images/linphone_call.png)

## 5. 常见问题排查
* **注册失败 (Registration Failed)**:
    * 检查防火墙是否放行了 5060 端口 (UDP/TCP)。
    * 检查 FreeSWITCH 控制台 (`fs_cli`) 是否有注册尝试日志。
    * 确保密码正确（默认为 `1234` 或您修改后的值）。
* **无声音 (No Audio)**:
    * 检查 NAT 设置。如果在公网测试，可能需要配置 STUN/TURN 服务器。
    * 检查防火墙是否放行了 RTP 端口范围 (默认 16384-32768 UDP)。
* **单通 (One-way Audio)**:
    * 通常是 NAT 穿透问题，请检查 FreeSWITCH 的 `ext-rtp-ip` 和 `ext-sip-ip` 配置。

---
*注：本目录下的 `images/` 文件夹用于存放上述步骤中提到的截图。*
