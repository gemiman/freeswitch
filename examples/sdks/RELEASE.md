# FreeSWITCH SDK 发布说明

## 版本信息

### 监控SDK (vcswitch-mnt-api)

- **版本号**: 1.0.4
- **发布时间**: 2024年1月16日
- **npm 包**: `vcswitch-mnt-api`
- **仓库地址**: https://github.com/freeswitch/mod_ussd

### 座席SDK (vcswitch-agent-api)

- **版本号**: 1.1.0
- **发布时间**: 2024年1月16日
- **npm 包**: `vcswitch-agent-api`
- **仓库地址**: https://github.com/freeswitch/mod_ussd

## 功能概述

### 监控SDK

专为监控端设计，提供了班长席功能，包括：

**核心功能：**
- 登录/登出
- 查询座席列表和状态
- 强制置闲/置忙/登出
- 监听、强插、耳语、强制挂断等操作
- 实时监控技能组排队数量
- 网关状态警告
- 语音识别事件监听

**适用场景：**
- 呼叫中心监控
- 座席状态管理
- 实时数据监控
- 技能组管理

### 座席SDK

专为座席端设计，提供了完整的呼叫控制功能，包括：

**核心功能：**
- 座席登录/登出
- 呼叫控制（外呼、挂机、应答等）
- 呼叫操作（保持、静音、转移等）
- 视频通话支持
- 语音变音功能
- 客户评分
- 实时事件监听

**适用场景：**
- 座席工作台
- 呼叫中心客户端
- 在线客服系统
- 电话销售系统

## 技术特性

### 1. WebAssembly 技术

- 使用 Rust 语言编写，确保高性能和安全性
- 编译为 WebAssembly，可在所有现代浏览器中运行
- 与 JavaScript 深度集成，提供简单的 API
- 内存安全，避免常见的 JavaScript 安全问题

### 2. 模块化设计

- 监控SDK和座席SDK分离，可独立使用
- 清晰的接口设计，易于理解和使用
- 事件驱动架构，响应式编程

### 3. 类型安全

- 自动生成 TypeScript 类型定义
- 强类型系统，减少运行时错误
- 编译时类型检查

### 4. 跨平台支持

- 支持所有现代浏览器（Chrome 57+, Firefox 52+, Safari 11+, Edge 16+）
- 与主流框架兼容（React, Vue, Angular, Svelte）
- 支持 Webpack, Rollup, Vite 等打包工具

## API 设计

### 命名规范

- 方法名使用蛇形命名法（snake_case），符合 Rust 语言规范
- 事件监听器使用 `set_*_event_listener` 格式
- 类和接口使用 PascalCase

### 通信方式

- 使用 WebSocket 进行实时通信
- 事件驱动的异步通信
- 支持断线重连和错误处理

## 安装和使用

### 安装

```bash
npm install vcswitch-mnt-api --save
npm install vcswitch-agent-api --save
```

### 快速开始

**监控SDK：**

```javascript
import * as VcMonitorApi from 'vcswitch-mnt-api';

const api = VcMonitorApi.VcMonitorApi.create_agent_control('127.0.0.1', 9059, 'ws');

api.set_agent_status_change_event_listener(event => {
    console.log('座席状态变化:', event);
});

api.admin_login('8003', '888888@172.16.20.181');
```

**座席SDK：**

```javascript
import * as VoiceCommAPI from 'vcswitch-agent-api';

VoiceCommAPI.VoiceCommAPI.set_mode('development');

VoiceCommAPI.VoiceCommAPI.set_configure_server({
    websocketHost: '127.0.0.1',
    websocketPort: 9058,
    webSocketUrl: 'ws://127.0.0.1:9058',
    domain: '172.16.20.181',
    trunkNumber: '+862160568356',
    gateway: 'dfdea3e4-ba6c-4398-8b9d-781cc71dbb98',
    outno: '98'
});

const agent = VoiceCommAPI.VoiceCommAPI.create_agent_control();
agent.login('agent123', '8001', 'password123', ['888888']);
```

## 架构设计

### 系统架构

