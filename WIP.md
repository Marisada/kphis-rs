## TODOs
> - Consider using https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html#json with `JSON_ARRAYAGG()` instead of `GROUP_CONCAT()` or `JSON_OBJECT()` instead of `CONCAT()` (MariaDB 10.2+ or MySQL 5.7.8+) but MariaDB says "The maximum returned length in bytes is determined by the group_concat_max_len server system variable" [ref](https://mariadb.com/kb/en/json_objectagg/)
> - Seperate 'edit' in POST to PUT
> - `Create` will return STATUS_CODE `201 Created` instead of `200`
> - `Delete` will return STATUS_CODE `204 No Content` instead of `200`
> - Revise table index for performance
> - use `REPLACE INTO` instead of `INSERT INTO` when PK is provided and needed to overwrite
> - Implement `Web Push` when user can use app outside hospital
> - Implement `Web Authentication` to use `passcode` or os-specific login
> - RTF parser from lab_head
> - DRY on dom
> - Medical equipment connection
> - Run GitHub Actions locally with [act](https://github.com/nektos/act) and [VS Code extension](https://sanjulaganepola.github.io/github-local-actions-docs/)
> - Migrate Localstorage to IndexedDb via https://github.com/Alorel/rust-indexed-db or https://github.com/devashishdxt/idb (https://github.com/devashishdxt/rexie and https://github.com/devashishdxt/deli) or https://github.com/Ekleog/indexed-db
> - Use SIMD json
> - Sanitize input (server side) [api path, api params, payload]
> - Handler test
> - UI test
> - Rehydrate from server when data changed
> - Change form grid to form inline in ipd_admission_note_dr, ipd_admission_note_nurse
> - Check DrugAllergy, DrugDuplication, DrugInteraction from Template
> - Memorize progress note
> - Order input using tab (with edited mark)
> - SSE message for med-reconcile, vital-sign ?, drug allergy comfirming, review chart
> - Logs viewer
> - Prescrition Screen can edit HOSxP `ptnote` table : prefered to do it in HOSxP
> - eMAR report: Add `pharmacist-done` name and datetime to order detail
> - Current Medication + Home Medication report
> - Rehab component
> - ThaiMed component
> - Report to database
> - * Migration files
> - * db-util flag for update system reports
> - * report-binding in config file
> - Component/Page static input must be Mutable ?
> - remove str_some allocation

## PREVENTIVE CODING
- USE Unique naming prefix for easier code review by search [`sqlx_` for sqlx tests, `call_api_` for fetch from client]
- USE Enum matching without `_ => {}` for detecting NEW varient
- ALWAYS Trim whitespace for all text input
- ALWAYS `urlencoding::encode` any text input in url, and `urlencoding::decode` it at API side
- NOT USE `clone!` without `move`
- USE `let v = mut Vec::with_capacity(n);` instead of `let c = mut Vec::new();` when capacity is calculatable 
- BEWARE USING `&string_mutable.lock_ref()` instead of `&string_mutable.get_cloned()`, may be PANIC due to LOCKED MUTABLE STATE ex: as argument of time consuming future (maybe mutate at another place), mutate when ref is in scope
- BEWARE USING `.signal_cloned().map(|x| {..})` instead of `.signal_ref(|x| {..})`, may be PANIC due to LOCKED MUTABLE STATE
- NOT USE `.children(&mut [])`, USE `.children([])` instead
- NOT USE `.prop_signal("value",..)` with `.apply(mixins::string..)` because `mixins::string..` use `.prop_signal("value",..)` internally
- USE `check_an_can_execute()` or `check_an_opt_can_execute()` on every execute query that involve `AN` (`Admited` or `PreAdmit`/`Revoked`)
- CREATE TEST for every query, `kphis-api-router::tests::all_and_not_login` for ALL API and `kphis-api-router::tests::all_query_params` for every API that can 400
- GUARDING UI element with `Route::has_permission()` and `App::endpoint_is_allow()` and comment every associated Function/Struct with `Method` and `Endpoint`

## CHECK BEFORE RELEASE
- Call `cargo generate-lockfile` to check any upgradable crate version
- Call `cargo audit` ([cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)` needed)
- Call `cargo fmt --all` and use `#[rustfmt::skip]` to skip formatting in specific part
- Call `cargo-test`
- Call `cargo-test-wasm`
- Start `test_maria` docker and call `cargo-test-sql`, `cargo-test-api` and `cargo-test-pdf`
- Add changed to `volume/pwa/jsons/announcement.json`
- Add changed to `CHANGELOG.md`

## NOTE : Cargo Audit alert
> - `rsa` please follow https://github.com/RustCrypto/RSA/issues/19 and https://github.com/RustCrypto/RSA/pull/394
> - `paste` wait for `rav1e`,`hayagriva` and `biblatex` to change `paste` to a maintained one
> - `yaml-rust` wait for `syntect` and `two-face` to change `yaml-rust` to a maintained one

## NOTE : getrandom used by rand and ulid crate
> - v0.2 has a good supported for `wasm32-unknown-unknown` with `js` feature (has no effect on targets other than `wasm32-unknown-unknown`)
> - v0.3 need `rustflags = ['--cfg', 'getrandom_backend="wasm_js"']` for WASM and need crate `wasm_js` feature, can bloat Cargo.lock and is known to cause build issues on some non-web WASM platforms, even when a different backend is selected via getrandom_backend
> - v0.4 NOT support `wasm32-unknown-unknown`, read [getrandom](https://docs.rs/getrandom/latest/getrandom/) for more information
> - NOW we use only v0.2 and `Any crate included in frontend that use getrandom v0.4` will error on compile/runtime, ex. `rand`, `ulid`, `totp`, `orion` crate
