# DeskLink 鎶€鏈爤涓庡紑鍙戞寚瀵?
## 1. 椤圭洰瀹氫綅

DeskLink 鐨勭洰鏍囦笉鏄仛杩滅▼妗岄潰锛岃€屾槸鍋氫竴涓笓娉ㄤ簬灞€鍩熺綉鐜鐨勮法璁惧閿紶鍗忓悓宸ュ叿銆?
褰撳墠鏍稿績鍦烘櫙锛?
- 閿洏銆侀紶鏍囩墿鐞嗚繛鎺ュ湪 Windows 涓绘満銆?- Windows 涓绘満鎷ユ湁澶氫釜鏄剧ず鍣ㄣ€?- Ubuntu 鏄嫭绔嬬數鑴戯紝鎷ユ湁鑷繁鐨勬樉绀哄櫒銆?- Windows 涓?Ubuntu 浣嶄簬鍚屼竴灞€鍩熺綉銆?- 鐢ㄦ埛鍙互鍦ㄩ厤缃晫闈腑鑷敱鎸囧畾 Ubuntu 灞忓箷浣嶄簬 Windows 鍝潡鏄剧ず鍣ㄧ殑宸︺€佸彸銆佷笂銆佷笅銆?- 榧犳爣绉诲姩鍒板搴旇竟缂樺悗鑷姩杩涘叆 Ubuntu銆?- 閿洏杈撳叆璺熼殢褰撳墠榧犳爣鎵€鍦ㄨ澶囥€?- 榧犳爣浠?Ubuntu 瀵瑰簲杈圭紭绉诲姩鍚庤繑鍥?Windows銆?- 涓嶄紶杈撴闈㈢敾闈紝鍙紶杈撹緭鍏ヤ簨浠讹紝鍥犳浼樺厛杩芥眰浣庡欢杩熶笌绋冲畾鎬с€?
寮€鍙戝師鍒欙細

```text
绋冲畾鎬?> 杈撳叆瀹夊叏 > 浣庡欢杩?> 浣跨敤浣撻獙 > 闄勫姞鍔熻兘
```

绗竴鐗堜笉瑕佽拷姹傚姛鑳芥暟閲忥紝鍏堟妸锛?
```text
Windows 杈撳叆鎹曡幏
        鈫?璺ㄥ睆鐘舵€佹満
        鈫?灞€鍩熺綉浼犺緭
        鈫?Ubuntu uinput
```

杩欎竴鏉￠摼璺交搴曞仛绋冲畾銆?
---

# 2. 鎺ㄨ崘璇█

## 涓讳綋璇█锛歊ust

寤鸿鏁翠釜椤圭洰涓讳綋浣跨敤锛?
```text
Rust
```

鍖呮嫭锛?
- Windows Host
- Ubuntu Client
- 閫氫俊鍗忚
- 鐘舵€佹満
- 灞忓箷鎷撴墤
- 閰嶇疆绠＄悊
- GUI
- 鏃ュ織
- 鑷姩鍙戠幇
- 鍚庣画鍔犲瘑閫氫俊

涓嶅缓璁涓€鐗堜娇鐢?Python 浣滀负涓讳綋銆?
Python 寰堥€傚悎鍋氬師鍨嬶紝浣嗛敭榧犲崗鍚屽睘浜庨暱鏈熷悗鍙拌繍琛岀殑杞欢锛岄渶瑕侊細

- 楂橀杈撳叆浜嬩欢澶勭悊
- 杈冧綆寤惰繜
- 绋冲畾鍐呭瓨鍗犵敤
- 绯荤粺 API 璋冪敤
- 澶氱嚎绋?寮傛缃戠粶
- 宕╂簝椋庨櫓鎺у埗
- 鍗曟枃浠堕儴缃?
Rust 鏇村悎閫傘€?
涔熶笉寤鸿涓€寮€濮嬬敤 C++銆?
C++ 瀹屽叏鍙互瀹炵幇锛屼絾 Windows Hook銆佺綉缁溿€佺嚎绋嬨€佽祫婧愰噴鏀俱€丩inux ioctl 绛夐儴鍒嗕細浜х敓鏇村鍐呭瓨鍜岀敓鍛藉懆鏈熺鐞嗗伐浣溿€俁ust 鍙互淇濈暀鎺ヨ繎 C/C++ 鐨勬€ц兘锛屽悓鏃跺噺灏戝ぇ閲忚祫婧愮鐞嗛棶棰樸€?
鎺ㄨ崘缁撹锛?
```text
鏍稿績绋嬪簭锛歊ust
閰嶇疆鏂囦欢锛歍OML
GUI锛歊ust 鍘熺敓 GUI
鍗忚锛氳嚜瀹氫箟浜岃繘鍒跺崗璁?Windows API锛歸indows-rs
Linux 杈撳叆锛?dev/uinput
缃戠粶锛歍okio + UDP/TCP
```

---

# 3. 鎬讳綋鎶€鏈爤

鎺ㄨ崘锛?
| 妯″潡 | 鎶€鏈?|
|---|---|
| 涓昏瑷€ | Rust |
| Windows API | `windows` crate / Win32 API |
| Linux 杈撳叆娉ㄥ叆 | `/dev/uinput` |
| Linux 绯荤粺鎺ュ彛 | `evdev` / `nix` / 蹇呰鏃剁洿鎺?ioctl |
| 寮傛杩愯鏃?| Tokio |
| 楂橀杈撳叆浼犺緭 | UDP |
| 鎺у埗閫氶亾 | TCP |
| 搴忓垪鍖?| 鑷畾涔変簩杩涘埗鍗忚 |
| 閰嶇疆 | serde + toml |
| 鏃ュ織 | tracing + tracing-subscriber |
| GUI | egui / eframe |
| 鎵樼洏 | tray-icon |
| 鑷姩鍙戠幇 | mDNS 鎴?UDP Broadcast |
| 鍔犲瘑 | 鍚庢湡 rustls / QUIC |
| Linux 鍚庡彴鏈嶅姟 | systemd |
| Windows 鍚庡彴鍚姩 | Startup / Task Scheduler |
| 鏋勫缓绠＄悊 | Cargo Workspace |
| CI | GitHub Actions |

---

# 4. 涓轰粈涔堜笉寤鸿 Electron / Qt / Web 鎶€鏈綔涓烘牳蹇?
閿紶鍗忓悓鐨勬牳蹇冮摼璺笉搴旇渚濊禆 GUI銆?
閿欒鏋舵瀯锛?
```text
Mouse
 鈫?GUI
 鈫?JavaScript
 鈫?WebSocket
 鈫?GUI
 鈫?Linux
```

鎺ㄨ崘鏋舵瀯锛?
```text
Mouse
 鈫?Native Input Thread
 鈫?Protocol
 鈫?UDP
 鈫?Native Receiver
 鈫?uinput
```

