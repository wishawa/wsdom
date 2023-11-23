use std::{collections::HashMap, error::Error, fmt::Arguments, fmt::Write, task::Waker};

pub struct WrmiLink {
    pub(crate) retrieve_wakers: HashMap<u64, Waker>,
    pub(crate) retrieve_values: HashMap<u64, String>,
    last_id: u64,
    commands_buf: String,
    dead: Option<Box<dyn Error>>,
}

impl WrmiLink {
    pub fn receive(&mut self, message: String) {
        match message
            .split_once(':')
            .and_then(|(id, _)| id.parse::<u64>().ok())
        {
            Some(id) => {
                if let Some(waker) = self.retrieve_wakers.remove(&id) {
                    self.retrieve_values.insert(id, message);
                    waker.wake();
                }
            }
            None => self.kill(Box::new(InvalidReturn)),
        }
    }
    pub fn raw_commands_buf(&mut self) -> &mut String {
        &mut self.commands_buf
    }
    pub fn send_command(&mut self, cmd: Arguments<'_>) {
        write!(&mut self.commands_buf, "{{{}}}\n", cmd).unwrap();
    }
    pub(crate) fn get_new_id(&mut self) -> u64 {
        self.last_id += 1;
        self.last_id
    }
    pub(crate) fn kill(&mut self, err: Box<dyn Error>) {
        if self.dead.is_none() {
            self.dead = Some(err);
        }
    }
}

struct InvalidReturn;
impl std::fmt::Debug for InvalidReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InvalidReturn").finish()
    }
}
impl std::fmt::Display for InvalidReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
impl std::error::Error for InvalidReturn {}
