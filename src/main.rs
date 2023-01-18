use anyhow::Result;
use serde::Deserialize;
use shellfn::shell;
use std::error::Error;
/**
 * What if we wrote a derive macro that would take in a shell function and then
 * send the output result an argument to the function that called it. For instance,
 *
 * #[call(ls)]
 * fn ls(result: Result<Vec<ListItem>>) -> () {
 *  // Do something with the data
 * }
 *
 * THIS WOULD BE SO NICE TO WORK WITH!
 */

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
struct YabaiFrame {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
struct YabaiWindow {
    id: u32,
    pid: u32,
    app: String,
    title: String,
    frame: YabaiFrame,
    role: String,
    subrole: String,
    tags: String,
    display: u32,
    space: u32,
    level: u32,
    opacity: f32,
    #[serde(alias = "split-type")]
    split_type: String,
    #[serde(alias = "stack-index")]
    stack_index: u32,
    #[serde(alias = "can-move")]
    can_move: bool,
    #[serde(alias = "can-resize")]
    can_resize: bool,
    #[serde(alias = "has-focus")]
    has_focus: bool,
    #[serde(alias = "has-shadow")]
    has_shadow: bool,
    #[serde(alias = "has-border")]
    has_border: bool,
    #[serde(alias = "has-parent-zoom")]
    has_parent_zoom: bool,
    #[serde(alias = "has-fullscreen-zoom")]
    has_fullscreen_zoom: bool,
    #[serde(alias = "is-native-fullscreen")]
    is_native_fullscreen: bool,
    #[serde(alias = "is-visible")]
    is_visible: bool,
    #[serde(alias = "is-minimized")]
    is_minimized: bool,
    #[serde(alias = "is-hidden")]
    is_hidden: bool,
    #[serde(alias = "is-floating")]
    is_floating: bool,
    #[serde(alias = "is-sticky")]
    is_sticky: bool,
    #[serde(alias = "is-topmost")]
    is_topmost: bool,
    #[serde(alias = "is-grabbed")]
    is_grabbed: bool,
}

#[derive(Debug)]
struct YabaiWindows {
    windows: Vec<YabaiWindow>,
}

impl YabaiWindows {
    pub fn init() -> anyhow::Result<Self> {
        let raw_windows_json = raw_window_data()?;
        let windows: Vec<YabaiWindow> = serde_json::from_str(&raw_windows_json)?;

        Ok(Self { windows })
    }
}

#[derive(Debug)]
struct YabaiContext {
    windows: YabaiWindows,
}

impl YabaiContext {
    pub fn new(windows: YabaiWindows) -> Self {
        Self { windows }
    }
}

#[shell]
fn raw_window_data() -> Result<String> {
    r#"
    yabai -m query --windows
"#
}

fn main() {
    let context = YabaiContext::new(YabaiWindows::init().unwrap());
}

/// Returns the focused window
fn focused_window(windows: &Vec<YabaiWindow>) -> Option<YabaiWindow> {
    for window in windows {
        if window.has_focus {
            return Some(window.clone());
        }
    }

    None
}

/// Shrink left window
fn yabai_resize_left() -> () {
    todo!()
}
/// Shrink right window
fn yabai_resize_right() -> () {
    todo!()
}
/// Swap two windows
fn yabai_swap() -> () {
    todo!()
}
/// Focus on the next window (cycles)
fn yabai_focus_next() {
    todo!()
}
/// Make all of the windows on a display fullscreen or non-fullscreen
fn yabai_toggle_fullscreen() {
    todo!()
}