GUI 鍙槸鎺у埗灞傦細

```text
GUI
 鈹溾攢 淇敼閰嶇疆
 鈹溾攢 鏄剧ず杩炴帴鐘舵€? 鈹溾攢 鎺掑垪灞忓箷
 鈹斺攢 鏌ョ湅寤惰繜
```

GUI 宕╂簝鐢氳嚦鍏抽棴鏃讹紝涓嶅簲璇ュ奖鍝嶈緭鍏ユ牳蹇冩湇鍔°€?
鍥犳鎺ㄨ崘锛?
```text
Core Service
+
Optional GUI
```

鑰屼笉鏄細

```text
GUI Application = Everything
```

---

# 5. 宸ョ▼缁撴瀯寤鸿

閲囩敤 Cargo Workspace銆?
```text
laninput/
鈹?鈹溾攢鈹€ Cargo.toml
鈹?鈹溾攢鈹€ crates/
鈹?  鈹?鈹?  鈹溾攢鈹€ protocol/
鈹?  鈹?  鈹溾攢鈹€ src/
鈹?  鈹?  鈹?  鈹溾攢鈹€ packet.rs
鈹?  鈹?  鈹?  鈹溾攢鈹€ input.rs
鈹?  鈹?  鈹?  鈹溾攢鈹€ control.rs
鈹?  鈹?  鈹?  鈹斺攢鈹€ lib.rs
鈹?  鈹?鈹?  鈹溾攢鈹€ topology/
鈹?  鈹?  鈹溾攢鈹€ src/
鈹?  鈹?  鈹?  鈹溾攢鈹€ screen.rs
鈹?  鈹?  鈹?  鈹溾攢鈹€ edge.rs
鈹?  鈹?  鈹?  鈹溾攢鈹€ mapping.rs
鈹?  鈹?  鈹?  鈹斺攢鈹€ lib.rs
鈹?  鈹?鈹?  鈹溾攢鈹€ network/
鈹?  鈹?  鈹溾攢鈹€ src/
鈹?  鈹?  鈹?  鈹溾攢鈹€ udp.rs
鈹?  鈹?  鈹?  鈹溾攢鈹€ tcp.rs
鈹?  鈹?  鈹?  鈹溾攢鈹€ heartbeat.rs
鈹?  鈹?  鈹?  鈹溾攢鈹€ discovery.rs
鈹?  鈹?  鈹?  鈹斺攢鈹€ lib.rs
鈹?  鈹?鈹?  鈹溾攢鈹€ config/
鈹?  鈹?  鈹溾攢鈹€ src/
鈹?  鈹?  鈹?  鈹斺攢鈹€ lib.rs
鈹?  鈹?鈹?  鈹斺攢鈹€ common/
鈹?      鈹溾攢鈹€ src/
鈹?      鈹?  鈹溾攢鈹€ error.rs
鈹?      鈹?  鈹溾攢鈹€ state.rs
鈹?      鈹?  鈹斺攢鈹€ lib.rs
鈹?鈹溾攢鈹€ apps/
鈹?  鈹?鈹?  鈹溾攢鈹€ windows-host/
鈹?  鈹?  鈹溾攢鈹€ src/
鈹?  鈹?  鈹?  鈹溾攢鈹€ raw_input.rs
鈹?  鈹?  鈹?  鈹溾攢鈹€ keyboard_hook.rs
鈹?  鈹?  鈹?  鈹溾攢鈹€ mouse_hook.rs
鈹?  鈹?  鈹?  鈹溾攢鈹€ monitor.rs
鈹?  鈹?  鈹?  鈹溾攢鈹€ hotkey.rs
鈹?  鈹?  鈹?  鈹溾攢鈹€ switcher.rs
鈹?  鈹?  鈹?  鈹斺攢鈹€ main.rs
鈹?  鈹?鈹?  鈹溾攢鈹€ linux-client/
鈹?  鈹?  鈹溾攢鈹€ src/
鈹?  鈹?  鈹?  鈹溾攢鈹€ uinput.rs
鈹?  鈹?  鈹?  鈹溾攢鈹€ keyboard.rs
鈹?  鈹?  鈹?  鈹溾攢鈹€ mouse.rs
鈹?  鈹?  鈹?  鈹溾攢鈹€ cursor_state.rs
鈹?  鈹?  鈹?  鈹斺攢鈹€ main.rs
鈹?  鈹?鈹?  鈹斺攢鈹€ laninput-gui/
鈹?      鈹溾攢鈹€ src/
鈹?      鈹?  鈹溾攢鈹€ layout.rs
鈹?      鈹?  鈹溾攢鈹€ settings.rs
鈹?      鈹?  鈹溾攢鈹€ status.rs
鈹?      鈹?  鈹斺攢鈹€ main.rs
鈹?鈹溾攢鈹€ docs/
鈹斺攢鈹€ tests/
```

鏍稿績鍘熷垯锛?
```text
protocol 涓嶄緷璧?Windows/Linux
topology 涓嶄緷璧?Windows/Linux
network 涓嶄緷璧?GUI
GUI 涓嶇洿鎺ュ鐞嗛敭榧犱簨浠?```

杩欐牱浠ュ悗鎵╁睍锛?
```text
Windows 鈫?Windows
Linux 鈫?Linux
Windows 鈫?澶氬彴 Linux
```

涓嶉渶瑕侀噸鍐欐牳蹇冩灦鏋勩€?
---

# 6. Windows Host 鎶€鏈柟妗?
Windows 鏄涓€鐗堢殑杈撳叆涓绘満銆?
## 6.1 Windows API

Rust 浣跨敤锛?
```text
windows
```

crate 璋冪敤鍘熺敓 Win32 API銆?
閲嶇偣 API锛?
```text
RegisterRawInputDevices
GetRawInputData
SetWindowsHookExW
CallNextHookEx
GetCursorPos
SetCursorPos
ClipCursor
EnumDisplayMonitors
GetMonitorInfoW
QueryDisplayConfig
RegisterHotKey
```

---

# 7. Windows 榧犳爣杈撳叆

寤鸿缁勫悎浣跨敤锛?
```text
Raw Input
+
WH_MOUSE_LL
```

涓よ€呰亴璐ｄ笉鍚屻€?
## Raw Input

璐熻矗鑾峰彇楂樼簿搴﹁緭鍏ワ細

```text
dx
dy
button
wheel
```

浼樼偣锛?
- 閫傚悎楂樿疆璇㈢巼榧犳爣
- 寰楀埌鐩稿杩愬姩閲?- 涓嶄緷璧栨闈㈤紶鏍囧姞閫熷害鍚庣殑鏈€缁堝潗鏍?- 鏇撮€傚悎缃戠粶杞彂

