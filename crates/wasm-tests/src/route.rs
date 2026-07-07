use wasm_bindgen_test::*;

use kphis_model::route::Route;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn test_route_from_url_error() {
    let r = Route::from_url("localhost/#/ipd-main/nurse/1234/io/2020-12-34/12", "localhost");
    assert_eq!(
        r,
        Route::External {
            path: String::from("localhost/#/ipd-main/nurse/1234/io/2020-12-34/12")
        }
    );
}

#[wasm_bindgen_test]
pub fn test_route_from_url_hash() {
    let r = Route::from_url("#/info", "localhost");
    assert_eq!(r, Route::Info);
}

#[wasm_bindgen_test]
pub fn test_route_from_url_without_hash() {
    let r = Route::from_url("http://localhost/ipd-main", "localhost");
    assert_eq!(
        r,
        Route::External {
            path: String::from("http://localhost/ipd-main")
        }
    );
}

#[wasm_bindgen_test]
pub fn test_route_from_url_root() {
    let root_another_host = Route::from_url("http://somehost", "localhost");
    assert_eq!(
        root_another_host,
        Route::External {
            path: String::from("http://somehost")
        }
    );
    let root_without_slash = Route::from_url("http://localhost", "localhost");
    assert_eq!(root_without_slash, Route::Root);
    let root_with_slash = Route::from_url("http://localhost/", "localhost");
    assert_eq!(root_with_slash, Route::Root);
    let hash_without_slash = Route::from_url("http://localhost/#", "localhost");
    assert_eq!(hash_without_slash, Route::Root);
    let hash_with_slash = Route::from_url("http://localhost/#/", "localhost");
    assert_eq!(hash_with_slash, Route::Root);
}

#[wasm_bindgen_test]
pub fn test_route_from_url_with_path() {
    let no_slash_not_nested = Route::from_url("http://localhost/some#location", "localhost");
    assert_eq!(
        no_slash_not_nested,
        Route::External {
            path: String::from("http://localhost/some#location")
        }
    );
    let no_slash_nested = Route::from_url("http://localhost/some#other/location", "localhost");
    assert_eq!(
        no_slash_nested,
        Route::External {
            path: String::from("http://localhost/some#other/location")
        }
    );
    let with_slash = Route::from_url("http://localhost/some/#/ipd-main/1234/io/2020-12-34/12", "localhost");
    assert_eq!(
        with_slash,
        Route::External {
            path: String::from("http://localhost/some/#/ipd-main/1234/io/2020-12-34/12")
        }
    );
}

#[wasm_bindgen_test]
pub fn test_route_from_url_nested() {
    let r = Route::from_url("http://localhost/#/ipd-main/nurse/1234/io/2020-12-34/12", "localhost");
    assert_eq!(
        r,
        Route::IpdMain {
            view_by: String::from("nurse"),
            an: String::from("1234"),
            tab: String::from("io"),
            sub: String::from("2020-12-34"),
            id: 12,
        }
    );
}

#[wasm_bindgen_test]
pub fn test_route_from_url_hash_text() {
    let with_suffix = Route::from_url("http://localhost/#some", "localhost");
    assert_eq!(
        with_suffix,
        Route::External {
            path: String::from("http://localhost/#some")
        }
    );
    let with_suffix_nested = Route::from_url("http://localhost/#some/thing", "localhost");
    assert_eq!(
        with_suffix_nested,
        Route::External {
            path: String::from("http://localhost/#some/thing")
        }
    );
}

#[wasm_bindgen_test]
pub fn test_route_from_url_unknown_hash() {
    let r = Route::from_url("http://localhost/#/unknown/1234/io/2020-12-34/12", "localhost");
    assert_eq!(
        r,
        Route::NotFound {
            path: String::from("#/unknown/1234/io/2020-12-34/12")
        }
    );
}

#[wasm_bindgen_test]
pub fn test_route_from_empty_url() {
    let r = Route::from_url("", "localhost");
    assert_eq!(r, Route::NotFound { path: String::new() });
}

#[wasm_bindgen_test]
pub fn test_route_string() {
    let r = Route::Info;
    assert_eq!(r.string(), String::from("#/info"));

    let n = Route::NotFound { path: String::from("#/unknown") };
    assert_eq!(n.string(), String::from("#/notfound/#/unknown"));

    let u = Route::UnAuthorized { hash: String::from("#/info") };
    assert_eq!(u.string(), String::from("#/unauthorized/#/info"));

    let long = Route::IpdMain {
        view_by: String::from("nurse"),
        an: String::from("1234"),
        tab: String::from("io"),
        sub: String::from("2020-12-34"),
        id: 12,
    };
    assert_eq!(long.string(), String::from("#/ipd-main/nurse/1234/io/2020-12-34/12"));
}

#[wasm_bindgen_test]
pub fn test_route_from_hash() {
    let h = "#/info";
    assert_eq!(Route::from_hash(h), Route::Info);

    let n = "#/notfound/#/unknown";
    assert_eq!(Route::from_hash(n), Route::NotFound { path: String::from("#/unknown") });

    let u = "#/unauthorized/#/info";
    assert_eq!(Route::from_hash(u), Route::UnAuthorized { hash: String::from("#/info") });

    let long = "#/ipd-main/nurse/1234/io/2020-12-34/12";
    assert_eq!(
        Route::from_hash(long),
        Route::IpdMain {
            view_by: String::from("nurse"),
            an: String::from("1234"),
            tab: String::from("io"),
            sub: String::from("2020-12-34"),
            id: 12,
        }
    );
}
