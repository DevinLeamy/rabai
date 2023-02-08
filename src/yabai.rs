use std::process;

use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum WindowTarget {
    Next,
    First,
    Last,
    Previous,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Left => "left".to_string(),
            Direction::Right => "right".to_string(),
        }
    }
}

impl ToString for WindowTarget {
    fn to_string(&self) -> String {
        match self {
            WindowTarget::Next => "next".to_string(),
            WindowTarget::First => "first".to_string(),
            WindowTarget::Last => "last".to_string(),
            WindowTarget::Previous => "prev".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum YabaiCommand {
    Focus(WindowTarget),
    Swap(WindowTarget),
    Resize(Direction, i32),
}

impl Into<Vec<String>> for YabaiCommand {
    fn into(self) -> Vec<String> {
        match self {
            YabaiCommand::Focus(target) => vec![
                "window".to_string(),
                "--focus".to_string(),
                target.to_string(),
            ],
            YabaiCommand::Swap(target) => vec![
                "window".to_string(),
                "--swap".to_string(),
                target.to_string(),
            ],
            YabaiCommand::Resize(direction, amount) => vec![
                "window".to_string(),
                "--resize".to_string(),
                format!("{}:{}:0", direction.to_string(), amount),
            ],
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
