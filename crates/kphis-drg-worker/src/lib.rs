pub mod drg;
pub mod i10;

// try removing `kphis_worker::api! {` + `}` container to debug
kphis_worker::api! {

    pub async fn run(input_json: String) -> String {
        drg::grouper::GROUPER.run(&input_json)
    }

    pub async fn get_icd10_desc(i10tm: String) -> String {
        drg::grouper::GROUPER.i10vx.get(&i10tm).map(|vx| vx.desc.to_owned()).unwrap_or_default()
    }

    pub async fn get_icd10_ex_desc(i10tm: String) -> String {
        drg::grouper::GROUPER.i10vx_ex.get(&i10tm).map(|vx| vx.desc.to_owned()).unwrap_or_default()
    }

    pub async fn get_proc_desc(i9cm: String) -> String {
        drg::grouper::GROUPER.i9vx.get(&i9cm).map(|p| p.desc.to_owned()).unwrap_or_default()
    }

    // Vec<(Arc<I10vx>, f32, u8)>
    pub async fn search_icd10_desc(text: String) -> Vec<u8> {
        let v = drg::grouper::GROUPER.search_i10vx_desc(false, &text);
        bitcode::encode(&v)
    }

    // Vec<(Arc<I10vx>, f32, u8)>
    pub async fn search_icd10_ex_desc(text: String) -> Vec<u8> {
        let v = drg::grouper::GROUPER.search_i10vx_desc(true, &text);
        bitcode::encode(&v)
    }

    // Vec<(Arc<I9vx>, f32, u8)>
    pub async fn search_proc_desc(text: String) -> Vec<u8> {
        let v = drg::grouper::GROUPER.search_proc_desc(&text);
        bitcode::encode(&v)
    }

    // Vec<(Arc<I10vx>, f32, u8)>
    pub async fn search_icd10_code_prefix(prefix: String) -> Vec<u8> {
        let v = drg::grouper::GROUPER.search_i10vx_code_prefix(false, &prefix);
        bitcode::encode(&v)
    }

    // Vec<(Arc<I10vx>, f32, u8)>
    pub async fn search_icd10_ex_code_prefix(prefix: String) -> Vec<u8> {
        let v = drg::grouper::GROUPER.search_i10vx_code_prefix(true, &prefix);
        bitcode::encode(&v)
    }

    // Vec<(Arc<I9vx>, f32, u8)>
    pub async fn search_proc_code_prefix(prefix: String) -> Vec<u8> {
        let v = drg::grouper::GROUPER.search_proc_code_prefix(&prefix);
        bitcode::encode(&v)
    }

    // // Option<Arc<I9vx>>
    // pub async fn search_proc_code_exact(proc: String) -> Vec<u8> {
    //     let v = drg::grouper::GROUPER.search_proc_exact(&proc);
    //     bitcode::encode(&v)
    // }

    // Option<Arc<I10Detail>>
    pub async fn get_i10_detail(code: String) -> Vec<u8> {
        let v = i10::I10_DETAIL.get_detail(&code);
        bitcode::encode(&v)
    }

    // Vec<((String, Arc<I10Pointer>), f32, u8)>
    pub async fn search_i10_index_diagnosis(code: String) -> Vec<u8> {
        let v = i10::I10_INDEX.search_diagnosis(&code);
        bitcode::encode(&v)
    }

    // Vec<((String, Arc<I10Pointer>), f32, u8)>
    pub async fn search_i10_index_external(code: String) -> Vec<u8> {
        let v = i10::I10_INDEX.search_external(&code);
        bitcode::encode(&v)
    }

    // Vec<((String, Arc<I10Pointer>), f32, u8)>
    pub async fn search_i10_index_substance(code: String) -> Vec<u8> {
        let v = i10::I10_INDEX.search_substance(&code);
        bitcode::encode(&v)
    }

    // use ICD10 uppercase without dot
    // input: HashSet<String>
    // output: (Vec<(Option<String>, String)>, HashMap<String, Arc<I10vx>>) as (Vec<(Option(Dagger), Asterisk)>, HashMap<code, I10vx>
    pub async fn find_dagger_asterisk_pairs(bytes: Vec<u8>) -> Vec<u8> {
        if let Ok(codes) = bitcode::decode::<std::collections::HashSet<String>>(&bytes) {
            let map = codes.iter().filter_map(|code| {
                if code.starts_with(['V', 'W', 'X', 'Y']) {
                    drg::grouper::GROUPER.i10vx_ex.get(code).map(|vx| (code.to_owned(), vx.clone()))
                } else {
                    drg::grouper::GROUPER.i10vx.get(code).map(|vx| (code.to_owned(), vx.clone()))
                }
            }).collect::<std::collections::HashMap<String, std::sync::Arc<crate::drg::model::I10vx>>>();
            let codes_no_ex = codes.iter().filter_map(|c| {
                (!c.starts_with(['V', 'W', 'X', 'Y'])).then(|| c.to_owned())
            }).collect::<std::collections::HashSet<String>>();
            let pairs = i10::I10_INDEX.find_dagger_aster_pairs(&codes_no_ex);

            bitcode::encode(&(pairs, map))
        } else {
            Vec::new()
        }
    }
}
