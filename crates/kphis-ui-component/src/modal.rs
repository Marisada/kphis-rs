pub mod consult_form;
pub mod drug_details;
pub mod drug_duplication;
pub mod drug_interaction;
pub mod drug_notify;
pub mod index_note_form;
pub mod index_plan_action_form;
pub mod ipd_passcode;
pub mod lab_history;
pub mod lab_selector;
pub mod lab_wbc;
pub mod med_reconcile_remed;
pub mod medplan_form;
pub mod opd_er_order_new;
pub mod pre_admit_new;
pub mod pre_order_new;
pub mod pre_order_preview;
pub mod pre_order_select;
pub mod report;
pub mod scoring;
pub mod vs_selector;

use dominator::{Dom, html};

// Bootstrap5 modal always need modal gut
pub fn blank_modal() -> Dom {
    html!("div", {
        .class("modal-dialog")
        .attr("role", "document")
        .child(html!("div", {
            .class("modal-content")
            .child(html!("div", {.class("modal-body")}))
        }))
    })
}