鏍稿績浣跨敤锛?
```text
RegisterRawInputDevices
WM_INPUT
GetRawInputData
```

## Low Level Mouse Hook

璐熻矗杩滅▼鎺у埗鐘舵€佷笅闃绘杈撳叆缁х画浣滅敤浜?Windows 鏈満銆?
娉ㄦ剰锛?
```text
Raw Input 鍙兘璇诲彇杈撳叆
涓嶈兘澶╃劧闃绘杈撳叆缁х画鍙戦€佺粰 Windows 搴旂敤
```

鍥犳杩涘叆 Ubuntu 鍚庯紝闇€瑕佸彟澶栨帶鍒?Windows 鏈湴杈撳叆銆?
寤鸿锛?
```text
LOCAL 鐘舵€侊細
    姝ｅ父鏀捐

REMOTE 鐘舵€侊細
    鎹曡幏杈撳叆
    杞彂 Ubuntu
    蹇呰鏃堕樆姝㈡湰鍦伴紶鏍囨寜閿?婊氳疆浜嬩欢
```

绗竴鐗堝彲浠ュ厛浣跨敤锛?
```text
WH_MOUSE_LL
```

瀹炵幇杩滅▼鐘舵€佷笅鐨勮緭鍏ユ姂鍒躲€?
---

# 8. Windows 閿洏杈撳叆

鎺ㄨ崘锛?
```text
WH_KEYBOARD_LL
+
蹇呰鏃?Raw Keyboard Input
```

Low Level Keyboard Hook 鐨勪竴涓噸瑕佺敤閫旀槸锛?
```text
REMOTE 鐘舵€佷笅
閿洏浜嬩欢鍙戝線 Ubuntu
鍚屾椂涓嶅啀浜ょ粰 Windows 褰撳墠搴旂敤
```

蹇呴』澶勭悊锛?
```text
KeyDown
KeyUp
SysKeyDown
SysKeyUp
```

灏ゅ叾娉ㄦ剰锛?
```text
Ctrl
Alt
Shift
Win
```

涓嶈兘鍙彂閫佸瓧绗︺€?
鍗忚灞傚簲璇ュ彂閫侊細

```text
Physical Key Code
+
Pressed/Released
```

涓嶈鍙戦€侊細

```text
"a"
"b"
"c"
```

鍥犱负杩欎細鐮村潖锛?
- 蹇嵎閿?- 娓告垙鎸夐敭
- 涓嶅悓杈撳叆娉?- Ctrl/Alt/Shift
- 鍔熻兘閿?- 灏忛敭鐩?- 澶氬獟浣撻敭

---

# 9. 绱ф€ユ仮澶嶉敭

杩欐槸 P0 鍔熻兘銆?
寤鸿锛?
```text
Ctrl + Alt + Esc
```

浣跨敤 Windows锛?
```text
RegisterHotKey
```

鎴栧崟鐙殑 Hook 鍒ゆ柇銆?
璇ュ揩鎹烽敭涓嶈兘琚彂閫佺粰 Ubuntu銆?
鏃犺褰撳墠鐘舵€佸浣曪紝閮芥墽琛岋細

```text
1. active_device = Windows
2. 鍋滄杩滅▼杈撳叆鍙戦€?3. ReleaseAllRemoteKeys
4. 瑙ｉ櫎 ClipCursor
5. 鎭㈠ Windows 榧犳爣
6. 娓呯┖杈撳叆鐘舵€?```

鍗充娇锛?
- Ubuntu 姝绘満
- 缃戠粶鏂紑
- Client 宕╂簝
- UDP 绾跨▼寮傚父

涔熷繀椤绘湁鏁堛€?
---

# 10. Windows 澶氭樉绀哄櫒鎶€鏈?
浣跨敤锛?
```text
EnumDisplayMonitors
GetMonitorInfoW
QueryDisplayConfig
```

鑾峰彇姣忎竴鍧楀疄闄呭睆骞曠殑淇℃伅銆?
鍐呴儴鏁版嵁缁撴瀯寤鸿锛?
```rust
struct Monitor {
    id: MonitorId,
    name: String,

    x: i32,
    y: i32,

    width: u32,
    height: u32,

    scale: f32,
    rotation: Rotation,

    primary: bool,
}
```

娉ㄦ剰 Windows 澶氬睆鍧愭爣鍙互鍑虹幇锛?
```text
璐熸暟
```

渚嬪锛?
```text
宸︿晶鏄剧ず鍣細

x = -1920
y = 0
```

鍥犳缁濆涓嶈兘浣跨敤锛?
```text
u32
```

瀛樺偍灞忓箷鍧愭爣銆?
蹇呴』锛?
```text
i32
```

---

# 11. Ubuntu 灞忓箷浣嶇疆璁捐

涓嶈鍐欐锛?
```text
Ubuntu = Windows 鏈€鍙充晶
```

搴旇寤虹珛閫昏緫鎷撴墤銆?
渚嬪锛?
```rust
struct LogicalScreen {
    id: ScreenId,
    device: DeviceId,

    x: i32,
    y: i32,

    width: u32,
    height: u32,
}
```

Windows锛?
```text
WIN-1
WIN-2
WIN-3
```

Ubuntu锛?
```text
UBUNTU-1
```

鍏ㄩ儴鏀惧叆鍚屼竴涓櫄鎷熷潗鏍囩郴銆?
渚嬪锛?
```text
             鈹屸攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?             鈹?Ubuntu     鈹?             鈹斺攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?
鈹屸攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?鈹屸攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?鈹?WIN-1      鈹?鈹?WIN-2      鈹?鈹斺攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?鈹斺攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?```

鎴栬€咃細

```text
鈹屸攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?鈹屸攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?鈹屸攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?鈹?WIN-1      鈹?鈹?WIN-2      鈹?鈹?Ubuntu     鈹?鈹斺攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?鈹斺攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?鈹斺攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?```

鐢ㄦ埛鎷栧姩 Ubuntu 灞忓箷鏂瑰潡鍚庯紝鍙慨鏀癸細

```text
x
y
```

涓嶉渶瑕佹敼杈撳叆浠ｇ爜銆?
---

# 12. 灞忓箷閭绘帴绠楁硶

寤鸿涓嶈鍙繚瀛橈細

```text
position = right
```

鏈€缁堝簲璇ユ牴鎹睆骞曠煩褰㈣嚜鍔ㄨ绠楃浉閭昏竟銆?
渚嬪锛?
```text
Screen A
x = 0
y = 0
w = 1920
h = 1080

Screen B
x = 1920
y = 300
w = 2560
h = 1440
```

涓ゅ潡灞忓箷鍙湁锛?
```text
Y = 300 ~ 1080
```

