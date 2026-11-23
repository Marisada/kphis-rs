[typst](https://github.com/typst/typst)
9084ffb300741e15170ec0a569fef8569e9ec4e7
- Added Sig to PDF
- Edit svg width=100%
- Allow highlight Error tag
- Added merge

> last check date = 2026-11-11
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
: 22
```diff
    pub use self::metadata::{Timestamp, Timezone};
+   pub use krilla::metadata::PdfSig;
```

add merge function
: 49
```diff
    /// Export a document into a PDF file.
    ///
    /// Returns the raw bytes making up the PDF file.
    #[typst_macros::time(name = "pdf")]
    pub fn pdf(
        document: &PagedDocument,
        options: &PdfOptions,
    ) -> Warned<SourceResult<Vec<u8>>> {
        let mut warnings = EcoVec::new();
        let output = convert::convert(document, options, &[], None, &mut warnings);
        Warned { output, warnings }
    }

+   /// Merge documents into a PDF file.
+   ///
+   /// Returns the raw bytes making up the PDF file.
+   #[typst_macros::time(name = "merge")]
+   pub fn merge(documents: &[PagedDocument], options: &PdfOptions) -> SourceResult<Vec<u8>> {
+       let mut warnings = EcoVec::new();
+       convert::merge(documents, options, &mut warnings)
+   }
```

add signer to PDfOptions
:100
```diff
    /// If not `None`, shall be the creation timestamp of the document. It will
    /// only be used if `set document(date: ..)` is `auto`.
    pub timestamp: Option<Timestamp>,
    /// Format options that override the defaults set by the document.
    pub format: PdfFormatOptions<F>,
+   pub signer: Option<PdfSig>,
}
```

add signer to PDfOptions's resolve()
:160
```diff
        Ok(PdfOptions {
            ident: self.ident.clone(),
            creator: self.creator.clone(),
            timestamp: self.timestamp,
            format,
+           signer: self.signer.clone(),
        })
    }
}
```

## typst/crates/typst-pdf/src/convert.rs
: 96
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

: 180
add merge() and add_page()
```rust
// #[typst_macros::time(name = "merge document")]
pub fn merge(
    typst_documents: &[PagedDocument],
    options: &PdfOptions,
) -> SourceResult<Vec<u8>> {
    let options = match typst_documents.first() {
        Some(typst_document) => {
            options.resolve(typst_document.options().get::<PdfFormat>(), warnings)?
        }
        None => {
            return Ok(Vec::new());
        }
    };

    let settings = SerializeSettings {
        compress_content_streams: !options.format.pretty.v,
        no_device_cs: true,
        ascii_compatible: options.format.pretty.v,
        xmp_metadata: true,
        cmyk_profile: None,
        configuration: options.format.standard.v.config,
        enable_tagging: options.tagged(),
        render_svg_glyph_fn: render_svg_glyph,
        pretty: options.format.pretty.v,
    };

    let mut document = Document::new_with(settings);
    for typst_document in typst_documents {
        let page_index_converter = PageIndexConverter::new(typst_document, &options);
        let named_destinations =
            collect_named_destinations(&mut document, typst_document, &[], &page_index_converter);
        let tags = tags::init(typst_document, &options)?;

        let mut gc = GlobalContext::new(
            typst_document,
            &options,
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

    finish_without_gc(document)
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

edit finish()
:660
```rust
            KrillaError::Limit(LimitError::TooLargeFloat) => bail!(
                Span::detached(),
                "a PDF floating point number is larger than the allowed limit";
                hint: "set the PDF version to PDF 1.5 or later";
            ),
+           KrillaError::StandardFont(_) => bail!(
+               Span::detached(),
+               "standard font error";
+               hint: "please check standard font";
+           ),
        },
    }
```

add finish_without_gc()
: 669
```rust
/// Finish a krilla document and handle export errors.
// #[typst_macros::time(name = "finish export without gc")]
fn finish_without_gc(document: Document) -> SourceResult<Vec<u8>> {
    match document.finish() {
        Ok(bytes) => Ok(bytes),
        Err(err) => match err {
            KrillaError::Font(_, err) => {
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
                    Span::detached(),
                    "Validation error";
                    hint: "{ve_len} errors occur";
                );
                // Err(ve
                //     .iter()
                //     .map(|(e, validators)| convert_validation_error(&gc, e, *validators))
                //     .collect())
                // }
            }
            KrillaError::Image(_, loc, err) => {
                let span = to_span(loc);
                bail!(span, "failed to process image ({err})");
            }
            KrillaError::SixteenBitImage(_, _) => {
                // let span = &gc.image_to_spans[&image];
                bail!(
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
                    PdfError::VersionMismatch(embedded) => {
                        let embedded = embedded.as_str();
                        // let target = gc.options.version().as_str();
                        bail!(span,
                            "the version of the PDF is too high";
                            hint: "the current export target is ??, while the PDF \
                                   has version {embedded}";
                            hint: "raise the export target to {embedded} or higher";
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
            KrillaError::StandardFont(_) => bail!(
                Span::detached(),
                "standard font error";
                hint: "please check standard font";
            ),
        },
    }
}
```

## typst/crates/typst-svg/src/lib.rs
:473
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
        format: PdfFormatOptions {
            pages: config.pages.clone().map(Some),
            standard: config.pdf_standards.clone(),
            tagged: config.tagged.map(Smart::Custom),
            pretty: config.pretty,
        },
+       signer: None,
    }
```

## typst/crates/typst-syntax/src/highlight.rs
406
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
