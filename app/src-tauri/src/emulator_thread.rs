//! Thread ownership for the emulator.
//!
//! The GBA core is single threaded (it is full of `Rc`s), so it can never be
//! moved between threads or shared behind a `Mutex`. Instead one dedicated
//! thread owns the [`EmulatorHost`] for the whole process lifetime and receives
//! commands over an `mpsc` channel.

use std::path::PathBuf;
use std::sync::mpsc::{self, Sender};

use tauri::ipc::Channel;

use crate::emulator_host::{EmulatorHost, InputEvent};

/// A request from the UI to the emulator thread.
pub enum Command {
    /// Load a ROM and start streaming frames to the given channel.
    Start {
        rom: Vec<u8>,
        sha_hex: String,
        saves_dir: PathBuf,
        frames: Channel<Vec<u8>>,
        reply: Sender<Result<(), String>>,
    },
    /// Apply button transitions.
    Input(Vec<InputEvent>),
    /// Write a save state into `dir` and report the path.
    SaveState {
        dir: PathBuf,
        reply: Sender<Result<String, String>>,
    },
    /// Read a save state from `dir`.
    LoadState {
        dir: PathBuf,
        reply: Sender<Result<(), String>>,
    },
    /// Report whether a ROM is loaded, whether a BIOS exists, and the FPS.
    Status { reply: Sender<(bool, bool, f64)> },
    /// Persist the battery save.
    Flush,
}

/// Handle used by Tauri commands to talk to the emulator thread.
#[derive(Clone)]
pub struct EmulatorHandle {
    tx: Sender<Command>,
}

impl EmulatorHandle {
    /// Spawns the emulator thread and returns a handle to it.
    pub fn spawn() -> Self {
        let (tx, rx) = mpsc::channel::<Command>();

        std::thread::Builder::new()
            .name("oemu-emulator".into())
            .spawn(move || {
                let mut host = EmulatorHost::new();
                // `None` while no ROM is loaded; replaced by `Start`.
                let mut frames: Option<Channel<Vec<u8>>> = None;

                loop {
                    // Block for the next command, but never longer than a frame
                    // so emulation keeps running in the meantime.
                    let next = rx.recv_timeout(std::time::Duration::from_micros(500));

                    match next {
                        Ok(Command::Start {
                            rom,
                            sha_hex,
                            saves_dir,
                            frames: channel,
                            reply,
                        }) => {
                            let result = host.start(rom, &sha_hex, &saves_dir);
                            if result.is_ok() {
                                frames = Some(channel);
                            }
                            let _ = reply.send(result);
                        }
                        Ok(Command::Input(events)) => host.apply_input(&events),
                        Ok(Command::SaveState { dir, reply }) => {
                            let _ = reply.send(host.save_state(&dir));
                        }
                        Ok(Command::LoadState { dir, reply }) => {
                            let _ = reply.send(host.load_state(&dir));
                        }
                        Ok(Command::Status { reply }) => {
                            let _ = reply.send((host.has_rom(), host.bios_found(), host.fps()));
                        }
                        Ok(Command::Flush) => host.flush_battery(),
                        Err(mpsc::RecvTimeoutError::Timeout) => {}
                        Err(mpsc::RecvTimeoutError::Disconnected) => break,
                    }

                    if let Some(frame) = host.step() {
                        if let Some(channel) = &frames {
                            // A closed UI channel means the window went away.
                            if channel.send(frame).is_err() {
                                frames = None;
                            }
                        }
                    }
                }

                host.flush_battery();
            })
            .expect("cannot spawn the emulator thread");

        Self { tx }
    }

    /// Sends a command, ignoring failures (the thread only dies on shutdown).
    pub fn send(&self, command: Command) {
        let _ = self.tx.send(command);
    }

    /// Sends a command and waits for its reply.
    pub fn ask<T>(&self, make: impl FnOnce(Sender<T>) -> Command) -> Result<T, String> {
        let (tx, rx) = mpsc::channel();
        self.tx
            .send(make(tx))
            .map_err(|_| "emulator thread is gone".to_string())?;
        rx.recv_timeout(std::time::Duration::from_secs(10))
            .map_err(|_| "emulator did not answer in time".to_string())
    }
}