杩欎竴娈佃竟缂樼浉閭汇€?
鍥犳鍙厑璁革細

```text
Screen A 鍙宠竟缂樺搴斿尯闂?```

杩涘叆 Ubuntu銆?
鎺ㄨ崘鏁版嵁缁撴瀯锛?
```rust
struct EdgeLink {
    from: ScreenId,
    to: ScreenId,

    from_edge: Edge,
    to_edge: Edge,

    start_ratio: f32,
    end_ratio: f32,
}
```

浠ュ悗鍙互鑷姩鐢熸垚 EdgeLink銆?
---

# 13. 涓€涓叧閿璁★細涓嶈渚濊禆 Wayland 鑾峰彇榧犳爣鍧愭爣

杩欐槸鏁翠釜椤圭洰寰堥噸瑕佺殑涓€鐐广€?
Ubuntu Wayland 鐜涓嬶紝鏅€氱▼搴忛€氬父涓嶈兘鍍?X11 涓€鏍烽殢鎰忔煡璇㈠叏灞€榧犳爣浣嶇疆銆?
鍥犳涓嶈璁捐鎴愶細

```text
涓嶆柇璇㈤棶 Ubuntu锛?榧犳爣鐜板湪鍦ㄥ摢锛?```

鎺ㄨ崘锛?
```text
DeskLink 鑷繁缁存姢 Remote Cursor State
```

Windows 鍙戦€侊細

```text
dx
dy
```

Linux Client 涓€杈瑰皢锛?
```text
dx/dy
```

鍐欏叆 `uinput`锛?
涓€杈硅嚜宸辩疮璁￠€昏緫榧犳爣鍧愭爣锛?
```rust
cursor_x += dx;
cursor_y += dy;
```

骞舵寜鐓?Ubuntu 灞忓箷灏哄锛?
```text
0 <= x < width
0 <= y < height
```

缁存姢涓€涓€昏緫鍧愭爣銆?
杩欐牱灏卞彲浠ュ垽鏂細

```text
cursor_x <= 0
```

鏄惁鍑嗗浠?Ubuntu 宸︿晶杩斿洖 Windows銆?
杩欎釜璁捐鍙互澶у箙鍑忓皯锛?
```text
X11 / Wayland / GNOME / KDE
```

涔嬮棿鐨勫樊寮傘€?
---

# 14. Linux 杈撳叆娉ㄥ叆

Ubuntu 绔牳蹇冩妧鏈細

```text
/dev/uinput
```

涓嶈鎶婏細

```text
xdotool
XTest
```

浣滀负鏍稿績杈撳叆鏂规銆?
uinput 鍙互鍒涘缓 Linux 鍐呮牳绾ц櫄鎷熻緭鍏ヨ澶囥€?
寤鸿鍒涘缓涓や釜璁惧锛?
```text
DeskLink Virtual Keyboard
DeskLink Virtual Mouse
```

鑰屼笉鏄竴涓澶囧悓鏃舵ā鎷熸墍鏈変簨浠躲€?
Mouse锛?
```text
EV_REL
REL_X
REL_Y
REL_WHEEL
REL_HWHEEL

BTN_LEFT
BTN_RIGHT
BTN_MIDDLE
BTN_SIDE
BTN_EXTRA
```

Keyboard锛?
```text
EV_KEY
KEY_A
KEY_B
KEY_LEFTCTRL
KEY_LEFTSHIFT
...
```

Rust 灞傚彲浠ワ細

```text
浼樺厛浣跨敤鎴愮啛 evdev/uinput 灏佽
```

濡傛灉灏佽涓嶈兘婊¤冻瑕佹眰锛屽啀閫氳繃锛?
```text
nix
libc
ioctl
```

鐩存帴鎿嶄綔 `/dev/uinput`銆?
鏍稿績鏋舵瀯涓嶈缁戝畾鏌愪竴涓涓夋柟 crate銆?
---

# 15. uinput 鏉冮檺

Linux 鏅€氱敤鎴烽€氬父涓嶈兘鐩存帴璁块棶锛?
```text
/dev/uinput
```

涓嶈绠€鍗曡姹傜敤鎴锋瘡娆★細

```bash
sudo laninput
```

鏈€缁堝缓璁€氳繃锛?
```text
udev rule
```

缁?DeskLink 浣跨敤鎵€闇€鏉冮檺銆?
鎴栬€呰 Linux Core Service锛?
```text
systemd
```

浠ュ彈鎺ф潈闄愯繍琛屻€?
鎺ㄨ崘鏈€缁堢粨鏋勶細

```text
laninput-linux-service
        鈫?      uinput

laninput-gui
        鈫?IPC
laninput-linux-service
```

GUI 鏈韩涓嶉渶瑕?root銆?
---

# 16. 缃戠粶鏂规

绗竴鐗堟帹鑽愶細

```text
TCP + UDP
```

涓嶈绗竴鐗堢洿鎺ヤ笂锛?
```text
WebRTC
HTTP
gRPC
QUIC
ZeroMQ
ROS2
```

杩欎簺閮戒笉鏄綋鍓嶉棶棰樻墍闇€瑕佺殑銆?
---

# 17. TCP 鎺у埗閫氶亾

TCP 鐢ㄤ簬浣庨鍙潬淇℃伅锛?
```text
杩炴帴鎻℃墜
璁惧淇℃伅
鏄剧ず鍣ㄤ俊鎭?閰嶇疆鍚屾
鐘舵€佸悓姝?蹇冭烦
閰嶅
鍓创鏉?閿欒鎶ュ憡
```

Tokio锛?
```text
tokio::net::TcpListener
tokio::net::TcpStream
```

鍗冲彲銆?
---

# 18. UDP 杈撳叆閫氶亾

UDP 鐢ㄤ簬锛?
```text
MouseMove
MouseWheel
```

鍘熷洜锛?
榧犳爣浜嬩欢鏈€閲嶈鐨勬槸锛?
```text
鏈€鏂扮姸鎬?```

涓€涓や釜鏃?MouseMove 鍖呬涪澶憋紝涓嶅€煎緱璁╁悗闈㈢殑鍖呯瓑寰呴噸浼犮€?
TCP 濡傛灉鍑虹幇涓㈠寘锛?
```text
Packet 100 涓㈠け
Packet 101
Packet 102
Packet 103
```

鍚庨潰鐨勫寘鍙兘绛夊緟锛?
```text
100 閲嶄紶
```

浜х敓 Head-of-Line Blocking銆?
榧犳爣浼氳〃鐜颁负锛?
```text
绐佺劧鍗′竴涓?```

UDP 鏇寸鍚堥珮棰戦紶鏍囨暟鎹€?
---

# 19. 閿洏鏄惁浣跨敤 UDP

閿洏鍜岄紶鏍囦笉鍚屻€?
閿洏缁濆涓嶈兘杞绘槗涓細

