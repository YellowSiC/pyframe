from enum import Enum
from pathlib import Path
from typing import Any, Callable, Dict, List, Literal, Optional, Tuple
from uuid import UUID, uuid4

from pydantic import (BaseModel, ConfigDict, Field, computed_field,
                      model_validator)
from pydantic.alias_generators import to_camel

from ..executers.menu_executer import system_registry
from ..utils import current_folder_name


class BaseSchema(BaseModel):
    model_config = ConfigDict(
        alias_generator=to_camel,
        populate_by_name=True,
        from_attributes=True,
    )

    @model_validator(mode="after")
    def auto_register(self) -> "BaseSchema":
        # WICHTIG: Direkter Zugriff, nicht self["command"]
        if getattr(self, "command", None):
            event_id = system_registry.register(self.command)
            system_registry._callbacks[event_id]["item"] = self
            self._registered_uuid = event_id
        return self

    @computed_field  # NUR im JSON sichtbar, NICHT als Modell-Attribut!
    def command_id(self) -> Optional[str]:
        if hasattr(self, "_registered_uuid"):
            return str(self._registered_uuid)
        return None


class ActivationPolicy(str, Enum):
    regular = "regular"
    accessory = "accessory"
    prohibited = "prohibited"


class FrameBackgroundThrottlingPolicy(str, Enum):
    disabled = "disabled"
    suspend = "suspend"
    throttle = "throttle"


