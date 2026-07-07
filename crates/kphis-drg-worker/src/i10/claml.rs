use std::{collections::BTreeMap, sync::Arc};

use kphis_util::{
    british_american::TRANSLATOR,
    util::{first_char_uppercase, sanity_space},
};

pub const PROPOSITIONS: [&str; 2] = ["with", "without"];

#[derive(Clone, Debug, bitcode::Encode, bitcode::Decode)]
pub struct I10Detail {
    /// - kind = chapter: roman number ex. "IX"
    /// - kind = block: range ex. "A00-A09"
    /// - kind = category: full icd10 ex. "Q50.6"
    pub code: String,
    // pub kind: ClassKind,
    pub usage: Option<UsageKind>,
    /// full icd10
    pub superclass: Option<String>,
    /// (full icd10, PreferredLong or Preferred Rubric)
    pub subclasses: Vec<(String, Vec<Rubric>)>,
    /// Modifier code NEED to apply as SubClass
    pub modified_by: Option<ModifierDetail>,
    /// Modifier code NEED to apply as SubClass
    pub sub_modifier: Option<ModifierDetail>,
    /// PreferedLong or Preferred
    pub r_prefered: Vec<Rubric>,
    /// Definition
    pub r_definitions: Vec<Rubric>,
    /// Text
    pub r_texts: Vec<Rubric>,
    /// Inclusion
    pub r_inclusions: Vec<Rubric>,
    /// Exclusion
    pub r_exclusions: Vec<Rubric>,
    /// CodingHint
    pub r_coding_hints: Vec<Rubric>,
    /// Note
    pub r_notes: Vec<Rubric>,
    /// FootNote
    pub r_foot_notes: Vec<Rubric>,
}

#[derive(Clone, Debug, bitcode::Encode, bitcode::Decode)]
pub struct ModifierDetail {
    pub code: String,
    /// (full icd10, PreferredLong or Preferred Rubric)
    pub subclasses: Vec<(String, Vec<Rubric>)>,
    /// (rubric_id, Rubric)
    pub rubrics: BTreeMap<String, Rubric>,
}

#[derive(Debug, Default)]
pub struct State {
    pub parse_at: ParseAt,
    // (code, kind)
    pub reference: Option<(Option<String>, Option<UsageKind>)>,
    pub asset: I10Claml,
}

#[derive(Clone, Debug, Default, bitcode::Encode, bitcode::Decode)]
pub struct I10Claml {
    /// (code, Modifier)
    pub modifiers: BTreeMap<String, Modifier>,
    /// (modifier|code, ModifierClass)
    pub modifier_classes: BTreeMap<String, ModifierClass>,
    /// (code, Class)
    pub classes: BTreeMap<String, Class>,
}

impl I10Claml {
    pub fn new() -> Self {
        let bytes = include_bytes!("../../dump/i10-claml.dump");
        bitcode::decode(bytes).expect("Cannot decode I10_DETAIL binary")
    }

