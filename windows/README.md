# DeskLink Windows Host

Windows Host 负责输入源、状态机和 LAN 发送。当前命令行 MVP 用于验证网络链路；实际接入 Raw Input/`WH_KEYBOARD_LL`/`WH_MOUSE_LL` 时，将事件转换为 `desklink_protocol::InputEvent` 后调用 `InputSender::send`。

环境变量：`DESKLINK_TARGET=Ubuntu_IP:24801`。