```text
KEY_DOWN Ctrl
```

鎴栬€咃細

```text
KEY_UP Ctrl
```

鍚﹀垯鍙兘鍑虹幇锛?
```text
Ctrl 鍗′綇
```

绗竴鐗堝缓璁細

```text
MouseMove        鈫?UDP
MouseWheel       鈫?UDP

MouseButton      鈫?Reliable Channel
Keyboard         鈫?Reliable Channel
Control          鈫?TCP
```

绗竴鐗堝彲闈犻€氶亾鐩存帴浣跨敤 TCP 鍗冲彲銆?
绛夋牳蹇冨姛鑳藉畬鍏ㄧǔ瀹氫箣鍚庯紝鍐嶈€冭檻锛?
```text
Reliable UDP
```

鎴栬€咃細

```text
QUIC
```

娌℃湁蹇呰鎻愬墠澶嶆潅鍖栥€?
---

# 20. 缃戠粶鍗忚

涓嶈浣跨敤 JSON 鍙戦€侀紶鏍囦簨浠躲€?
閿欒锛?
```json
{
  "type": "mouse_move",
  "dx": 12,
  "dy": -4
}
```

楂橀浜嬩欢寤鸿浣跨敤鍥哄畾闀垮害浜岃繘鍒跺寘銆?
渚嬪锛?
```rust
#[repr(C)]
struct MouseMovePacket {
    version: u8,
    kind: u8,
    sequence: u32,
    dx: i16,
    dy: i16,
}
```

鍗曚釜鍖呮帶鍒跺湪闈炲父灏忕殑澶у皬銆?
鎺у埗淇℃伅鍙互浣跨敤锛?
```text
serde
+
bincode/postcard
```

鎴栬嚜宸辩殑缂栫爜銆?
寤鸿绗竴鐗堬細

```text
Input Event 鈫?鍥哄畾浜岃繘鍒剁粨鏋?Control     鈫?serde 搴忓垪鍖?```

---

# 21. Sequence ID

UDP MouseMove 鍖呭繀椤诲寘鍚細

```text
sequence
```

渚嬪锛?
```text
1001
1002
1003
1004
```

Ubuntu 鏀跺埌锛?
```text
1001
1003
1002
```

鍙互鐩存帴涓㈠純锛?
```text
1002
```

鍥犱负瀹冨凡缁忚繃鏃躲€?
杩欐牱鍙互閬垮厤 UDP 涔卞簭瀵艰嚧锛?
```text
榧犳爣鍊掗€€
```

---

# 22. 杈撳叆鐘舵€佹満

涓嶈鐢ㄥぇ閲忥細

```rust
if remote {
    ...
}
```

鍒嗘暎鍦ㄤ唬鐮佸悇澶勩€?
搴旇寤虹珛鏄庣‘鐘舵€佹満銆?
渚嬪锛?
```rust
enum ControlState {
    Local,
    SwitchingToRemote,
    Remote,
    SwitchingToLocal,
    Disconnected,
}
```

鏍稿績杞Щ锛?
```text
LOCAL
  鈹?  鈹?EdgeCross
  鈻?SWITCHING_TO_REMOTE
  鈹?  鈹?ACK
  鈻?REMOTE
  鈹?  鈹?RemoteEdgeCross
  鈻?SWITCHING_TO_LOCAL
  鈹?  鈻?LOCAL
```

寮傚父锛?
```text
REMOTE
  鈹?  鈹?Timeout
  鈻?DISCONNECTED
  鈹?  鈻?LOCAL
```

鎵€鏈夛細

```text
閿洏鎹曡幏
榧犳爣鎹曡幏
缃戠粶
灞忓箷鍒囨崲
```

閮借鍙栬繖涓€浠界姸鎬併€?
---

# 23. 榧犳爣杩涘叆 Ubuntu 鐨勬彙鎵?
涓嶈锛?
```text
纰拌竟
鈫?绔嬪嵆寮€濮嬪彂閫佹墍鏈夐敭榧?```

鎺ㄨ崘锛?
```text
Windows:
EDGE_ENTER_REQUEST
        鈫?Ubuntu:
READY
        鈫?Windows:
REMOTE
```

瀹屾垚涔嬪悗鍐嶇湡姝ｆ崟鑾锋湰鍦拌緭鍏ャ€?
杩欐牱鍙互閬垮厤锛?
```text
Ubuntu Client 宸叉寕
浣嗘槸 Windows 榧犳爣宸茬粡琚攣浣?```

---

# 24. Keyboard State

Host 缁存姢锛?
```rust
struct KeyboardState {
    pressed_keys: HashSet<KeyCode>,
}
```

鍒囨崲璁惧鏃跺彂閫侊細

```text
KeyboardSnapshot
```

鏂紑杩炴帴锛?
```text
ReleaseAll
```

鐗瑰埆澶勭悊锛?
```text
Ctrl
Shift
Alt
Win
```

鍚屾椂寤鸿瀹氭湡鍙戦€佷綆棰戠姸鎬佸揩鐓т綔涓哄閿欍€?
---

# 25. GUI 鎶€鏈€夋嫨

鎺ㄨ崘锛?
```text
egui + eframe
```

鍘熷洜锛?
- Rust 鍘熺敓
- 涓嶉渶瑕?Node.js
- 涓嶉渶瑕佹祻瑙堝櫒 Runtime
- 閫傚悎閰嶇疆宸ュ叿
- 璺?Windows/Linux
- 缁樺埗灞忓箷甯冨眬寰堟柟渚?
鎴戜滑鐨?GUI 鏈韩姣旇緝绠€鍗曪細

```text
璁惧鍒楄〃
灞忓箷鏂瑰潡
鎷栧姩鎺掑垪
杩炴帴鐘舵€?寤惰繜
蹇嵎閿缃?寮€鏈哄惎鍔?```

egui 寰堥€傚悎銆?
涓嶅缓璁涓€鐗堜娇鐢細

```text
Electron
```

Tauri 鍙互鐢紝浣嗗杩欎釜椤圭洰娌℃湁鏄庢樉蹇呰銆?
濡傛灉浠ュ悗 GUI 寰堝鏉傦紝鍙互鍐嶈€冭檻 Tauri銆?
---

# 26. 灞忓箷鎷栨嫿 GUI

閰嶇疆绐楀彛鏍稿績鐢诲竷锛?
```text
鈹屸攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?鈹?                                鈹?鈹?       鈹屸攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?            鈹?鈹?       鈹?WIN-2    鈹?            鈹?鈹?       鈹斺攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?            鈹?鈹?                                鈹?鈹?鈹屸攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?鈹屸攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?     鈹?鈹?鈹?WIN-1    鈹?鈹?WIN-3    鈹?     鈹?鈹?鈹斺攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?鈹斺攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?     鈹?鈹?                        鈹屸攢鈹€鈹€鈹€鈹€鈹?鈹?鈹?                        鈹俇buntu鈹?鈹?鈹?                        鈹斺攢鈹€鈹€鈹€鈹€鈹?鈹?鈹斺攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?```