    pub fn get_detail(&self, code: &str) -> Option<Arc<I10Detail>> {
        self.classes.get(code).map(|class| {
            // normal subclass
            let subclasses = class
                .subclasses
                .iter()
                .map(|sub_code| {
                    let sub = self.get_class_preferred_rubric(sub_code);
                    (sub_code.to_owned(), sub)
                })
                .collect();
            // subclass of last heirarchy
            let modified_by = class
                .modified_by_1
                .as_ref()
                .and_then(|modifier_code| self.get_modifier_detail(modifier_code, code))
                .or(class.modified_by_2.as_ref().and_then(|modifier_code| self.get_modifier_detail(modifier_code, code)));
            let sub_modifier = class.sub_modifier.as_ref().and_then(|modifier_code| self.get_modifier_detail(modifier_code, code));
            let mut r_prefered = Vec::new();
            let mut v_prefered = Vec::new();
            let mut r_definitions = Vec::new();
            let mut r_texts = Vec::new();
            let mut r_inclusions = Vec::new();
            let mut r_exclusions = Vec::new();
            let mut r_coding_hints = Vec::new();
            let mut r_notes = Vec::new();
            let mut r_foot_notes = Vec::new();
            // BTreeMap key is sorted
            // 1.1 class rubrics
            for (_, rubric) in class.rubrics.iter() {
                match rubric.kind {
                    RubricKind::Footnote => r_foot_notes.push(rubric.to_owned()),
                    RubricKind::Text => r_texts.push(rubric.to_owned()),
                    RubricKind::CodingHint => r_coding_hints.push(rubric.to_owned()),
                    RubricKind::Definition => r_definitions.push(rubric.to_owned()),
                    RubricKind::Introduction => {}
                    RubricKind::Modifierlink => {}
                    RubricKind::Note => r_notes.push(rubric.to_owned()),
                    RubricKind::Exclusion => r_exclusions.push(rubric.to_owned()),
                    RubricKind::Inclusion => r_inclusions.push(rubric.to_owned()),
                    RubricKind::PreferredLong => v_prefered.push(rubric.to_owned()),
                    RubricKind::Preferred => v_prefered.push(rubric.to_owned()),
                }
            }
            // 1.2 without_modifier_class (un-modified class) rubrics
            if let Some(unmodified_class) = class.without_modifier_class.as_ref().and_then(|class_code| self.classes.get(class_code)) {
                for (_, rubric) in unmodified_class.rubrics.iter() {
                    match rubric.kind {
                        RubricKind::Footnote => r_foot_notes.push(rubric.to_owned()),
                        RubricKind::Text => r_texts.push(rubric.to_owned()),
                        RubricKind::CodingHint => r_coding_hints.push(rubric.to_owned()),
                        RubricKind::Definition => r_definitions.push(rubric.to_owned()),
                        RubricKind::Introduction => {}
                        RubricKind::Modifierlink => {}
                        RubricKind::Note => r_notes.push(rubric.to_owned()),
                        RubricKind::Exclusion => r_exclusions.push(rubric.to_owned()),
                        RubricKind::Inclusion => r_inclusions.push(rubric.to_owned()),
                        RubricKind::PreferredLong => v_prefered.push(rubric.to_owned()),
                        RubricKind::Preferred => v_prefered.push(rubric.to_owned()),
                    }
                }
            }
            // 1.3 collect un-modified preferred + clear preferred
            if let Some(preferred) = v_prefered
                .iter()
                .find(|r| matches!(r.kind, RubricKind::PreferredLong))
                .or(v_prefered.iter().find(|r| matches!(r.kind, RubricKind::Preferred)))
                .cloned()
            {
                r_prefered.push(preferred);
            }
            v_prefered.clear();
            // 2.1 with_modifier_class_1
            if let Some(modifier_class_1) = class.with_modifier_class_1.as_ref().and_then(|modifier_class_code| self.modifier_classes.get(modifier_class_code)) {
                for (_, rubric) in modifier_class_1.rubrics.iter() {
                    match rubric.kind {
                        RubricKind::Footnote => r_foot_notes.push(rubric.to_owned()),
                        RubricKind::Text => r_texts.push(rubric.to_owned()),
                        RubricKind::CodingHint => r_coding_hints.push(rubric.to_owned()),
                        RubricKind::Definition => r_definitions.push(rubric.to_owned()),
                        RubricKind::Introduction => {}
                        RubricKind::Modifierlink => {}
                        RubricKind::Note => r_notes.push(rubric.to_owned()),
                        RubricKind::Exclusion => r_exclusions.push(rubric.to_owned()),
                        RubricKind::Inclusion => r_inclusions.push(rubric.to_owned()),
                        RubricKind::PreferredLong => v_prefered.push(rubric.to_owned()),
                        RubricKind::Preferred => v_prefered.push(rubric.to_owned()),
                    }
                }
            }
            // 2.2. collect modifier_1 preferred + clear preferred
            if let Some(preferred) = v_prefered
                .iter()
                .find(|r| matches!(r.kind, RubricKind::PreferredLong))
                .or(v_prefered.iter().find(|r| matches!(r.kind, RubricKind::Preferred)))
                .cloned()
            {
                r_prefered.push(preferred);
            }
            v_prefered.clear();
            // 3.1 with_modifier_class_2
            if let Some(modifier_class_2) = class.with_modifier_class_2.as_ref().and_then(|modifier_class_code| self.modifier_classes.get(modifier_class_code)) {
                for (_, rubric) in modifier_class_2.rubrics.iter() {
                    match rubric.kind {
                        RubricKind::Footnote => r_foot_notes.push(rubric.to_owned()),
                        RubricKind::Text => r_texts.push(rubric.to_owned()),
                        RubricKind::CodingHint => r_coding_hints.push(rubric.to_owned()),
                        RubricKind::Definition => r_definitions.push(rubric.to_owned()),
                        RubricKind::Introduction => {}
                        RubricKind::Modifierlink => {}
                        RubricKind::Note => r_notes.push(rubric.to_owned()),
                        RubricKind::Exclusion => r_exclusions.push(rubric.to_owned()),
                        RubricKind::Inclusion => r_inclusions.push(rubric.to_owned()),
                        RubricKind::PreferredLong => v_prefered.push(rubric.to_owned()),
                        RubricKind::Preferred => v_prefered.push(rubric.to_owned()),
                    }
                }
            }
            // 3.2. collect modifier_1 preferred + clear preferred
            if let Some(preferred) = v_prefered
                .iter()
                .find(|r| matches!(r.kind, RubricKind::PreferredLong))
                .or(v_prefered.iter().find(|r| matches!(r.kind, RubricKind::Preferred)))
                .cloned()
            {
                r_prefered.push(preferred);
            }

            Arc::new(I10Detail {
                code: class.code.to_owned(),
                // kind: class.kind.to_owned(),
                usage: class.usage.to_owned(),
                superclass: class.superclass.to_owned(),
                subclasses,
                modified_by,
                sub_modifier,
                r_prefered,
                r_definitions,
                r_texts,
                r_inclusions,
                r_exclusions,
                r_coding_hints,
                r_notes,
                r_foot_notes,
            })
        })
    }

