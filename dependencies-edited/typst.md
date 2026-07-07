[typst](https://github.com/typst/typst)
9084ffb300741e15170ec0a569fef8569e9ec4e7
- Added Sig to PDF
- Edit svg width=100%
- Allow highlight Error tag
- Added merge

> last check date = 2026-06-20
> cargo fmt --all
> cargo generate-lockfile
admin
> cargo test --workspace --test tests -- --update

## Cargo.toml

edit krilla
```diff
- krilla = { version = "0.8.0", default-features = false, features = ["raster-images", "comemo", "rayon", "pdf"] }
- krilla-svg = "0.8.0"

+ # krilla = { path = "../krilla/crates/krilla/", default-features = false, features = ["raster-images", "comemo", "rayon", "pdf"] }
+ # krilla-svg = { path = "../krilla/crates/krilla-svg/" }

+ krilla = { git = "https://codeberg.org/Marisada/krilla", default-features = false, features = ["raster-images", "comemo", "rayon", "pdf"] }
+ krilla-svg = { git = "https://codeberg.org/Marisada/krilla" }
```

## typst/crates/typst-pdf/src/lib.rs
add PdfSig public
: 17
```diff
    pub use self::metadata::{Timestamp, Timezone};
+   pub use krilla::metadata::PdfSig;
```

add merge function
: 36
```diff
    /// Export a document into a PDF file.
    ///
    /// Returns the raw bytes making up the PDF file.
    #[typst_macros::time(name = "pdf")]
    pub fn pdf(document: &PagedDocument, options: &PdfOptions) -> SourceResult<Vec<u8>> {
        convert::convert(document, options, &[], None)
    }

+    /// Merge documents into a PDF file.
+    ///
+    /// Returns the raw bytes making up the PDF file.
+    #[typst_macros::time(name = "merge")]
+    pub fn merge(documents: &[PagedDocument], options: &PdfOptions) -> SourceResult<Vec<u8>> {
+        convert::merge(documents, options)
+    }
```

add signer to PDfOptions
:72
```diff
/// Settings for PDF export.
#[derive(Debug, Hash)]
pub struct PdfOptions {
    /// If not `Smart::Auto`, shall be a string that uniquely and stably
    /// identifies the document. It should not change between compilations of
    /// the same document.  **If you cannot provide such a stable identifier,
    /// just pass `Smart::Auto` rather than trying to come up with one.** The
    /// CLI, for example, does not have a well-defined notion of a long-lived
    /// project and as such just passes `Smart::Auto`.
    ///
    /// If an `ident` is given, the hash of it will be used to create a PDF
    /// document identifier (the identifier itself is not leaked). If `ident` is
    /// `Auto`, a hash of the document's title and author is used instead (which
    /// is reasonably unique and stable).
    pub ident: Smart<String>,
    /// Configures the `/Creator` metadata in the resulting PDF. When set to
    /// `Smart::Auto`, defaults to `Typst $version`.
    pub creator: Smart<Option<String>>,
    /// If not `None`, shall be the creation timestamp of the document. It will
    /// only be used if `set document(date: ..)` is `auto`.
    pub timestamp: Option<Timestamp>,
    /// Specifies which ranges of pages should be exported in the PDF. When
    /// `None`, all pages should be exported.
    pub page_ranges: Option<PageRanges>,
    /// A list of PDF standards that Typst will enforce conformance with.
    pub standards: PdfStandards,
    /// By default, even when not producing a `PDF/UA-1` document, a tagged PDF
    /// document is written to provide a baseline of accessibility. In some
    /// circumstances, for example when trying to reduce the size of a document,
    /// it can be desirable to disable tagged PDF.
    pub tagged: bool,
    /// Whether to format the PDF in a human-readable way.
    pub pretty: bool,
+   pub signer: Option<PdfSig>,
}
```

add signer to PDfOptions's Default impl
:109
```diff
impl Default for PdfOptions<'_> {
    fn default() -> Self {
        Self {
            ident: Smart::Auto,
            creator: Smart::Auto,
            timestamp: None,
            page_ranges: None,
            standards: PdfStandards::default(),
            tagged: true,
            pretty: false,
+           signer: None,
        }
    }
}
```

## typst/crates/typst-pdf/src/convert.rs
: 88
inject PdfSig to krilla at pub fn convert()
```diff
    convert_pages(&mut gc, &mut document)?;
    attach_files(&gc, &mut document)?;
    let (doc_lang, tree) = tags::resolve(&mut gc)?;

    document.set_outline(build_outline(&gc));
    document.set_metadata(build_metadata(&gc, doc_lang));
    document.set_tag_tree(tree);
+   if let Some(sig) = &options.signer {
+       document.set_signer(sig.clone());
+   }

    finish(document, gc, options.standards.config)
```

: 177
add merge() and add_page()
```rust
// #[typst_macros::time(name = "merge document")]
pub fn merge(
    typst_documents: &[PagedDocument],
    options: &PdfOptions,
) -> SourceResult<Vec<u8>> {
    let settings = SerializeSettings {
        compress_content_streams: true,
        no_device_cs: true,
        ascii_compatible: false,
        xmp_metadata: true,
        cmyk_profile: None,
        configuration: options.standards.config,
        enable_tagging: options.tagged,
        render_svg_glyph_fn: render_svg_glyph,
        pretty: false,
    };

    let mut document = Document::new_with(settings);
    for typst_document in typst_documents {
        let page_index_converter = PageIndexConverter::new(typst_document, options);
        let named_destinations =
            collect_named_destinations(&mut document, typst_document, &[], &page_index_converter);
        let tags = tags::init(typst_document, options)?;

        let mut gc = GlobalContext::new(
            typst_document,
            options,
            None,
            named_destinations,
            page_index_converter,
            tags,
        );

        add_page(&mut gc, &mut document)?;
    }

    // let (doc_lang, tree) = tags::resolve(&mut gc)?;

    // document.set_outline(build_outline(&gc));
    // document.set_metadata(build_metadata(&gc, doc_lang));
    // document.set_tag_tree(tree);
    if let Some(sig) = &options.signer {
        document.set_signer(sig.clone());
    }

    finish_without_gc(document, options.standards.config)
}

fn add_page(gc: &mut GlobalContext, document: &mut Document) -> SourceResult<()> {
    for (i, typst_page) in gc.document.pages().iter().enumerate() {
        if gc.page_index_converter.pdf_page_index(i).is_none() {
            // Don't export this page.
            continue;
        }

        // PDF 1.4 upwards to 1.7 specifies a minimum page size of 3x3 units.
        // PDF 2.0 doesn't define an explicit limit, but krilla and probably
        // some viewers won't handle pages that have zero sized pages.
        let mut settings = PageSettings::from_wh(
            (typst_page.frame.width() + typst_page.bleed.left + typst_page.bleed.right)
                .to_f32()
                .max(3.0),
            (typst_page.frame.height() + typst_page.bleed.top + typst_page.bleed.bottom)
                .to_f32()
                .max(3.0),
        )
        .expect_internal("invalid page size")
        .at(Span::detached())?;

        if !typst_page.bleed.is_zero() {
            settings = settings.with_trim_box(Rect::from_ltrb(
                typst_page.bleed.left.to_f32(),
                typst_page.bleed.top.to_f32(),
                (typst_page.bleed.left + typst_page.frame.width()).to_f32(),
                (typst_page.bleed.top + typst_page.frame.height()).to_f32(),
            ));
        }

        // if let Some(label) = typst_page
        //     .numbering
        //     .as_ref()
        //     .and_then(|num| PageLabel::generate(num, typst_page.number))
        //     .or_else(|| {
        //         // When some pages were ignored from export, we show a page label with
        //         // the correct real (not logical) page number.
        //         // This is for consistency with normal output when pages have no numbering
        //         // and all are exported: the final PDF page numbers always correspond to
        //         // the real (not logical) page numbers. Here, the final PDF page number
        //         // will differ, but we can at least use labels to indicate what was
        //         // the corresponding real page number in the Typst document.
        //         gc.page_index_converter
        //             .has_skipped_pages()
        //             .then(|| PageLabel::arabic((i + 1) as u64))
        //     })
        // {
        //     settings = settings.with_page_label(label);
        // }

        let mut page = document.start_page_with(settings);
        let mut surface = page.surface();
        let page_idx = gc.page_index_converter.pdf_page_index(i);
        let mut fc = FrameContext::new(page_idx, typst_page.frame.size() + typst_page.bleed.sum_by_axis());

        tags::page(gc, &mut surface, |gc, surface| {
            handle_frame(
                &mut fc,
                &typst_page.frame,
                typst_page.bleed,
                typst_page.fill_or_transparent(),
                surface,
                gc,
            )
        })?;

        surface.finish();

        let link_annotations = fc.link_annotations.into_values().flatten();
        tags::add_link_annotations(gc, &mut page, link_annotations);
    }

    Ok(())
}
```

add finish_without_gc()
```rust 660
/// Finish a krilla document and handle export errors.
// #[typst_macros::time(name = "finish export without gc")]
fn finish_without_gc(
    document: Document,
    configuration: Configuration,
) -> SourceResult<Vec<u8>> {
    match document.finish() {
        Ok(r) => Ok(r),
        Err(e) => match e {
            KrillaError::Font(_f, err) => {
                bail!(
                    Span::detached(),
                    "failed to process font ({err})";
                    // display_font(gc.fonts_backward.get(&f));
                    hint: "make sure the font is valid";
                    hint: "the used font might be unsupported by Typst";
                );
            }
            KrillaError::Validation(ve) => {
                let ve_len = ve.len();
                bail!(
                    // *span,
                    Span::detached(),
                    "Validation error";
                    hint: "{ve_len} errors occur";
                );
                // let errors = ve
                //     .iter()
                //     .map(|(e, validators)| {
                //         convert_error(&gc, *validators, e, configuration.version())
                //     })
                //     .collect::<EcoVec<_>>();
                // Err(errors)
            }
            KrillaError::Image(_, loc, err) => {
                let span = to_span(loc);
                bail!(span, "failed to process image ({err})");
            }
            KrillaError::SixteenBitImage(_image, _) => {
                // let span = gc.image_to_spans.get(&image).unwrap();
                bail!(
                    // *span,
                    Span::detached(),
                    "16 bit images are not supported in this export mode";
                    hint: "convert the image to 8 bit instead";
                )
            }
            KrillaError::Pdf(_, e, loc) => {
                let span = to_span(loc);
                match e {
                    // We already validated in `typst-library` that the page index is valid.
                    PdfError::InvalidPage(_) => bail!(
                        span,
                        "invalid page number for PDF file";
                        hint: "please report this as a bug";
                    ),
                    PdfError::VersionMismatch(v) => {
                        let pdf_ver = v.as_str();
                        let config_ver = configuration.version();
                        let cur_ver = config_ver.as_str();
                        bail!(span,
                            "the version of the PDF is too high";
                            hint: "the current export target is {cur_ver}, while the PDF \
                                   has version {pdf_ver}";
                            hint: "raise the export target to {pdf_ver} or higher";
                            hint: "or preprocess the PDF to convert it to a lower version";
                        );
                    }
                }
            }
            KrillaError::DuplicateTagId(_, loc) => {
                let span = to_span(loc);
                bail!(span,
                    "duplicate tag id";
                    hint: "please report this as a bug";
                );
            }
            KrillaError::UnknownTagId(_, loc) => {
                let span = to_span(loc);
                bail!(span,
                    "unknown tag id";
                    hint: "please report this as a bug";
                );
            }
            KrillaError::DuplicateNamedDestination(_) => {
                bail!(Span::detached(),
                    "duplicate named destination";
                    hint: "please report this as a bug";
                );
            }
            KrillaError::Limit(LimitError::TooLongArray) => bail!(
                Span::detached(),
                "a PDF array is longer than 8191 elements";
                hint: "set the PDF version to PDF 1.5 or later";
                hint: "this can happen if you have a very long text in a single line";
            ),
            KrillaError::Limit(LimitError::TooLongDictionary) => bail!(
                Span::detached(),
                "a PDF dictionary has more than 4095 entries";
                hint: "set the PDF version to PDF 1.5 or later";
                hint: "alternatively, try reducing the complexity of your document";
            ),
            KrillaError::Limit(LimitError::TooLargeFloat) => bail!(
                Span::detached(),
                "a PDF floating point number is larger than the allowed limit";
                hint: "set the PDF version to PDF 1.5 or later";
            ),
        },
    }
}
```

## typst/crates/typst-svg/src/lib.rs
:448
fix fn svg_header_with_custom_attrs()
```diff
    write_custom_attrs(&mut svg);

    svg.attr_with("viewBox", |attr| {
        attr.push_nums([0.0, 0.0, size.x.to_pt(), size.y.to_pt()])
    });
-   svg.attr_with("width", |attr| {
-       attr.push_num(size.x.to_pt());
-       attr.push_str("pt");
-   });
-   svg.attr_with("height", |attr| {
-       attr.push_num(size.y.to_pt());
-       attr.push_str("pt");
-   });
+   svg.attr("width", "100%");
    svg.attr("xmlns", "http://www.w3.org/2000/svg");
```

## typst/crates/typst-cli/src/compile.rs
:614
fix fn pdf_options()
```diff
    PdfOptions {
        ident: Smart::Auto,
        creator: Smart::Auto,
        timestamp,
        page_ranges: config.pages.clone(),
        standards: config.pdf_standards.clone(),
        tagged: config.tagged,
        pretty: config.pretty,
+       signer: None,
    }
```

## typst/crates/typst-syntax/src/highlight.rs
:398
```diff
/// Highlight one source node, emitting HTML.
fn highlight_html_impl(html: &mut String, node: &LinkedNode) {
    let mut span = false;
    if let Some(tag) = highlight(node)
-       && tag != Tag::Error
    {
        span = true;
        html.push_str("<span class=\"");
        html.push_str(tag.css_class());
        html.push_str("\">");
    }
```
