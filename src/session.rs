use std::os::unix::process::CommandExt;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

pub enum Status {
    Running,
    Exited(Option<i32>),
}

pub struct Session {
    pub id: usize,
    pub command: String,
    pub pid: u32,
    pub child: Child,
    pub status: Status,
    pub started_at: Instant,
    pub exited_at: Option<Instant>,
}

impl Session {
    pub fn new(id: usize, cmd_str: &str) -> Result<Self, std::io::Error> {
        let parts: Vec<&str> = cmd_str.split_whitespace().collect();
        if parts.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Empty command",
            ));
        }
        let program = parts[0];
        let args = &parts[1..];

        let mut cmd = Command::new(program);
        cmd.args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        unsafe {
            cmd.pre_exec(|| {
                libc::setsid();
                Ok(())
            });
        }
        let child = cmd.spawn()?;
        let pid = child.id();

        Ok(Self {
            id,
            command: cmd_str.to_string(),
            pid,
            child,
            status: Status::Running,
            started_at: Instant::now(),
            exited_at: None,
        })
    }

    pub fn update_status(&mut self) {
        if matches!(self.status, Status::Running) {
            match self.child.try_wait() {
                Ok(Some(status)) => {
                    self.status = Status::Exited(status.code());
                    self.exited_at = Some(Instant::now());
                }
                Ok(None) => {} // Still running
                Err(_) => {}   // Error waiting
            }
        }
    }

    pub fn kill(&mut self) -> Result<(), std::io::Error> {
        self.child.kill()
    }

    pub fn uptime(&self) -> Duration {
        if let Some(exited) = self.exited_at {
            exited.duration_since(self.started_at)
        } else {
            self.started_at.elapsed()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_command_fails() {
        let res = Session::new(1, "");
        assert!(res.is_err());
    }

    #[test]
    fn test_whitespace_command_fails() {
        let res = Session::new(1, "   ");
        assert!(res.is_err());
    }
}