    fn get_class_preferred_rubric(&self, class_code: &str) -> Vec<Rubric> {
        let mut results = Vec::new();
        // modified class
        if let Some(class) = self.classes.get(class_code) {
            if let Some(unmodified_class) = class.without_modifier_class.as_ref().and_then(|unmodified_class_code| self.classes.get(unmodified_class_code)) {
                if let Some((_, preferred)) = unmodified_class
                    .rubrics
                    .iter()
                    .find(|(_, r)| matches!(r.kind, RubricKind::PreferredLong))
                    .or(unmodified_class.rubrics.iter().find(|(_, r)| matches!(r.kind, RubricKind::Preferred)))
                {
                    results.push(preferred.to_owned())
                }
                if let Some(modifier_class_1) = class.with_modifier_class_1.as_ref().and_then(|modifier_class_1_code| self.modifier_classes.get(modifier_class_1_code)) {
                    if let Some((_, preferred)) = modifier_class_1
                        .rubrics
                        .iter()
                        .find(|(_, r)| matches!(r.kind, RubricKind::PreferredLong))
                        .or(modifier_class_1.rubrics.iter().find(|(_, r)| matches!(r.kind, RubricKind::Preferred)))
                    {
                        results.push(preferred.to_owned())
                    }
                }
                if let Some(modifier_class_2) = class.with_modifier_class_2.as_ref().and_then(|modifier_class_2_code| self.modifier_classes.get(modifier_class_2_code)) {
                    if let Some((_, preferred)) = modifier_class_2
                        .rubrics
                        .iter()
                        .find(|(_, r)| matches!(r.kind, RubricKind::PreferredLong))
                        .or(modifier_class_2.rubrics.iter().find(|(_, r)| matches!(r.kind, RubricKind::Preferred)))
                    {
                        results.push(preferred.to_owned())
                    }
                }
            // normal class
            } else if let Some((_, preferred)) = class
                .rubrics
                .iter()
                .find(|(_, r)| matches!(r.kind, RubricKind::PreferredLong))
                .or(class.rubrics.iter().find(|(_, r)| matches!(r.kind, RubricKind::Preferred)))
            {
                results.push(preferred.to_owned())
            }
        }

        results
    }

    fn get_modifier_detail(&self, modifier_code: &str, class_code: &str) -> Option<ModifierDetail> {
        self.modifiers.get(modifier_code).map(|modifier| {
            let mod_subclasses = modifier
                .subclasses
                .iter()
                .map(|sub_code| {
                    let code = match (class_code.contains('.'), sub_code.starts_with('.')) {
                        (true, true) => [class_code, sub_code.trim_start_matches('.')].concat(),
                        (false, false) => [class_code, ".", sub_code].concat(),
                        (_, _) => [class_code, sub_code].concat(),
                    };
                    let sub = self.get_class_preferred_rubric(&code);
                    (code, sub)
                })
                .collect();

            ModifierDetail {
                code: modifier.code.to_owned(),
                subclasses: mod_subclasses,
                rubrics: modifier.rubrics.to_owned(),
            }
        })
    }
}

#[derive(Clone, Debug, bitcode::Encode, bitcode::Decode)]
pub enum ClassKind {
    Category,
    Block,
    Chapter,
}

