use std::process;

use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum WindowTarget {
    Next,
    First,
    Last,
    Previous,
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
