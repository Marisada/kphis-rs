// from https://github.com/fenjalien/obsidian-typst
// customized for WASM note:
// - pass `token` around via SystemWorld and ReadFn

use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    term,
};
use ecow::eco_format;
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use typst::{LibraryExt, WorldExt, compile};
use typst_layout::PagedDocument;
use typst_library::{
    Library, World,
    diag::{EcoString, FileError, FileResult, Severity, SourceDiagnostic, Warned},
    foundations::{Bytes, Datetime, Duration},
    text::{Font, FontBook},
};
use typst_syntax::{DiagSpan, FileId, Lines, RootedPath, Source, VirtualPath, VirtualRoot};
use typst_timing::timed;
use typst_utils::{LazyHash, hash128};

use kphis_util::{
    datetime::js_now,
    pdf::{DATA_PATH, INPUT_PATH, create_source, start_embedded_fonts},
};

// path + token -> file
type ReadFn = fn(PathBuf, &Option<String>) -> FileResult<Vec<u8>>;
type CodespanResult<T> = Result<T, CodespanError>;
type CodespanError = codespan_reporting::files::Error;

/// A world that provides access to the operating system.
pub struct SystemWorld {
    /// The input source, generate when call self.compile()
    main: Option<FileId>,
    /// Typst's standard library.
    library: LazyHash<Library>,
    /// Metadata about discovered fonts.
    book: LazyHash<FontBook>,
    /// Storage of fonts
    fonts: Vec<Font>,
    /// Maps file ids to source files and buffers.
    slots: Arc<Mutex<HashMap<FileId, FileSlot>>>,
    /// read file function
    read_fn: Option<ReadFn>,
    /// token
    token: Option<String>,
}

impl Default for SystemWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemWorld {
    pub fn new() -> SystemWorld {
        let (book, fonts) = start_embedded_fonts();

        Self {
            main: None,
            library: LazyHash::new(Library::builder().build()),
            book: LazyHash::new(book),
            fonts,
            slots: Arc::new(Mutex::new(HashMap::new())),
            read_fn: None,
            token: None,
        }
    }

    /// create Document from typst syntax string
    pub fn compile(&mut self, input: &str, data: &str, read_fn: ReadFn, token: Option<String>) -> Result<PagedDocument, String> {
        self.reset();
        let input_id = FileId::new(RootedPath::new(VirtualRoot::Project, VirtualPath::new(INPUT_PATH).unwrap()));
        let data_id = FileId::new(RootedPath::new(VirtualRoot::Project, VirtualPath::new(DATA_PATH).unwrap()));
        self.main = Some(input_id);

        let mut input_slot = FileSlot::new(input_id);
        input_slot.source.set(|| Ok(input.as_bytes().to_vec()), |bytes, prev| create_source(input_id, &bytes, prev));
        input_slot.file.set(|| Ok(input.as_bytes().to_vec()), |bytes, _| Ok(Bytes::new(bytes)));

        let mut data_slot = FileSlot::new(data_id);
        data_slot.source.set(|| Ok(data.as_bytes().to_vec()), |bytes, prev| create_source(data_id, &bytes, prev));
        data_slot.file.set(|| Ok(data.as_bytes().to_vec()), |bytes, _| Ok(Bytes::new(bytes)));
        match self.slots.lock() {
            Ok(mut map) => {
                map.insert(input_id, input_slot);
                map.insert(data_id, data_slot);
            }
            Err(_) => {
                return Err(String::from("Locked"));
            }
        }
        self.read_fn = Some(read_fn);
        self.token = token;

        let Warned { output, warnings } = compile(self);

        output.map_err(|errors| self.format_diagnostic(&errors, &warnings))
    }

