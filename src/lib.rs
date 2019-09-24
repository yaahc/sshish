use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::process::Child;
use std::process::ExitStatus;
use std::process::Output;
use std::process::Stdio;

type Result<T> = std::result::Result<T, std::io::Error>;

pub struct Session {
    host: OsString,
}

impl Session {
    pub fn new(host: impl AsRef<OsStr>) -> Self {
        Self {
            host: host.as_ref().into(),
        }
    }

    pub fn command(cmd: impl AsRef<OsStr>) -> Command {
        unimplemented!()
    }
}

pub struct Command;

impl Command {
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
        unimplemented!()
    }

    pub fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        unimplemented!()
    }

    pub fn stderr<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        unimplemented!()
    }

    pub fn spawn(&mut self) -> Result<Child> {
        unimplemented!()
    }

    pub fn output(&mut self) -> Result<Output> {
        unimplemented!()
    }

    pub fn status(&mut self) -> Result<ExitStatus> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