class AcceleratorCode(str, Enum):
    backquote = "backquote"
    backslash = "backslash"
    bracket_left = "bracketLeft"
    bracket_right = "bracketRight"
    comma = "comma"
    digit0 = "digit0"
    digit1 = "digit1"
    digit2 = "digit2"
    digit3 = "digit3"
    digit4 = "digit4"
    digit5 = "digit5"
    digit6 = "digit6"
    digit7 = "digit7"
    digit8 = "digit8"
    digit9 = "digit9"
    equal = "equal"
    intl_backslash = "intlBackslash"
    intl_ro = "intlRo"
    intl_yen = "intlYen"
    key_a = "keyA"
    key_b = "keyB"
    key_c = "keyC"
    key_d = "keyD"
    key_e = "keyE"
    key_f = "keyF"
    key_g = "keyG"
    key_h = "keyH"
    key_i = "keyI"
    key_j = "keyJ"
    key_k = "keyK"
    key_l = "keyL"
    key_m = "keyM"
    key_n = "keyN"
    key_o = "keyO"
    key_p = "keyP"
    key_q = "keyQ"
    key_r = "keyR"
    key_s = "keyS"
    key_t = "keyT"
    key_u = "keyU"
    key_v = "keyV"
    key_w = "keyW"
    key_x = "keyX"
    key_y = "keyY"
    key_z = "keyZ"
    minus = "minus"
    period = "period"
    quote = "quote"
    semicolon = "semicolon"
    slash = "slash"
    alt_left = "altLeft"
    alt_right = "altRight"
    backspace = "backspace"
    caps_lock = "capsLock"
    context_menu = "contextMenu"
    control_left = "controlLeft"
    control_right = "controlRight"
    enter = "enter"
    meta_left = "metaLeft"
    meta_right = "metaRight"
    shift_left = "shiftLeft"
    shift_right = "shiftRight"
    space = "space"
    tab = "tab"
    convert = "convert"
    kana_mode = "kanaMode"
    lang1 = "lang1"
    lang2 = "lang2"
    lang3 = "lang3"
    lang4 = "lang4"
    lang5 = "lang5"
    non_convert = "nonConvert"
    delete = "delete"
    end = "end"
    help = "help"
    home = "home"
    insert = "insert"
    page_down = "pageDown"
    page_up = "pageUp"
    arrow_down = "arrowDown"
    arrow_left = "arrowLeft"
    arrow_right = "arrowRight"
    arrow_up = "arrowUp"
    num_lock = "numLock"
    numpad0 = "numpad0"
    numpad1 = "numpad1"
    numpad2 = "numpad2"
    numpad3 = "numpad3"
    numpad4 = "numpad4"
    numpad5 = "numpad5"
    numpad6 = "numpad6"
    numpad7 = "numpad7"
    numpad8 = "numpad8"
    numpad9 = "numpad9"
    numpad_add = "numpadAdd"
    numpad_backspace = "numpadBackspace"
    numpad_clear = "numpadClear"
    numpad_clear_entry = "numpadClearEntry"
    numpad_comma = "numpadComma"
    numpad_decimal = "numpadDecimal"
    numpad_divide = "numpadDivide"
    numpad_enter = "numpadEnter"
    numpad_equal = "numpadEqual"
    numpad_hash = "numpadHash"
    numpad_memory_add = "numpadMemoryAdd"
    numpad_memory_clear = "numpadMemoryClear"
    numpad_memory_recall = "numpadMemoryRecall"
    numpad_memory_store = "numpadMemoryStore"
    numpad_memory_subtract = "numpadMemorySubtract"
    numpad_multiply = "numpadMultiply"
    numpad_paren_left = "numpadParenLeft"
    numpad_paren_right = "numpadParenRight"
    numpad_star = "numpadStar"
    numpad_subtract = "numpadSubtract"
    escape = "escape"
    fn = "fn"
    fn_lock = "fnLock"
    print_screen = "printScreen"
    scroll_lock = "scrollLock"
    pause = "pause"
    browser_back = "browserBack"
    browser_favorites = "browserFavorites"
    browser_forward = "browserForward"
    browser_home = "browserHome"
    browser_refresh = "browserRefresh"
    browser_search = "browserSearch"
    browser_stop = "browserStop"
    eject = "eject"
    launch_app1 = "launchApp1"
    launch_app2 = "launchApp2"
    launch_mail = "launchMail"
    media_play_pause = "mediaPlayPause"
    media_select = "mediaSelect"
    media_stop = "mediaStop"
    media_track_next = "mediaTrackNext"
    media_track_previous = "mediaTrackPrevious"
    power = "power"
    sleep = "sleep"
    audio_volume_down = "audioVolumeDown"
    audio_volume_mute = "audioVolumeMute"
    audio_volume_up = "audioVolumeUp"
    wake_up = "wakeUp"
    hyper = "hyper"
    super_ = "super"
    turbo = "turbo"
    abort = "abort"
    resume = "resume"
    suspend = "suspend"
    again = "again"
    copy = "copy"
    cut = "cut"
    find = "find"
    open_ = "open"
    paste = "paste"
    props = "props"
    select = "select"
    undo = "undo"
    hiragana = "hiragana"
    katakana = "katakana"
    unidentified = "unidentified"
    f1 = "f1"
    f2 = "f2"
    f3 = "f3"
    f4 = "f4"
    f5 = "f5"
    f6 = "f6"
    f7 = "f7"
    f8 = "f8"
    f9 = "f9"
    f10 = "f10"
    f11 = "f11"
    f12 = "f12"
    f13 = "f13"
    f14 = "f14"
    f15 = "f15"
    f16 = "f16"
    f17 = "f17"
    f18 = "f18"
    f19 = "f19"
    f20 = "f20"
    f21 = "f21"
    f22 = "f22"
    f23 = "f23"
    f24 = "f24"
    f25 = "f25"
    f26 = "f26"
    f27 = "f27"
    f28 = "f28"
    f29 = "f29"
    f30 = "f30"
    f31 = "f31"
    f32 = "f32"
    f33 = "f33"
    f34 = "f34"
    f35 = "f35"
    brightness_down = "brightnessDown"
    brightness_up = "brightnessUp"
    display_toggle_int_ext = "displayToggleIntExt"
    keyboard_layout_select = "keyboardLayoutSelect"
    launch_assistant = "launchAssistant"
    launch_control_panel = "launchControlPanel"
    launch_screen_saver = "launchScreenSaver"
    mail_forward = "mailForward"
    mail_reply = "mailReply"
    mail_send = "mailSend"
    media_fast_forward = "mediaFastForward"
    media_pause = "mediaPause"
    media_play = "mediaPlay"
    media_record = "mediaRecord"
    media_rewind = "mediaRewind"
    microphone_mute_toggle = "microphoneMuteToggle"
    privacy_screen_toggle = "privacyScreenToggle"
    select_task = "selectTask"
    show_all_windows = "showAllWindows"
    zoom_toggle = "zoomToggle"


