//! 系统语言检测与界面文本国际化.
//!
//! 所有面向用户的界面文本 (CLI、TUI、GUI) 通过 [`detect_lang`] 检测系统语言后,
//! 由 [`tr`] 查找对应翻译. 中文 (含 zh-CN/zh-TW/zh-HK/zh-SG 等变体) 返回中文,
//! 其他语言返回英文.
//!
//! 设计原则:
//! - 零分配: 所有翻译均为 `&'static str`, 无运行时内存分配.
//! - [`ops`] 返回的日志行保持中文不变 (日志为技术信息, 面向调试而非普通用户) .

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lang {
    Zh,
    En,
}

/// 检测系统语言.
///
/// Windows: 调用 `GetUserDefaultUILanguage`, 中文 LCID 包括
/// 0x0804(zh-CN)、0x0404(zh-TW)、0x0c04(zh-HK)、0x1004(zh-SG) 等.
/// 非 Windows: 读取 `LANG` 环境变量, `zh*` 视为中文.
pub fn detect_lang() -> Lang {
    #[cfg(windows)]
    {
        use windows_sys::Win32::Globalization::GetUserDefaultUILanguage;
        let lang_id = unsafe { GetUserDefaultUILanguage() };
        // 简体/繁体中文及常见 zh 变体 LCID
        if lang_id == 0x0804 || lang_id == 0x0404 || lang_id == 0x0c04 || lang_id == 0x1004 {
            return Lang::Zh;
        }
        // 兜底: 检查系统 UI 语言是否为中文.
        // 某些 OEM/Region 变体可能上报其他 LCID 但仍是中文系统.
        // 通过 GEO 或 LANGID 主语言来判断.
        let primary = lang_id & 0x03FF;
        if primary == 0x0004 {
            return Lang::Zh;
        }
        Lang::En
    }
    #[cfg(not(windows))]
    {
        let lang = std::env::var("LANG").unwrap_or_default();
        if lang.to_lowercase().starts_with("zh") {
            Lang::Zh
        } else {
            Lang::En
        }
    }
}

/// 根据 key 和语言返回翻译字符串. 未匹配的 key 原样返回.
pub fn tr(key: &str, lang: Lang) -> &str {
    match lang {
        Lang::Zh => tr_zh(key),
        Lang::En => tr_en(key),
    }
}

macro_rules! tr_table {
    ($lang:ident, { $($key:expr => $val:expr),* $(,)? }) => {
        fn $lang(key: &str) -> &str {
            match key {
                $($key => $val,)*
                _ => key,
            }
        }
    };
}

