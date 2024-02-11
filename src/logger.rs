use std::collections::HashMap;

use indicatif::{ProgressBar, ProgressStyle};
use termion::{color, style, cursor, clear};

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct WorkID(usize);

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct WorkInfo {
    msg: WorkMessage, 
    line: u16,
    indent: u16,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum WorkIndex {
    Some {
        position: usize,
        max: usize,
    },
    None,
}

impl WorkIndex {
    pub fn new(position: usize, max: usize) -> Self {
        Self::Some { position, max }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct WorkMessage {
    pub colored: &'static str,
    pub uncolored: &'static str,
    pub index: WorkIndex,
}

impl WorkMessage {
    pub fn new(colored: &'static str, uncolored: &'static str, index: WorkIndex) -> Self {
        Self { colored, uncolored, index }
    }
}

bitflags::bitflags! {
    pub struct ProgressBarElements: u8 {
        const POSITION = 1 << 0;
        const PERCENTAGE = 1 << 1;
        const ELAPSED = 1 << 2;
        const ETA = 1 << 3;
    }
}

impl ProgressBarElements {
    fn get_template(&self) -> String {
        let mut template = String::new();
        if self.contains(Self::POSITION) && self.contains(Self::PERCENTAGE) {
            template += "[{human_pos}/{human_len} ({percent}%)]";
        } else if self.contains(Self::POSITION) {
            template += "[{human_pos}/{human_len}]";
        } else if self.contains(Self::PERCENTAGE) {
            template += "[{percent}%]";
        }

        if self.contains(Self::ELAPSED) {
            template += " | Elapsed: {elapsed}";
        }
        if self.contains(Self::ETA) {
            template += " | ETA: {eta}";
        }

        template += " [{bar:50.cyan/blue}]";
        template
    }
}

pub struct TerminalLogger {
    indentation: u16,
    cur_line: u16,

    cur_id: WorkID,
    active_work: HashMap<WorkID, WorkInfo>
}

impl TerminalLogger {
    pub fn new() -> Self {
        Self {
            indentation: 1,
            cur_line: 1,
            cur_id: WorkID(0),
            active_work: HashMap::new(),
        }
    }

    pub fn begin_section(&mut self) {
        self.indentation += 2;
    }
    pub fn end_section(&mut self) {
        self.indentation = match self.indentation.checked_sub(2) {
            Some(indent) => indent,
            None => 0
        }
    }

    pub fn reset_pos(&self) {
        println!("{}", cursor::Goto(self.indentation, self.cur_line));
    }

    pub fn clear(&self) {
        println!("{}", clear::All);
    }
    pub fn newline(&mut self) {
        self.cur_line += 1;
        println!();
    }
    pub fn sleep(&self, secs: f32) {
        std::thread::sleep(std::time::Duration::from_secs_f32(secs));
    }

    pub fn create_progress(&self, max: u64, elements: ProgressBarElements) -> ProgressBar {
        let mut template = elements.get_template();
        for _ in 0..(self.indentation + 1) {
            template.insert(0, ' ');
        }
        let bar = ProgressBar::new(max)
            .with_style(
                ProgressStyle::with_template(&template)
                .unwrap()
                .progress_chars("=>·")
            );
        bar
    }

    pub fn initialize(&mut self) {
        self.clear();
        self.newline();
    }
    pub fn finish(&self) {
        self.reset_pos();
    }

    pub fn begin_work(&mut self, msg: WorkMessage) -> WorkID {
        self.cur_id.0 = self.cur_id.0.wrapping_add(1);
        self.active_work.insert(self.cur_id, WorkInfo {
            msg,
            line: self.cur_line,
            indent: self.indentation
        });
        match msg.index {
            WorkIndex::Some { position, max } => {
                print!(
                    "{}{}{}[{}/{}] {}{} {}{}{}...\n", 
                    cursor::Goto(self.indentation, self.cur_line), 
                    clear::CurrentLine, 
                    style::Bold,
                    position,
                    max,
                    color::Fg(color::Cyan),
                    msg.colored,
                    color::Fg(color::Reset),
                    style::Reset,
                    msg.uncolored,
                );
            },
            WorkIndex::None => {
                print!(
                    "{}{}{}{}{} {}{}{}...\n", 
                    cursor::Goto(self.indentation, self.cur_line), 
                    clear::CurrentLine, 
                    style::Bold,
                    color::Fg(color::Cyan),
                    msg.colored,
                    color::Fg(color::Reset),
                    style::Reset,
                    msg.uncolored,
                );
            }
        }
        self.cur_line += 1;
        self.cur_id
    }
    pub fn finish_work(&mut self, id: WorkID) {
        let info = self.active_work.get(&id).expect("WorkID does not correspond to any active work");
        
        match info.msg.index {
            WorkIndex::Some { position, max } => {
                print!(
                    "{}{}{}[{}/{}] {}{} {}{}{} {}\n", 
                    cursor::Goto(info.indent, info.line), 
                    clear::CurrentLine, 
                    style::Bold,
                    position,
                    max,
                    color::Fg(color::Green),
                    "Finished",
                    color::Fg(color::Reset),
                    style::Reset,
                    info.msg.colored,
                    info.msg.uncolored,
                );
            },
            WorkIndex::None => {
                print!(
                    "{}{}{}{}{} {}{}{} {}\n", 
                    cursor::Goto(info.indent, info.line), 
                    clear::CurrentLine, 
                    style::Bold,
                    color::Fg(color::Green),
                    "Finished",
                    color::Fg(color::Reset),
                    style::Reset,
                    info.msg.colored,
                    info.msg.uncolored,
                );
            }
        }

        self.active_work.remove(&id);
    }
    pub fn fail_work_panic(&mut self, id: WorkID) {
        let info = self.active_work.get(&id).expect("WorkID does not correspond to any active work");
        
        match info.msg.index {
            WorkIndex::Some { position, max } => {
                print!(
                    "{}{}{}[{}/{}] {}{} {}{}{} {}\n", 
                    cursor::Goto(info.indent, info.line), 
                    clear::CurrentLine, 
                    style::Bold,
                    position,
                    max,
                    color::Fg(color::Red),
                    "Failed",
                    color::Fg(color::Reset),
                    style::Reset,
                    info.msg.colored,
                    info.msg.uncolored,
                );
            },
            WorkIndex::None => {
                print!(
                    "{}{}{}{}{} {}{}{} {}\n", 
                    cursor::Goto(info.indent, info.line), 
                    clear::CurrentLine, 
                    style::Bold,
                    color::Fg(color::Red),
                    "Failed",
                    color::Fg(color::Reset),
                    style::Reset,
                    info.msg.colored,
                    info.msg.uncolored,
                );
            }
        }

        self.reset_pos();
        panic!();
    }
    pub fn fail_work(&mut self, id: WorkID, err_msg: &'static str) {
        let info = self.active_work.get(&id).expect("WorkID does not correspond to any active work");
        
        match info.msg.index {
            WorkIndex::Some { position, max } => {
                print!(
                    "{}{}{}[{}/{}] {}{} {}{}{} {}, {}{}{}...\n", 
                    cursor::Goto(info.indent, info.line), 
                    clear::CurrentLine, 
                    style::Bold,
                    position,
                    max,
                    color::Fg(color::Red),
                    "Failed",
                    color::Fg(color::Reset),
                    style::Reset,
                    info.msg.colored,
                    info.msg.uncolored,
                    color::Fg(color::Yellow),
                    err_msg,
                    color::Fg(color::Reset),
                );
            },
            WorkIndex::None => {
                print!(
                    "{}{}{}{}{} {}{}{} {}, {}...\n", 
                    cursor::Goto(info.indent, info.line), 
                    clear::CurrentLine, 
                    style::Bold,
                    color::Fg(color::Red),
                    "Failed",
                    color::Fg(color::Reset),
                    style::Reset,
                    info.msg.colored,
                    info.msg.uncolored,
                    err_msg,
                );
            }
        }
    }
}