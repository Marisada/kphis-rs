use dominator::{Dom, html};
use std::sync::LazyLock;

use kphis_ui_core::class;

pub static MRA_NA: LazyLock<MraDoc> = LazyLock::new(|| {
    MraDoc::div_head_tail(
        "NA",
        "หมายถึง เวชระเบียนฉบับนั้นไม่จำเป็นต้องมีบันทึกเกี่ยวกับหัวข้อเรื่องนั้นๆ เนื่องจากไม่มีส่วนเกี่ยวข้องกับการให้บริการ ให้กากบาทช่อง NA",
        Vec::new(),
    )
});

pub static MRA_MISSING: LazyLock<MraDoc> = LazyLock::new(|| {
    MraDoc::div_head_tail(
        "Missing",
        "หมายถึง เวชระเบียนฉบับนั้นจำเป็นต้องมีบันทึกเกี่ยวกับหัวข้อเรื่องนั้นๆ แต่ปรากฏว่าไม่มีเอกสารที่เกี่ยวข้องให้ประเมิน ให้กากบาทช่อง Missing",
        Vec::new(),
    )
});

pub static MRA_NO: LazyLock<MraDoc> = LazyLock::new(|| MraDoc::div_head_tail("No", "หมายถึง มีเอกสารที่เกียวข้องให้ประเมิน แต่ไม่มีการบันทึก ให้กากบาทลงในช่อง No", Vec::new()));

#[derive(Clone, Default)]
pub struct MraDoc {
    pub indent: bool,
    pub tag: MraDocTag,
    pub document: Vec<MraInline>,
    pub con_tag: MraDocTag,
    pub items: Vec<MraDoc>,
}

impl MraDoc {
    pub fn dom(&self) -> Dom {
        html!(self.tag.tag(), {
            .apply_if(self.indent, |dom| dom.class("ms-3"))
            .apply_if(matches!(self.tag, MraDocTag::Ol), |dom| dom.style("list-style","none"))
            .children(self.document.iter().map(|doc| doc.dom()))
            .apply_if(!self.items.is_empty(), |dom| { dom
                .child(html!(self.con_tag.tag(), {
                    .apply_if(matches!(self.con_tag, MraDocTag::Ol), |d| d.style("list-style","none"))
                    .children(self.items.iter().map(|item| item.dom()))
                }))
            })
        })
    }

