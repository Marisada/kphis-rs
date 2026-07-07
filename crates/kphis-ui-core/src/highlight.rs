pub mod asset;

use js_sys::JSON;
use syntect::{
    Error,
    parsing::{SyntaxReference, SyntaxSet},
    util::LinesWithEndings,
};
use typst_syntax::Tag;
use wasm_bindgen::JsValue;

use asset::{SYNTAX_SET, THEME};

pub fn json_pretty(json: &str) -> String {
    JSON::parse(json)
        .and_then(|v| JSON::stringify_with_replacer_and_space(&v, &JsValue::NULL, &JsValue::from(4)))
        .ok()
        .and_then(|js| js.as_string())
        .unwrap_or(json.to_owned())
}

pub fn highlight_stylesheet_typ() -> String {
    let mut css = String::new();
    for tag in Tag::LIST {
        let line = match tag {
            Tag::Comment => ".typ-comment {color: #b0b3c2}\n",
            Tag::Punctuation => ".typ-punct {color: #ffd700}\n",
            Tag::Escape => ".typ-escape {color: #cc7700}\n",
            Tag::Strong => ".typ-strong {font-weight: 700}\n",
            Tag::Emph => ".typ-emph {font-style: italic}\n",
            Tag::Link => ".typ-link {text-decoration: underline}\n",
            Tag::Raw => ".typ-raw {color: #caccd6}\n",
            Tag::Label => ".typ-label {color: #d9f8f4}\n",
            Tag::Ref => ".typ-ref {color: #ccffff}\n",
            Tag::Heading => ".typ-heading {font-weight: 700;text-decoration: underline}\n",
            Tag::ListMarker => ".typ-marker {color: #caa8ff}\n",
            Tag::ListTerm => ".typ-term {font-weight: 700}\n",
            Tag::MathDelimiter => ".typ-math-delim {color: #33cc33}\n",
            Tag::MathOperator => ".typ-math-op {color: #ffcccc}\n",
            Tag::Keyword => ".typ-key {color: #ffa49d}\n",
            Tag::Operator => ".typ-op {color: #ff5577}\n",
            Tag::Number => ".typ-num {color: #ff7d79}\n",
            Tag::String => ".typ-str {color: #80f4b6}\n",
            Tag::Function => ".typ-func {color: #7cd5ff}\n",
            Tag::Interpolated => ".typ-pol {color: #caa8ff}\n",
            Tag::Error => ".typ-error {color: #f8f8f0;background-color: #f92672}\n",
            Tag::MathGroupingParens => "typ-math-group {color: #caa8cc}\n",
        };
        css.push_str(line);
    }
    css
}

// //=======================//
// // style sheet version   //
// // - not show error      //
// // - not nesting element //
// //=======================//

// use syntect::{
//     easy::HighlightLines,
//     highlighting::Theme,
//     html::{IncludeBackground, append_highlighted_html_for_styled_line},
// };

// pub fn highlight_sql(code: &str) -> String {
//     let syntax_ref = SYNTAX_SET.find_syntax_by_name("SQL").unwrap();
//     highlighted_html_for_string(code, &SYNTAX_SET, syntax_ref, &THEME).unwrap_or(code.to_owned())
// }

// pub fn highlight_json(code: &str) -> String {
//     let syntax_ref = SYNTAX_SET.find_syntax_by_name("JSON").unwrap();
//     highlighted_html_for_string(code, &SYNTAX_SET, syntax_ref, &THEME).unwrap_or(code.to_owned())
// }

// // modify from https://github.com/trishume/syntect/blob/master/src/html.rs
// // change <pre> to <code>
// fn highlighted_html_for_string(
//     s: &str,
//     syntax_set: &SyntaxSet,
//     syntax: &SyntaxReference,
//     theme: &Theme,
// ) -> Result<String, Error> {
//     let mut highlighter = HighlightLines::new(syntax, theme);
//     let mut output = String::from("<code>");
//     for line in LinesWithEndings::from(s) {
//         let regions = highlighter.highlight_line(line, syntax_set)?;
//         append_highlighted_html_for_styled_line(
//             &regions[..],
//             IncludeBackground::No,
//             &mut output,
//         )?;
//     }
//     output.push_str("</code>\n");
//     Ok(output)
// }

use syntect::html::{ClassStyle, ClassedHTMLGenerator, css_for_theme_with_class_style};

pub fn highlight_sql(code: &str) -> String {
    let syntax_ref = SYNTAX_SET.find_syntax_by_name("SQL").unwrap();
    highlighted_html_for_string(code, &SYNTAX_SET, syntax_ref).unwrap_or(code.to_owned())
}

pub fn highlight_json(code: &str) -> String {
    let syntax_ref = SYNTAX_SET.find_syntax_by_name("JSON").unwrap();
    highlighted_html_for_string(code, &SYNTAX_SET, syntax_ref).unwrap_or(code.to_owned())
}

// modify from https://github.com/trishume/syntect/blob/master/src/html.rs
// change <pre> to <code>
// css version, generate nested span for one char
fn highlighted_html_for_string(s: &str, syntax_set: &SyntaxSet, syntax: &SyntaxReference) -> Result<String, Error> {
    let mut html_generator = ClassedHTMLGenerator::new_with_class_style(syntax, &syntax_set, ClassStyle::SpacedPrefixed { prefix: "tm-" });
    for line in LinesWithEndings::from(s) {
        html_generator.parse_html_for_line_which_includes_newline(line)?;
    }
    let output = ["<code>", &html_generator.finalize(), "</code>"].concat();
    Ok(output)
}

pub fn highlight_stylesheet() -> String {
    css_for_theme_with_class_style(&THEME, ClassStyle::SpacedPrefixed { prefix: "tm-" }).unwrap()
}