impl ClassKind {
    pub fn new(kind: &str) -> Option<Self> {
        match kind {
            "category" => Some(Self::Category),
            "block" => Some(Self::Block),
            "chapter" => Some(Self::Chapter),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, bitcode::Encode, bitcode::Decode, PartialEq)]
pub enum UsageKind {
    Aster,
    Dagger,
}

impl UsageKind {
    pub fn new(kind: &str) -> Option<Self> {
        match kind {
            "aster" => Some(Self::Aster),
            "dagger" => Some(Self::Dagger),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, bitcode::Encode, bitcode::Decode)]
pub enum RubricKind {
    Footnote,
    Text,
    CodingHint,
    Definition,
    /// only use in chapter and block, NEED MORE TAG PARSOR
    Introduction,
    /// contains Reference with text, read parent's ModifiedBy for key
    Modifierlink,
    Note,
    Exclusion,
    Inclusion,
    /// Preferred + Parent detail
    PreferredLong,
    Preferred,
}

impl RubricKind {
    pub fn new(kind: &str) -> Option<Self> {
        match kind {
            "footnote" => Some(Self::Footnote),
            "text" => Some(Self::Text),
            "coding-hint" => Some(Self::CodingHint),
            "definition" => Some(Self::Definition),
            "introduction" => Some(Self::Introduction),
            "modifierlink" => Some(Self::Modifierlink),
            "note" => Some(Self::Note),
            "exclusion" => Some(Self::Exclusion),
            "inclusion" => Some(Self::Inclusion),
            "preferredLong" => Some(Self::PreferredLong),
            "preferred" => Some(Self::Preferred),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ParseAt {
    /// (code, rubric_id)
    Modifier(String, Option<String>),
    /// (modifier|code, rubric_id)
    ModifierClass(String, Option<String>),
    /// (code, rubric_id)
    Class(String, Option<String>),
    #[default]
    Others,
}

impl ParseAt {
    pub fn new_modifier(code: &str) -> Self {
        Self::Modifier(code.to_owned(), None)
    }

    pub fn new_modifier_class(concat: String) -> Self {
        Self::ModifierClass(concat, None)
    }

    pub fn new_class(code: &str) -> Self {
        Self::Class(code.to_owned(), None)
    }

    pub fn set_rubric_id(&mut self, id: &str) {
        match self {
            ParseAt::Modifier(_, rubric_id) => {
                rubric_id.replace(id.to_owned());
            }
            ParseAt::ModifierClass(_, rubric_id) => {
                rubric_id.replace(id.to_owned());
            }
            ParseAt::Class(_, rubric_id) => {
                rubric_id.replace(id.to_owned());
            }
            ParseAt::Others => {}
        }
    }

    pub fn clear_rubric_id(&mut self) {
        match self {
            ParseAt::Modifier(_, rubric_id) => {
                rubric_id.take();
            }
            ParseAt::ModifierClass(_, rubric_id) => {
                rubric_id.take();
            }
            ParseAt::Class(_, rubric_id) => {
                rubric_id.take();
            }
            ParseAt::Others => {}
        }
    }
}

/// - `Modifier`[code]
/// - `Modifier` - `SubClass`[code]
/// - `Modifier` - *`Rubric`[kind] - `Label` - *`Para` - Charactor
#[derive(Clone, Debug, bitcode::Encode, bitcode::Decode)]
pub struct Modifier {
    pub code: String,
    /// "0"-"9" or ".0" - ".9"
    pub subclasses: Vec<String>,
    /// (rubric_id, Rubric)
    pub rubrics: BTreeMap<String, Rubric>,
}

impl Modifier {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_owned(),
            subclasses: Vec::new(),
            rubrics: BTreeMap::new(),
        }
    }

    pub fn add_subclass(&mut self, code: &str) {
        self.subclasses.push(code.to_owned());
    }

    pub fn add_rubric(&mut self, rubric_id: &str, rubric: Rubric) {
        self.rubrics.insert(rubric_id.to_owned(), rubric);
    }

    pub fn get_rubric(&mut self, rubric_id: &str) -> Option<&mut Rubric> {
        self.rubrics.get_mut(rubric_id)
    }
}

/// - `ModifierClass`[code][modifier][usage]
/// - `ModifierClass` - `SuperClass`[code]
/// - `ModifierClass` - *`Rubric`[kind] - `Label` - Charactor
/// - `ModifierClass` - *`Rubric`[kind] - `Label` - `Reference`[usage] - Charactor
/// - `ModifierClass` - *`Rubric`[kind] - `Label` - *`Fragment`[type=list/item][usage] - Charactor
/// - `ModifierClass` - *`Rubric`[kind] - `Label` - *`Fragment`[type=list/item][usage] - `Reference`[usage] - Charactor
#[derive(Clone, Debug, bitcode::Encode, bitcode::Decode)]
pub struct ModifierClass {
    /// "0"-"9" or ".0" - ".9"
    pub code: String,
    /// Modifier code
    pub modifier: String,
    pub usage: Option<UsageKind>,
    // same as modifier above (we already test it)
    pub superclass: Option<String>,
    /// (rubric_id, Rubric)
    pub rubrics: BTreeMap<String, Rubric>,
}

impl ModifierClass {
    pub fn new(code: &str, modifier: &str, usage: Option<UsageKind>) -> Self {
        Self {
            code: code.to_owned(),
            modifier: modifier.to_owned(),
            usage,
            superclass: None,
            rubrics: BTreeMap::new(),
        }
    }

