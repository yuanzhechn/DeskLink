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

Linux Client 连续 5 秒收不到主机数据时，会自动回到本地状态并释放按键。Windows 处于 `remote` 状态时会通过心跳重复同步状态，因此 Windows 和 Linux 可以任意顺序启动；只要 Windows 进程仍在运行，Linux 后启动后即可接管状态。

## 5. 防火墙

Ubuntu：`sudo ufw allow 24801/udp`。Windows 弹出网络访问提示时，请允许 DeskLink 访问专用网络。

当前版本已实现协议、UDP、序列号去重、状态切换、uinput、心跳和断线恢复，并接入 Windows 全局低级鼠标/键盘 Hook。输入 `remote` 后，真实鼠标点击、移动、滚轮和键盘事件会被转发到 Linux；输入 `local` 或 `release` 后恢复 Windows 本地输入。终端 `move/click/key` 命令仍保留用于调试。图形化配置界面和 Raw Input 高精度相对位移仍属于后续阶段。
