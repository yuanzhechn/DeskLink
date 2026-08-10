# DeskLink Windows Host

Windows Host 负责输入源、屏幕布局、状态机和 LAN 发送。当前版本通过 `WH_KEYBOARD_LL`/`WH_MOUSE_LL` 捕获输入，并在光标穿过有效 EdgeLink 时自动把控制切换到 Linux。

高频输入使用 UDP 24801；双向纯文本剪贴板使用 TCP 24800，并通过 Windows 原生剪贴板 API 读写。详细配置、编译与启动方法见项目根目录 `README.md`。