    pub fn set_superclass(&mut self, code: &str) {
        if self.superclass.is_some() {
            panic!("Cannot set 2nd SuperClass at {}", self.code);
        }
        self.superclass.replace(code.to_owned());
    }

    pub fn add_rubric(&mut self, rubric_id: &str, rubric: Rubric) {
        self.rubrics.insert(rubric_id.to_owned(), rubric);
    }

    pub fn get_rubric(&mut self, rubric_id: &str) -> Option<&mut Rubric> {
        self.rubrics.get_mut(rubric_id)
    }
}

/// - `Class`[code][kind][usage]
/// - `Class` - `SuperClass`[code]
/// - `Class` - `SubClass`[code]
/// - `Class` - *`Rubric`[kind] - `Label` - Charactor
/// - `Class` - *`Rubric`[kind] - `Label` - `Reference`[usage] - Charactor
/// - `Class` - *`Rubric`[kind] - `Label` - `*Fragment`[type=list/item][usage] - Charactor
/// - `Class` - *`Rubric`[kind] - `Label` - `*Fragment`[type=list/item][usage] - `Reference`[usage] - Charactor
#[derive(Clone, Debug, bitcode::Encode, bitcode::Decode)]
pub struct Class {
    /// - kind = chapter: roman number ex. "IX"
    /// - kind = block: range ex. "A00-A09"
    /// - kind = category: full icd10 ex. "Q50.6"
    pub code: String,
    pub kind: ClassKind,
    pub usage: Option<UsageKind>,
    /// full icd10
    pub superclass: Option<String>,
    /// full icd10(s)
    pub subclasses: Vec<String>,
    /// Modifier code NEED to apply as SubClass
    pub modified_by_1: Option<String>,
    /// Modifier code NEED to apply as SubClass
    pub modified_by_2: Option<String>,
    // Modifier code(s) from SuperClass that NOT NEED to apply as SubClass
    pub exclude_modifiers: Vec<String>,
    /// full icd10 without `with_modifier_class_1` and `with_modifier_class_2`
    pub without_modifier_class: Option<String>,
    /// modifier|code already applied to this class
    pub with_modifier_class_1: Option<String>,
    /// modifier|code already applied to this class
    pub with_modifier_class_2: Option<String>,
    /// modifier as subclass
    pub sub_modifier: Option<String>,
    /// (rubric_id, Rubric)
    pub rubrics: BTreeMap<String, Rubric>,
}

impl Class {
    pub fn new(code: &str, kind: ClassKind, usage: Option<UsageKind>) -> Self {
        Self {
            code: code.to_owned(),
            kind,
            usage,
            superclass: None,
            subclasses: Vec::new(),
            modified_by_1: None,
            modified_by_2: None,
            exclude_modifiers: Vec::new(),
            without_modifier_class: None,
            with_modifier_class_1: None,
            with_modifier_class_2: None,
            sub_modifier: None,
            rubrics: BTreeMap::new(),
        }
    }

    pub fn set_superclass(&mut self, code: &str) {
        if self.superclass.is_some() {
            panic!("Cannot set 2nd SuperClass at {}", self.code);
        }
        self.superclass.replace(code.to_owned());
    }

    pub fn add_subclass(&mut self, code: &str) {
        self.subclasses.push(code.to_owned());
    }

