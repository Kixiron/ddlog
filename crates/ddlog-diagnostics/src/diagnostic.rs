use crate::{FileCache, Span};
use ariadne::{CharSet as OutputCharSet, Config, Report, ReportBuilder, ReportKind};
use std::{
    borrow::Cow,
    io::{self, Write},
    mem::take,
};

#[derive(Debug)]
pub struct DiagnosticBuilder {
    message: Option<Cow<'static, str>>,
    note: Option<Cow<'static, str>>,
    help: Option<Cow<'static, str>>,
    labels: Vec<Label>,
    level: Level,
    error_code: Option<u32>,
}

impl DiagnosticBuilder {
    #[inline]
    pub const fn new(level: Level) -> Self {
        Self {
            message: None,
            note: None,
            help: None,
            labels: Vec::new(),
            level,
            error_code: None,
        }
    }

    #[inline]
    pub const fn error() -> Self {
        Self::new(Level::Error)
    }

    #[inline]
    pub const fn warning() -> Self {
        Self::new(Level::Warning)
    }

    #[inline]
    pub const fn note() -> Self {
        Self::new(Level::Note)
    }

    #[inline]
    pub fn with_message<M>(&mut self, message: M) -> &mut Self
    where
        M: Into<Cow<'static, str>>,
    {
        self.message = Some(message.into());
        self
    }

    #[inline]
    pub fn with_note<N>(&mut self, note: N) -> &mut Self
    where
        N: Into<Cow<'static, str>>,
    {
        self.note = Some(note.into());
        self
    }

    #[inline]
    pub fn with_help<H>(&mut self, help: H) -> &mut Self
    where
        H: Into<Cow<'static, str>>,
    {
        self.help = Some(help.into());
        self
    }

    #[inline]
    pub fn with_error_code(&mut self, error_code: u32) -> &mut Self {
        self.error_code = Some(error_code);
        self
    }

    #[inline]
    pub fn with_label(&mut self, label: Label) -> &mut Self {
        self.labels.push(label);
        self
    }

    #[inline]
    pub fn with_labels<I>(&mut self, labels: I) -> &mut Self
    where
        I: IntoIterator<Item = Label>,
    {
        self.labels.extend(labels);
        self
    }

