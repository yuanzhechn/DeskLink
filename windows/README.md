# DeskLink Windows Host

Windows Host 负责输入源、状态机和 LAN 发送。当前版本已接入 `WH_KEYBOARD_LL`/`WH_MOUSE_LL`：输入 `remote` 后会捕获真实键盘、鼠标移动、按键和滚轮事件；输入 `local` 或 `release` 后停止捕获并恢复本地输入。终端命令仍可用于链路调试。

环境变量：`DESKLINK_TARGET=Ubuntu_IP:24801`。
