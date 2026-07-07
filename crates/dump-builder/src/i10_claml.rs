// https://icd.who.int/browse10/2016/en
//
// NOTE
//
// - Element's name has no prefix: we can use `OwnedName.local_name` directly
// - Element's attributes is BtreeMap, use `.get(name)` or `.contains(name)`
// - We use only `Modifier`, `ModifierClass` and `class` element
// - Rubric with `introduction` kind contains markup tags, we still concat all (list and table will unreadable)

// Fragment type: all can concat with ' '
// 1. list
//   - First Fragment is header, next is bullet
//   - Next Rubric use the `same` Previous Rubric as header, 1-2a, 1-2b-3a, 1-2c, 1-2c-3b, 1-2c-3c
// 2. item
//   - Single fragment (for usage)
//   - Next separate from previous list/item with vertical bar

// TM (page with #p=xx)
// flip book: http://www.thcc.or.th/ebook1/2016/mobile/index.html#p=73
// page image: http://www.thcc.or.th/ebook1/2016/files/mobile/73.jpg

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    io::Cursor,
    rc::Rc,
};

use xml::{
    attribute::OwnedAttribute,
    reader::{EventReader, XmlEvent},
};

use kphis_drg_worker::i10::claml::{Class, ClassKind, I10Claml, Modifier, ModifierClass, ParseAt, Reference, Rubric, RubricKind, State, UsageKind};
use kphis_util::util::icd10_dot;

pub fn new_i10_claml() -> I10Claml {
    let mut asset = parse_xml_tm();
    // remove `Introduction` and `Modifierlink` rubric
    remove_unused_rubrics(&mut asset);
    // Fix: use `label` as `code` if possible
    fixed_references(&mut asset);
    // Fix: icd102016en.xml contains WRONG ModifiedBy and ExcludeModifier
    fixed_modified_by(&mut asset);
    fixed_exclude_modifier(&mut asset);
    // Update ModifiedBy of `category` classes from every SuperClass level
    chain_modifier(&mut asset);
    // generate classes from modifier
    add_class_from_modifier(&mut asset);
    asset
}

fn parse_xml_tm() -> I10Claml {
    let file_bytes = include_bytes!("../raw-icd-who/icd102016en-tm.xml");
    let file = Cursor::new(file_bytes); // Buffering is important for performance
    let parser = EventReader::new(file);

    let state = Rc::new(RefCell::new(State::default()));
    for result in parser {
        match result {
            Ok(e) => {
                triage(&e, state.clone());
            }
            Err(err) => {
                println!("Error xml parsing: {}", err);
            }
        }
    }

    let state_ref = state.borrow();
    state_ref.asset.clone()
}

/// remove
/// - Introduction: TODO parse more tags
/// - Modifierlink: use Class's `modified_by_1` and `modified_by_2` instead
fn remove_unused_rubrics(asset: &mut I10Claml) {
    for (_, modifier) in asset.modifiers.iter_mut() {
        modifier.rubrics.retain(|_, r| !matches!(r.kind, RubricKind::Introduction | RubricKind::Modifierlink));
    }
    for (_, modifier_class) in asset.modifier_classes.iter_mut() {
        modifier_class.rubrics.retain(|_, r| !matches!(r.kind, RubricKind::Introduction | RubricKind::Modifierlink));
    }
    for (_, class) in asset.classes.iter_mut() {
        class.rubrics.retain(|_, r| !matches!(r.kind, RubricKind::Introduction | RubricKind::Modifierlink));
    }
}

/// reference label as `A00-A09`, `A01.-`, `A09.0` can be `code` directly
fn fixed_references(asset: &mut I10Claml) {
    let classes = asset.classes.clone();
    for (_, class) in asset.classes.iter_mut() {
        for (_, rubric) in class.rubrics.iter_mut() {
            for reference in rubric.reference.iter_mut() {
                let key = reference.label.trim_end_matches(['.', '-']);
                if classes.contains_key(key) {
                    reference.code.replace(key.to_owned());
                }
            }
        }
    }
}

// `S20W00_4` = Place of occurrence code : use only W00-Y34, except Y06 and Y07
// `S20V01T_5` = Activity code : use only V01-Y34
//
/// - remove ModifiedBy `S20W00_4` and `S20V01T_5` from `Y35-Y36`, `Y40-Y84`, `Y85-Y89` and `Y90-Y98`
fn fixed_modified_by(asset: &mut I10Claml) {
    for (code, class) in asset.classes.iter_mut() {
        if ["Y35-Y36", "Y40-Y84", "Y85-Y89", "Y90-Y98"].contains(&code.as_str()) {
            class.modified_by_1.take();
            class.modified_by_2.take();
        }
    }
}

/// - remove ExcludeModifier `S20W00_4` and `S20V01T_5` from all, keep only `S20W00_4` for `V01-V99`
/// - add ExcludeModifier `S20W00_4` and `S20V01T_5` to `W26`, `X34` and `X59`
/// - add ExcludeModifier `S20W00_4` to `Y06`, `Y07`
fn fixed_exclude_modifier(asset: &mut I10Claml) {
    for (code, class) in asset.classes.iter_mut() {
        if class.exclude_modifiers.contains(&String::from("S20W00_4")) || class.exclude_modifiers.contains(&String::from("S20V01T_5")) {
            if code == "V01-V99" {
                class.exclude_modifiers.retain(|c| c == "S20W00_4");
            } else {
                class.exclude_modifiers = Vec::new();
            }
        }
        if ["Y06", "Y07"].contains(&code.as_str()) {
            class.exclude_modifiers.push(String::from("S20W00_4"));
        } else if ["W26", "X34", "X59"].contains(&code.as_str()) {
            class.exclude_modifiers.push(String::from("S20W00_4"));
            class.exclude_modifiers.push(String::from("S20V01T_5"));
        }
    }
}