Windows 灞忓箷浣嶇疆锛?
```text
鑷姩璇诲彇
```

Ubuntu锛?
```text
鍏佽鎷栧姩
```

鎷栧姩缁撴潫鍚庯細

```text
1. 鍚搁檮鍒版渶杩戝睆骞曡竟缂?2. 璁＄畻閫昏緫 x/y
3. 璁＄畻 EdgeLink
4. 淇濆瓨 TOML
```

绗竴鐗?GUI 涓嶉渶瑕佽嚜鐢辨诞鐐瑰竷灞€锛屽彲浠ュ仛锛?
```text
Snap To Edge
```

杩欐牱閫昏緫鏇寸畝鍗曘€?
---

# 27. 閰嶇疆鏂囦欢

浣跨敤锛?
```text
serde
+
toml
```

渚嬪锛?
```toml
[general]
emergency_hotkey = "Ctrl+Alt+Esc"
edge_delay_ms = 80

[network]
control_port = 24800
input_port = 24801

[[devices]]
id = "ubuntu-main"
name = "Ubuntu PC"
address = "192.168.66.101"

[[screens]]
id = "win-1"
device = "windows"
x = 0
y = 0
width = 1920
height = 1080

[[screens]]
id = "ubuntu-1"
device = "ubuntu-main"
x = 1920
y = 0
width = 2560
height = 1440
```

涓嶈鎶婄敤鎴疯缃啓姝诲湪浠ｇ爜閲屻€?
---

# 28. 鏃ュ織

浣跨敤锛?
```text
tracing
+
tracing-subscriber
```

鏃ュ織绛夌骇锛?
```text
ERROR
WARN
INFO
DEBUG
TRACE
```

姝ｅ父妯″紡涓嶈璁板綍姣忎竴涓?MouseMove銆?
鍚﹀垯锛?
```text
1000 Hz 榧犳爣
```

鍙兘姣忕浜х敓 1000 鏉℃棩蹇椼€?
鎺ㄨ崘锛?
```text
INFO:
杩炴帴
鍒囧睆
鏂紑
璁惧鍙戠幇
寮傚父

DEBUG:
鐘舵€佽浆绉?閰嶇疆
灞忓箷鎷撴墤

TRACE:
杈撳叆浜嬩欢
```

TRACE 榛樿鍏抽棴銆?
---

# 29. 绾跨▼妯″瀷

涓嶈璁╄緭鍏?Hook 涓庣綉缁?IO 娣峰湪鍚屼竴涓嚎绋嬨€?
Windows 鎺ㄨ崘锛?
```text
Main Thread
鈹?鈹溾攢 Windows Message Loop
鈹?鈹溾攢 Raw Input
鈹?鈹斺攢 Hooks
        鈹?        鈻?Lock-Free / Bounded Channel
        鈹?        鈻?Network Runtime
        鈹?        鈻?UDP/TCP
```

杈撳叆 Hook 涓姝細

```text
缃戠粶绛夊緟
鏂囦欢 IO
澶嶆潅鏃ュ織
澶嶆潅搴忓垪鍖?sleep
```

Hook 鍙礋璐ｏ細

```text
璇诲彇
鍒ゆ柇
鍏ラ槦
杩斿洖
```

杩欐槸浣庡欢杩熺殑閲嶈鍘熷垯銆?
Rust 鍙互浣跨敤锛?
```text
crossbeam-channel
tokio::sync
```

鏍规嵁瀹為檯鎯呭喌閫夋嫨銆?
---

# 30. 楂樿疆璇㈢巼榧犳爣

蹇呴』浠庝竴寮€濮嬭€冭檻锛?
```text
125 Hz
500 Hz
1000 Hz
2000 Hz
```

鐢氳嚦鏇撮珮棰戣緭鍏ャ€?
涓嶈瀵规瘡涓?MouseMove锛?
```text
spawn 涓€涓?async task
```

姝ｇ‘鏂瑰紡锛?
```text
Input Queue
    鈫?Dedicated Sender
```

蹇呰鏃跺彲浠ヨ繘琛屾瀬鐭椂闂寸獥鍙ｇ殑 MouseMove 鍚堝苟锛?
```text
dx1 + dx2 + dx3
dy1 + dy2 + dy3
```

浣嗙涓€鐗堝缓璁厛淇濇寔鍘熷浜嬩欢锛屾€ц兘涓嶈冻鏃跺啀浼樺寲銆?
---

# 31. 鑷姩鍙戠幇

绗竴鐗堬細

```text
鐩存帴濉啓 Ubuntu IP
```

灏卞浜嗐€?
绗簩鐗堝姞鍏ワ細

```text
mDNS
```

鎴栵細

```text
UDP Broadcast
```

鍙戠幇锛?
```text
DeskLink Ubuntu Client
```

渚嬪锛?
```text
YZ-Ubuntu
192.168.66.101
```

鐒跺悗鐢ㄦ埛鐐瑰嚮閰嶅銆?
涓嶈涓轰簡鑷姩鍙戠幇鑰借 MVP銆?
---

# 32. 瀹夊叏

绗竴鐗堝彧鍦ㄨ嚜宸辩殑鍙俊灞€鍩熺綉娴嬭瘯锛屽彲浠ュ厛閲囩敤锛?
```text
Shared Secret
```

浣嗘槸杞欢姝ｅ紡浣跨敤鍓嶈嚦灏戦渶瑕侊細

```text
璁惧璁よ瘉
```

鍚庢湡寤鸿锛?
```text
TLS
```

鎴栵細

```text
QUIC + TLS
```

涓嶈鍏佽锛?
```text
灞€鍩熺綉浠绘剰涓绘満
```

鐩存帴鍙?UDP InputPacket 鎺у埗 Ubuntu銆?
Ubuntu Client 蹇呴』鏍￠獙锛?
```text
Session ID
Device ID
Token
Sequence
```

---

# 33. 鍓创鏉?
鍓创鏉夸笉瑕佸拰杈撳叆閫氶亾鑰﹀悎銆?
鏈潵鐙珛妯″潡锛?
```text
clipboard/
```

閫氳繃鍙潬 TCP 閫氶亾浼狅細

```text
UTF-8 Text
```

绗竴鐗堝彧鍋氭枃鏈€?
鍚庣画鍐嶅仛锛?
```text
Image
HTML
File List
```

---

# 34. 鏂囦欢浼犺緭