class AcceleratorModifier(str, Enum):
    alt = "alt"
    altgraph = "altgraph"
    scrolllock = "scrolllock"
    shift = "shift"
    super_ = "super"  # 'super' ist ein reserviertes Wort in Python, daher '_'
    symbol = "symbol"
    symbollock = "symbollock"
    fn = "fn"
    fnlock = "fnlock"
    numlock = "numlock"
    capslock = "capslock"
    control = "control"
    hyper = "hyper"
    meta = "meta"


class HeaderData(BaseSchema):
    key: str
    value: Optional[str]


class IpcMessage(BaseSchema):
    body: dict
    method: str
    headers: List[HeaderData]
    uri: str


class MenuItem(BaseSchema):
    text: str
    enabled: bool
    modifier: AcceleratorModifier
    key: AcceleratorCode
    command: Optional[Callable[..., Any]] = Field(
        default=None, exclude=True
    )  # bleibt unsichtbar!


class CheckMenuItem(BaseSchema):
    text: str
    enabled: bool
    checked: bool
    modifier: AcceleratorModifier
    key: AcceleratorCode
    command: Optional[Callable[..., Any]] = Field(
        default=None, exclude=True
    )  # bleibt unsichtbar!


class IconMenuItem(BaseSchema):
    text: str
    enabled: bool
    icon_path: Path
    modifier: AcceleratorModifier
    key: AcceleratorCode
    command: Optional[Callable[..., Any]] = Field(
        default=None, exclude=True
    )  # bleibt unsichtbar!


class AboutMetadata(BaseSchema):
    name: Optional[str] = None
    version: Optional[str] = None
    short_version: Optional[str] = None
    authors: Optional[List[str]] = None
    comments: Optional[str] = None
    copyright: Optional[str] = None
    license: Optional[str] = None
    website: Optional[str] = None
    website_label: Optional[str] = None
    credits: Optional[str] = None
    icon: Optional[Path] = None


class PredefinedMenuItem(BaseSchema):
    item_type: str
    text: Optional[str] = None
    metadata: Optional[AboutMetadata] = None
    command: Optional[Callable[..., Any]] = Field(
        default=None, exclude=True
    )  # bleibt unsichtbar!


class Submenu(BaseSchema):
    text: str
    enabled: bool
    menu_items: Optional[List[MenuItem]] = None
    check_menu: Optional[List[CheckMenuItem]] = None
    icon_menu: Optional[List[IconMenuItem]] = None
    predefined_menu: Optional[List[PredefinedMenuItem]] = None
    command: Optional[Callable[..., Any]] = Field(
        default=None, exclude=True
    )  # bleibt unsichtbar!


class SystemTray(BaseSchema):
    """
    Represents the tray icon configuration, matching the Rust-side options.
    """

    icon: Optional[str] = Field(None, description="Path to the icon file (PNG).")
    title: Optional[str] = Field(
        None,
        description=(
            "Title for the tray icon (only for non-Windows operating systems)."
        ),
    )
    is_template: Optional[bool] = Field(
        None,
        description=(
            "Whether the icon should be treated as a template (only on macOS and non-Linux systems)."
        ),
    )
    temp_dir_path: Optional[str] = Field(
        None,
        description="Temporary directory for the icon (only for Linux).",
    )
    tooltip: Optional[str] = Field(
        None,
        description="Tooltip text for the tray icon (only for non-Linux systems).",
    )


