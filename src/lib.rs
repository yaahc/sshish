use std::collections::BTreeMap;
use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::process::Child;
use std::process::ExitStatus;
use std::process::Output;
use std::process::Stdio;

type Result<T> = std::result::Result<T, std::io::Error>;

pub struct PreCommand {
    cmd: OsString,
    port: u16,
}

pub struct Command {
    ssh: std::process::Command,
    cmd: OsString,
    cwd: Option<OsString>,
    args: Vec<OsString>,
    envs: BTreeMap<OsString, Option<OsString>>,
}

impl PreCommand {
    pub fn port(self, port: u16) -> Self {
        Self { port, ..self }
    }

    pub fn host<S: AsRef<OsStr>>(self, host: S) -> Command {
        let mut cmd = std::process::Command::new("ssh");
        cmd.arg(host);

        Command {
            ssh: cmd,
            cmd: self.cmd,
            cwd: None,
            args: vec![],
            envs: BTreeMap::new(),
        }
    }
}

impl Command {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<S: AsRef<OsStr>>(program: S) -> PreCommand {
        PreCommand {
            cmd: program.as_ref().into(),
            port: 22,
        }
    }

    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
        unimplemented!()
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        unimplemented!()
    }

    pub fn env<K, V>(&mut self, key: K, val: V) -> &mut Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        unimplemented!()
    }

    pub fn envs<I, K, V>(&mut self, vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        unimplemented!()
    }

    pub fn env_remove<K: AsRef<OsStr>>(&mut self, key: K) -> &mut Self {
        unimplemented!()
    }

    pub fn env_clear(&mut self) -> &mut Command {
        unimplemented!()
    }

    pub fn current_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Command {
        unimplemented!()
    }

    pub fn stdin<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        self.ssh.stdin(cfg);

        self
    }

    pub fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        self.ssh.stdout(cfg);

        self
    }

    pub fn stderr<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        self.ssh.stderr(cfg);

        self
    }

    pub fn spawn(&mut self) -> Result<Child> {
        self.apply_envs();
        self.apply_args();

        self.ssh.spawn()
    }

    /// Blah
    ///
    /// ```
    /// use std::io::{self, Write};
    ///
    /// let output = sshish::Command::new("ls")
    ///                  .host("localhost")
    ///                  .output()
    ///                  .unwrap();
    ///
    /// println!("status: {}", output.status);
    ///
    /// assert!(!output.stdout.is_empty());
    /// assert!(output.stderr.is_empty());
    ///
    /// io::stdout().write_all(&output.stdout).unwrap();
    /// io::stderr().write_all(&output.stderr).unwrap();
    ///
    /// assert!(output.status.success());
    /// ```
    pub fn output(&mut self) -> Result<Output> {
        self.apply_envs();
        self.apply_args();

        self.ssh.output()
    }

    pub fn status(&mut self) -> Result<ExitStatus> {
        self.apply_envs();
        self.apply_args();

        self.ssh.status()
    }

    fn apply_envs(&mut self) {
        for (key, value) in &mut self.envs {
            unimplemented!()
        }
    }

    fn apply_args(&mut self) {
        self.ssh.arg(self.cmd.as_os_str());
        for arg in &mut self.args {
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
