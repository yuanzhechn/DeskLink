# DeskLink

DeskLink 是一个只传输键盘、鼠标输入事件的局域网跨设备工具。

## 目录结构

```text
DeskLink/                    ← 编译工作区根目录
├─ Cargo.toml                ← Rust workspace
├─ protocol/                 ← Windows/Linux 共用协议
├─ windows/                  ← Windows Host 源码
├─ linux/                    ← Ubuntu/Linux Client 源码
└─ packaging/                ← Linux udev 权限规则
```

## 1. 安装 Rust

略

## 2. Windows 端编译和启动

打开 PowerShell，进入 **workspace 根目录**：

```powershell
cd .\DeskLink\
cargo build -p desklink-windows --release
```

编译结果：`target\release\desklink-windows.exe`。

设置 Ubuntu Client 的局域网 IP 后启动：

```powershell
$env:DESKLINK_TARGET = "192.168.1.20:24801"
cargo run -p desklink-windows --release
```

也可以直接启动：

```powershell
.\target\release\desklink-windows.exe
```

## 3. Linux 端编译和启动

在 Ubuntu 上进入 **同一个 workspace 根目录**：

```bash
cd DeskLink/
cargo build -p desklink-linux --release
```

首次运行前配置 `/dev/uinput` 权限：

```bash
sudo usermod -aG input "$USER"
sudo cp packaging/70-desklink-uinput.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

重新登录 Ubuntu 后启动：

```bash
cargo run -p desklink-linux --release
```

或直接启动 `./target/release/desklink-linux`。默认监听 `0.0.0.0:24801`；可用 `DESKLINK_BIND=0.0.0.0:24801` 修改。

## 4. 两端联调顺序

1. 先在 Ubuntu 启动 `desklink-linux`。
2. 确认 Windows 能访问 Ubuntu 的 UDP `24801` 端口。
3. Windows 设置 `DESKLINK_TARGET` 后启动 `desklink-windows`。
4. Windows 端输入 `remote`，再输入 `move 20 0` 验证鼠标移动。
5. 测试结束输入 `local` 或 `release`。

远程模式下终端键盘也会被转发；如果无法输入 `local`，使用 Windows 全局紧急快捷键 `Ctrl+Alt+Esc`，程序会停止远程捕获、发送 `ReleaseAll` 并恢复本地输入。

Linux Client 超过配置的断线时间（默认 6 秒）收不到主机数据时，会自动回到本地状态并释放按键。Windows 处于 `remote` 状态时会通过心跳重复同步状态，因此 Windows 和 Linux 可以任意顺序启动；只要 Windows 进程仍在运行，Linux 后启动后即可接管状态。

## 配置文件

复制示例配置，并在 Windows/Linux 两端使用相同的安全令牌：

```text
config.example.toml → desklink.toml
```

Windows PowerShell：

```powershell
Copy-Item config.example.toml desklink.toml
```

Ubuntu：

```bash
cp config.example.toml desklink.toml
```

编辑 `desklink.toml`：Windows 主要使用 `network.target`，Linux 使用 `network.bind`。两端的 `security.token` 必须完全一致。环境变量 `DESKLINK_TARGET`、`DESKLINK_BIND`、`DESKLINK_TOKEN`、`DESKLINK_CONFIG` 会覆盖配置文件。

## 屏幕拼图与自动穿越

Windows Host 启动后，在 Windows 浏览器打开：

```text
http://127.0.0.1:24802
```

页面会显示每块 Windows 显示器的真实坐标、尺寸和相对位置。拖动绿色 Linux 屏幕到任意 Windows 屏幕旁边；距离边缘 80 像素以内时自动吸附。只有实际贴合并且有重叠区间的边缘才会生成 EdgeLink。

布局保存到本地 `desklink.toml` 并立即生效。程序重启后继续使用；如果 Windows 显示器数量、分辨率或系统坐标发生变化，旧布局会自动失效并回到默认位置，随后可再次通过 localhost 页面调整。

连接在线后不需要输入 `remote`：鼠标停留在有效 EdgeLink 边缘达到 `edge_delay_ms`（默认 80ms）后自动进入 Linux；从 Linux 对应边缘向外移动会自动返回正确的 Windows 显示器和对应高度。整个返回判断使用 DeskLink 自己维护的逻辑光标，不依赖 X11/Wayland 全局光标接口。

## 5. 防火墙

Ubuntu：`sudo ufw allow 24801/udp`。Windows 弹出网络访问提示时，请允许 DeskLink 访问专用网络。

当前版本已实现协议、UDP、序列号去重、状态切换、uinput、心跳和断线恢复，并接入 Windows 全局低级鼠标/键盘 Hook。输入 `remote` 后，真实鼠标点击、移动、滚轮和键盘事件会被转发到 Linux；输入 `local` 或 `release` 后恢复 Windows 本地输入。终端 `move/click/key` 命令仍保留用于调试。图形化配置界面和 Raw Input 高精度相对位移仍属于后续阶段。
