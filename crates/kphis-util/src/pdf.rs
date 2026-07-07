// from https://github.com/fenjalien/obsidian-typst
// used by typst_worker::runtime.rs and backend::pdf::runtime.rs

use std::str::Utf8Error;

use typst_library::{
    diag::FileResult,
    foundations::Bytes,
    text::{Font, FontBook},
};
use typst_syntax::{FileId, Source};

pub const INPUT_PATH: &str = "";
pub const DATA_PATH: &str = "data.json";

// add default fonts
pub fn start_embedded_fonts() -> (FontBook, Vec<Font>) {
    let mut book = FontBook::new();
    let mut fonts = Vec::new();

    let mut process = |bytes: &'static [u8]| {
        let buffer = Bytes::new(bytes);
        for font in Font::iter(buffer) {
            book.push(font.info().clone());
            fonts.push(font);
        }
    };

    macro_rules! add {
        ($filename:literal) => {
            process(include_bytes!(concat!("../assets/fonts/", $filename)));
        };
    }

    // Embed default fonts.
    add!("THSarabunNew.otf");
    add!("THSarabunNew-Bold.otf");
    add!("THSarabunNew-Italic.otf");
    add!("THSarabunNew-BoldItalic.otf");
    // add!("FiraMath-Regular.otf"); // #show math.equation: set text(font: "Fira Math")
    // add!("NewCMMath-Book.otf"); // #show math.equation: set text(font: "New Computer Modern Math")

    (book, fonts)
}

/// create `Source` from `id` and `bytes`<br>
/// if prev is `Some`, replace `prev` with `bytes` (`id` not change)
pub fn create_source(id: FileId, bytes: &[u8], prev: Option<Source>) -> FileResult<Source> {
    let text = decode_utf8(bytes)?;
    match prev {
        Some(mut prev) => {
            prev.replace(text);
            Ok(prev)
        }
        None => Ok(Source::new(id, text.into())),
    }
}

fn decode_utf8(buf: &[u8]) -> Result<&str, Utf8Error> {
    // The UTF-8 representation of the BOM is the (hexadecimal) byte sequence EF BB BF
    // Remove UTF-8 BOM (0xEF, 0xBB, 0xBF)
    std::str::from_utf8(buf.strip_prefix(b"\xef\xbb\xbf").unwrap_or(buf))
}

#[cfg(test)]
#[rustfmt::skip]
pub mod tests {

    use std::num::NonZero;
    use super::*;

    #[test]
    fn test_create_source() {
        let old_id = FileId::from_raw(NonZero::new(1u16).unwrap());
        let new_id = FileId::from_raw(NonZero::new(2u16).unwrap());
        let prev = Source::new(old_id, String::from("OLD"));

        let new_source = create_source(new_id, b"NEW", None).unwrap();
        assert_eq!(new_source.id(), new_id);
        assert_eq!(new_source.text(), "NEW");

        let prev_source = create_source(new_id, b"NEW", Some(prev)).unwrap();
        assert_eq!(prev_source.id(), old_id);
        assert_eq!(prev_source.text(), "NEW");
    }

    #[test]
    fn test_decode_utf8() {
        assert_eq!(decode_utf8(b"\xef\xbb\xbf_TEST").unwrap(), "_TEST");
        assert_eq!(decode_utf8(b"TEST").unwrap(), "TEST");
    }
}
