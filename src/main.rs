use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use shellfn::shell;

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
 *
 *
 * Yabai problem (bug?)
 * - Sometimes windows end up in a 50:50 horizontal split.
 * - I only want VERTICAL splits. Why does this happen???
 */
mod args;
use args::{Args, Command};
mod yabai;
use yabai::{WindowTarget, YabaiCommand};

fn main() {
    let context = YabaiContext::new(YabaiWindows::init().unwrap());
    let command = Args::parse().command;

    for window in context.windows.windows {
        println!("{:?}", window.id);
    }

    match command {
        Command::Next => yabai_focus_next(),
        Command::Previous => yabai_focus_previous(),
        Command::Swap => yabai_swap(),
        _ => {}
    };

    println!("{:?}", command);
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
// Note: When yabai is started, the contents of "~/.yabairc" are run
//
// This config is NOT a replacement for ".yabairc", but rather something
// more like Amethyst's configuration options. I.e. configuration for the
// current layout, size to shift, perhaps the specific keybindings for
// given operations, etc...
//
// At the very least, it will contain all of the imformation I need to implement
// my little hacky scripts, in a better way.
struct YabaiConfig {
    /// The amount that a window is shifted when resized to the left
    /// or right
    resize_shift: u32,
}

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
fn yabai_resize_left() -> () {}
/// Shrink right window
fn yabai_resize_right() -> () {
    todo!()
}
/// Swap two windows
/// Swaps with the next window, or the first if the current
/// window is the last window
fn yabai_swap() -> () {
    if YabaiCommand::Swap(WindowTarget::Next).run().is_err() {
        // Swap with the first window
        YabaiCommand::Swap(WindowTarget::First).run().unwrap();
    }
}

/// Focus on the next window (cycles)
fn yabai_focus_next() {
    if YabaiCommand::Focus(WindowTarget::Next).run().is_err() {
        // Cycle to the first window
        YabaiCommand::Focus(WindowTarget::First).run().unwrap();
    }
}

/// Focus on the next window (cycles)
fn yabai_focus_previous() {
    if YabaiCommand::Focus(WindowTarget::Previous).run().is_err() {
        // Cycle to the first window
        YabaiCommand::Focus(WindowTarget::Last).run().unwrap();
    }
}
/// Make all of the windows on a display fullscreen or non-fullscreen
fn yabai_toggle_fullscreen() {
    todo!()
}
