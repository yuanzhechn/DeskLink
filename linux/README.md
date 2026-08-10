# DeskLink Linux Client

Linux Client 监听 UDP 24801，并创建 DeskLink uinput 键盘、鼠标和绝对指针设备。运行用户需要访问 `/dev/uinput`；生产安装应通过 udev 规则授予 `input` 组权限，而不是每次使用 `sudo`。

双向纯文本剪贴板监听 TCP 24800，并使用 `wl-copy`/`wl-paste`：

```bash
sudo apt install wl-clipboard
```

```bash
sudo cp packaging/70-desklink-uinput.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

详细配置、编译与启动方法见项目根目录 `README.md`。