    // check typst-cli/src/compile.rs:fn print_diagnostics()
    fn format_diagnostic(&self, errors: &[SourceDiagnostic], warnings: &[SourceDiagnostic]) -> String {
        let mut bytes = Vec::new();
        let config = term::Config { tab_width: 2, ..Default::default() };

        for diagnostic in warnings.iter().chain(errors) {
            let diag = match diagnostic.severity {
                Severity::Error => Diagnostic::error(),
                Severity::Warning => Diagnostic::warning(),
            }
            .with_message(diagnostic.message.clone())
            .with_notes(diagnostic.hints.iter().filter(|s| s.span.is_detached()).map(|s| (eco_format!("hint: {}", s.v)).into()).collect())
            .with_labels(
                self.label(diagnostic.span)
                    .into_iter()
                    .chain(diagnostic.hints.iter().filter_map(|hint| {
                        let id = hint.span.id()?;
                        let range = self.range(hint.span)?;
                        Some(Label::secondary(id, range).with_message(&hint.v))
                    }))
                    .collect(),
            );

            if let Err(e) = term::emit_to_io_write(&mut bytes, &config, self, &diag) {
                bytes.extend(e.to_string().as_bytes().to_vec());
            }

            bytes.push(b'\n');
            for point in &diagnostic.trace {
                let message = point.v.to_string();
                let help = Diagnostic::help().with_message(message).with_labels(self.label(point.span.into()).into_iter().collect());

                if let Err(e) = term::emit_to_io_write(&mut bytes, &config, self, &help) {
                    bytes.extend(e.to_string().as_bytes().to_vec());
                }

                bytes.push(b'\n');
            }
        }

        String::from_utf8(bytes).unwrap_or_default().trim().to_owned()
    }

    /// Create a label for a span.
    fn label(&self, span: DiagSpan) -> Option<Label<FileId>> {
        Some(Label::primary(span.id()?, self.range(span)?))
    }

    /// Lookup line metadata for a file by id.
    #[track_caller]
    pub fn lookup(&self, id: FileId) -> Lines<String> {
        self.source(id).expect("file is not valid").lines().clone()
    }

    /// Reset the compilation state in preparation of a new compilation.
    pub fn reset(&self) {
        if let Ok(mut map) = self.slots.lock() {
            for slot in map.values_mut() {
                slot.reset();
            }
        }
    }

    /// Access the canonical slot for the given file id.
    fn slot<F, T>(&self, id: FileId, f: F) -> FileResult<T>
    where
        F: FnOnce(&mut FileSlot) -> FileResult<T>,
    {
        match self.slots.lock() {
            Ok(mut map) => f(map.entry(id).or_insert_with(|| FileSlot::new(id))),
            Err(_) => Err(FileError::Other(Some("Locked".into()))),
        }
    }
}

impl World for SystemWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.main.unwrap_or(FileId::new(RootedPath::new(VirtualRoot::Project, VirtualPath::new(INPUT_PATH).unwrap())))
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        self.slot(id, |slot| slot.source(self.read_fn, &self.token))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.slot(id, |slot| slot.file(self.read_fn, &self.token))
    }

    fn font(&self, index: usize) -> Option<Font> {
        Some(self.fonts[index].clone())
    }

    fn today(&self, _: Option<Duration>) -> Option<Datetime> {
        Some(Datetime::Date(js_now().date()))
    }
}

impl<'a> codespan_reporting::files::Files<'a> for SystemWorld {
    type FileId = FileId;
    type Name = String;
    type Source = Lines<String>;

    fn name(&'a self, id: FileId) -> CodespanResult<Self::Name> {
        let vpath = id.vpath();
        Ok(match id.root() {
            VirtualRoot::Project => format!("{}", vpath.get_without_slash()),
            VirtualRoot::Package(package) => {
                format!("{package}{}", vpath.get_with_slash())
            }
        })
    }

    fn source(&'a self, id: FileId) -> CodespanResult<Self::Source> {
        Ok(self.lookup(id))
    }

    fn line_index(&'a self, id: FileId, given: usize) -> CodespanResult<usize> {
        let source = self.lookup(id);
        source.byte_to_line(given).ok_or_else(|| CodespanError::IndexTooLarge { given, max: source.len_bytes() })
    }

    fn line_range(&'a self, id: FileId, given: usize) -> CodespanResult<std::ops::Range<usize>> {
        let source = self.lookup(id);
        source.line_to_range(given).ok_or_else(|| CodespanError::LineTooLarge { given, max: source.len_lines() })
    }

    fn column_number(&'a self, id: FileId, _: usize, given: usize) -> CodespanResult<usize> {
        let source = self.lookup(id);
        source.byte_to_column(given).ok_or_else(|| {
            let max = source.len_bytes();
            if given <= max {
                CodespanError::InvalidCharBoundary { given }
            } else {
                CodespanError::IndexTooLarge { given, max }
            }
        })
    }
}

