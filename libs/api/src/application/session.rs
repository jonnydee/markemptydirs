use self::Execution::*;
use application::ApplicationInfo;
use commands::{Command, Config, Context, Execution, Result};

#[derive(Debug)]
pub struct Session {
    pub command: Box<Command>,
    pub context: Context,
}

impl Session {
    pub fn new(appinfo: ApplicationInfo, cfg: Config, exec: Execution) -> Session {
        let (cmd, dry_run) = match exec {
            DryRun(cmd) => (cmd, true),
            Run(cmd) => (cmd, false),
        };

        let ctx = Context::new(appinfo, cfg, dry_run);

        Session {
            command: cmd,
            context: ctx,
        }
    }

    pub fn run(&self) -> Result<()> {
        self.command.execute(&self.context)
    }
}
