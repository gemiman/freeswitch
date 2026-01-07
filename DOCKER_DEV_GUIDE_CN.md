# FreeSWITCH Docker 镜像开发与扩展指南

本文档旨在指导开发者如何从源码构建镜像、添加新的 FreeSWITCH 模块、引入第三方依赖以及发布新版本。

## 1. 镜像构建原理

本项目的 `Dockerfile` 采用 **多阶段构建 (Multi-stage builds)** 策略：

1.  **Builder 阶段 (`builder`)**:
    *   基于 `debian:bookworm`。
    *   安装完整的编译工具链 (`gcc`, `cmake`, `automake` 等)。
    *   编译核心依赖库 (`libks`, `signalwire-c`, `sofia-sip`, `spandsp`)。
    *   编译 FreeSWITCH 主程序及模块。
2.  **Runtime 阶段 (最终镜像)**:
    *   基于 `debian:bookworm` (更轻量，无编译工具)。
    *   仅安装运行时所需的动态链接库 (`libssl`, `libpq` 等) 和 `nginx`。
    *   从 Builder 阶段复制编译好的二进制文件和库。

## 2. 如何添加新模块

要向镜像中添加未启用的 FreeSWITCH 模块，通常涉及以下三个步骤：

### 2.1 第一步：启用模块配置
修改 `Dockerfile` 中的 `sed` 命令部分，取消对目标模块的注释。

例如，要启用 `mod_v8` (Google V8 JavaScript 引擎)：

**修改前 (Dockerfile):**
```dockerfile
# ...
RUN cp build/modules.conf.in build/modules.conf.in.bak \
    && sed -i 's|#applications/mod_callcenter|applications/mod_callcenter|' build/modules.conf.in \
    # ...
```

**修改后:**
```dockerfile
RUN cp build/modules.conf.in build/modules.conf.in.bak \
    && sed -i 's|#applications/mod_callcenter|applications/mod_callcenter|' build/modules.conf.in \
    && sed -i 's|#languages/mod_v8|languages/mod_v8|' build/modules.conf.in \
    # ...
```

### 2.2 第二步：添加系统依赖 (关键)
大多数模块都需要特定的系统库支持。如果缺少依赖，`configure` 或 `make` 阶段会报错。

你需要查找该模块的依赖。例如 `mod_v8` 可能需要 `libv8-dev` (或自行编译)，`mod_python` 需要 `python3-dev`。

在 `Dockerfile` 的 `builder` 阶段的 `apt-get install` 列表中添加这些包：

```dockerfile
RUN apt-get update && apt-get install -y --no-install-recommends \
    # ... 原有依赖 ...
    libpython3-dev \
    # ...
```

**注意：** 如果模块引入了新的动态链接库（.so），你也需要在 `Runtime` 阶段的 `apt-get install` 中安装对应的运行时库包（通常是不带 `-dev` 后缀的包名）。

### 2.3 第三步：重新构建
```bash
docker build --platform linux/amd64 -t freeswitch:custom .
```

## 3. 引入第三方模块

如果要添加不在 FreeSWITCH 源码树中的第三方模块：

1.  **克隆代码**: 在 `Dockerfile` 中，在编译 FreeSWITCH 之前，将第三方模块源码克隆到 `src/mod/` 下的适当分类目录。
    ```dockerfile
    WORKDIR /usr/src/freeswitch/src/mod/applications
    RUN git clone https://github.com/someuser/mod_amazing.git
    ```
2.  **启用编译**: 修改 `build/modules.conf.in`，添加新模块的路径（如果它不在默认列表中）。
    ```dockerfile
    RUN echo "applications/mod_amazing" >> build/modules.conf.in
    ```
3.  **构建**: 标准的 FreeSWITCH 构建过程会自动遍历 `modules.conf` 并编译列出的模块。

## 4. 开发调试技巧

### 4.1 使用缓存加速
Docker 构建缓存能极大加快调试速度。
*   尽量不要改变 `apt-get install` 行的顺序或内容，除非必须。
*   将变动频繁的指令（如 `COPY . /usr/src/freeswitch`）放在 `Dockerfile` 靠后的位置。

### 4.2 进入构建容器
如果构建失败，可以注释掉 `COPY` 之后的行，构建一个临时镜像，然后进入交互式调试：

```bash
docker run --rm -it <builder-image-id> bash
cd /usr/src/freeswitch
./configure ...
make
```
这样可以快速定位缺少哪个库。

## 5. 版本发布流程

1.  **构建镜像**:
    ```bash
    docker build --platform linux/amd64 -t gemiman/freeswitch:0.0.4-amd64 .
    ```
2.  **测试运行**:
    ```bash
    docker run --rm gemiman/freeswitch:0.0.4-amd4 fs_cli -x status
    ```
3.  **推送到 Docker Hub**:
    ```bash
    docker login
    docker push gemiman/freeswitch:0.0.4-amd64
    ```
4.  **更新文档**: 更新 `FREESWITCH_DOCKER_MANUAL_CN.md` 中的版本号。