fn read_file(id: FileId, read_fn: ReadFn, token: &Option<String>) -> FileResult<Vec<u8>> {
    let vpath = id.vpath().get_without_slash();
    let mut path = PathBuf::new();
    if let Some(spec) = match id.root() {
        VirtualRoot::Project => None,
        VirtualRoot::Package(package) => Some(package),
    } {
        path.push("typsts");
        path.push(spec.name.as_str());
        path.push(spec.version.to_string());
    }
    path.push(vpath);
    read_fn(path, token)
}

/// Holds the processed data for a file ID.
///
/// Both fields can be populated if the file is both imported and read().
struct FileSlot {
    /// The slot's file id.
    id: FileId,
    /// The lazily loaded and incrementally updated source file.
    source: SlotCell<Source>,
    /// The lazily loaded raw byte buffer.
    file: SlotCell<Bytes>,
}

impl FileSlot {
    /// Create a new path slot.
    fn new(id: FileId) -> Self {
        Self {
            id,
            file: SlotCell::new(),
            source: SlotCell::new(),
        }
    }

    /// Marks the file as not yet accessed in preparation of the next
    /// compilation.
    fn reset(&mut self) {
        self.source.reset();
        self.file.reset();
    }

    /// Retrieve the source for this file.
    fn source(&mut self, read_fn: Option<ReadFn>, token: &Option<String>) -> FileResult<Source> {
        self.source
            .get_or_init(read_fn, |reader| read_file(self.id, reader, token), |data, prev| create_source(self.id, &data, prev))
    }

    /// Retrieve the file's bytes.
    fn file(&mut self, read_fn: Option<ReadFn>, token: &Option<String>) -> FileResult<Bytes> {
        self.file.get_or_init(read_fn, |reader| read_file(self.id, reader, token), |data, _| Ok(Bytes::new(data)))
    }
}

// different `read_fn`
/// Lazily processes data for a file.
struct SlotCell<T> {
    /// The processed data.
    data: Option<FileResult<T>>,
    /// A hash of the raw file contents / access error.
    fingerprint: u128,
    /// Whether the slot has been accessed in the current compilation.
    accessed: bool,
}

impl<T: Clone> SlotCell<T> {
    /// Creates a new, empty cell.
    fn new() -> Self {
        Self {
            data: None,
            fingerprint: 0,
            accessed: false,
        }
    }

    /// Marks the cell as not yet accessed in preparation of the next
    /// compilation.
    fn reset(&mut self) {
        self.accessed = false;
    }

    /// Gets the contents of the cell or initialize them.
    fn get_or_init(&mut self, read_fn: Option<ReadFn>, load: impl FnOnce(ReadFn) -> FileResult<Vec<u8>>, f: impl FnOnce(Vec<u8>, Option<T>) -> FileResult<T>) -> FileResult<T> {
        // If we accessed the file already in this compilation, retrieve it.
        if std::mem::replace(&mut self.accessed, true) {
            if let Some(data) = &self.data {
                return data.clone();
            }
        }

        // Read and hash the file.
        if let Some(reader) = read_fn {
            let result = timed!("loading file", load(reader));
            let fingerprint = timed!("hashing file", hash128(&result));

            // If the file contents didn't change, yield the old processed data.
            if std::mem::replace(&mut self.fingerprint, fingerprint) == fingerprint {
                if let Some(data) = &self.data {
                    return data.clone();
                }
            }

            let prev = self.data.take().and_then(Result::ok);
            let value = result.and_then(|data| f(data, prev));
            self.data = Some(value.clone());

            value
        } else {
            Err(FileError::Other(Some(EcoString::from("No Reader Fn"))))
        }
    }

    /// Sets the contents of the cell.
    fn set(&mut self, load: impl FnOnce() -> FileResult<Vec<u8>>, f: impl FnOnce(Vec<u8>, Option<T>) -> FileResult<T>) {
        // Read and hash the file.
        let result = timed!("loading file", load());
        self.fingerprint = timed!("hashing file", hash128(&result));
        self.accessed = true;

        let value = result.and_then(|data| f(data, None));
        self.data = Some(value);
    }
}
