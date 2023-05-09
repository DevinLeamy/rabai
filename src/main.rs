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
use yabai::{Direction, WindowTarget, YabaiCommand};

fn main() -> Result<()> {
    let windows: Vec<YabaiWindow> = serde_json::from_str(&raw_window_data()?)?;
    let displays: Vec<YabaiDisplay> = serde_json::from_str(&raw_display_data()?)?;
    let spaces: Vec<YabaiSpace> = serde_json::from_str(&raw_space_data()?)?;

    let context = YabaiContext::new(displays, spaces, windows, YabaiConfig { resize_shift: 80 });
    if !context.has_focused_window() {
        return Ok(());
    }
    let command = Args::parse().command;

    match command {
        Command::Next => context.focus_next(),
        Command::Previous => context.focus_previous(),
        Command::NextSpace => context.focus_space_next()?,
        Command::PreviousSpace => context.focus_space_previous()?,
        Command::Swap => context.swap(),
        Command::Resize { direction } => {
            if &direction == "left" {
                context.resize_left().unwrap();
            } else if &direction == "right" {
                context.resize_right().unwrap();
            } else {
                println!("Error: Invalid argument to resize");
            }
        }
        Command::ToggleFullscreen => context.toggle_fullscreen(),
        _ => {}
    };

    Ok(())
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
    /// or right.
    resize_shift: i32,
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
    tags: Option<String>,
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

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
struct YabaiDisplay {
    id: u32,
    uuid: String,
    index: u32,
    frame: YabaiFrame,
    spaces: Vec<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
struct YabaiSpace {
    id: u32,
    uuid: String,
    index: u32,
    label: String,
    #[serde(alias = "type")]
    display_type: String,
    display: u32,
    windows: Vec<u32>,
    #[serde(alias = "first-window")]
    first_window: u32,
    #[serde(alias = "last-window")]
    last_window: u32,
    #[serde(alias = "has-focus")]
    is_focused: bool,
    #[serde(alias = "is-visible")]
    is_visible: bool,
    #[serde(alias = "is-native-fullscreen")]
    is_fullscreen: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Location {
    Next,
    Previous,
    First,
    Last,
}

#[derive(Debug)]
struct YabaiContext {
    displays: Vec<YabaiDisplay>,
    spaces: Vec<YabaiSpace>,
    windows: Vec<YabaiWindow>,
    config: YabaiConfig,
}

impl YabaiContext {
    pub fn new(
        displays: Vec<YabaiDisplay>,
        spaces: Vec<YabaiSpace>,
        windows: Vec<YabaiWindow>,
        config: YabaiConfig,
    ) -> Self {
        Self {
            displays,
            spaces,
            windows,
            config,
        }
    }
    /// Returns the focused window.
    fn focused_window(&self) -> YabaiWindow {
        for window in &self.windows {
            if window.has_focus {
                return window.clone();
            }
        }
        panic!("No focused window found");
    }

    fn focused_display(&self) -> YabaiDisplay {
        let focused_window = self.focused_window();
        println!("Focused window: {:?}", focused_window);
        for display in &self.displays {
            if display.index == focused_window.display {
                return display.clone();
            }
        }
        panic!("No focused display found");
    }

    // The space on the focused display that is currently visible.
    fn active_space(&self) -> YabaiSpace {
        let focused_window = self.focused_window();
        for space in &self.spaces {
            if space.is_visible && space.display == focused_window.display {
                return space.clone();
            }
        }
        panic!("No focused space found");
    }

    fn space_on_display(&self, location: Location) -> Option<u32> {
        let focused_display = self.focused_display();
        let focused_space = self.active_space();
        if matches!(location, Location::First) || matches!(location, Location::Last) {
            if matches!(location, Location::First) {
                return focused_display.spaces.first().copied();
            } else {
                return focused_display.spaces.last().copied();
            }
        } else {
            let current_index = focused_space.index;
            if matches!(location, Location::Next)
                && current_index < *focused_display.spaces.last()?
            {
                return Some(current_index + 1);
            } else if matches!(location, Location::Previous)
                && current_index > *focused_display.spaces.first()?
            {
                return Some(current_index - 1);
            }
        }
        None
    }

    fn has_focused_window(&self) -> bool {
        for window in &self.windows {
            if window.has_focus {
                return true;
            }
        }
        false
    }

    /// Shrink left window
    fn resize_left(&self) -> Result<()> {
        if YabaiCommand::Resize(Direction::Left, -self.config.resize_shift)
            .run()
            .is_err()
        {
            YabaiCommand::Resize(Direction::Right, -self.config.resize_shift).run()?;
        }
        Ok(())
    }

    /// Shrink right window
    fn resize_right(&self) -> Result<()> {
        if YabaiCommand::Resize(Direction::Right, self.config.resize_shift)
            .run()
            .is_err()
        {
            YabaiCommand::Resize(Direction::Left, self.config.resize_shift).run()?;
        }
        Ok(())
    }

    /// Swap two windows
    /// Swaps with the next window, or the first if the current
    /// window is the last window
    fn swap(&self) {
        if YabaiCommand::Swap(WindowTarget::Next).run().is_err() {
            // Swap with the first window
            YabaiCommand::Swap(WindowTarget::First).run().unwrap();
        }
    }

    /// Focus on the next window (cycles)
    fn focus_next(&self) {
        if YabaiCommand::Focus(WindowTarget::Next).run().is_err() {
            // Cycle to the first window
            YabaiCommand::Focus(WindowTarget::First).run().unwrap();
        }
    }

    /// Focus on the next window (cycles)
    fn focus_previous(&self) {
        if YabaiCommand::Focus(WindowTarget::Previous).run().is_err() {
            // Cycle to the first window
            YabaiCommand::Focus(WindowTarget::Last).run().unwrap();
        }
    }
    /// Make all of the windows on a display fullscreen or non-fullscreen
    fn toggle_fullscreen(&self) {
        let focused_window = self.focused_window();
        let focused_space = focused_window.space;
        let new_fullscreen_setting = !focused_window.has_fullscreen_zoom;

        for window in self.windows.iter() {
            if window.space != focused_space {
                continue;
            }
            if window.has_fullscreen_zoom != new_fullscreen_setting {
                YabaiCommand::ToggleFullscreen(WindowTarget::Id(window.id))
                    .run()
                    .unwrap();
            }
        }
    }

    fn focus_space_next(&self) -> Result<()> {
        if let Some(space) = self.space_on_display(Location::Next) {
            YabaiCommand::FocusSpace(space).run()?;
        } else if let Some(space) = self.space_on_display(Location::First) {
            YabaiCommand::FocusSpace(space).run()?;
        }
        Ok(())
    }

    fn focus_space_previous(&self) -> Result<()> {
        if let Some(space) = self.space_on_display(Location::Previous) {
            YabaiCommand::FocusSpace(space).run()?;
        } else if let Some(space) = self.space_on_display(Location::Last) {
            YabaiCommand::FocusSpace(space).run()?;
        }
        Ok(())
    }
}

#[shell]
fn raw_window_data() -> Result<String> {
    r#"
    yabai -m query --windows
"#
}

#[shell]
fn raw_display_data() -> Result<String> {
    r#"
    yabai -m query --displays
"#
}

#[shell]
fn raw_space_data() -> Result<String> {
    r#"
    yabai -m query --spaces
"#
}
