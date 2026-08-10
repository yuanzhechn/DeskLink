# DeskLink

DeskLink 是一个面向局域网的 Windows → Linux 跨设备键盘、鼠标和剪贴板协同工具。它不传输桌面画面。

当前版本：`0.6.1`。

## 功能

- Windows 捕获真实键盘、鼠标输入。
- Linux 通过 `/dev/uinput` 创建虚拟键盘和虚拟鼠标。
- UDP 传输高频输入，带序列号、会话校验、共享令牌和 ACK。
- Windows 多显示器支持不同分辨率、缩放比例、负坐标和不规则排列。
- Linux 作为一个逻辑屏幕加入屏幕拼图。
- 鼠标通过有效 EdgeLink 自动进入 Linux，并可从对应边缘返回 Windows。
- Windows 和 Linux 可以任意顺序启动。
- `Ctrl+Alt+Esc` 紧急恢复 Windows 本地输入。
- localhost 页面拖动、保存 Linux 屏幕位置。
- Windows 与 Linux 双向同步 UTF-8 纯文本剪贴板。

## 工程目录

```text
DeskLink/
├─ Cargo.toml
├─ config/       配置模型
├─ protocol/     两端通信协议
├─ topology/     ScreenRect 与 EdgeLink 算法
├─ windows/      Windows Host
├─ linux/        Linux Client
└─ packaging/    Linux udev 规则
```

所有 Cargo 命令均在项目根目录执行。

## Windows 编译

```powershell
cd D:\githubProject\DeskLink
cargo build -p desklink-windows --release
```

生成文件：

```text
target\release\desklink-windows.exe
```

## Linux 编译

```bash
cd ~/DeskLink
cargo build -p desklink-linux --release
```

首次运行前安装 Wayland 剪贴板工具，并配置 uinput 权限规则：

```bash
sudo apt install wl-clipboard xclip
sudo usermod -aG input "$USER"
sudo cp packaging/70-desklink-uinput.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

完成后注销并重新登录 Linux。

## 配置

两台电脑分别复制配置文件：

Windows：

```powershell
Copy-Item config.example.toml desklink.toml
```

Linux：

```bash
cp config.example.toml desklink.toml
```

Windows 的 `desklink.toml`：

```toml
[network]
target = "192.168.66.1:24801"
ui_bind = "127.0.0.1:24802"
control_port = 24800

[security]
token = "请替换为自己的随机令牌"
```

Linux 的 `desklink.toml`：

```toml
[network]
bind = "0.0.0.0:24801"
control_bind = "0.0.0.0:24800"

[security]
token = "请替换为自己的随机令牌"
```

剪贴板默认配置如下。`max_bytes` 默认是 64 MiB；如果需要同步更大的文本，可在两端同时调大。设置 `enabled = false` 可关闭剪贴板同步。

```toml
[clipboard]
enabled = true
poll_ms = 400
max_bytes = 67108864
```

两端 `security.token` 必须完全一致。环境变量 `DESKLINK_TARGET`、`DESKLINK_BIND`、`DESKLINK_TOKEN` 和 `DESKLINK_CONFIG` 的优先级高于配置文件。

`desklink.toml` 包含本地地址和令牌，已加入 `.gitignore`，不会提交到 Git。

## 启动

Linux：

```bash
./target/release/desklink-linux
```

Windows：

```powershell
.\target\release\desklink-windows.exe
```

启动顺序不限。连接成功后 Windows 日志会显示：

```text
Linux client acknowledged; connection ready
```

Linux 日志会显示：

```text
Windows host authorized
```

剪贴板通道建立后，两端会显示 `clipboard channel connected`。Linux 会根据当前图形会话自动选择 `wayland/wl-clipboard` 或 `x11/xclip` 后端。复制纯文本后无需手工触发，通常会在 `poll_ms` 指定的时间内同步到另一端；文件、图片和富文本暂不同步。

## 屏幕布局

Windows Host 启动后，在 Windows 浏览器打开：

```text
http://127.0.0.1:24802
```

页面会显示每块 Windows 显示器的真实矩形。拖动绿色 Linux 屏幕到任意 Windows 屏幕旁边，靠近边缘时会自动吸附。

只有两个屏幕实际贴合，并且边缘存在重叠区间时，才会生成 EdgeLink。布局会立即保存到 Windows 本地 `desklink.toml`，重启后继续使用。

如果 Windows 显示器数量、分辨率、缩放或坐标发生变化，DeskLink 会检测到布局指纹变化，使旧布局失效，并提示重新打开 localhost 页面调整。

## 自动穿越

Linux Client 在线并完成 ACK 后，把鼠标移动到有效 EdgeLink。鼠标在边缘停留 `edge_delay_ms`（默认 80ms）后自动进入 Linux，不需要输入终端命令。

Linux Client 不读取 Wayland/X11 全局光标，而是根据收到的 `dx/dy` 维护逻辑光标。移动到对应返回边缘后，Windows 会把光标恢复到正确显示器和对应位置。

紧急恢复快捷键：

```text
Ctrl + Alt + Esc
```

## 防火墙

Ubuntu/Linux：

```bash
sudo ufw allow 24801/udp
sudo ufw allow 24800/tcp
```

Windows 出现网络访问提示时，允许 DeskLink 访问专用网络。

## 故障排查

如果 Windows 显示 `Linux client ACK timed out`：

1. 确认 Linux Client 正在运行。
2. 确认两端均由当前源码重新编译，协议版本一致。
3. 确认两端 `security.token` 完全一致。
4. 确认 Windows 的 `network.target` 是 Linux 的真实局域网 IP。
5. 确认 UDP 24801 未被防火墙拦截。
6. 注意 PowerShell 环境变量可能覆盖 `desklink.toml`。

查看 Linux 监听状态：

```bash
ss -lunp | grep 24801
ss -ltnp | grep 24800
pgrep -af desklink-linux
```

版本变化参见 [CHANGELOG.md](CHANGELOG.md)。
