use std::process;

use anyhow::Result;
use YabaiCommand::*;

#[derive(Debug, Clone, Copy)]
pub enum WindowTarget {
    Next,
    First,
    Last,
    Previous,
    Id(u32),
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

impl Into<String> for Direction {
    fn into(self) -> String {
        match self {
            Direction::Left => "left".into(),
            Direction::Right => "right".into(),
        }
    }
}

impl Into<String> for WindowTarget {
    fn into(self) -> String {
        match self {
            WindowTarget::Next => "next".into(),
            WindowTarget::First => "first".into(),
            WindowTarget::Last => "last".into(),
            WindowTarget::Previous => "prev".into(),
            WindowTarget::Id(id) => id.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum YabaiCommand {
    Focus(WindowTarget),
    FocusSpace(u32),
    Swap(WindowTarget),
    Resize(Direction, i32),
    ToggleFullscreen(WindowTarget),
}

impl YabaiCommand {
    fn into(self) -> Vec<String> {
        match self {
            Focus(target) => vec!["window".into(), "--focus".into(), target.into()],
            FocusSpace(space_id) => {
                vec!["space".into(), "--focus".into(), u32::to_string(&space_id)]
            }
            Swap(target) => vec!["window".into(), "--swap".into(), target.into()],
            Resize(direction, amount) => vec![
                "window".into(),
                "--resize".into(),
                format!("{}:{}:0", Into::<String>::into(direction), amount),
            ],
            ToggleFullscreen(target) => {
                vec![
                    "window".into(),
                    target.into(),
                    "--toggle".into(),
                    "zoom-fullscreen".into(),
                ]
            }
        }
    }
}

impl YabaiCommand {
    /// TODO: Might have to make this generic over T.
    pub fn run(self) -> Result<()> {
        let args: Vec<String> = self.into();

        println!("{:?}", args);

        let output = process::Command::new("yabai")
            .arg("-m")
            .args(args.as_slice())
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Yabai command failed"));
        }
        Ok(())
    }
}