/// Update ModifiedBy of `category` classes by using ALL SuperClass nesting ModifiedBy and ExcludeModifier
fn chain_modifier(asset: &mut I10Claml) {
    let clone = asset.clone();

    for (_, class) in asset.classes.iter_mut() {
        if matches!(class.kind, ClassKind::Category) {
            // we found that maximum nested level is 7 (include lowest class)
            let mut modified_by_1s = vec![None; 7];
            let mut modified_by_2s = vec![None; 7];
            let mut exclude_modified = vec![Vec::new(); 7];

            modified_by_1s[6] = class.modified_by_1.clone();
            modified_by_2s[6] = class.modified_by_2.clone();
            exclude_modified[6] = class.exclude_modifiers.clone();

            if let Some(first_parent_code) = &class.superclass {
                if let Some(first_parent) = clone.classes.get(first_parent_code) {
                    modified_by_1s[5] = first_parent.modified_by_1.clone();
                    modified_by_2s[5] = first_parent.modified_by_2.clone();
                    exclude_modified[5] = first_parent.exclude_modifiers.clone();
                    if let Some(second_parent_code) = &first_parent.superclass {
                        if let Some(second_parent) = clone.classes.get(second_parent_code) {
                            modified_by_1s[4] = second_parent.modified_by_1.clone();
                            modified_by_2s[4] = second_parent.modified_by_2.clone();
                            exclude_modified[4] = second_parent.exclude_modifiers.clone();
                            if let Some(third_parent_code) = &second_parent.superclass {
                                if let Some(third_parent) = clone.classes.get(third_parent_code) {
                                    modified_by_1s[3] = third_parent.modified_by_1.clone();
                                    modified_by_2s[3] = third_parent.modified_by_2.clone();
                                    exclude_modified[3] = third_parent.exclude_modifiers.clone();
                                    if let Some(fourth_parent_code) = &third_parent.superclass {
                                        if let Some(fourth_parent) = clone.classes.get(fourth_parent_code) {
                                            modified_by_1s[2] = fourth_parent.modified_by_1.clone();
                                            modified_by_2s[2] = fourth_parent.modified_by_2.clone();
                                            exclude_modified[2] = fourth_parent.exclude_modifiers.clone();
                                            if let Some(fifth_parent_code) = &fourth_parent.superclass {
                                                if let Some(fifth_parent) = clone.classes.get(fifth_parent_code) {
                                                    modified_by_1s[1] = fifth_parent.modified_by_1.clone();
                                                    modified_by_2s[1] = fifth_parent.modified_by_2.clone();
                                                    exclude_modified[1] = fifth_parent.exclude_modifiers.clone();
                                                    if let Some(sixth_parent_code) = &fifth_parent.superclass {
                                                        if let Some(sixth_parent) = clone.classes.get(sixth_parent_code) {
                                                            modified_by_1s[0] = sixth_parent.modified_by_1.clone();
                                                            modified_by_2s[0] = sixth_parent.modified_by_2.clone();
                                                            exclude_modified[0] = sixth_parent.exclude_modifiers.clone();
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let mut modified_by_1 = None;
            let mut modified_by_2 = None;
            for i in 0..7 {
                if !exclude_modified[i].is_empty() {
                    modified_by_1.take_if(|v| exclude_modified[i].contains(v));
                    modified_by_2.take_if(|v| exclude_modified[i].contains(v));
                }
                if modified_by_1s[i].is_some() {
                    if modified_by_1.is_none() {
                        modified_by_1 = modified_by_1s[i].clone();
                    } else if modified_by_2.is_none() {
                        modified_by_2 = modified_by_1s[i].clone();
                    } else {
                        panic!("Cannot set ModifiedBy 1 {}", modified_by_1s[i].clone().unwrap_or_default());
                    }
                }
                if modified_by_2s[i].is_some() {
                    if modified_by_1.is_none() {
                        modified_by_1 = modified_by_2s[i].clone();
                    } else if modified_by_2.is_none() {
                        modified_by_2 = modified_by_2s[i].clone();
                    } else {
                        panic!("Cannot set ModifiedBy 2 {}", modified_by_2s[i].clone().unwrap_or_default());
                    }
                }
            }

            class.modified_by_1 = modified_by_1;
            class.modified_by_2 = modified_by_2;
            class.exclude_modifiers = Vec::new();
        }
    }
}

/// - use ModifiedBy to add class (SubClass), `category` class whthout SubClass
/// - new class NOT has Rubric, use Rubric from `without_modifier_class` and `with_modifier_class_x`
/// - use sub_modifier as subclass
fn add_class_from_modifier(asset: &mut I10Claml) {
    let clone = asset.clone();
    for (code, class) in clone.classes.iter() {
        if class.subclasses.is_empty() {
            // try add modified_by_1, may has modified_by_2 (set as sub_modifier)
            if let Some(modified_by_1) = &class.modified_by_1 {
                if let Some(modifier_1) = clone.modifiers.get(modified_by_1) {
                    for modifier_1_subclass_code in modifier_1.subclasses.iter() {
                        let code_1 = concat_icd10_with_dot(code, modifier_1_subclass_code);
                        let with_modclass_1 = [modified_by_1, "|", modifier_1_subclass_code].concat();
                        if let Some(modifier_class_1) = clone.modifier_classes.get(&with_modclass_1) {
                            let mod_1_usage = class.usage.clone().or(modifier_class_1.usage.clone());
                            let mut new_1 = Class::new(&code_1, ClassKind::Category, mod_1_usage.clone());
                            new_1.set_superclass(code);
                            new_1.set_without_modifier_class(code);
                            new_1.set_with_modifier_class_1(&with_modclass_1);
                            // SPECIAL modified_by_2
                            // - R87.6 of ICD10TM, apply S18R83_4 to R87 as modified_by_1, so apply TM_R876 as modified_by_2 to R87.6 here
                            // - F10-F19 apply S05F10_4 as modified_by_1 but only .0, .2, .3, .4, .5 and .7 has 5th charactor code, so apply TM_F15x as modified_by_2 here
                            let modified_by_2_sp = if code_1.as_str() == "R87.6" {
                                &Some(String::from("TM_R876"))
                            } else if modified_by_1.as_str() == "S05F10_4" {
                                match modifier_1_subclass_code.as_str() {
                                    ".0" => &Some(String::from("TM_F150")),
                                    ".2" => &Some(String::from("TM_F152")),
                                    ".3" => &Some(String::from("TM_F153")),
                                    ".4" => &Some(String::from("TM_F154")),
                                    ".5" => &Some(String::from("TM_F155")),
                                    ".7" => &Some(String::from("TM_F157")),
                                    _ => &None,
                                }
                            } else {
                                &class.modified_by_2
                            };
                            if let Some(sub_modifier) = &modified_by_2_sp {
                                new_1.set_sub_modifier(sub_modifier);
                            }
                            asset.classes.insert(code_1.clone(), new_1);
                            // try add modified_by_2
                            if let Some(modified_by_2) = modified_by_2_sp {
                                if let Some(modifier_2) = clone.modifiers.get(modified_by_2) {
                                    for modifier_2_subclass_code in modifier_2.subclasses.iter() {
                                        let code_2 = concat_icd10_with_dot(&code_1, modifier_2_subclass_code);
                                        let with_modclass_2 = [modified_by_2, "|", modifier_2_subclass_code].concat();
                                        if let Some(modifier_class_2) = clone.modifier_classes.get(&with_modclass_2) {
                                            let mod_2_usage = mod_1_usage.clone().or(modifier_class_2.usage.clone());
                                            let mut new_2 = Class::new(&code_2, ClassKind::Category, mod_2_usage);
                                            new_2.set_superclass(&code_1);
                                            new_2.set_without_modifier_class(code);
                                            new_2.set_with_modifier_class_1(&with_modclass_1);
                                            new_2.set_with_modifier_class_2(&with_modclass_2);
                                            asset.classes.insert(code_2, new_2);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            // try add modified_by_2 without modified_by_1
            } else {
                if let Some(modified_by_2) = &class.modified_by_2 {
                    if let Some(modifier_2) = clone.modifiers.get(modified_by_2) {
                        for modifier_2_subclass_code in modifier_2.subclasses.iter() {
                            let code_2 = concat_icd10_with_dot(code, modifier_2_subclass_code);
                            let with_modclass_2 = [modified_by_2, "|", modifier_2_subclass_code].concat();
                            if let Some(modifier_class_2) = clone.modifier_classes.get(&with_modclass_2) {
                                let mod_2_usage = class.usage.clone().or(modifier_class_2.usage.clone());
                                let mut new_2 = Class::new(&code_2, ClassKind::Category, mod_2_usage);
                                new_2.set_superclass(&code);
                                new_2.set_without_modifier_class(code);
                                new_2.set_with_modifier_class_2(&with_modclass_2);
                                asset.classes.insert(code_2, new_2);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn triage(e: &XmlEvent, state: Rc<RefCell<State>>) {
    let parse_at = state.borrow().parse_at.clone();
    match &parse_at {
        ParseAt::Others => {
            let mut state_mut = state.borrow_mut();
            match e {
                XmlEvent::StartDocument {
                    version: _,
                    encoding: _,
                    standalone: _,
                } => {}
                XmlEvent::EndDocument => {}
                XmlEvent::ProcessingInstruction { name: _, data: _ } => {}
                XmlEvent::StartElement { name, attributes, namespace: _ } => match name.local_name.as_str() {
                    "Modifier" => {
                        if let Some(code) = get_attr(attributes, "code") {
                            let modifier = Modifier::new(code);
                            state_mut.asset.modifiers.insert(code.to_owned(), modifier);
                            state_mut.parse_at = ParseAt::new_modifier(code);
                        } else {
                            panic!("No 'code' in Modifier");
                        }
                    }
                    "ModifierClass" => {
                        if let (Some(code), Some(modifier)) = (get_attr(attributes, "code"), get_attr(attributes, "modifier")) {
                            let usage = get_attr(attributes, "usage").and_then(|kind| UsageKind::new(kind));
                            let concat = [modifier, "|", code].concat();
                            let modifier_class = ModifierClass::new(code, modifier, usage);
                            state_mut.asset.modifier_classes.insert(concat.clone(), modifier_class);
                            state_mut.parse_at = ParseAt::new_modifier_class(concat);
                        } else {
                            panic!("No 'code' and 'modifier' in ModifierClass");
                        }
                    }
                    "Class" => {
                        if let (Some(code), Some(kind)) = (get_attr(attributes, "code"), get_attr(attributes, "kind").and_then(|kind| ClassKind::new(kind))) {
                            let usage = get_attr(attributes, "usage").and_then(|kind| UsageKind::new(kind));
                            let class = Class::new(code, kind, usage);
                            state_mut.asset.classes.insert(code.to_owned(), class);
                            state_mut.parse_at = ParseAt::new_class(code);
                        } else {
                            panic!("No 'code' and 'kind' in Class");
                        }
                    }
                    _ => {}
                },
                XmlEvent::EndElement { name } => match name.local_name.as_str() {
                    "Rubric" => {
                        state_mut.parse_at.clear_rubric_id();
                    }
                    "Reference" => {
                        state_mut.reference.take();
                    }
                    _ => {}
                },
                XmlEvent::CData(_) => {}
                XmlEvent::Comment(_) => {}
                XmlEvent::Characters(_) => {}
                XmlEvent::Whitespace(_) => {}
                XmlEvent::Doctype { syntax: _ } => {}
            }
        }
        ParseAt::Modifier(code, rubric_id_opt) => {
            parse_modifier(&code, rubric_id_opt, e, state.clone());
        }
        ParseAt::ModifierClass(concat, rubric_id_opt) => {
            parse_modifier_class(&concat, rubric_id_opt, e, state.clone());
        }
        ParseAt::Class(code, rubric_id_opt) => {
            parse_class(&code, rubric_id_opt, e, state.clone());
        }
    }
}

// - `Modifier`[code]
// - `Modifier` - `SubClass`[code]
// - `Modifier` - *`Rubric`[kind] - `Label` - *`Para` - Charactor
fn parse_modifier(current: &str, rubric_id_opt: &Option<String>, e: &XmlEvent, state: Rc<RefCell<State>>) {
    let mut state_mut = state.borrow_mut();
    match e {
        XmlEvent::StartDocument {
            version: _,
            encoding: _,
            standalone: _,
        } => {}
        XmlEvent::EndDocument => {}
        XmlEvent::ProcessingInstruction { name: _, data: _ } => {}
        XmlEvent::StartElement { name, attributes, namespace: _ } => {
            if let Some(modifier) = state_mut.asset.modifiers.get_mut(current) {
                match name.local_name.as_str() {
                    "SubClass" => {
                        if let Some(code) = get_attr(attributes, "code") {
                            modifier.add_subclass(code);
                        } else {
                            panic!("No 'code' in SubClass");
                        }
                    }
                    "Rubric" => {
                        if let (Some(rubric_id), Some(kind)) = (get_attr(attributes, "id"), get_attr(attributes, "kind")) {
                            let rubric = Rubric::new(kind);
                            modifier.add_rubric(rubric_id, rubric);
                            state_mut.parse_at.set_rubric_id(rubric_id);
                        } else {
                            panic!("No 'id' and 'kind' in Rubric");
                        }
                    }
                    "Reference" => {
                        let code = get_attr(attributes, "code").cloned();
                        let usage = get_attr(attributes, "usage").and_then(|kind| UsageKind::new(kind));
                        state_mut.reference.replace((code, usage));
                    }
                    _ => {}
                }
            }
        }
        XmlEvent::EndElement { name } => {
            if name.local_name.as_str() == "Modifier" {
                state_mut.parse_at = ParseAt::Others;
            }
        }
        XmlEvent::CData(_) => {}
        XmlEvent::Comment(_) => {}
        XmlEvent::Characters(s) => {
            let usage_opt = state_mut.reference.take();
            if let Some(modifier) = state_mut.asset.modifiers.get_mut(current) {
                if let Some(rubric_id) = rubric_id_opt {
                    if let Some(rubric) = modifier.get_rubric(rubric_id) {
                        if let Some((code, usage)) = usage_opt {
                            let position = rubric.text.len();
                            let reference = Reference::new(s, code, usage, position);
                            rubric.add_reference(reference);
                        } else {
                            rubric.add_text(s);
                        };
                    }
                }
            }
        }
        XmlEvent::Whitespace(_) => {}
        XmlEvent::Doctype { syntax: _ } => {}
    }
}

// - `ModifierClass`[code][modifier][usage]
// - `ModifierClass` - `SuperClass`[code]
// - `ModifierClass` - *`Rubric`[kind] - `Label` - Charactor
// - `ModifierClass` - *`Rubric`[kind] - `Label` - `Reference`[usage] - Charactor
// - `ModifierClass` - *`Rubric`[kind] - `Label` - *`Fragment`[type=list/item][usage] - Charactor
// - `ModifierClass` - *`Rubric`[kind] - `Label` - *`Fragment`[type=list/item][usage] - `Reference`[usage] - Charactor
fn parse_modifier_class(current: &str, rubric_id_opt: &Option<String>, e: &XmlEvent, state: Rc<RefCell<State>>) {
    let mut state_mut = state.borrow_mut();
    match e {
        XmlEvent::StartDocument {
            version: _,
            encoding: _,
            standalone: _,
        } => {}
        XmlEvent::EndDocument => {}
        XmlEvent::ProcessingInstruction { name: _, data: _ } => {}
        XmlEvent::StartElement { name, attributes, namespace: _ } => {
            if let Some(modifier_class) = state_mut.asset.modifier_classes.get_mut(current) {
                match name.local_name.as_str() {
                    "SuperClass" => {
                        if let Some(code) = get_attr(attributes, "code") {
                            modifier_class.set_superclass(code);
                        } else {
                            panic!("No 'code' in SuperClass");
                        }
                    }
                    "Rubric" => {
                        if let (Some(rubric_id), Some(kind)) = (get_attr(attributes, "id"), get_attr(attributes, "kind")) {
                            let rubric = Rubric::new(kind);
                            modifier_class.add_rubric(rubric_id, rubric);
                            state_mut.parse_at.set_rubric_id(rubric_id);
                        } else {
                            panic!("No 'id' and 'kind' in Rubric");
                        }
                    }
                    "Fragment" => {
                        if let Some(rubric_id) = rubric_id_opt {
                            if let Some(rubric) = modifier_class.get_rubric(rubric_id) {
                                let usage = get_attr(attributes, "usage").and_then(|kind| UsageKind::new(kind));
                                rubric.add_usage(usage);
                            }
                        }
                    }
                    "Reference" => {
                        let code = get_attr(attributes, "code").cloned();
                        let usage = get_attr(attributes, "usage").and_then(|kind| UsageKind::new(kind));
                        state_mut.reference.replace((code, usage));
                    }
                    _ => {}
                }
            }
        }
        XmlEvent::EndElement { name } => {
            if name.local_name.as_str() == "ModifierClass" {
                state_mut.parse_at = ParseAt::Others;
            }
        }
        XmlEvent::CData(_) => {}
        XmlEvent::Comment(_) => {}
        XmlEvent::Characters(s) => {
            let usage_opt = state_mut.reference.take();
            if let Some(modifier_class) = state_mut.asset.modifier_classes.get_mut(current) {
                if let Some(rubric_id) = rubric_id_opt {
                    if let Some(rubric) = modifier_class.get_rubric(rubric_id) {
                        if let Some((code, usage)) = usage_opt {
                            let position = rubric.text.len();
                            let reference = Reference::new(s, code, usage, position);
                            rubric.add_reference(reference);
                        } else {
                            rubric.add_text(s);
                        };
                    }
                }
            }
        }
        XmlEvent::Whitespace(_) => {}
        XmlEvent::Doctype { syntax: _ } => {}
    }
}

// - `Class`[code][kind][usage]
// - `Class` - `SuperClass`[code]
// - `Class` - `SubClass`[code]
// - `Class` - *`Rubric`[kind] - `Label` - Charactor
// - `Class` - *`Rubric`[kind] - `Label` - `Reference`[usage] - Charactor
// - `Class` - *`Rubric`[kind] - `Label` - `*Fragment`[type=list/item][usage] - Charactor
// - `Class` - *`Rubric`[kind] - `Label` - `*Fragment`[type=list/item][usage] - `Reference`[usage] - Charactor
fn parse_class(current: &str, rubric_id_opt: &Option<String>, e: &XmlEvent, state: Rc<RefCell<State>>) {
    let mut state_mut = state.borrow_mut();
    match e {
        XmlEvent::StartDocument {
            version: _,
            encoding: _,
            standalone: _,
        } => {}
        XmlEvent::EndDocument => {}
        XmlEvent::ProcessingInstruction { name: _, data: _ } => {}
        XmlEvent::StartElement { name, attributes, namespace: _ } => {
            if let Some(class) = state_mut.asset.classes.get_mut(current) {
                match name.local_name.as_str() {
                    "SuperClass" => {
                        if let Some(code) = get_attr(attributes, "code") {
                            class.set_superclass(code);
                        } else {
                            panic!("No 'code' in SuperClass");
                        }
                    }
                    "SubClass" => {
                        if let Some(code) = get_attr(attributes, "code") {
                            class.add_subclass(code);
                        } else {
                            panic!("No 'code' in SubClass");
                        }
                    }
                    "ModifiedBy" => {
                        if let Some(code) = get_attr(attributes, "code") {
                            class.set_modified_by(code);
                        } else {
                            panic!("No 'code' in ModifiedBy");
                        }
                    }
                    "ExcludeModifier" => {
                        if let Some(code) = get_attr(attributes, "code") {
                            class.add_exclude_modifier(code);
                        } else {
                            panic!("No 'code' in ExcludeModifier");
                        }
                    }
                    "Rubric" => {
                        if let (Some(rubric_id), Some(kind)) = (get_attr(attributes, "id"), get_attr(attributes, "kind")) {
                            let rubric = Rubric::new(kind);
                            class.add_rubric(rubric_id, rubric);
                            state_mut.parse_at.set_rubric_id(rubric_id);
                        } else {
                            panic!("No 'id' and 'kind' in Rubric");
                        }
                    }
                    "Fragment" => {
                        if let Some(rubric_id) = rubric_id_opt {
                            if let Some(rubric) = class.get_rubric(rubric_id) {
                                let usage = get_attr(attributes, "usage").and_then(|kind| UsageKind::new(kind));
                                rubric.add_usage(usage);
                            }
                        }
                    }
                    "Reference" => {
                        let code = get_attr(attributes, "code").cloned();
                        let usage = get_attr(attributes, "usage").and_then(|kind| UsageKind::new(kind));
                        state_mut.reference.replace((code, usage));
                    }
                    _ => {}
                }
            }
        }
        XmlEvent::EndElement { name } => {
            if name.local_name.as_str() == "Class" {
                state_mut.parse_at = ParseAt::Others;
            }
        }
        XmlEvent::CData(_) => {}
        XmlEvent::Comment(_) => {}
        XmlEvent::Characters(s) => {
            let usage_opt = state_mut.reference.take();
            if let Some(class) = state_mut.asset.classes.get_mut(current) {
                if let Some(rubric_id) = rubric_id_opt {
                    if let Some(rubric) = class.get_rubric(rubric_id) {
                        if let Some((code, usage)) = usage_opt {
                            let position = rubric.text.len();
                            let reference = Reference::new(s, code, usage, position);
                            rubric.add_reference(reference);
                        } else {
                            rubric.add_text(s);
                        };
                    }
                }
            }
        }
        XmlEvent::Whitespace(_) => {}
        XmlEvent::Doctype { syntax: _ } => {}
    }
}

fn get_attr<'a>(attrs: &'a [OwnedAttribute], key: &'a str) -> Option<&'a String> {
    attrs.iter().find(|attr| attr.name.local_name.as_str() == key).map(|attr| &attr.value)
}

fn concat_icd10_with_dot(a: &str, b: &str) -> String {
    if a.len() == 3 && !b.starts_with('.') { [a, ".", b].concat() } else { [a, b].concat() }
}

pub fn get_claml_asterisk_dagger() -> HashMap<String, HashSet<String>> {
    let mut inclusion_usage = Vec::new();
    let mut exclusion_usage = Vec::new();
    let claml = new_i10_claml();
    for (code, class) in claml.classes.iter() {
        let class_usage = class.usage.as_ref();
        for (_, ru) in class.rubrics.iter() {
            let rubric_usage = ru.usage.as_ref();
            if matches!(ru.kind, RubricKind::Exclusion) {
                let daggers = ru
                    .reference
                    .iter()
                    .filter(|rf| rf.usage.as_ref().map(|u| matches!(u, UsageKind::Dagger)).unwrap_or_default())
                    .collect::<Vec<&Reference>>();
                let asters = ru
                    .reference
                    .iter()
                    .filter(|rf| rf.usage.as_ref().map(|u| matches!(u, UsageKind::Aster)).unwrap_or_default())
                    .collect::<Vec<&Reference>>();
                if !daggers.is_empty() && !asters.is_empty() {
                    exclusion_usage.push((daggers, asters));
                }
            } else {
                for rf in ru.reference.iter() {
                    let ref_usage = rf.usage.as_ref();
                    if class_usage.is_some() || rubric_usage.is_some() || ref_usage.is_some() {
                        inclusion_usage.push(((code, class_usage), (code, rubric_usage), (&rf.label, ref_usage)));
                    }
                }
            }
        }
    }
    // dbg!(&inclusion_usage);
    let mut pairs = Vec::new();
    let mut maybe = Vec::new();
    for ((class_c, class_k), (rubric_c, rubric_k), (ref_c, ref_k)) in inclusion_usage {
        let mut daggers = HashSet::new();
        let mut asters = HashSet::new();
        match class_k {
            Some(UsageKind::Dagger) => {
                daggers.insert(class_c);
            }
            Some(UsageKind::Aster) => {
                asters.insert(class_c);
            }
            _ => {}
        }
        match rubric_k {
            Some(UsageKind::Dagger) => {
                daggers.insert(rubric_c);
            }
            Some(UsageKind::Aster) => {
                asters.insert(rubric_c);
            }
            _ => {}
        }
        match ref_k {
            Some(UsageKind::Dagger) => {
                daggers.insert(ref_c);
            }
            Some(UsageKind::Aster) => {
                asters.insert(ref_c);
            }
            _ => {}
        }
        match (daggers.len(), asters.len()) {
            (1, 1) => {
                pairs.push((daggers.into_iter().next().cloned().unwrap_or_default(), asters.into_iter().next().cloned().unwrap_or_default()));
            }
            (0, 1) | (1, 0) => {}
            (_, _) => {
                maybe.push((daggers, asters));
            }
        }
    }

    // dbg!(&exclusion_usage);
    assert_eq!(exclusion_usage.len(), 4);
    assert!(exclusion_usage.iter().all(|(a, b)| a.len() == 1 || b.len() == 1));
    // test all `exclusion_usage` were in `pairs`
    for (d, a) in exclusion_usage.iter().map(|(d_s, a_s)| (&d_s[0].label, &a_s[0].label)) {
        assert!(pairs.iter().any(|(dd, aa)| dd == d && aa == a));
    }

    assert_eq!(pairs.len(), 822);
    assert!(maybe.is_empty());

    let mut claml_all_pairs = HashMap::new();
    let codes = crate::drg_grouper::new_grouper().valid_codes();
    for (d, a) in pairs.iter() {
        // println!("{}, {}", d, a);
        let dags = get_codes(d, &codes);
        let asts = get_codes(a, &codes);
        for ast in asts.iter() {
            for dag in dags.iter() {
                claml_all_pairs.entry(ast.replace('.', "")).or_insert(HashSet::new()).insert(dag.replace('.', ""));
            }
        }
    }

    claml_all_pairs
}

// code as 4 format:
// - `A00.0`
// - `A00.-`
// - `A00-A10`
// - `E10-E14 with common fourth character .X`
/// keywords, codes and results are ICD10 with dot
fn get_codes(keyword: &str, codes: &[String]) -> Vec<String> {
    if keyword.starts_with("E10-E14 with") {
        if let Some(last) = keyword.chars().last() {
            if last.is_ascii_digit() {
                ["E10.", "E11.", "E12.", "E13.", "E14."].iter().map(|s| [s, last.to_string().as_str()].concat()).collect()
            } else {
                panic!("{} is E10-E14 with xx but not ended with digit", keyword);
            }
        } else {
            panic!("NO LAST CHARS !!!");
        }
    } else if keyword.ends_with('-') {
        let k = keyword.trim_end_matches(['-', '.']);
        codes.iter().filter(|c| c.starts_with(k)).cloned().collect()
    } else if keyword.contains('-') {
        let split = keyword.split('-').collect::<Vec<&str>>();
        if split.len() == 2 {
            let split_0 = split[0].replace('.', "");
            let split_1 = split[1].replace('.', "");
            let (ks_a, ks_r) = split_0.split_at(1);
            let (ke_a, ke_r) = split_1.split_at(1);
            let ks_r_len = ks_r.len();
            let ks_ru = ks_r.parse::<usize>().unwrap();
            let ke_ru = ke_r.parse::<usize>().unwrap();
            if ks_a == ke_a {
                (ks_ru..=ke_ru)
                    .flat_map(|u| {
                        let new = icd10_dot(&[ks_a, u.to_string().as_str()].concat());
                        codes.iter().filter(|c| c.starts_with(&new)).cloned().collect::<Vec<String>>()
                    })
                    .collect()
            } else {
                let ks_c = ks_a.chars().next().unwrap();
                let ke_c = ke_a.chars().next().unwrap();
                let ks_r_max = match ks_r_len {
                    2 => 99,
                    3 => 999,
                    4 => 9999,
                    5 => 99999,
                    s => panic!("Cannot assign MAX value of {} size", s),
                };
                let keys = ks_c..=ke_c;
                let keys_len = keys.clone().count();
                keys.enumerate()
                    .flat_map(|(i, ks)| {
                        if i == 0 {
                            (ks_ru..=ks_r_max)
                                .flat_map(|u| {
                                    let new = icd10_dot(&[ks.to_string(), u.to_string()].concat());
                                    codes.iter().filter(|c| c.starts_with(&new)).cloned().collect::<Vec<String>>()
                                })
                                .collect::<Vec<String>>()
                        } else if i == keys_len - 1 {
                            (0..=ke_ru)
                                .flat_map(|u| {
                                    let new = icd10_dot(&[ks.to_string(), u.to_string()].concat());
                                    codes.iter().filter(|c| c.starts_with(&new)).cloned().collect::<Vec<String>>()
                                })
                                .collect::<Vec<String>>()
                        } else {
                            (0..=ks_r_max)
                                .flat_map(|u| {
                                    let new = icd10_dot(&[ks.to_string(), u.to_string()].concat());
                                    codes.iter().filter(|c| c.starts_with(&new)).cloned().collect::<Vec<String>>()
                                })
                                .collect::<Vec<String>>()
                        }
                    })
                    .collect()
            }
        } else {
            panic!("{} has '-' but has more than 2 value", keyword);
        }
    } else {
        vec![keyword.to_owned()]
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use std::{collections::{BTreeMap, HashSet}, fs::File, io::{BufWriter, Read, Write}, sync::Arc};
    use kphis_drg_worker::{drg::model::I10vx, i10::claml::RubricKind};
    use kphis_util::util::icd10_dot;

    use crate::write_to;
    use super::*;

    fn parse_xml_en() -> I10Claml {
        let file_bytes = include_bytes!("../raw-icd-who/icd102016en.xml");
        let file = Cursor::new(file_bytes); // Buffering is important for performance
        let parser = EventReader::new(file);

        let state = Rc::new(RefCell::new(State::default()));
        for result in parser {
            match result {
                Ok(e) => {
                    triage(&e, state.clone());
                }
                Err(err) => {
                    println!("Error xml parsing: {}", err);
                }
            }
        }

        let state_ref = state.borrow();
        state_ref.asset.clone()
    }

    #[test]
    fn test_dump_i10_detail() {
        let write_i10_detail = new_i10_claml();
        let write_bytes = bitcode::encode(&write_i10_detail);
        // dbg!(std::env::current_dir().unwrap()); // kphis/crates/kphis-dump-builder
        let path = "../kphis-drg-worker/dump/i10-claml.dump";
        write_to(&write_bytes, path);

        let mut read_file = File::open(path).unwrap();
        let mut read_bytes = Vec::new();
        read_file.read_to_end(&mut read_bytes).unwrap();
        let read_i10_detail = bitcode::decode::<I10Claml>(&read_bytes).unwrap();

        let classes_len = 41277;
        assert_eq!(read_i10_detail.classes.len(), classes_len);
        assert_eq!(write_i10_detail.classes.len() ,classes_len);
    }

    #[test]
    fn test_parse_detail() {
        let verbose = false;

        let mut asset = parse_xml_tm();
        assert_eq!(asset.modifiers.len(), 83);
        assert_eq!(asset.modifier_classes.len(), 481);
        assert_eq!(asset.classes.len(), 12183);

        // check all modifier_class has superclass == modifier
        for (_, modifier_class) in asset.modifier_classes.iter() {
            assert_eq!(modifier_class.modifier, modifier_class.superclass.clone().unwrap_or_default());
        }

        // check maximum of nested superclass
        count_nest_superclass(&asset);

        // check class CANNOT HAS both ModifiedBy and ExcludeModifier at the same time
        let has_both_modified_by_and_exclude = asset.classes.iter().any(|(_, c)| {
            (c.modified_by_1.is_some() || c.modified_by_2.is_some() ) && !c.exclude_modifiers.is_empty()
        });
        assert!(!has_both_modified_by_and_exclude);

        // check class with ExcludeModifier MUST HAS SuperClass
        let has_exclude_but_no_superclass = asset.classes.iter().any(|(_, c)| {
            !c.exclude_modifiers.is_empty() && c.superclass.is_none()
        });
        assert!(!has_exclude_but_no_superclass);

        // test_with_print(&asset);

        remove_unused_rubrics(&mut asset);
        // dbg!(asset.classes.get("K27")); // has modifierlink rubric

        fixed_references(&mut asset);
        // dbg!(asset.classes.get("I80")); // has empty code in reference

        fixed_modified_by(&mut asset);
        fixed_exclude_modifier(&mut asset);

        chain_modifier(&mut asset);
        assert_eq!(asset.classes.get("V01").map(|c| (c.modified_by_1.clone(), c.modified_by_2.clone())), Some((Some(String::from("S20V01_4")), Some(String::from("S20V01T_5")))));

        add_class_from_modifier(&mut asset);

        // dbg!(asset.classes.get("V86")); // superclass
        // dbg!(asset.classes.get("V86.0")); // target
        // dbg!(asset.classes.get("V86.01")); // use mod_by_2
        // dbg!(asset.classes.get("W01")); // target
        // dbg!(asset.classes.get("W01.0")); // use mod_by_1
        // dbg!(asset.classes.get("W01.01")); // use mod_by_2

        // dbg!(asset.classes.get("W26.0")); // without mod_by_1 and mod_by_2
        // dbg!(asset.classes.get("X34.0")); // without mod_by_1 and mod_by_2
        // dbg!(asset.classes.get("X59.0")); // without mod_by_1 and mod_by_2
        // dbg!(asset.classes.get("Y06.00")); // use only S20V01T_5
        // dbg!(asset.classes.get("Y07.00")); // use only S20V01T_5

        // dbg!(asset.classes.get("A18.1")); // dagger

        // dbg!(asset.classes.get("R87.6")); // ICD10-TM special
        // dbg!(asset.classes.get("R87.69")); // ICD10-TM special
        // dbg!(asset.classes.get("F15.2")); // ICD10-TM special
        // dbg!(asset.classes.get("F15.24")); // ICD10-TM special

        // dbg!(asset.classes.get("I70.219")); // ICD10-TM mod_by

        // dbg!(asset.modifier_classes.get("S05F10_4|.6"));

        // check all 'preferred' Rubric of ModifierClass NOT has Reference
        for (_, modifier_class) in asset.modifier_classes.iter() {
            for (id, rubric) in modifier_class.rubrics.iter() {
                if matches!(rubric.kind, RubricKind::Preferred) {
                    if !rubric.reference.is_empty() {
                        panic!("Rubric {} is 'preferred' and has Reference (ModifierClass)", id);
                    }
                }
            }
        }
        // check all 'preferred' Rubric of 'without_modifier_class' class NOT has Reference
        for (_, class) in asset.classes.iter() {
            if class.without_modifier_class.is_some() {
                for (id, rubric) in class.rubrics.iter() {
                    if matches!(rubric.kind, RubricKind::Preferred) {
                        if !rubric.reference.is_empty() {
                            panic!("Rubric {} is 'preferred' and has Reference ('without_modifier_class' class)", id);
                        }
                    }
                }
            }
        }

        // Max Reference in 'prefereed' Rubric
        let mut max_ref_in_preferred = 0;
        for (_, class) in asset.classes.iter() {
            for (id, rubric) in class.rubrics.iter() {
                if matches!(rubric.kind, RubricKind::Preferred) && rubric.reference.len() > max_ref_in_preferred {
                    max_ref_in_preferred = rubric.reference.len();
                    if verbose && rubric.reference.len() == 4 {
                        println!("Rubric {} is 'preferred' and has 4 references (MAX)", id)
                    }
                }
            }
        }
        assert_eq!(max_ref_in_preferred, 4);

        // Class with 'preferredLong' Rubric has the same usage as 'preferred'
        for (_, class) in asset.classes.iter() {
            let long = class.rubrics.iter().filter(|(_, r)| matches!(r.kind, RubricKind::PreferredLong)).collect::<Vec<(&String, &Rubric)>>();
            let pref = class.rubrics.iter().filter(|(_, r)| matches!(r.kind, RubricKind::Preferred)).collect::<Vec<(&String, &Rubric)>>();
            if let (Some((_, long_r)), Some((_, pref_r))) = (long.first(), pref.first()) {
                assert!(long_r.usage == pref_r.usage)
            }
        }

        // Max Reference in 'inclusion' Rubric
        let mut max_ref_in_inclusion = 0;
        for (_, class) in asset.classes.iter() {
            for (id, rubric) in class.rubrics.iter() {
                if matches!(rubric.kind, RubricKind::Inclusion) && rubric.reference.len() > max_ref_in_inclusion {
                    max_ref_in_inclusion = rubric.reference.len();
                    if verbose && rubric.reference.len() == 2 {
                        println!("Rubric {} is 'inclusive' and has 2 references (MAX)", id)
                    }
                }
            }
        }
        assert_eq!(max_ref_in_inclusion, 2);

        // Max Reference in 'exclusion' Rubric
        let mut max_ref_in_exclusion = 0;
        for (_, class) in asset.classes.iter() {
            for (id, rubric) in class.rubrics.iter() {
                if matches!(rubric.kind, RubricKind::Exclusion) && rubric.reference.len() > max_ref_in_exclusion {
                    max_ref_in_exclusion = rubric.reference.len();
                    if verbose && rubric.reference.len() == 4 {
                        println!("Rubric {} is 'exclusive' and has 4 references (MAX)", id)
                    }
                }
            }
        }
        assert_eq!(max_ref_in_exclusion, 4);

        // check all 'prefereed' and 'inclusion' Rubric has the same UserKind References
        for (_, class) in asset.classes.iter() {
            for (id, rubric) in class.rubrics.iter() {
                if matches!(rubric.kind, RubricKind::Preferred | RubricKind::Inclusion) {
                    let ref_all = rubric.reference.iter().map(|r| {
                        match r.usage {
                            Some(UsageKind::Dagger) => 1,
                            Some(UsageKind::Aster) => 2,
                            None => 3,
                        }
                    }).collect::<HashSet<u8>>();
                    if ref_all.len() > 1 {
                        panic!("Rubric {} has different Reference usage kind (Preferred / Inclusion Rubric kind)", id);
                    }
                }
            }
        }

        // 'exclusion' can has 'Dagger' and 'Aster' in the same Rubric's reference
        for (_, class) in asset.classes.iter() {
            for (id, rubric) in class.rubrics.iter() {
                if matches!(rubric.kind, RubricKind::Exclusion) {
                    let ref_all = rubric.reference.iter().map(|r| {
                        match r.usage {
                            Some(UsageKind::Dagger) => 1,
                            Some(UsageKind::Aster) => 2,
                            None => 3,
                        }
                    }).collect::<HashSet<u8>>();
                    if ref_all.len() > 1 && ref_all.contains(&3) {
                        panic!("Rubric {} has Non-Dagger/Non-Aster mixed Reference usage kind (Exclusion Rubric kind)", id);
                    }
                }
            }
        }

        // dbg!(asset.classes.iter().filter_map(|(c, class)| {
        //     class.rubrics.iter().any(|(_, r)| r.usage.is_some()).then(|| c)
        // }).collect::<Vec<&String>>());

        let all_len = asset.classes.len();
        let category_len = asset.classes.iter().filter(|(_, c)| matches!(c.kind, ClassKind::Category)).count();
        let category_not_ext = asset.classes.iter().filter(|(c, cl)| !c.starts_with(&['V','W','X','Y']) && matches!(cl.kind, ClassKind::Category));
        let category_not_ext_len = category_not_ext.clone().count();
        assert_eq!(all_len, 41277);
        assert_eq!(category_len, 40981);
        assert_eq!(category_not_ext_len, 16744);
        assert_eq!(category_len - category_not_ext_len, 24237);
    }

    #[test]
    fn test_diff_from_grouper() {
        let grouper = crate::drg_grouper::new_grouper();
        let mut vx_noex = grouper.i10vx.iter().filter_map(|(c, vx)| {
            vx.is_tm.then(|| (c.to_owned(), vx.clone()))
        }).collect::<Vec<(String, Arc<I10vx>)>>();
        let mut vx_ex = grouper.i10vx_ex.iter().filter_map(|(c, vx)| {
            vx.is_tm.then(|| (c.to_owned(), vx.clone()))
        }).collect::<Vec<(String, Arc<I10vx>)>>();
        vx_noex.sort_by(|(c1, _), (c2, _)| c1.cmp(c2));
        vx_ex.sort_by(|(c1, _), (c2, _)| c1.cmp(c2));
        assert_eq!(vx_noex.len(), 1622);
        assert!(vx_ex.is_empty());
        write_tm_over("1-vx-noex.txt",&vx_noex.iter().map(|(code, vx)| {
            [code, " : ", &vx.desc].concat()
        }).collect());

        // ICD-10-WHO
        let mut asset = parse_xml_en();
        remove_unused_rubrics(&mut asset);
        fixed_references(&mut asset);
        fixed_modified_by(&mut asset);
        fixed_exclude_modifier(&mut asset);
        chain_modifier(&mut asset);
        add_class_from_modifier(&mut asset);

        // assert_eq!(strip_icd10_to_dot("A910", 1), String::from("A91"));
        // dbg!(asset.classes.get("I70.20"));

        let mut result = Vec::new();
        for (code, vx) in vx_noex {
            let parent = asset.classes.get(&strip_icd10_to_dot(&code, 1)) 
                .or(asset.classes.get(&strip_icd10_to_dot(&code, 2)))
                .or(asset.classes.get(&strip_icd10_to_dot(&code, 3)));
            result.push((code, vx, parent));
        }

        let mut xml_map: BTreeMap<String, Vec<(String, [String; 6])>> = BTreeMap::new();
        let mut nested = Vec::new();
        let mut special = Vec::new();
        let mut modified_by_1 = Vec::new();
        let mut modified_by_2 = Vec::new();
        let mut not_found = Vec::new();
        let mut parent_code = String::new();
        for (code, vx, opt) in result.iter() {
            if let Some(parent) = opt {
                let preferred = parent.rubrics.iter().find(|(_, r)| matches!(r.kind, RubricKind::PreferredLong))
                    .or(parent.rubrics.iter().find(|(_, r)| matches!(r.kind, RubricKind::Preferred)))
                    .map(|(_, r)| r.text.to_owned()).or(parent.with_modifier_class_1.clone()).or(parent.with_modifier_class_2.clone()).unwrap_or(String::from("???"));
                if parent.with_modifier_class_1.is_some() {
                    if parent_code != parent.code {
                        modified_by_1.push([&parent.code, " : ", &preferred].concat());
                        parent_code = parent.code.clone();
                    }
                    modified_by_1.push(["    ", code, " : ", &vx.desc].concat());
                } else if parent.with_modifier_class_2.is_some() {
                    if parent_code != parent.code {
                        modified_by_2.push([&parent.code, " : ", &preferred].concat());
                        parent_code = parent.code.clone();
                    }
                    modified_by_2.push(["    ", code, " : ", &vx.desc].concat());
                // F20.x use Modifier,
                // I70.3, M62.7, S32.6 are missing subclass, we insert to icd102016en-tm.xml manually
                } else if ["F20.","I70","M62","S32"].iter().any(|m| parent.code.starts_with(m)) {
                    if parent_code != parent.code {
                        special.push([&parent.code, " : ", &preferred].concat());
                        parent_code = parent.code.clone();
                    }
                    special.push(["    ", code, " : ", &vx.desc].concat());
                } else if (code.len() - parent.code.replace('.',"").len()) > 1 {
                    if parent_code != parent.code {
                        nested.push([&parent.code, " : ", &preferred].concat());
                        parent_code = parent.code.clone();
                    }
                    nested.push([&parent.code, " : " , code, " : ", &vx.desc].concat());
                } else {
                    let code_dot = icd10_dot(&code);
                    let subclass = ["        <SubClass code=\"", &code_dot, "\"/>"].concat();
                    let lines = [
                        ["    <Class code=\"", &code_dot, "\" kind=\"category\">"].concat(),
                        ["        <SuperClass code=\"", &parent.code, "\"/>"].concat(),
                        ["        <Rubric id=\"id-ICD10TM2016_v2016-September-11-TM_", &code.replace('.', ""), "-00\" kind=\"preferred\">"].concat(),
                        ["            <Label xml:lang=\"en\" xml:space=\"default\">", &vx.desc, "</Label>"].concat(),
                        String::from("        </Rubric>"),
                        String::from("    </Class>"),
                    ];
                    
                    xml_map.entry(parent.code.to_owned()).or_insert(Vec::new()).push((subclass, lines));
                    // if parent_code != parent.code {
                    //     normal.push([&parent.code, " : ", &preferred].concat());
                    //     parent_code = parent.code.clone();
                    // }
                    // normal.push(["    ", code, " : ", &vx.desc].concat());
                }
            } else {
                not_found.push(["??? : ", code, " : ", &vx.desc].concat());
            }
        }
        let xml_lines = xml_map.values().map(|tuples| {
            let (subclasses, classes): (Vec<String>, Vec<[String; 6]>) = tuples.clone().into_iter().unzip();
            [subclasses, classes.into_iter().flatten().collect::<Vec<String>>(), vec![String::new()]]
        }).flatten().flatten().collect::<Vec<String>>();
        write_tm_over("2-vx-noex-to-xml.xml", &xml_lines);
        write_tm_over("2-vx-noex-with-nested.txt", &nested);
        write_tm_over("2-vx-noex-with-special.txt", &special);
        write_tm_over("2-vx-noex-with-mod-by-1.txt", &modified_by_1);
        write_tm_over("2-vx-noex-with-mod-by-2.txt", &modified_by_2);
        write_tm_over("2-vx-noex-with-no-parent.txt", &not_found);
    }

    fn strip_icd10_to_dot(no_dot: &str, strip_size: usize) -> String {
        let mut clone = no_dot.to_string();
        let _ = clone.split_off(no_dot.len() - strip_size);
        icd10_dot(&clone)
    }

    fn write_tm_over(file_name: &str, lines: &Vec<String>) {
        let mut path = std::env::current_dir().unwrap();
        path.push("debug");
        path.push("claml-vx-diff");
        if !std::fs::exists(&path).unwrap() {
            std::fs::create_dir_all(&path).unwrap();
        }
        path.push(file_name);
        let file = std::fs::File::create(path).unwrap();
        let mut writer = BufWriter::new(file);
        for line in lines {
            writeln!(writer, "{line}").unwrap();
        }
    }

    fn count_nest_superclass(asset: &I10Claml) {

        let mut first_count = 0;
        let mut second_count = 0;
        let mut third_count = 0;
        let mut fourth_count = 0;
        let mut fifth_count = 0;
        let mut sixth_count = 0;
        let mut seventh_count = 0;

        for (_, class) in asset.classes.iter() {
            if let Some(first_parent_code) = &class.superclass {
                if let Some(first_parent) = asset.classes.get(first_parent_code) {
                    first_count += 1;
                    if let Some(second_parent_code) = &first_parent.superclass {
                        if let Some(second_parent) = asset.classes.get(second_parent_code) {
                            second_count += 1;
                            if let Some(third_parent_code) = &second_parent.superclass {
                                if let Some(third_parent) = asset.classes.get(third_parent_code) {
                                    third_count += 1;
                                    if let Some(fourth_parent_code) = &third_parent.superclass {
                                        if let Some(fourth_parent) = asset.classes.get(fourth_parent_code) {
                                            fourth_count += 1;
                                            if let Some(fifth_parent_code) = &fourth_parent.superclass {
                                                if let Some(fifth_parent) = asset.classes.get(fifth_parent_code) {
                                                    fifth_count += 1;
                                                    if let Some(sixth_parent_code) = &fifth_parent.superclass {
                                                        if let Some(sixth_parent) = asset.classes.get(sixth_parent_code) {
                                                            sixth_count += 1;
                                                            if let Some(seventh_parent_code) = &sixth_parent.superclass {
                                                                if let Some(_seventh_parent) = asset.classes.get(seventh_parent_code) {
                                                                    seventh_count += 1;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        assert_eq!(first_count, 12161);
        assert_eq!(second_count, 11950);
        assert_eq!(third_count, 10329);
        assert_eq!(fourth_count, 2459);
        assert_eq!(fifth_count, 530);
        assert_eq!(sixth_count, 21);
        assert_eq!(seventh_count, 0);
    }

    #[allow(dead_code)]
    fn test_with_print(asset: &I10Claml) {
        // check class with ModifiedBy and SubClass
        for (code, class) in asset.classes.iter().filter(|(_, c)| {
            (c.modified_by_1.is_some() || c.modified_by_2.is_some()) && !c.subclasses.is_empty()
        }) {
            println!("{} has SubClass and ModifiedBy [{}, {}]", code, class.modified_by_1.clone().unwrap_or_default(), class.modified_by_2.clone().unwrap_or_default());
        }

        // check class with ExcludeModifier and SuperClass
        for (code, class) in asset.classes.iter().filter(|(_, c)| {
            !c.exclude_modifiers.is_empty() && c.superclass.is_some()
        }) {
            println!("{} has SuperClass and ExcludeModifier [{}]", code, class.exclude_modifiers.join(", "));
        }
    }

    #[test]
    fn test_get_claml_asterisk_dagger() {
        let verbose = false;

        let claml_all_pairs = get_claml_asterisk_dagger();
        assert_eq!(claml_all_pairs.keys().len(), 433);

        if verbose {
            let keyword = ["A181".to_string(), "N518".to_string()].into_iter().collect::<HashSet<String>>();
            let start_all_pairs = std::time::Instant::now();
            let all_result = find_dagger_aster_pairs(&claml_all_pairs, &keyword);
            let all_duration = start_all_pairs.elapsed();
            println!("All execution time: {:.2?}\n", all_duration);
            println!("All found '{}' pairs", all_result.len());
        }
    }

    /// use ICD10 without dot
    /// find asterisk in codes -> find dagger in codes
    fn find_dagger_aster_pairs(stores: &HashMap<String, HashSet<String>>, codes: &HashSet<String>) -> Vec<(Option<String>, String)> {
        codes
            .iter()
            .filter_map(|aster| {
                let mut rest = codes.clone();
                rest.remove(aster);
                stores.get(aster).map(|daggers| {
                    let results = daggers.intersection(&rest).collect::<Vec<&String>>();
                    if results.is_empty() {
                        // Asterisk without Dagger matched
                        vec![(None, aster.to_owned())]
                    } else {
                        results.into_iter().map(|dagger| (Some(dagger.to_owned()), aster.to_owned())).collect()
                    }
                })
            })
            .flatten()
            .collect()
    }

    #[test]
    fn test_get_code() {
        let verbose = false;

        let codes = crate::drg_grouper::new_grouper().valid_codes();
        // let codes = claml.classes.keys().into_iter().filter(|c| !c.contains('-')).cloned().collect::<Vec<String>>();
        let vectores = ["N08.2", "D57.-", "E00-E34", "C00-D48", "E10-E14 with common fourth character .3"];
        for vector in vectores {
            let res = get_codes(vector, &codes);
            if verbose {
                println!("{} has {} codes", vector, res.len());
            }
        }
    }

    #[test]
    fn test_get_single_code() {
        let verbose = false;

        let claml = new_i10_claml();
        let result = claml.get_detail("E74.8");
        if verbose && let Some(res) = result {
            dbg!(res);
        }
    }
}