鏂囦欢浼犺緭浼樺厛绾у簲璇ラ潪甯镐綆銆?
閿紶杞欢鐨勬垚鍔熸爣鍑嗕笉鏄細

```text
鍔熻兘澶?```

鑰屾槸锛?
```text
姣忓ぉ鐢ㄥ嚑涓皬鏃堕兘鎰熻涓嶅埌瀹冨瓨鍦?```

鍥犳鏂囦欢鎷栨嫿寤鸿鏀惧埌闈炲父鍚庨潰銆?
---

# 35. 鎺ㄨ崘寮€鍙戦樁娈?
## Phase 0锛氭妧鏈獙璇?
鍙獙璇侊細

```text
Windows 榧犳爣绉诲姩
        鈫?UDP
        鈫?Ubuntu uinput
```

鍔熻兘锛?
- Windows 璇诲彇 Raw Input
- Ubuntu 鍒涘缓 Virtual Mouse
- UDP 鍙戦€?dx/dy
- Ubuntu 榧犳爣鍙互绉诲姩

涓嶈锛?
- GUI
- 澶氬睆
- 閿洏
- 鑷姩鍙戠幇

---

# 36. Phase 1锛氬畬鏁撮敭榧犻摼璺?
澧炲姞锛?
```text
Mouse Button
Mouse Wheel
Keyboard
```

楠屾敹锛?
```text
Ubuntu 涓婂彲浠ユ甯革細
鐐瑰嚮
鎷栧姩
婊氬姩
鎵撳瓧
Ctrl+C
Ctrl+V
Ctrl+Shift+T
Alt+Tab
```

---

# 37. Phase 2锛歀ocal / Remote 鐘舵€佹満

瀹炵幇锛?
```text
LOCAL
REMOTE
```

浠ュ強锛?
```text
Ctrl + Alt + Esc
```

姝ゆ椂鍏堥€氳繃蹇嵎閿細

```text
Ctrl+Alt+2 鈫?Ubuntu
Ctrl+Alt+1 鈫?Windows
```

鎵嬪伐鍒囨崲銆?
鍏堜笉瑕佸仛杈圭紭绌胯秺銆?
濡傛灉鎵嬪姩鍒囨崲閮戒笉绋冲畾锛岃竟缂樺垏鎹㈠彧浼氳璋冭瘯鏇村洶闅俱€?
---

# 38. Phase 3锛歐indows 澶氭樉绀哄櫒

瀹炵幇锛?
```text
EnumDisplayMonitors
QueryDisplayConfig
```

姝ｇ‘璇诲彇锛?
- 灞忓箷鏁伴噺
- 浣嶇疆
- 鍒嗚鲸鐜?- DPI
- 涓诲睆
- 鏃嬭浆
- 璐熷潗鏍?
杈撳嚭璋冭瘯淇℃伅銆?
---

# 39. Phase 4锛氬睆骞曟嫇鎵?
瀹炵幇锛?
```text
LogicalScreen
EdgeLink
```

鍏堥€氳繃 TOML 閰嶇疆锛?
```text
Ubuntu 鍦?WIN-3 鍙宠竟
```

瀹炵幇锛?
```text
榧犳爣浠?WIN-3 鍙宠竟杩涘叆 Ubuntu
Ubuntu 宸﹁竟杩斿洖 WIN-3
```

---

# 40. Phase 5锛氬畨鍏ㄦ仮澶?
瀹屾垚锛?
```text
Heartbeat
Timeout
Disconnect Recovery
ReleaseAllKeys
Emergency Hotkey
```

楠屾敹鏂瑰紡锛?
Ubuntu 姝ｅ湪琚帶鍒舵椂鐩存帴鎵ц锛?
```bash
sudo poweroff
```

Windows 蹇呴』鑷姩鎭㈠鎺у埗銆?
杩欐槸姝ｅ紡鏃ョ敤鍓嶇殑纭寚鏍囥€?
---

# 41. Phase 6锛欸UI

鏍稿績绋冲畾涔嬪悗鍐嶅姞鍏ワ細

```text
egui
```

瀹炵幇锛?
```text
Windows 灞忓箷鑷姩鏄剧ず
Ubuntu 灞忓箷鎷栧姩
杈圭紭鍚搁檮
璁惧鐘舵€?寤惰繜
蹇嵎閿缃?```

---

# 42. Phase 7锛氫綋楠屽寮?
澧炲姞锛?
```text
鑷姩鍙戠幇
璁惧閰嶅
鏂囨湰鍓创鏉?寮€鏈哄惎鍔?鎵樼洏
閰嶇疆鑷姩淇濆瓨
```

鍒拌繖閲屽熀鏈彲浠ヤ綔涓烘棩甯歌蒋浠朵娇鐢ㄣ€?
---

# 43. 绗竴鐗堟槑纭笉鍋氱殑浜嬫儏

涓轰簡閬垮厤椤圭洰澶辨帶锛孧VP 涓嶅仛锛?
```text
macOS
鍏綉鎺у埗
杩滅▼妗岄潰
瑙嗛
闊抽
鏂囦欢鎷栨斁
澶嶆潅鏉冮檺绯荤粺
Web 绠＄悊鐣岄潰
浜戣处鎴?绉诲姩绔?ROS
澶氱敤鎴?鎻掍欢绯荤粺
```

鍙湇鍔★細

```text
Windows 澶氬睆 Host
        +
Ubuntu Client
        +
LAN
```

---

# 44. 鍏抽敭鎶€鏈毦鐐规帓搴?
## 闅剧偣 1锛歐indows 杈撳叆鎶戝埗

涓嶆槸璇诲彇閿紶锛岃€屾槸锛?
```text
杩涘叆 Ubuntu 鍚?Windows 鑷繁涓嶈兘鍚屾椂鍝嶅簲杩欎簺鎸夐敭
```

杩欐槸蹇呴』浼樺厛楠岃瘉鐨勬妧鏈偣銆?
---

## 闅剧偣 2锛氶敭鐩樼姸鎬佷竴鑷存€?
鐗瑰埆鏄細

```text
Ctrl
Alt
Shift
Win
```

浠讳綍 KEY_UP 涓㈠け閮藉彲鑳介€犳垚鍗￠敭銆?
鍥犳闇€瑕侊細

```text
State Snapshot
ReleaseAll
鍙潬閿洏閫氶亾
```

---

## 闅剧偣 3锛氳法灞忚繛缁€?
灞忓箷锛?
```text
鍒嗚鲸鐜囦笉鍚?姣斾緥涓嶅悓
涓婁笅娌℃湁瀹屽叏瀵归綈
```

閮藉繀椤绘纭鐞嗐€?
鍥犳蹇呴』浠庝竴寮€濮嬭璁★細

```text
LogicalScreen
+
EdgeLink
```

---

## 闅剧偣 4锛歐ayland