class MenuFrame(BaseSchema):
    menu_items: Optional[List[MenuItem]] = None
    sub_menu: Optional[List[Submenu]] = None
    check_menu: Optional[List[CheckMenuItem]] = None
    icon_menu: Optional[List[IconMenuItem]] = None
    predefined_menu: Optional[List[PredefinedMenuItem]] = None
    system_tray: Optional[SystemTray] = None


class FrameShortcutOption(BaseModel):
    modifier: Optional[AcceleratorModifier] = (
        None  # Falls es ein Enum ist, kann man es als Enum definieren
    )
    key: AcceleratorCode
    accelerator_str: str
    id: int


class LinuxWindowConfig(BaseSchema):
    skip_taskbar: Optional[bool] = None
    transient_for: Optional[int] = None
    transparent_draw: Optional[bool] = None
    double_buffered: Optional[bool] = None
    rgba_visual: Optional[bool] = None
    app_paintable: Optional[bool] = None
    cursor_moved_event: Optional[bool] = None
    default_vbox: Optional[bool] = None
    extensions_path: Optional[Path] = None


class WindowsWindowConfig(BaseSchema):
    parent_window: Optional[int] = None
    owner_window: Optional[int] = None
    menu: Optional[int] = None
    taskbar_icon: Optional[str] = None
    no_redirection_bitmap: Optional[bool] = None
    drag_and_drop: Optional[bool] = None
    skip_taskbar: Optional[bool] = None
    window_classname: Optional[str] = None
    undecorated_shadow: Optional[bool] = None
    rtl: Optional[bool] = None
    additional_browser_args: Optional[str] = None
    browser_accelerator_keys: Optional[bool] = None
    default_context_menus: Optional[bool] = None
    theme: Optional[str] = None
    https_scheme: Optional[bool] = None
    scroll_bar_style: Optional[str] = None
    browser_extensions_enabled: Optional[bool] = None
    extensions_path: Optional[Path] = None


class MacOSWindowConfig(BaseSchema):
    activation_policy: Optional[ActivationPolicy] = None
    default_menu_creation: Optional[bool] = None
    activate_ignoring_other_apps: Optional[bool] = None
    parent_window: Optional[int] = None
    movable_by_window_background: Optional[bool] = None
    titlebar_transparent: Optional[bool] = None
    titlebar_hidden: Optional[bool] = None
    titlebar_buttons_hidden: Optional[bool] = None
    title_hidden: Optional[bool] = None
    fullsize_content_view: Optional[bool] = None
    resize_increments: Optional[Tuple[float, float]] = None
    disallow_hidpi: Optional[bool] = None
    has_shadow: Optional[bool] = None
    traffic_light_inset: Optional[Tuple[int, int]] = None
    automatic_window_tabbing: Optional[bool] = None
    tabbing_identifier: Optional[str] = None
    data_store_identifier: Optional[List[int]] = None
    allow_link_preview: Optional[bool] = None


