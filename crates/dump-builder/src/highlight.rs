use std::io::Cursor;
use syntect::{
    highlighting::{Theme, ThemeSet},
    parsing::{SyntaxSet, SyntaxSetBuilder, syntax_definition::SyntaxDefinition},
};

pub(crate) fn create_syntax_set() -> SyntaxSet {
    // syntect still not support ST4, use latest ST3 at https://github.com/sublimehq/Packages/releases/tag/v3211
    // [] https://github.com/sublimehq/Packages/blob/master/SQL/MySQL.sublime-syntax
    // [] https://github.com/sublimehq/Packages/blob/fa6b8629c95041bf262d4c1dab95c456a0530122/SQL/SQL.sublime-syntax
    // [x] https://github.com/sublimehq/Packages/releases/tag/v3211
    let read_mysql_def = include_str!("../raw-highlight/SQL.sublime-syntax");
    let mysql_def = SyntaxDefinition::load_from_str(&read_mysql_def, true, None).unwrap();
    // [] https://github.com/sublimehq/Packages/blob/master/JSON/JSON.sublime-syntax
    // [x] https://github.com/trishume/syntect/blob/master/testdata/JSON.sublime-syntax
    // [] https://github.com/sublimehq/Packages/releases/tag/v3211
    let read_json_def = include_str!("../raw-highlight/JSON.sublime-syntax");
    let json_def = SyntaxDefinition::load_from_str(&read_json_def, true, None).unwrap();
    let mut builder = SyntaxSetBuilder::new();
    builder.add(mysql_def);
    builder.add(json_def);
    builder.build()
}

pub(crate) fn create_theme() -> Theme {
    // // https://github.com/narze/Monokai-Dark.tmTheme/blob/master/Monokai%20Dark.tmTheme
    // let mut reader = Cursor::new(include_str!("Monokai-Dark.tmTheme").as_bytes());
    // let mut reader = Cursor::new(include_str!("base16-mocha.dark.tmTheme").as_bytes());
    // let mut reader = Cursor::new(include_str!("base16-eighties.dark.tmTheme").as_bytes());
    // let mut reader = Cursor::new(include_str!("base16-mocha.dark.tmTheme").as_bytes());
    let read_bytes = include_bytes!("../raw-highlight/My.tmTheme");
    let mut reader = Cursor::new(read_bytes);

    ThemeSet::load_from_reader(&mut reader).unwrap()
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use std::io::Read;
    use syntect::dumps;
    use super::*;

    #[test]
    fn test_dump_syntax_set() {
        let ss = create_syntax_set();
        let path = "../kphis-ui-core/src/highlight/syntax_set.dump";
        dumps::dump_to_file(&ss, path).unwrap();

        let mut read_file = std::fs::File::open(path).unwrap();
        let mut read_bytes = Vec::new();
        read_file.read_to_end(&mut read_bytes).unwrap();
        let ss2: SyntaxSet = dumps::from_binary(&read_bytes);

        assert_eq!(ss.syntaxes().len(), ss2.syntaxes().len());
    }

    #[test]
    fn test_dump_theme() {
        let theme = create_theme();
        let path = "../kphis-ui-core/src/highlight/theme.dump";
        dumps::dump_to_file(&theme, path).unwrap();

        let mut read_file = std::fs::File::open(path).unwrap();
        let mut read_bytes = Vec::new();
        read_file.read_to_end(&mut read_bytes).unwrap();
        let theme2: Theme = dumps::from_binary(&read_bytes);

        assert_eq!(theme.scopes.len(), theme2.scopes.len());
    }
}