    #[inline]
    #[track_caller]
    pub fn finish(&mut self) -> Diagnostic {
        // TODO: Do some checks & validation here
        Diagnostic {
            message: take(&mut self.message),
            note: take(&mut self.note),
            help: take(&mut self.help),
            labels: take(&mut self.labels),
            level: self.level,
            error_code: self.error_code,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Diagnostic {
    #[doc(hidden)]
    pub message: Option<Cow<'static, str>>,
    #[doc(hidden)]
    pub note: Option<Cow<'static, str>>,
    #[doc(hidden)]
    pub help: Option<Cow<'static, str>>,
    #[doc(hidden)]
    pub labels: Vec<Label>,
    #[doc(hidden)]
    pub level: Level,
    #[doc(hidden)]
    pub error_code: Option<u32>,
}

impl Diagnostic {
    #[inline]
    pub const fn new(level: Level) -> Self {
        Self {
            message: None,
            note: None,
            labels: Vec::new(),
            level,
            error_code: None,
            help: None,
        }
    }

    #[inline]
    pub const fn error() -> Self {
        Self::new(Level::Error)
    }

    #[inline]
    pub const fn warning() -> Self {
        Self::new(Level::Warning)
    }

    #[inline]
    pub const fn note() -> Self {
        Self::new(Level::Note)
    }

    #[inline]
    pub fn with_message<M>(mut self, message: M) -> Self
    where
        M: Into<Cow<'static, str>>,
    {
        self.message = Some(message.into());
        self
    }

    #[inline]
    pub fn with_note<N>(mut self, note: N) -> Self
    where
        N: Into<Cow<'static, str>>,
    {
        self.note = Some(note.into());
        self
    }

    #[inline]
    pub const fn with_code(mut self, code: u32) -> Self {
        self.error_code = Some(code);
        self
    }

    #[inline]
    pub fn with_label(mut self, label: Label) -> Self {
        self.labels.push(label);
        self
    }

    #[inline]
    pub fn primary_span(&self) -> Span {
        self.labels
            .iter()
            .find_map(|label| label.is_primary.then(|| label.span))
            .expect("expected a primary label or a message span within a diagnostic but failed to get one")
    }

    #[inline]
    #[track_caller]
    pub fn emit_to<W>(
        self,
        config: &DiagnosticConfig,
        cache: &mut FileCache,
        writer: W,
    ) -> io::Result<()>
    where
        W: Write,
    {
        let diagnostic = self.into_report(config);
        diagnostic.write(cache, writer)?;

        Ok(())
    }

    #[track_caller]
    fn into_report(self, config: &DiagnosticConfig) -> Report<Span> {
        let primary_span = self.primary_span();

        let mut diagnostic: ReportBuilder<Span> = Report::build(
            self.level.report_kind(),
            primary_span.file(),
            primary_span.start() as usize,
        );

        // Setup the proper config options for the diagnostic
        diagnostic = diagnostic.with_config(
            Config::default()
                .with_color(config.colored)
                .with_compact(config.compact)
                .with_char_set(config.charset.into_output()),
        );

        if let Some(message) = self.message {
            diagnostic.set_message(message);
        }
        if let Some(note) = self.note {
            diagnostic.set_note(note);
        }
        if let Some(help) = self.help {
            diagnostic.set_help(help);
        }
        if let Some(code) = self.error_code {
            diagnostic = diagnostic.with_code(code);
        }

        diagnostic.add_labels(self.labels.into_iter().map(Label::into_report));

        diagnostic.finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    Error,
    Warning,
    Note,
}

impl Level {
    fn report_kind(self) -> ReportKind {
        match self {
            Self::Error => ReportKind::Error,
            Self::Warning => ReportKind::Warning,
            Self::Note => ReportKind::Advice,
        }
    }
}

impl Default for Level {
    #[inline]
    fn default() -> Self {
        Self::Error
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label {
    #[doc(hidden)]
    pub message: Option<Cow<'static, str>>,
    #[doc(hidden)]
    pub span: Span,
    #[doc(hidden)]
    pub is_primary: bool,
}

impl Label {
    #[inline]
    pub const fn primary(span: Span) -> Self {
        Self {
            message: None,
            span,
            is_primary: true,
        }
    }

    #[inline]
    pub const fn secondary(span: Span) -> Self {
        Self {
            message: None,
            span,
            is_primary: false,
        }
    }

    #[inline]
    pub fn with_message<M>(mut self, message: M) -> Self
    where
        M: Into<Cow<'static, str>>,
    {
        self.message = Some(message.into());
        self
    }

    fn into_report(self) -> ariadne::Label<Span> {
        let mut label = ariadne::Label::new(self.span);

        if let Some(message) = self.message {
            label = label.with_message(message);
        }
        label = label.with_priority(if self.is_primary { 1 } else { 0 });

        label
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticConfig {
    colored: bool,
    compact: bool,
    charset: CharSet,
}

impl DiagnosticConfig {
    #[inline]
    pub const fn new() -> Self {
        Self {
            colored: true,
            compact: false,
            charset: CharSet::Ascii,
        }
    }

    #[inline]
    pub const fn with_color(self, colored: bool) -> Self {
        Self { colored, ..self }
    }

    #[inline]
    pub const fn with_compact(self, compact: bool) -> Self {
        Self { compact, ..self }
    }

    #[inline]
    pub const fn with_charset(self, charset: CharSet) -> Self {
        Self { charset, ..self }
    }
}

impl Default for DiagnosticConfig {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharSet {
    Ascii,
    Unicode,
}

impl CharSet {
    const fn into_output(self) -> OutputCharSet {
        match self {
            Self::Ascii => OutputCharSet::Ascii,
            Self::Unicode => OutputCharSet::Unicode,
        }
    }
}

impl Default for CharSet {
    #[inline]
    fn default() -> Self {
        Self::Ascii
    }
}
