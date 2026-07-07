use wasm_bindgen_test::*;

use kphis_ui_core::token::get_claim_encoded_key_public;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// reverse of test_gen_new_access_token_and_get_claim_public()
#[wasm_bindgen_test]
pub fn test_get_claim_from_token_and_key() {
    let token = "v4.public.eyJzdWIiOiIwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDBaNyIsIm5hbWUiOiJzb21lb25lIiwiYWN0IjoiYWNjZXNzIiwiaWF0IjoxNzgzMzQwMjk5LCJleHAiOjE3ODMzNDA0NzksInJleHAiOjE3ODMzNDIwOTl9cUBKjubOwH7vSOlByZpQj120XNDvbkmXMbsG_03s5priVoWeC6StUiw7doeL5Qr1ycx7yOekxnMT4Z19C7VLAQ";
    let key = "-eD89Q9Wno5i5VjuL6Uyu0YoKdHeqq7njgHQay6KWDg";
    let iat = 1783340299u64;

    let claims = get_claim_encoded_key_public(token, key).unwrap();

    assert_eq!(claims.sub, String::from("000000000000000000000000Z7")); // Ulid(999).to_string());
    assert_eq!(claims.name, String::from("someone"));
    assert_eq!(claims.act, String::from("access"));
    assert_eq!(claims.iat, iat);
    assert_eq!(claims.exp, iat + (3 * 60));
    assert_eq!(claims.rexp, iat + (30 * 60));
}