class WindowConfig(BaseSchema):
    entry: Optional[str] = None
    window_inner_size: Optional[Tuple[float, float]] = None
    window_min_inner_size: Optional[Tuple[float, float]] = None
    window_max_inner_size: Optional[Tuple[float, float]] = None
    window_position: Optional[Tuple[float, float]] = None
    window_resizable: Optional[bool] = None
    window_minimizable: Optional[bool] = None
    window_maximizable: Optional[bool] = None
    window_closable: Optional[bool] = None
    window_title: Optional[str] = None
    window_fullscreen: Optional[bool] = None
    window_maximized: Optional[bool] = None
    window_visible: Optional[bool] = None
    window_transparent: Optional[bool] = None
    window_inner_size_constraints: Optional[Tuple[float, float]] = None
    window_decorations: Optional[bool] = None
    window_always_on_bottom: Optional[bool] = None
    window_always_on_top: Optional[bool] = None
    window_window_icon: Optional[str] = None
    window_theme: Optional[str] = None
    window_focused: Optional[bool] = None
    window_content_protection: Optional[bool] = None
    window_visible_on_all_workspaces: Optional[bool] = None
    window_background_color: Optional[Tuple[int, int, int, int]] = None
    webview_context_id: Optional[str] = None
    webview_id: Optional[str] = None
    webview_transparent: Optional[bool] = None
    webview_background_color: Optional[Tuple[int, int, int, int]] = None
    webview_visible: Optional[bool] = None
    webview_autoplay: Optional[bool] = None
    webview_initialization_scripts: Optional[List[str]] = None
    webview_initialization_main_only: Optional[List[Tuple[str, bool]]] = None
    webview_headers: Optional[Dict[str, str]] = None
    webview_user_agent: Optional[str] = None
    webview_devtools: Optional[bool] = None
    webview_hotkeys_zoom: Optional[bool] = None
    webview_clipboard: Optional[bool] = None
    webview_incognito: Optional[bool] = None
    webview_focused: Optional[bool] = None
    webview_bounds: Optional[Tuple[int, int, int, int]] = None
    webview_javascript_disabled: Optional[bool] = None
    webview_accept_first_mouse: Optional[bool] = None
    webview_back_forward_navigation_gestures: Optional[bool] = None
    webview_background_throttling: Optional[FrameBackgroundThrottlingPolicy] = None
    webview_proxy_config: Optional[dict] = None
    webview_initialization_script_for_main_only: Optional[Tuple[str, bool]] = None


class SocketSettings(BaseSchema):
    path: Optional[str] = None
    force_new: Optional[bool] = None
    multiplex: Optional[bool] = None
    add_trailing_slash: Optional[bool] = None
    auto_unref: Optional[bool] = None
    close_on_beforeunload: Optional[bool] = None
    extra_headers: Optional[Dict[str, str]] = None
    force_base64: Optional[bool] = None
    protocols: Optional[List[str]] = None
    query: Optional[dict] = None
    remember_upgrade: Optional[bool] = None
    timestamp_param: Optional[str] = None
    timestamp_requests: Optional[bool] = None
    transport_options: Optional[dict] = None
    transports: Optional[List[str]] = None
    try_all_transports: Optional[bool] = None
    upgrade: Optional[bool] = None
    with_credentials: Optional[bool] = None
    auto_connect: Optional[bool] = None
    parser: Optional[str] = None
    randomization_factor: Optional[float] = None
    reconnection: Optional[bool] = None
    reconnection_attempts: Optional[int] = None
    reconnection_delay: Optional[int] = None
    reconnection_delay_max: Optional[int] = None
    timeout: Optional[int] = None
    ack_timeout: Optional[int] = None
    auth: Optional[dict] = None
    retries: Optional[int] = None

    def model_post_init(self, __context):
        if self.transports is None:
            self.transports = ["websocket", "polling"]


class AppOptions(BaseSchema):
    name: str = Field(default_factory=current_folder_name)
    uuid: UUID = Field(default_factory=uuid4)
    host: Optional[str] = None
    port: Optional[int] = None
    icon: Optional[str] = None
    web_proto: Optional[str] = None
    internal_api: Optional[bool] = True
    debug_devtools: Optional[bool] = None
    menu_mode: Optional[Literal["menu", "tray", "menu_tray"]] = None
    debug_resource: Optional[str] = None
    debug_entry: Optional[str] = None
    socket_settings: SocketSettings = Field(default_factory=SocketSettings)
    window: WindowConfig = Field(default_factory=WindowConfig)
    workers: Optional[int] = None
    windows_extra: Optional[WindowsWindowConfig] = None
    linux_extra: Optional[LinuxWindowConfig] = None
    macos_extra: Optional[MacOSWindowConfig] = None
    tray: Optional[SystemTray] = None
    shortcuts: Optional[FrameShortcutOption] = None
    window_menu: Optional[MenuFrame] = None