    pub fn br() -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Br,
            document: Vec::new(),
            con_tag: MraDocTag::Div,
            items: Vec::new(),
        }
    }

    pub fn con_div(items: Vec<MraDoc>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Div,
            document: Vec::new(),
            con_tag: MraDocTag::Div,
            items,
        }
    }

    pub fn div(txt: &'static str, items: Vec<MraDoc>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Div,
            document: vec![MraInline::N(txt)],
            con_tag: MraDocTag::Div,
            items,
        }
    }

    pub fn div_i(txt: &'static str, items: Vec<MraDoc>) -> Self {
        Self {
            indent: true,
            tag: MraDocTag::Div,
            document: vec![MraInline::N(txt)],
            con_tag: MraDocTag::Div,
            items,
        }
    }

    pub fn divs(document: Vec<MraInline>, items: Vec<MraDoc>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Div,
            document,
            con_tag: MraDocTag::Div,
            items,
        }
    }

    pub fn divs_i(document: Vec<MraInline>, items: Vec<MraDoc>) -> Self {
        Self {
            indent: true,
            tag: MraDocTag::Div,
            document,
            con_tag: MraDocTag::Div,
            items,
        }
    }

    pub fn div_bp(txt: &'static str, items: Vec<MraDoc>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Div,
            document: vec![MraInline::Bp(txt)],
            con_tag: MraDocTag::Div,
            items,
        }
    }

    pub fn div_bbp(txt: &'static str, items: Vec<MraDoc>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Div,
            document: vec![MraInline::Bbp(txt)],
            con_tag: MraDocTag::Div,
            items,
        }
    }

    pub fn div_bbp_ol(txt: &'static str, items: Vec<MraDoc>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Div,
            document: vec![MraInline::Bbp(txt)],
            con_tag: MraDocTag::Ol,
            items,
        }
    }

    pub fn div_bbp_ul(txt: &'static str, items: Vec<MraDoc>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Div,
            document: vec![MraInline::Bbp(txt)],
            con_tag: MraDocTag::Ul,
            items,
        }
    }

    pub fn div_bbp_i(txt: &'static str, items: Vec<MraDoc>) -> Self {
        Self {
            indent: true,
            tag: MraDocTag::Div,
            document: vec![MraInline::Bbp(txt)],
            con_tag: MraDocTag::Div,
            items,
        }
    }

    pub fn div_bbn(txt: &'static str, items: Vec<MraDoc>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Div,
            document: vec![MraInline::Bbn(txt)],
            con_tag: MraDocTag::Div,
            items,
        }
    }

    pub fn div_head_tail_ol(head: &'static str, tail: &'static str, items: Vec<MraDoc>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Div,
            document: vec![MraInline::Bp(head), MraInline::N(tail)],
            con_tag: MraDocTag::Ol,
            items,
        }
    }

    pub fn div_head_btail_ol(head: &'static str, tail: &'static str, items: Vec<MraDoc>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Div,
            document: vec![MraInline::Bp(head), MraInline::B(tail)],
            con_tag: MraDocTag::Ol,
            items,
        }
    }

    pub fn div_head_tail_ul(head: &'static str, tail: &'static str, items: Vec<MraDoc>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Div,
            document: vec![MraInline::Bp(head), MraInline::N(tail)],
            con_tag: MraDocTag::Ul,
            items,
        }
    }

    pub fn div_head_tail(head: &'static str, tail: &'static str, items: Vec<MraDoc>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Div,
            document: vec![MraInline::Bp(head), MraInline::N(tail)],
            con_tag: MraDocTag::Div,
            items,
        }
    }

    pub fn div_head_tails(head: &'static str, tails: Vec<MraInline>, items: Vec<MraDoc>) -> Self {
        let mut document = vec![MraInline::Bp(head)];
        document.extend(tails);
        Self {
            indent: false,
            tag: MraDocTag::Div,
            document,
            con_tag: MraDocTag::Div,
            items,
        }
    }

    pub fn div_head_tails_ol(head: &'static str, tails: Vec<MraInline>, items: Vec<MraDoc>) -> Self {
        let mut document = vec![MraInline::Bp(head)];
        document.extend(tails);
        Self {
            indent: false,
            tag: MraDocTag::Div,
            document,
            con_tag: MraDocTag::Ol,
            items,
        }
    }

    pub fn div_head_tails_ul(head: &'static str, tails: Vec<MraInline>, items: Vec<MraDoc>) -> Self {
        let mut document = vec![MraInline::Bp(head)];
        document.extend(tails);
        Self {
            indent: false,
            tag: MraDocTag::Div,
            document,
            con_tag: MraDocTag::Ul,
            items,
        }
    }

    pub fn div_head_tail_i(head: &'static str, tail: &'static str, items: Vec<MraDoc>) -> Self {
        Self {
            indent: true,
            tag: MraDocTag::Div,
            document: vec![MraInline::Bp(head), MraInline::N(tail)],
            con_tag: MraDocTag::Div,
            items,
        }
    }

    pub fn div_ol(txt: &'static str, items: Vec<MraDoc>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Div,
            document: vec![MraInline::N(txt)],
            con_tag: MraDocTag::Ol,
            items,
        }
    }

    pub fn div_ul(txt: &'static str, items: Vec<MraDoc>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Div,
            document: vec![MraInline::N(txt)],
            con_tag: MraDocTag::Ul,
            items,
        }
    }

    pub fn divs_ul(document: Vec<MraInline>, items: Vec<MraDoc>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Div,
            document,
            con_tag: MraDocTag::Ul,
            items,
        }
    }

    pub fn li(txt: &'static str) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Li,
            document: vec![MraInline::N(txt)],
            con_tag: MraDocTag::Div,
            items: Vec::new(),
        }
    }

    pub fn lis(document: Vec<MraInline>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Li,
            document,
            con_tag: MraDocTag::Div,
            items: Vec::new(),
        }
    }

    pub fn li_bbp(txt: &'static str) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Li,
            document: vec![MraInline::Bbp(txt)],
            con_tag: MraDocTag::Div,
            items: Vec::new(),
        }
    }

    pub fn li_red(txt: &'static str) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Li,
            document: vec![MraInline::Np(txt)],
            con_tag: MraDocTag::Div,
            items: Vec::new(),
        }
    }

    pub fn li_ol(txt: &'static str, items: Vec<MraDoc>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Li,
            document: vec![MraInline::N(txt)],
            con_tag: MraDocTag::Ol,
            items,
        }
    }

    pub fn lis_ol(document: Vec<MraInline>, items: Vec<MraDoc>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Li,
            document,
            con_tag: MraDocTag::Ol,
            items,
        }
    }

    pub fn li_ul(txt: &'static str, items: Vec<MraDoc>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Li,
            document: vec![MraInline::N(txt)],
            con_tag: MraDocTag::Ul,
            items,
        }
    }

    pub fn li_div(txt: &'static str, items: Vec<MraDoc>) -> Self {
        Self {
            indent: false,
            tag: MraDocTag::Li,
            document: vec![MraInline::N(txt)],
            con_tag: MraDocTag::Div,
            items,
        }
    }
}

// use block element
#[derive(Clone, Default)]
pub enum MraDocTag {
    #[default]
    Div,
    Ul, // always use `list-style: none;`
    Ol,
    Li,
    Br, // new line
}

impl MraDocTag {
    pub fn tag(&self) -> &'static str {
        match self {
            MraDocTag::Div => "div",
            MraDocTag::Ul => "ul",
            MraDocTag::Ol => "ol",
            MraDocTag::Li => "li",
            MraDocTag::Br => "br",
        }
    }
}

// use <span>
#[derive(Clone)]
pub enum MraInline {
    N(&'static str),   // normal
    B(&'static str),   // bold
    U(&'static str),   // underline
    Nbu(&'static str), // normal + bold + underline
    Np(&'static str),  // normal + pink
    Bp(&'static str),  // bold + pink
    Bbn(&'static str), // big + bold
    Bbp(&'static str), // big + bold + pink
}

impl MraInline {
    pub fn dom(&self) -> Dom {
        match self {
            MraInline::N(txt) => html!("span", {.class("me-1").text(txt)}),
            MraInline::B(txt) => html!("span", {.class(class::BOLD).text(txt)}),
            MraInline::U(txt) => html!("span", {.class(class::TXT_U_L).text(txt)}),
            MraInline::Nbu(txt) => html!("span", {.class(class::BOLD_U_L).text(txt)}),
            MraInline::Np(txt) => html!("span", {.class(class::TXT_RED_L).text(txt)}),
            MraInline::Bp(txt) => html!("span", {.class(class::BOLD_RED_L).text(txt)}),
            MraInline::Bbn(txt) => html!("span", {.class(class::BOLD_FS5_L).text(txt)}),
            MraInline::Bbp(txt) => html!("span", {.class(class::TXT_BOLD_RED_FS5_L).text(txt)}),
        }
    }
}

impl Default for MraInline {
    fn default() -> Self {
        Self::N("")
    }
}