瑙ｅ喅鍘熷垯涓嶆槸锛?
```text
鎯冲姙娉曟帶鍒?Wayland
```

鑰屾槸锛?
```text
灏介噺缁曞紑妗岄潰鍗忚
```

杈撳叆锛?
```text
uinput
```

榧犳爣杈圭紭鍒ゆ柇锛?
```text
鑷繁鐨?Remote Cursor State
```

杩欐牱鍙渶澶х▼搴﹀噺灏?Wayland 鐗规畩澶勭悊銆?
---

# 45. 涓嶅缓璁殑鎶€鏈矾绾?
## 涓嶅缓璁?Python 鍋氭渶缁?Core

鍘熷瀷鍙互锛岄暱鏈熷悗鍙版牳蹇冧笉寤鸿銆?
---

## 涓嶅缓璁?Electron

閿紶鏈嶅姟娌℃湁蹇呰寮曞叆娴忚鍣?Runtime銆?
---

## 涓嶅缓璁?WebSocket 浼犻紶鏍?
灞€鍩熺綉鍘熺敓 UDP 鏇寸洿鎺ャ€?
---

## 涓嶅缓璁?ROS2

杩欎釜闂瀹屽叏娌℃湁蹇呰寮曞叆 DDS銆?
---

## 涓嶅缓璁?ZeroMQ 浣滀负绗竴鐗堜緷璧?
鍙互瀹炵幇锛屼絾褰撳墠闇€姹傦細

```text
UDP + TCP
```

宸茬粡瓒冲銆?
---

## 涓嶅缓璁洿鎺ヤ娇鐢ㄧ粷瀵归紶鏍囧潗鏍囦綔涓轰富瑕佸崗璁?
鎺ㄨ崘锛?
```text
dx / dy
```

璺ㄤ笉鍚屽垎杈ㄧ巼鏇村姞鑷劧銆?
---

## 涓嶅缓璁涓€鐗堝仛鍏ㄥ钩鍙?
鍏堝彧淇濊瘉锛?
```text
Windows 11
Ubuntu 22.04
```

绋冲畾銆?
绋冲畾涔嬪悗鍐嶆娊璞″叾浠栧钩鍙般€?
---

# 46. 鎺ㄨ崘渚濊禆鏂瑰悜

Cargo 渚濊禆鍙互鍥寸粫浠ヤ笅鏂瑰悜閫夋嫨锛?
```text
windows
tokio
serde
toml
tracing
tracing-subscriber
thiserror
bytes
crossbeam-channel
egui
eframe
tray-icon
```

Linux锛?
```text
evdev
nix
libc
```

涓嶉渶瑕佷竴寮€濮嬪叏閮ㄥ姞鍏ャ€?
閬靛惊鍘熷垯锛?
```text
闇€瑕佷竴涓姛鑳?鍐嶅姞鍏ヤ竴涓緷璧?```

閬垮厤鍒涘缓涓€涓潪甯搁噸鐨勫垵濮嬪伐绋嬨€?
---

# 47. MVP 鏈€灏忎緷璧?
鏈€鍒濇妧鏈獙璇佺敋鑷冲彧闇€瑕侊細

```text
windows
tokio
thiserror
```

Linux 鍐嶅姞锛?
```text
libc / nix
```

鎴栬€?uinput 灏佽銆?
鍏堝畬鎴愶細

```text
Windows:
Raw Input 鈫?UDP

Linux:
UDP 鈫?uinput
```

鍐嶇户缁€?
---

# 48. 寮€鍙戠幆澧冨缓璁?
Windows锛?
```text
Windows 11
Rust stable
Visual Studio Build Tools
Git
VS Code / RustRover
rust-analyzer
```

Ubuntu锛?
```text
Ubuntu 22.04
Rust stable
build-essential
pkg-config
Git
```

璋冭瘯锛?
```text
Windows Host锛?cargo run -p windows-host

Ubuntu锛?cargo run -p linux-client
```

---

# 49. 娴嬭瘯宸ュ叿

缃戠粶锛?
```text
ping
iperf3
Wireshark
```

Linux 杈撳叆锛?
```text
evtest
libinput debug-events
```

Windows锛?
```text
鏃ュ織杈撳嚭
Raw Input debug monitor
灞忓箷鍧愭爣 debug overlay
```

寤鸿寮€鍙戜竴涓?Debug Overlay锛?
```text
Current Monitor: WIN-3
Mouse: 1918, 532
State: LOCAL
Edge: RIGHT
Remote: Ubuntu
RTT: 0.72 ms
```

寮€鍙戦樁娈甸潪甯告湁甯姪銆?
姝ｅ紡鐗堝叧闂€?
---

# 50. 鏈€缁堝缓璁?
杩欎釜椤圭洰鏈€鍚堢悊鐨勬妧鏈矾绾挎槸锛?
```text
                         DeskLink
                            鈹?             鈹屸攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹粹攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?             鈹?                            鈹?      Windows Host                  Ubuntu Client
             鈹?                            鈹?       windows-rs                      /dev/uinput
             鈹?                            鈹?        Raw Input                        evdev
             鈹?                            鈹?    Keyboard/Mouse Hook                    鈹?             鈹斺攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?                            鈹?                      Tokio Network
                            鈹?                  鈹屸攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹粹攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹?                  鈹?                  鈹?                 UDP                 TCP
                  鈹?                  鈹?             Mouse Move        Key / Control
```

閰嶇疆锛?
```text
serde + TOML
```

GUI锛?
```text
egui / eframe
```

灞忓箷妯″瀷锛?
```text
Unified Virtual Desktop
+
LogicalScreen
+
EdgeLink
```

Ubuntu 鍏煎绛栫暐锛?
```text
uinput
+
鑷繁缁存姢 Remote Cursor State
```

寮€鍙戦『搴忥細

```text
1. Windows Raw Input
2. UDP
3. Ubuntu uinput
4. 榧犳爣
5. 閿洏
6. 鎵嬪姩鍒囨崲
7. 澶氭樉绀哄櫒
8. 灞忓箷鎷撴墤
9. 杈圭紭绌胯秺
10. 鏂嚎鎭㈠
11. GUI
12. 鍓创鏉?```

鏈€閲嶈鐨勫師鍒欙細

> 涓嶈浠?GUI 寮€濮嬶紝涓嶈浠庘€滃畬鏁磋蒋浠垛€濆紑濮嬨€傚厛鐢ㄦ渶灏戜唬鐮佽瘉鏄?Windows Raw Input 鈫?LAN 鈫?Ubuntu uinput 杩欐潯閾捐矾瓒冲绋冲畾锛岀劧鍚庢墍鏈夊姛鑳藉洿缁曡繖鏉℃牳蹇冮摼璺€愭澧炲姞銆?
