use anyhow::{Context, Result};
use backtrace::Backtrace;
use std::{
    env,
    fs::File,
    hash::{Hash, Hasher},
    io::{self, BufWriter, Write},
    mem::size_of,
    panic::{self, Location, PanicInfo},
    path::PathBuf,
    thread::{self, Thread, ThreadId},
};
use uuid::Uuid;

const HEX_PADDING: usize = size_of::<usize>() + 2;

// TODO: Shit, this may conflict with salsa's cancellation-via-panicking mechanism
//       which could really suck if we're making a bunch of report files due to
//       salsa canceling queries
pub fn set_panic_hook() {
    // We don't set our custom panic handler when debug assertions are enabled
    // or we're being run as a test
    if cfg!(not(any(debug_assertions, test))) {
        panic::set_hook(Box::new(|info| {
            let meta = PanicMeta::new(
                Some("https://github.com/vmware/differential-datalog-v2"),
                Some("I-ICE"),
                Some("ice.md"),
            );

            if let Err(error) = handle_dump(&meta, info) {
                eprintln!("error creating error report: {}", error);
            }
        }));
    }
}

pub struct PanicMeta {
    repo: Option<&'static str>,
    labels: Option<&'static str>,
    template: Option<&'static str>,
}

impl PanicMeta {
    pub fn new(
        repo: Option<&'static str>,
        labels: Option<&'static str>,
        template: Option<&'static str>,
    ) -> Self {
        Self {
            repo,
            labels,
            template,
        }
    }
}

struct PanicReport<'a> {
    message: Option<&'a str>,
    location: Option<&'a Location<'a>>,
    // TODO: Switch to std::backtrace::Backtrace
    backtrace: &'a Backtrace,
    thread: Thread,
}

impl<'a> PanicReport<'a> {
    fn new(
        message: Option<&'a str>,
        location: Option<&'a Location<'a>>,
        backtrace: &'a Backtrace,
    ) -> Self {
        Self {
            message,
            location,
            backtrace,
            thread: thread::current(),
        }
    }

    fn serialize_into(&self, mut target: impl Write) -> io::Result<()> {
        if let Some(message) = self.message {
            writeln!(target, "message = {message:?}")?;
        }

        if let Some(location) = self.location {
            writeln!(
                target,
                "file = {:?}\nline = {}\ncolumn = {}",
                location.file(),
                location.line(),
                location.column(),
            )?;
        }

        if let Some(name) = self.thread.name() {
            writeln!(target, "thread-name = {name:?}")?;
        }

        // TODO: Use `ThreadId::as_u64()` once rust/#67939 lands
        if let Some(thread_id) = thread_id_u64(self.thread.id()) {
            writeln!(target, "thread-id = {thread_id}")?;
        }

        writeln!(target, "frames = [")?;
        for frame in self.backtrace.frames() {
            writeln!(target, "    {{  }},")?
        }
        writeln!(target, "]")?;

        todo!()
    }
}

fn thread_id_u64(id: ThreadId) -> Option<u64> {
    struct ThreadIdHasher(Option<u64>);
    impl Hasher for ThreadIdHasher {
        fn finish(&self) -> u64 {
            0
        }

        fn write(&mut self, _: &[u8]) {
            if cfg!(debug_assertions) {
                unreachable!();
            }
        }

        fn write_u64(&mut self, id: u64) {
            self.0 = Some(id);
        }
    }

    // Hack to get the value of a thread id
    let mut hasher = ThreadIdHasher(None);
    id.hash(&mut hasher);
    hasher.0
}

fn handle_dump(meta: &PanicMeta, info: &PanicInfo) -> Result<PathBuf> {
    let mut dump_path = env::temp_dir();
    dump_path.push(format!(
        "ddlog-report-{}.toml",
        Uuid::new_v4().as_hyphenated(),
    ));

    let mut dump_file = BufWriter::new(File::create(&dump_path).with_context(|| {
        format!(
            "failed to create error report file '{}'",
            dump_path.display(),
        )
    })?);

    let backtrace = Backtrace::new();
    let message = info.payload().downcast_ref::<&str>().copied().or_else(|| {
        info.payload()
            .downcast_ref::<String>()
            .map(|message| &**message)
    });
    let location = info.location();

    let report = PanicReport::new(message, location, &backtrace);
    report.serialize_into(&mut dump_file).with_context(|| {
        format!(
            "failed to write to error report file '{}'",
            dump_path.display(),
        )
    })?;

    Ok(dump_path)
}