    pub fn set_modified_by(&mut self, code: &str) {
        let is_duplicate = [self.modified_by_1.as_ref(), self.modified_by_2.as_ref()].iter().flatten().any(|by| *by == code);
        if !is_duplicate {
            if self.modified_by_1.is_some() {
                if self.modified_by_2.is_some() {
                    panic!("Cannot set 3rd ModifiedBy at {}", self.code);
                } else {
                    self.modified_by_2.replace(code.to_owned());
                }
            } else {
                self.modified_by_1.replace(code.to_owned());
            }
        }
    }

    pub fn add_exclude_modifier(&mut self, code: &str) {
        self.exclude_modifiers.push(code.to_owned());
    }

    pub fn set_without_modifier_class(&mut self, code: &str) {
        if self.without_modifier_class.is_some() {
            panic!("Cannot set 2nd without_modifier_class at {}", self.code);
        }
        self.without_modifier_class.replace(code.to_owned());
    }

    pub fn set_with_modifier_class_1(&mut self, code: &str) {
        if self.with_modifier_class_1.is_some() {
            panic!("Cannot set 2nd with_modifier_class_1 at {}", self.code);
        }
        self.with_modifier_class_1.replace(code.to_owned());
    }

    pub fn set_with_modifier_class_2(&mut self, code: &str) {
        if self.with_modifier_class_2.is_some() {
            panic!("Cannot set 2nd with_modifier_class_2 at {}", self.code);
        }
        self.with_modifier_class_2.replace(code.to_owned());
    }

    pub fn set_sub_modifier(&mut self, code: &str) {
        if self.sub_modifier.is_some() {
            panic!("Cannot set 2nd sub_modifier at {}", self.code);
        }
        self.sub_modifier.replace(code.to_owned());
    }

    pub fn add_rubric(&mut self, rubric_id: &str, rubric: Rubric) {
        self.rubrics.insert(rubric_id.to_owned(), rubric);
    }

    pub fn get_rubric(&mut self, rubric_id: &str) -> Option<&mut Rubric> {
        self.rubrics.get_mut(rubric_id)
    }
}

// - 1 Rubric always has 1 Label, so we put it together
// - 1 Rubric has maximun of 1 Fragment usage
#[derive(Clone, Debug, bitcode::Encode, bitcode::Decode)]
pub struct Rubric {
    pub kind: RubricKind,
    pub text: String,
    // fragment's usage
    pub usage: Option<UsageKind>,
    pub reference: Vec<Reference>,
}

impl Rubric {
    pub fn new(kind: &str) -> Self {
        Self {
            kind: RubricKind::new(kind).unwrap(),
            text: String::new(),
            usage: None,
            reference: Vec::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if self.text.is_empty() {
            self.text.push_str(&sanity_first_detail(text));
        } else {
            self.text.push(' ');
            self.text.push_str(&sanity_first_detail(text));
        }
    }

    pub fn add_usage(&mut self, usage_opt: Option<UsageKind>) {
        if let Some(usage) = usage_opt {
            if self.usage.is_some() {
                panic!("Multiple usage detected at '{}'", &self.text);
            } else {
                self.usage.replace(usage);
            }
        }
    }

    pub fn add_reference(&mut self, reference: Reference) {
        self.reference.push(reference);
    }

    /// return (label, Reference)
    pub fn reference_with_lebel(&self) -> Vec<(String, Reference)> {
        let mut result = Vec::new();
        let mut pos = 0;
        let mut prev_pos = 0;
        for r in self.reference.iter() {
            if r.position > prev_pos {
                prev_pos = pos;
                pos = r.position;
            }
            if pos <= self.text.len() {
                result.push((self.text[prev_pos..pos].to_string(), r.to_owned()));
            }
        }
        result
    }
}

#[derive(Clone, Debug, bitcode::Encode, bitcode::Decode)]
pub struct Reference {
    pub label: String,
    pub code: Option<String>,
    pub usage: Option<UsageKind>,
    pub position: usize,
}

impl Reference {
    pub fn new(label: &str, code: Option<String>, usage: Option<UsageKind>, position: usize) -> Self {
        Self {
            label: label.to_owned(),
            code,
            usage,
            position,
        }
    }
}

/// translate British to American, remove extra spaces, start with uppercase
fn sanity_first_detail(input: &str) -> String {
    let first = sanity_space(input);
    let second = if PROPOSITIONS.iter().any(|p| first.starts_with(p)) { first } else { first_char_uppercase(&first) };
    TRANSLATOR.translate(&second)
}

// /// translate British to American, remove extra spaces, start with uppercase
// fn sanity_detail(input: &str) -> String {
//     TRANSLATOR.translate(&sanity_space(input))
// }
