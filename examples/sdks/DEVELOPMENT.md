# FreeSWITCH SDK 开发文档

## 概述

本文档详细描述了 FreeSWITCH 监控SDK和座席SDK的开发过程、构建流程以及发布方法。两个SDK都是使用 Rust 语言编写，并通过 WebAssembly 技术编译为可在浏览器环境中运行的 npm 包。

## 技术栈

- **Rust 2024** - 主要开发语言
- **wasm-bindgen** - Rust 到 JavaScript 的绑定生成工具
- **web-sys** - Web API 访问库
- **serde** - 序列化/反序列化库
- **npm** - 包管理和发布工具

## 项目结构

```
sdks/
├── vc-monitor-sdk/          # 监控SDK项目
│   ├── Cargo.toml           # Rust项目配置
│   ├── src/
│   │   └── lib.rs          # 核心实现
│   └── pkg/                # 编译产物
├── vc-agent-sdk/           # 座席SDK项目
│   ├── Cargo.toml          # Rust项目配置
│   ├── src/
│   │   └── lib.rs          # 核心实现
│   └── pkg/                # 编译产物
├── package.json            # 测试项目配置
├── test-monitor.js         # 监控SDK测试脚本
├── test-agent.js           # 座席SDK测试脚本
└── DEVELOPMENT.md          # 本文档
```

## 开发过程

### 1. 项目初始化

#### 监控SDK

```bash
cargo init vc-monitor-sdk --lib
cd vc-monitor-sdk
cargo add wasm-bindgen --features=serde
cargo add js-sys
cargo add web-sys --features=DomParser,Document,Element,Window,WebSocket,console
cargo add serde --features=derive
cargo add serde_json
```

#### 座席SDK

```bash
cargo init vc-agent-sdk --lib
cd vc-agent-sdk
cargo add wasm-bindgen --features=serde
cargo add js-sys
cargo add web-sys --features=DomParser,Document,Element,Window,WebSocket,console,HtmlAudioElement
cargo add serde --features=derive
cargo add serde_json
```

### 2. 核心实现

#### vc-monitor-sdk/src/lib.rs

主要功能：
- `VcMonitorApi` 类 - SDK 主入口
- `create_agent_control` 静态方法 - 创建控制实例
- `admin_login` - 登录
- 各种 `force_*` 方法 - 班长席操作
- `set_*_event_listener` 方法 - 事件监听器

#### vc-agent-sdk/src/lib.rs

主要功能：
- `VoiceCommAPI` 类 - 全局接口
- `AgentControl` 类 - 座席控制
- `CallControl` 类 - 呼叫控制
- 包含所有座席端和呼叫端的方法及事件监听器

### 3. 编译配置

在 Cargo.toml 中添加：

```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

## 构建过程

### 1. 安装 wasm-pack

```bash
cargo install wasm-pack
```

### 2. 编译 SDK

#### 监控SDK

```bash
cd vc-monitor-sdk
wasm-pack build --target bundler --release
```

#### 座席SDK

```bash
cd vc-agent-sdk
wasm-pack build --target bundler --release
```

### 3. 编译产物结构

每个 SDK 编译后在 `pkg/` 目录下生成：

```
pkg/
├── package.json            # npm 包配置
├── {name}.d.ts            # TypeScript 类型定义
├── {name}.js              # JavaScript 绑定
├── {name}_bg.js          # 内部辅助函数
├── {name}_bg.wasm        # WebAssembly 二进制文件
└── {name}_bg.wasm.d.ts  # WASM 类型定义
```

## 测试

### 1. 创建测试项目

```bash
npm init -y
```

### 2. 安装本地 SDK 包

```bash
npm install ./vc-monitor-sdk/pkg
npm install ./vc-agent-sdk/pkg
```

### 3. 编写测试脚本

见 `test-monitor.js` 和 `test-agent.js`。

### 4. 运行测试

```bash
npm run test:monitor
npm run test:agent
```

## 发布

### 1. 更新 package.json

在编译好的 pkg/ 目录下，更新 package.json：

```json
{
  "name": "vcswitch-mnt-api",
  "version": "1.0.4",
  "description": "FreeSWITCH 监控端 SDK - 使用 Rust + WebAssembly 实现",
  "main": "vc_monitor_sdk.js",
  "types": "vc_monitor_sdk.d.ts",
  "type": "module",
  "files": [
    "vc_monitor_sdk_bg.wasm",
    "vc_monitor_sdk.js",
    "vc_monitor_sdk_bg.js",
    "vc_monitor_sdk.d.ts"
  ],
  "sideEffects": [
    "./vc_monitor_sdk.js",
    "./snippets/*"
  ],
  "repository": {
    "type": "git",
    "url": "https://github.com/freeswitch/mod_ussd.git"
  },
  "keywords": ["freeswitch", "sdk", "webrtc", "monitor"],
  "author": "FreeSWITCH",
  "license": "MIT",
  "bugs": {
    "url": "https://github.com/freeswitch/mod_ussd/issues"
  },
  "homepage": "https://github.com/freeswitch/mod_ussd"
}
```

### 2. 发布到 npm

```bash
cd vc-monitor-sdk/pkg
npm login
npm publish --access=public
```

```bash
cd vc-agent-sdk/pkg
npm login
npm publish --access=public
```

## 使用说明

### 监控SDK

```javascript
import * as VcMonitorApi from 'vcswitch-mnt-api';