tr_table!(tr_zh, {
    // ── 通用 ──
    "app.title" => "MiPCM Patch",
    "btn.apply" => "应用",
    "btn.revert" => "还原",
    "btn.refresh" => "刷新",
    "btn.cancel" => "取消",
    "btn.wifi" => "WiFi",
    "btn.lan" => "LAN",
    "btn.select" => "选择",
    "btn.start-install" => "安装",
    "btn.confirm" => "确认卸载",
    "btn.clear-log" => "清空日志",
    "btn.browse" => "选择…",
    "hint.custom" => "自定义",
    "hint.model.code" => "机型代号",
    "hint.custom.confirmed" => "自定义机型已设置",
    "hint.custom.tui" => "c—自定义",
    "hint.preset.tui" => "m—切换预设",
    "hint.uninstall.msix" => "检测并移除 MiDrop Ext MSIX 包, 完成后重启资源管理器",
    "hint.uninstall.product" => "完整卸载主程序、子产品、移除服务、清理临时文件 (不可逆! ) ",
    "hint.uninstall.warn" => "注意: 卸载操作不可逆! ",
    "hint.log.empty" => "操作日志将显示在这里…",
    "hint.install.tui" => "💡 安装请使用命令行: MiPCM_CLI install",

    // ── 面板/章节标题 ──
    "section.status" => "安装状态",
    "section.patches" => "补丁操作",
    "section.uninstall" => "卸载",
    "section.install" => "安装",
    "section.log" => "日志",
    "section.confirm.uninstall" => "确认卸载",
    "section.input.model" => "自定义机型 — {target_label}",
    "section.status.full" => "安装状态",

    // ── 补丁标签 ──
    "patch.locale" => "地区伪装",
    "patch.camera" => "摄像头弹窗",
    "patch.hotkey" => "鼠标快捷键",
    "patch.audio" => "音频流转",
    "patch.device" => "设备伪装",
    "patch.xiaoai" => "超级小爱",
    "patch.locale.detail" => "地区伪装 (micont_rtm.dll)",
    "patch.camera.detail" => "摄像头弹窗抑制 (PcControlCenter.dll)",
    "patch.hotkey.detail" => "鼠标快捷键（含两个侧键）",
    "patch.audio.detail" => "音频流转广播模式",
    "patch.device.detail" => "设备伪装 (msimg32.dll)",
    "patch.xiaoai.detail" => "超级小爱 (userenv.dll)",
    "patch.locale.desc" => "伪装地区为中国大陆，解锁区域功能",
    "patch.camera.desc" => "抑制摄像头状态确认弹窗",
    "patch.hotkey.desc" => "允许将中键、右键及两个鼠标侧键设为快捷键",
    "patch.audio.desc" => "切换广播模式 / 修复双网卡音频断流",
    "patch.device.desc" => "释放 msimg32.dll 代理 + 写入机型注册表",
    "patch.xiaoai.desc" => "部署安装与运行所需的专用 userenv.dll；完成后需重启电脑",

    // ── CLI 命令 ──
    "cli.about" => "小米电脑管家功能增强补丁工具",
    "cli.cmd.status" => "查看安装信息与补丁状态",
    "cli.cmd.locale" => "地区伪装 (micont_rtm.dll) ",
    "cli.cmd.camera" => "抑制「请确认摄像头状态」弹窗 (PcControlCenter.dll) ",
    "cli.cmd.audio" => "MiPCAudio 音频流转广播模式 (无线/有线, 统一身份并修复双网卡媒体路由) ",
    "cli.cmd.device" => "设备伪装 (释放 msimg32.dll + 写入机型注册表) ",
    "cli.cmd.install" => "安装小米电脑管家 / 小米互联 (自动识别安装包所属产品) ",
    "cli.cmd.uninstall" => "卸载 MiDrop Ext MSIX 包 或 小米电脑管家 / 小米互联",
    "cli.cmd.apply" => "应用补丁",
    "cli.cmd.revert" => "还原补丁",
    "cli.cmd.audio.apply" => "切换广播模式",
    "cli.cmd.audio.revert" => "还原音频流转补丁",
    "cli.cmd.device.apply" => "应用设备伪装",
    "cli.cmd.device.revert" => "还原设备伪装",
    "cli.cmd.msix" => "卸载 MiDrop Ext MSIX 包 (完成后重启资源管理器) ",
    "cli.cmd.product" => "完整卸载小米电脑管家 / 小米互联 (主程序 + 子产品 + 服务 + 文件清理, 不可逆) ",

    // ── CLI 参数帮助 ──
    "cli.arg.dll" => "指定目标 DLL 路径 (默认自动探测安装目录) ",
    "cli.arg.no_kill" => "不自动关闭相关进程",
    "cli.arg.region" => "伪装地区代码 (默认 CN) ",
    "cli.arg.no_registry" => "不修改注册表",
    "cli.arg.mode" => "广播介质: wifi (无线, 默认) 或 lan  (有线) ",
    "cli.arg.dir" => "指定版本目录 (默认自动探测) ",
    "cli.arg.no_wifi_route" => "不自动管理 Wi-Fi 本地子网优先路由 (无线模式下用于修复双网卡同网段断流) ",
    "cli.arg.model" => "机型代号 (默认: 机型表首项) ",
    "cli.arg.installer" => "显式指定 .exe 安装包",
    "cli.arg.url" => "从 HTTP(S) 地址下载安装包",

    // ── CLI 运行时消息 ──
    "cli.error" => "错误: {error}",
    "cli.confirm.uninstall" => "确认卸载? 输入 y 继续: ",
    "cli.cancelled.uninstall" => "已取消卸载. ",
    "cli.cancelled.install" => "已取消安装. ",
    "cli.downloading" => "正在下载安装包到 {path}",
    "cli.found.installer" => "已找到{kind}安装包: {path}",
    "cli.found.xiaoai.installer" => "已找到超级小爱安装包: {path}",
    "cli.multiple.installers" => "找到多个安装包: ",
    "cli.choose.installer" => "请选择安装包: ",
    "cli.invalid.choice" => "无效的安装包选择",
    "cli.no.installer.found" => "未在 Patcher 同目录找到安装包 (小米电脑管家 `*_XiaomiPCManager_*.exe` 或小米互联 `小米互联*.exe`) . ",
    "cli.no.xiaoai.installer.found" => "未在 Patcher 同目录找到 XiaoaiAgent_Setup.exe 或 s6bK_XiaoaiAgent_3.5.0.220_31444585.exe。",
    "cli.cancel.option" => "取消",
    "cli.choose.option" => "请选择: ",
    "cli.menu.download_url" => "输入下载网址",
    "cli.menu.local_file" => "指定本地 .exe 安装包",
    "cli.prompt.url" => "请输入 HTTP(S) 下载地址: ",
    "cli.prompt.path" => "请输入 .exe 安装包路径: ",
    "cli.invalid.selection" => "无效选择",

    // ── 安装包分类 ──
    "install.kind.manager" => "小米电脑管家 (XiaomiPCManager)",
    "install.kind.continuity" => "小米互联 / 互联互通 (HyperConnect / PcContinuity)",

    // ── 安装 GUI ──
    "install.title" => "安装小米电脑管家 / 小米互联",
    "install.xiaoai.title" => "安装超级小爱并注入补丁",
    "install.xiaoai.note" => "安装窗口关闭后自动部署 userenv.dll；完成后请重启电脑",
    "install.download.placeholder" => "HTTP(S) 下载地址…",
    "install.file.placeholder" => "选择本地 .exe 安装包…",
    "install.download.and.install" => "下载并安装",

    // ── GUI 下载 ──
    "gui.downloading.start" => "开始下载: {url}",

    // ── GUI 安装操作标签 ──
    "gui.op.locale.apply" => "地区伪装",
    "gui.op.locale.revert" => "地区伪装·还原",
    "gui.op.device.apply" => "设备伪装({model})",
    "gui.op.device.revert" => "设备伪装·还原",
    "gui.op.camera.apply" => "摄像头",
    "gui.op.camera.revert" => "摄像头·还原",
    "gui.op.hotkey.apply" => "鼠标快捷键",
    "gui.op.hotkey.revert" => "鼠标快捷键·还原",
    "gui.op.audio.wifi" => "音频·WiFi",
    "gui.op.audio.lan" => "音频·LAN",
    "gui.op.audio.revert" => "音频·还原",
    "gui.op.uninstall.msix" => "卸载MSIX",
    "gui.op.uninstall.product" => "卸载产品",
    "gui.op.install" => "安装({path})",
    "gui.op.xiaoai.apply" => "超级小爱·应用",
    "gui.op.xiaoai.revert" => "超级小爱·还原",
    "gui.op.xiaoai.install" => "安装超级小爱({path})",

    // ── GUI 警告 ──
    "gui.warn.no_full" => "未检测到完整版 XiaomiPCManager\n摄像头/音频/设备伪装不可用. ",
    "gui.warn.recent_error" => "最近错误: ",

    // ── GUI 运行日志面板 ──
    "gui.log.title" => "运行日志",

    // ── GUI 卸载对话框 ──
    "gui.uninstall.msix" => "卸载 MiDrop Ext (MSIX)",
    "gui.uninstall.product" => "卸载小米电脑管家 / 小米互联",

    // ── GUI 文件选择对话框 ──
    "gui.browse.title" => "选择安装包",
    "gui.browse.filter" => "安装包",

    // ── TUI ──
    "tui.tab.patches" => "补丁",
    "tui.tab.install" => "安装",
    "tui.tab.uninstall" => "卸载",
    "tui.tab.log" => "日志",
    "tui.header.ready" => "就绪",
    "tui.header.limited" => "受限",
    "tui.overlay.confirm" => "确认操作",
    "tui.overlay.input.title" => "输入机型 — {target}",
    "tui.overlay.confirm.yn" => "按 Y 确认 / N 取消",
    "tui.overlay.input.hint" => "Enter 确认 / Esc 取消",
    "tui.shortcuts.global" => "退出",
    "tui.shortcuts.tab" => "切换面板",
    "tui.shortcuts.patches" => "Tab 切换面板  ↑↓ 选择  ←→ 按钮  Enter 执行  M 切换机型  C 自定义  R 刷新  Q 退出",
    "tui.shortcuts.install" => "Tab 切换面板  Enter 安装同目录超级小爱  R 刷新  Q 退出",
    "tui.shortcuts.uninstall" => "Tab 切换面板  ↑↓ 选择  Enter 确认  R 刷新  Q 退出",
    "tui.shortcuts.log" => "Tab 切换面板  ↑↓/PgUp/PgDn/Home/End 滚动  R 刷新  Q 退出",
    "tui.shortcuts.input" => "Enter 确认  Esc 取消  Backspace 退格",
    "tui.shortcuts.confirm" => "Y 确认  N/Esc 取消",
    "tui.shortcuts.scroll" => "滚动",
    "tui.shortcuts.model.hint" => "m—切换预设  c—自定义",
    "tui.shortcuts.refresh" => "R — 刷新状态",

    // ── TUI 标签 ──
    "tui.model.custom" => "(自定义)",
    "tui.model.preset" => "机型",
    "tui.log.start" => "▶ 开始: {label}",
    "tui.log.done" => "✓ 完成: {label}",
    "tui.log.failed" => "{label} 失败: {error}",
    "tui.log.cancelled" => "已取消操作",
    "tui.log.error" => "❌ 错误: {error}",

    // ── TUI 安装面板 ──
    "tui.install.title" => "支持从 URL 下载 或 指定本地安装包：",
    "tui.install.desc" => "工具自动识别安装包类型，安装到正确位置并处理残留清理",
    "tui.install.hint.focused" => "按 Enter — 查看安装指引 (请在命令行中使用 install 子命令)",
    "tui.install.hint.unfocused" => "切换到本面板后按 Enter 查看安装指引",
    "tui.install.hint.cmd1" => "  MiPCM_CLI install --url https://...",
    "tui.install.hint.cmd2" => "  MiPCM_CLI install --installer XiaomiPCManager.exe",
    "tui.install.hint.xiaoai1" => "  MiPCM_CLI xiaoai install --url https://...",
    "tui.install.hint.xiaoai2" => "  MiPCM_CLI xiaoai install --installer XiaoaiAgent_Setup.exe",
    "tui.install.xiaoai.enter" => "按 Enter：安装 Patcher 同目录中唯一的超级小爱安装包并注入补丁",

    // ── TUI 卸载提示 ──
    "tui.uninstall.msix.desc" => "卸载 MiDrop Ext MSIX 包（完成后重启资源管理器）",
    "tui.uninstall.product.desc" => "完整卸载小米电脑管家 / 小米互联（不可逆，需二次确认）",
    "tui.uninstall.key.hint" => "  Enter — 执行  ",

    // ── TUI 日志面板 ──
    "tui.log.empty" => "尚无日志 — 执行操作后此处显示结果",
        "tui.log.scroll.hint" => "% — ↑↓/PgUp/PgDn/Home/End 滚动",
    "tui.log.count" => "{current}/{total} 条",

    // ── TUI 输入对话框 ──
    "tui.input.device" => "设备伪装",
    "tui.input.title" => " 自定义机型 — {target_label} ",

    // ── TUI 状态 ──
    "tui.status.loading" => " (正在加载…) ",
    "tui.confirm.title" => " ⚠ 确认卸载 ",

    // ── TUI 机型日志 ──
    "tui.log.model.set" => "已设置机型: {model}",
    "tui.log.model.switch" => "📱 设备伪装机型切换为: {code} [{name}]",
    "tui.log.uninstall.fetch.error" => "✗ 获取卸载信息失败: {error}",

    // ── TUI 操作标签 ──
    "tui.op.locale.apply" => "地区伪装 · 应用",
    "tui.op.locale.revert" => "地区伪装 · 还原",
    "tui.op.camera.apply" => "摄像头弹窗抑制 · 应用",
    "tui.op.camera.revert" => "摄像头弹窗抑制 · 还原",
    "tui.op.hotkey.apply" => "鼠标快捷键 · 应用",
    "tui.op.hotkey.revert" => "鼠标快捷键 · 还原",
    "tui.op.audio.wifi" => "音频流转 · WiFi",
    "tui.op.audio.lan" => "音频流转 · LAN",
    "tui.op.audio.revert" => "音频流转 · 还原",
    "tui.op.device.apply" => "设备伪装 · 应用({model})",
    "tui.op.device.revert" => "设备伪装 · 还原",
    "tui.op.xiaoai.apply" => "超级小爱 · 应用",
    "tui.op.xiaoai.revert" => "超级小爱 · 还原",
    "tui.op.xiaoai.install" => "超级小爱 · 安装并注入",
    "tui.op.uninstall.msix" => "卸载 MiDrop Ext (MSIX)",
    "tui.op.uninstall.product" => "卸载小米电脑管家 / 小米互联",

    // ── 卸载模块 ──
    "uninstall.start.manager" => "开始卸载小米电脑管家: {root}",
    "uninstall.start.continuity" => "开始卸载小米互联 / PcContinuity: {root}",
    "uninstall.running.uninstaller" => "  正在运行卸载程序: {exe}",
    "uninstall.remover.not.removed" => "  ⚠ 卸载程序未删除自身, 卸载可能未完成: {exe}",
    "uninstall.main.done" => "  ✓ 主程序卸载完成",
    "uninstall.removing.services" => "  正在移除服务…",
    "uninstall.service.removed" => "    ✓ 已删除服务: {name}",
    "uninstall.service.failed" => "    ⚠ 删除服务 {name} 失败: {error}",
    "uninstall.cleaning.files" => "  正在清理文件…",
    "uninstall.done.manager" => "✓ 小米电脑管家卸载完成",
    "uninstall.done.continuity" => "✓ 小米互联 / PcContinuity 卸载完成",
    "uninstall.sub.not.installed" => "  - {label} 未安装, 跳过",
    "uninstall.sub.uninstalling" => "  正在卸载 {label}: {exe}",
    "uninstall.sub.done" => "    ✓ {label} 卸载完成",
    "uninstall.sub.not.removed" => "    ⚠ {label} 卸载程序未删除自身, 卸载可能未完成",
    "uninstall.sub.failed" => "    ⚠ 卸载 {label} 失败: {error}",
    "uninstall.sub.no.uninstaller" => "    ⚠ {label} 目录存在但未找到 uninstall.exe",
    "uninstall.sub.no.version" => "    ⚠ {label} 目录存在但无版本子目录",
    "uninstall.clean.programdata" => "    ✓ 已删除 C:\\ProgramData\\MI",
    "uninstall.clean.programdata.failed" => "    ⚠ 清理 C:\\ProgramData\\MI 失败: {error}",
    "uninstall.clean.dir" => "    ✓ 已删除 {path}",
    "uninstall.clean.dir.failed" => "    ⚠ 清理 {path} 失败: {error}",
    "uninstall.desc.manager" => "将卸载 小米电脑管家\n\n安装目录: {root}\n包含: 主程序, AIService, MiService\n将删除所有相关服务与临时文件\n\n此操作不可逆! ",
    "uninstall.desc.continuity" => "将卸载 小米互联 / PcContinuity\n\n安装目录: {root}\n将删除所有相关服务与临时文件\n\n此操作不可逆! ",
    "uninstall.desc.coexistence" => "同时检测到小米电脑管家和小米互联. \n当前不支持同时安装, 请逐一卸载. ",
    "uninstall.desc.none" => "未检测到已安装的小米电脑管家或小米互联",
    "uninstall.no.msix" => "未检测到 MiDrop Ext MSIX 包",
    "uninstall.msix.done" => "✓ 已卸载 MiDrop Ext MSIX 包: {pkg}",
    "uninstall.explorer.restarted" => "✓ 已重启资源管理器",

    // ── Ops 模块 ──
    "ops.restart.hint" => "提示: 补丁已完成, 请手动重新启动小米电脑管家使其生效. ",
});

