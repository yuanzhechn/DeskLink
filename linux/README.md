# DeskLink Linux Client

Linux Client 监听 UDP 24801，并创建 `DeskLink Virtual Input` uinput 设备。运行用户需要访问 `/dev/uinput`；生产安装应通过 udev 规则授予 `input` 组权限，而不是每次使用 `sudo`。

```bash
sudo cp packaging/70-desklink-uinput.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