```
┌──────────────────────────────────────────────────────────┐
│                     Browser                              │
│ ┌─────────────────────┐ ┌─────────────────────┐         │
│ │  vcswitch-mnt-api   │ │ vcswitch-agent-api  │         │
│ │  (监控SDK)          │ │ (座席SDK)           │         │
│ └─────────────────────┘ └─────────────────────┘         │
│              │                      │                    │
│              └──────────┬────────────┘                    │
│                         │                                 │
│                 WebSocket 连接                            │
│                         │                                 │
│                 ┌─────────────┐                            │
│                 │   信号服务器  │                            │
│                 └─────────────┘                            │
│                        │                                   │
│          ┌─────────────┴─────────────┐                     │
│          │                           │                     │
│    ┌─────────────┐         ┌─────────────┐                │
│    │  FreeSWITCH │         │   后端服务    │                │
│    └─────────────┘         └─────────────┘                │
└──────────────────────────────────────────────────────────┘
```

### 核心组件

#### 信号服务器

- 处理 WebSocket 连接
- 路由消息到相应的服务
- 管理会话状态
- 处理认证和授权

#### FreeSWITCH 模块

- 提供通话控制功能
- 处理语音流
- 管理座席状态
- 处理呼入/呼出

## 性能优化

### 1. 内存管理

- 使用 Rust 的所有权系统管理内存
- WebAssembly 内存优化
- 避免内存泄漏

### 2. 通信优化

- 最小化 WebSocket 消息大小
- 批处理操作
- 压缩传输数据

### 3. 渲染优化

- 虚拟 DOM 集成
- 事件防抖和节流
- 懒加载组件

## 安全性

### 1. 代码安全

- Rust 语言的内存安全
- 无缓冲区溢出
- 类型安全检查

### 2. 通信安全

- WebSocket 安全连接（wss）
- 消息加密
- 认证和授权

### 3. 数据安全

- 敏感数据加密
- 防止 XSS 攻击
- 防止 CSRF 攻击

## 兼容性

### 浏览器兼容性

| 浏览器 | 最低版本 | 支持特性 |
|:-------|:--------|:--------|
| Chrome | 57+     | 全部功能 |
| Firefox | 52+    | 全部功能 |
| Safari | 11+     | 全部功能 |
| Edge | 16+      | 全部功能 |

### 框架兼容性

- React 16+
- Vue 2.6+ / 3.0+
- Angular 8+
- Svelte 3+

## 更新日志

### 监控SDK (1.0.4)

**新增功能：**
- 支持全局跨技能组查询座席状态
- 优化事件推送机制
- 增强错误处理

**修复：**
- 修复了某些场景下的内存泄漏问题
- 优化了连接超时处理

### 座席SDK (1.1.0)

**新增功能：**
- 支持视频通话
- 新增语音变音功能
- 新增客户评分功能
- 优化呼叫转移流程

**改进：**
- 重构了事件监听器接口
- 优化了 WebSocket 重连机制
- 提升了音频处理性能

**修复：**
- 修复了在某些浏览器上的兼容性问题
- 修复了视频流处理的相关 bug

## 未来计划

### 短期计划 (1.2.x)

- [ ] 增加更多语言支持
- [ ] 优化视频通话质量
- [ ] 增加屏幕共享功能
- [ ] 改进文档和示例

### 中期计划 (2.x)

- [ ] 重构架构，支持微前端
- [ ] 增加 AI 智能分析功能
- [ ] 优化移动端体验
- [ ] 支持多租户架构

### 长期计划

- [ ] 提供完整的呼叫中心解决方案
- [ ] 支持更多通信协议
- [ ] 优化全球部署方案
- [ ] 提供企业级支持

## 贡献

欢迎提交 Pull Request 和 Issue。请阅读 CONTRIBUTING.md 了解如何为项目做出贡献。

## 许可证

MIT License - 详情请参阅 LICENSE 文件。

## 联系方式

- 项目主页：https://github.com/freeswitch/mod_ussd
- 问题反馈：https://github.com/freeswitch/mod_ussd/issues
- 邮件列表：dev@freeswitch.org
- 社区讨论：https://groups.google.com/forum/#!forum/freeswitch-users
