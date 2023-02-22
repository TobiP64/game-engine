// MIT License
//
// Copyright (c) 2019-2023 Tobias Pfeiffer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::{io::Write, lazy::SyncOnceCell};

static LOGGER: SyncOnceCell<Logger> = SyncOnceCell::new();

pub struct Logger {
    start: std::time::Instant,
}

pub fn logger() -> &'static dyn log::Log {
    LOGGER.get_or_init(|| Logger {
        start: std::time::Instant::now(),
    })
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        use log::Level::*;

        let (color, tag) = match record.level() {
            Trace => ("90", "TRACE"),
            Debug => ("32", "DEBUG"),
            Info  => ("27", "INFO"),
            Warn  => ("33", "WARN"),
            Error => ("31", "ERROR"),
        };

        let msg = format!(
            "\x1b[{0}m{2:.3}s \x1b[{0}m\x1b[7m{1}\x1b[27m\x1b[{0}m `{3}` {4}:{5} {6}\x1b[0m\n",
            color,
            tag,
            self.start.elapsed().as_secs_f32(),
            std::thread::current().name().unwrap_or("unnamed"),
            record.target(),
            record.line().unwrap_or(0),
            record.args()
        );

        std::io::stdout().write_all(msg.as_bytes()).unwrap_or(());
    }

    fn flush(&self) {}
}