tr_table!(tr_en, {
    // ── 通用 ──
    "app.title" => "MiPCM Patch",
    "btn.apply" => "Zastosuj",
    "btn.revert" => "Przywróć",
    "btn.refresh" => "Odśwież",
    "btn.cancel" => "Anuluj",
    "btn.wifi" => "WiFi",
    "btn.lan" => "LAN",
    "btn.select" => "Wybierz",
    "btn.start-install" => "Instaluj",
    "btn.confirm" => "Potwierdź odinstalowanie",
    "btn.clear-log" => "Wyczyść logi",
    "btn.browse" => "Przeglądaj…",
    "hint.custom" => "Własny",
    "hint.model.code" => "Kod modelu",
    "hint.custom.confirmed" => "Ustawiono własny model",
    "hint.custom.tui" => "c—Własny",
    "hint.preset.tui" => "m—Zmień preset",
    "hint.uninstall.msix" => "Wykryj i usuń pakiet MSIX MiDrop Ext, a następnie uruchom ponownie Eksplorator",
    "hint.uninstall.product" => "Pełne odinstalowanie: program główny, podprodukty, usługi, pliki tymczasowe (nieodwracalne!)",
    "hint.uninstall.warn" => "Uwaga: odinstalowanie jest nieodwracalne!",
    "hint.log.empty" => "Tutaj pojawią się logi operacji…",
    "hint.install.tui" => "💡 Instalacja z wiersza poleceń: MiPCM_CLI install",

    // ── 面板/章节标题 ──
    "section.status" => "Status instalacji",
    "section.patches" => "Operacje łatek",
    "section.uninstall" => "Odinstaluj",
    "section.install" => "Instaluj",
    "section.log" => "Logi",
    "section.confirm.uninstall" => "Potwierdź odinstalowanie",
    "section.input.model" => " Własny model — {target_label} ",
    "section.status.full" => "Status instalacji",

    // ── 补丁标签 ──
    "patch.locale" => "Maskowanie regionu",
    "patch.camera" => "Okno kamery",
    "patch.hotkey" => "Skróty myszy",
    "patch.audio" => "Strumień audio",
    "patch.device" => "Maskowanie urządzenia",
    "patch.xiaoai" => "Super XiaoAI",
    "patch.locale.detail" => "Maskowanie regionu (micont_rtm.dll)",
    "patch.camera.detail" => "Blokada okna stanu kamery (PcControlCenter.dll)",
    "patch.hotkey.detail" => "Skróty myszy (w tym dwa przyciski boczne)",
    "patch.audio.detail" => "Tryb rozgłaszania strumienia audio",
    "patch.device.detail" => "Maskowanie urządzenia (msimg32.dll)",
    "patch.xiaoai.detail" => "Super XiaoAI (userenv.dll)",
    "patch.locale.desc" => "Maskuj region jako Chiny kontynentalne, odblokuj funkcje regionalne",
    "patch.camera.desc" => "Ukryj wyskakujące okno potwierdzenia stanu kamery",
    "patch.hotkey.desc" => "Używaj środkowego, prawego oraz obu bocznych przycisków myszy jako skrótów",
    "patch.audio.desc" => "Przełącz tryb rozgłaszania / napraw przerywanie dźwięku przy dwóch kartach sieciowych",
    "patch.device.desc" => "Wdróż proxy msimg32.dll + zapisz model w rejestrze",
    "patch.xiaoai.desc" => "Wdróż dedykowany plik userenv.dll wymagany do instalacji i działania; uruchom ponownie komputer po zakończeniu",

    // ── CLI 命令 ──
    "cli.about" => "Narzędzie łatek rozszerzających funkcje MiPCManager",
    "cli.cmd.status" => "Pokaż informacje o instalacji i status łatek",
    "cli.cmd.locale" => "Maskowanie regionu (micont_rtm.dll)",
    "cli.cmd.camera" => "Ukryj okno stanu kamery (PcControlCenter.dll)",
    "cli.cmd.audio" => "Tryb rozgłaszania MiPCAudio (Wi-Fi/LAN, ujednolicona tożsamość, poprawka dla dwóch kart sieciowych)",
    "cli.cmd.device" => "Maskowanie urządzenia (wdrożenie msimg32.dll + wpis modelu w rejestrze)",
    "cli.cmd.install" => "Zainstaluj MiPCManager / MiContinuity (automatyczne wykrywanie produktu)",
    "cli.cmd.uninstall" => "Odinstaluj pakiet MSIX MiDrop Ext lub MiPCManager / MiContinuity",
    "cli.cmd.apply" => "Zastosuj łatkę",
    "cli.cmd.revert" => "Przywróć oryginał",
    "cli.cmd.audio.apply" => "Przełącz tryb rozgłaszania",
    "cli.cmd.audio.revert" => "Przywróć łatkę strumienia audio",
    "cli.cmd.device.apply" => "Zastosuj maskowanie urządzenia",
    "cli.cmd.device.revert" => "Przywróć maskowanie urządzenia",
    "cli.cmd.msix" => "Odinstaluj pakiet MSIX MiDrop Ext (uruchomi ponownie Eksplorator)",
    "cli.cmd.product" => "Pełne odinstalowanie MiPCManager / MiContinuity (program główny + podprodukty + usługi + czyszczenie, nieodwracalne)",
    
