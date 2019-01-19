use self::Execution::*;
use application::ApplicationInfo;
use commands::{Command, Config, Context, Execution, Result};
use fs;
use fs::FileSystemAccess;
use notification::{MessageLength, LogLevel, Notifier};

#[derive(Debug)]
pub struct Session {
    pub command: Box<Command>,
    pub context: Context,
}

impl Session {
    pub fn new(
        appinfo: ApplicationInfo,
        cfg: Config,
        exec: Execution,
        nofitier_factory: impl FnOnce(LogLevel, MessageLength) -> Box<Notifier>,
    ) -> Session {
        Session::new_with_custom_file_system_access(
            appinfo,
            cfg,
            exec,
            nofitier_factory,
            fs::create_file_system_access,
        )
    }

    pub fn new_with_custom_file_system_access(
        appinfo: ApplicationInfo,
        cfg: Config,
        exec: Execution,
        nofitier_factory: impl FnOnce(LogLevel, MessageLength) -> Box<Notifier>,
        fsaccess_factory: impl FnOnce(bool) -> Box<FileSystemAccess>,
    ) -> Session {
        let (cmd, dry_run) = match exec {
            DryRun(cmd) => (cmd, true),
            Run(cmd) => (cmd, false),
        };

        let ctx = Context::new(appinfo, cfg, dry_run, nofitier_factory, fsaccess_factory);

        Session {
            command: cmd,
            context: ctx,
        }
    }

    pub fn run(&self) -> Result<()> {
        self.command.execute(&self.context)
    }
}
