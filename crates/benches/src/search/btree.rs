use kphis_drg_worker::drg::model::I10vx;
use kphis_util::util::next_key;
use std::{collections::BTreeMap, sync::Arc};

// I10vx::search_with_prefix()
pub(crate) fn search_i10vx_btree(text: &str, source: &BTreeMap<String, Arc<I10vx>>) -> Vec<((String, Arc<I10vx>), f32, u8)> {
    let key = text.replace('.', "").to_ascii_uppercase();
    source.range(key.clone()..next_key(&key)).map(|(code, vx)| ((code.to_owned(), vx.to_owned()), 1.0, 1)).collect()
}