// 创建控制实例
const api = VcMonitorApi.VcMonitorApi.create_agent_control('127.0.0.1', 9059, 'ws');

// 设置事件监听器
api.set_agent_status_change_event_listener((event) => {
    console.log('座席状态变化:', event);
});

// 登录
api.admin_login('8003', '888888@172.16.20.181');
```

### 座席SDK

```javascript
import * as VoiceCommAPI from 'vcswitch-agent-api';

// 设置模式
VoiceCommAPI.VoiceCommAPI.set_mode('development');

// 配置服务器
VoiceCommAPI.VoiceCommAPI.set_configure_server({
    websocketHost: '127.0.0.1',
    websocketPort: 9058,
    webSocketUrl: 'ws://127.0.0.1:9058',
    domain: '172.16.20.181',
    trunkNumber: '+862160568356',
    gateway: 'dfdea3e4-ba6c-4398-8b9d-781cc71dbb98',
    outno: '98'
});

// 创建控制实例
const agent = VoiceCommAPI.VoiceCommAPI.create_agent_control();
const call = VoiceCommAPI.VoiceCommAPI.create_call_control();

// 登录
agent.login('agent123', '8001', 'password123', ['888888']);
```

## 事件处理

### 监控SDK事件

```javascript
api.set_agent_status_change_event_listener(event => {
    console.log('座席状态变化:', event);
});

api.set_agent_list_event_listener(event => {
    console.log('座席列表:', event);
});

api.set_members_count_event(event => {
    console.log('排队数量:', event);
});

api.set_gateway_warning_event_listener(event => {
    console.log('网关警告:', event);
});
```

### 座席SDK事件

```javascript
agent.set_agent_login_success_event_listener(event => {
    console.log('登录成功:', event);
});

call.set_channel_ringing_event_listener(event => {
    console.log('振铃:', event);
});

call.set_channel_this_party_answered_event_listener(event => {
    console.log('己方应答:', event);
});
```

## 常见问题

### 1. WebAssembly 支持

确保浏览器支持 WebAssembly：
- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

### 2. WebSocket 连接失败

- 检查服务器地址和端口是否正确
- 确保服务器已启动并监听指定端口
- 检查防火墙和网络设置

### 3. 方法调用失败

- 确保已正确初始化 SDK
- 检查方法参数类型是否正确
- 查看浏览器控制台的错误信息

## 更新和维护

### 更新依赖

```bash
cd vc-monitor-sdk
cargo update

cd ../vc-agent-sdk
cargo update
```

### 重新编译

```bash
cd vc-monitor-sdk
rm -rf pkg
wasm-pack build --target bundler --release

cd ../vc-agent-sdk
rm -rf pkg
wasm-pack build --target bundler --release
```

### 更新版本号

在 Cargo.toml 中修改 version 字段，然后重新编译。

## 文档资源

- [wasm-bindgen 文档](https://rustwasm.github.io/wasm-bindgen/)
- [web-sys 文档](https://rustwasm.github.io/wasm-bindgen/web_sys/index.html)
- [Rust 官方文档](https://doc.rust-lang.org/)
- [npm 发布指南](https://docs.npmjs.com/cli/v8/commands/npm-publish)
