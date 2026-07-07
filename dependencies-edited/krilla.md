[krilla](https://github.com/LaurenzV/krilla)
> last check date = 2026-06-20

open shell
> docker pull vallaris/sitro-backends
> git clone https://github.com/LaurenzV/krilla.git
> git clone https://github.com/Marisada/krilla.git
run on both git
> cargo test -p krilla-tests --features visreg -- --nocapture
expected the same failures on both git:
failures:
    font::colr::font_noto_color_emoji_colr_visreg_poppler
    font::svg::font_noto_color_emoji_svg_visreg_mupdf
    font::svg::font_noto_color_emoji_svg_visreg_poppler
    svg::custom_integration_matplotlib_stairs_visreg_pdfium
    svg::resvg_masking_clip_rule_clip_rule_evenodd_visreg_poppler

- Add PdfSig

## crates/krilla/src/interchange/metadata.rs
add
```rust
#[allow(missing_docs)]
#[derive(Clone, Debug, Hash)]
pub struct PdfSig {
    pub name: String,
    pub location: String,
    pub reason: String,
    pub contact_info: String,
}
```

## crates/krilla/src/serialize.rs
add: 26
```diff
    use crate::interchange::embed::EmbeddedFile;
+   use crate::interchange::metadata::PdfSig;
    use crate::interchange::outline::Outline;
```

add to pub(crate) struct SerializeContext : 275
```diff

    /// The current location, if set.
    pub(crate) location: Option<Location>,
+   pub(crate) signer: Option<PdfSig>,
}

impl SerializeContext {
    pub(crate) fn new(mut serialize_settings: SerializeSettings) -> Self {

        ..

        Self {
            cached_mappings: HashMap::new(),
            pdf2_ns,
            global_objects: GlobalObjects::default(),
            cur_ref,
            page_tree_ref,
            page_infos: vec![],
            location: None,
            validation_errors: vec![],
            serialize_settings: Arc::new(serialize_settings),
            chunk_settings,
            limits: Limits::new(),
            validation_store: ValidationStore::new(),
+           signer: None,
        }
    }

+   pub(crate) fn set_signer(&mut self, sig: PdfSig) {
+       self.signer = Some(sig)
+   }
```

## crates/krilla/src/documents.rs

edit :21
```diff
- use crate::interchange::metadata::Metadata;
+ use crate::interchange::metadata::{Metadata, PdfSig};
```

add : 113
```rust
    /// Set the tag tree of the document.
    pub fn set_tag_tree(&mut self, tag_tree: TagTree) {
        self.serializer_context.set_tag_tree(tag_tree);
    }

+   /// Set the Signer of the document.
+   pub fn set_signer(&mut self, sig: PdfSig) {
+       self.serializer_context.set_signer(sig);
+   }
```

## crates/krilla/src/chunk_container.rs

edit :8
```diff
- use crate::interchange::metadata::Metadata;
+ use crate::interchange::metadata::{Metadata, pdf_date};
```

replace `catalog.finish();` in pub(crate) fn finish(self, sc: &mut SerializeContext) -> KrillaResult<Pdf> : 344
from
```rust
            catalog.finish();
```
to
```rust
            // we create a placeholder for Contents and ByteRange here
            // then we will post-processing (after write to PDF binary) later.
            // post-processing includes
            // 1. update ByteRange to match actual signature content position then
            // 2. fill Contents with digest from 2 parts of bytes concatenated (not included '<' and '>')
            //   2.1 from BOF to before '<BEEFFACE00..00>'
            //   2.2 after '<BEEFFACE00..00>' to EOF
            // *Note*: 'BEEFFACE' and '88888888' just hex text for seeking position only
            // *NOTE*: please use the same Contents length in post-processing function
            if let (Some(sig), Some(date_pdf), Some(pt)) = (sc.signer.as_ref(), self.metadata.as_ref().and_then(|meta| meta.creation_date), &self.non_stream.page_tree) {

                let widget_id = remapped_ref.bump();
                let sig_id = remapped_ref.bump();

                // we need signature Contents from [cryptographic_message_syntax](https://github.com/indygreg/cryptography-rs)
                // to overwrite 'BEEFFACE00..00' later
                // cryptographic_message_syntax::signing::SignedDataBuilder::build_der() will return Vec<u8>
                // - rsa:4096 sha256: ~2,000 bytes
                // - timestamp: ~5,500 bytes
                // so 'BEEFFACE00..00' length should be >10,000 bytes (>20,000 hex string chars)
                // pdf_writer will generate '<BEEFFACE00..00>' from [190,239,250,206,0,0,..,0,0]
                let mut sig_contents = [0u8; 11110];
                sig_contents[0] = 190; // BE
                sig_contents[1] = 239; // EF
                sig_contents[2] = 250; // FA
                sig_contents[3] = 206; // CE

                catalog.insert(Name(b"Perms")).dict().pair(Name(b"DocMDP"), sig_id);

                let mut acro_form = catalog.insert(Name(b"AcroForm")).dict();
                acro_form
                    .pair(Name(b"SigFlags"), 3)
                    .insert(Name(b"Fields"))
                    .array()
                    .item(widget_id);
                acro_form.finish();
                catalog.finish();

                pdf.indirect(widget_id)
                    .dict()
                    .pair(Name(b"F"), 130)
                    .pair(Name(b"Type"), Name(b"Annot"))
                    .pair(Name(b"SubType"), Name(b"Widget"))
                    .pair(Name(b"Rect"), pdf_writer::Rect::new(0.0, 0.0, 0.0, 0.0))
                    .pair(Name(b"FT"), Name(b"Sig"))
                    .pair(Name(b"V"), sig_id)
                    .pair(Name(b"T"), TextStr("Signature"))
                    .pair(Name(b"P"), pt.0);

                pdf.indirect(sig_id)
                    .dict()
                    .pair(Name(b"Type"), Name(b"Sig"))
                    .pair(Name(b"Filter"), Name(b"Adobe.PPKLite"))
                    .pair(Name(b"SubFilter"), Name(b"adbe.pkcs7.detached"))
                    .pair(Name(b"M"), pdf_date(date_pdf.to_owned()))
                    .pair(Name(b"Name"), TextStr(sig.name.as_str()))
                    .pair(Name(b"Location"), TextStr(sig.location.as_str()))
                    .pair(Name(b"Reason"), TextStr(sig.reason.as_str()))
                    .pair(Name(b"ContactInfo"), TextStr(sig.contact_info.as_str()))
                    .pair(Name(b"Contents"), Str(&sig_contents))
                    // we prepare 37 chars placeholder for ByteRange '[0 x x x]'
                    // so max unit is '[0 0123456789 0123456789a 0123456789]'
                    .pair(
                        Name(b"ByteRange"),
                        pdf_writer::Rect::new(88888888.0, 88888888.0, 88888888.0, 88888888.0),
                    )
                    .insert(Name(b"Reference"))
                    .array()
                    .push()
                    .dict()
                    .pair(Name(b"Type"), Name(b"SigRef"))
                    .pair(Name(b"Data"), catalog_ref)
                    .pair(Name(b"TransformMethod"), Name(b"DocMDP"))
                    .insert(Name(b"TransformParams"))
                    .dict()
                    .pair(Name(b"Type"), Name(b"TransformParams"))
                    .pair(Name(b"V"), Name(b"1.2"))
                    .pair(Name(b"P"), 1);
            } else {
                catalog.finish();
            }
```
