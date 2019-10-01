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
        self.args.push(arg.as_ref().to_os_string());

        self
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        for arg in args {
            self.arg(arg);
        }

        self
    }

    pub fn env<K, V>(&mut self, key: K, val: V) -> &mut Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.envs.insert(
            key.as_ref().to_os_string(),
            Some(val.as_ref().to_os_string()),
        );

        self
    }

    pub fn envs<I, K, V>(&mut self, vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        for (key, val) in vars {
            self.env(key, val);
        }

        self
    }

    pub fn env_remove<K: AsRef<OsStr>>(&mut self, key: K) -> &mut Self {
        unimplemented!()
    }

    pub fn env_clear(&mut self) -> &mut Command {
        unimplemented!()
    }

    pub fn current_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Command {
        self.cwd = Some(dir.as_ref().as_os_str().to_os_string());

        self
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
        if let Some(dir) = &self.cwd {
            self.ssh.arg("cd").arg(dir).arg("&&");
        }

        for (key, value) in &mut self.envs {
            let key = key.to_string_lossy();
            let value = shell_escape::escape(value.as_ref().unwrap().to_string_lossy());
            let arg = format!("{}={}", key, value);
            self.ssh.arg(arg);
        }
    }

    fn apply_args(&mut self) {
        self.ssh.arg(self.cmd.as_os_str());
        for arg in &mut self.args {
            let escaped_arg = shell_escape::escape(arg.to_string_lossy());
            self.ssh.arg(escaped_arg.as_ref());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg_splitting() {
        let output = Command::new("./tests/countargs.sh")
            .host("localhost")
            .current_dir(std::env::current_dir().unwrap())
            .arg("hi")
            .arg(r#"hello what is up"#)
            .output()
            .unwrap();

        let output_str = std::str::from_utf8(&output.stdout).unwrap();
        println!("{}", output_str);
        assert_eq!(output_str.trim(), "2");
    }

    #[test]
    fn test_env_variables() {
        let val = "whatever it doesn't matter what I put here";
        let output = Command::new("bash")
            .host("localhost")
            .env("hihello", val)
            .arg("-c")
            .arg("echo $hihello")
            .output()
            .unwrap();

        let output_str = std::str::from_utf8(&output.stdout).unwrap();
        println!("{}", output_str);
        assert_eq!(output_str.trim(), val);
    }

    #[test]
    #[should_panic]
    fn test_env_variables_wrongly() {
        let val = "whatever it doesn't matter what I put here";
        let output = Command::new("echo")
            .host("localhost")
            .env("hihello", val)
            .arg("$hihello")
            .output()
            .unwrap();

        let output_str = std::str::from_utf8(&output.stdout).unwrap();
        println!("{}", output_str);
        assert_eq!(output_str.trim(), val);
    }
}
