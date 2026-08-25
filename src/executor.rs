use anyhow::{Context, Result};
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::Read;

pub struct CommandResult {
    pub command: String,
    pub raw_output: Vec<u8>,
}

pub fn execute_command(cmd: &str, cols: u16, rows: u16) -> Result<CommandResult> {
    let pty_system = NativePtySystem::default();
    let pair = pty_system
        .openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .context("Failed to open PTY")?;

    let mut cmd_builder = if cfg!(unix) {
        let mut cb = CommandBuilder::new("/bin/sh");
        cb.args(["-c", cmd]);
        cb
    } else {
        let mut cb = CommandBuilder::new("powershell.exe");
        cb.args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            cmd,
        ]);
        cb
    };
    cmd_builder.cwd(std::env::current_dir().context("Failed to get current directory")?);

    let mut child = pair
        .slave
        .spawn_command(cmd_builder)
        .context("Failed to spawn command")?;

    drop(pair.slave);

    let mut reader = pair.master.try_clone_reader().context("Failed to clone PTY reader")?;

    let reader_thread = std::thread::spawn(move || -> Vec<u8> {
        let mut buf = Vec::new();
        let _ = reader.read_to_end(&mut buf);
        buf
    });

    let _ = child.wait();
    drop(pair.master);

    let raw_output = reader_thread.join().unwrap_or_default();

    Ok(CommandResult {
        command: cmd.to_string(),
        raw_output,
    })
}

pub fn read_stdin() -> Result<CommandResult> {
    let mut raw_output = Vec::new();
    std::io::stdin()
        .read_to_end(&mut raw_output)
        .context("Failed to read from stdin")?;
    Ok(CommandResult {
        command: String::new(),
        raw_output,
    })
}
