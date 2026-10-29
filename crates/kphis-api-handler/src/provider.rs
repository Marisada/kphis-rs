use axum::{extract::{Extension, Query, State}, response::Redirect};
use moph_provider_client::response::OauthRedirectParams;
use tower_cookies::{Cookie, Cookies, cookie::SameSite};
use std::{collections::HashSet, net::SocketAddr};
use ulid::Ulid;

use kphis_api_core::{
    state::ApiState,
    token::{TokenType, gen_token_public},
};
use kphis_api_query::user::{config, login};
use kphis_model::user::permission::Permission;

use crate::user::his::COOKIE_TOKEN_NAME;

pub async fn get_oauth_uri(State(app): State<ApiState>) -> String {
    match app.provider_client.as_ref() {
        Some(client) => {
            let (uri, nonce) = client.generate_oauth_uri_with_state();
            app.provider_nonce_set.lock().await.insert(nonce);
            uri
        }
        None => String::new(),
    }
}

pub async fn get_redirect(
    Extension(real_addr): Extension<SocketAddr>,
    Query(params): Query<OauthRedirectParams>,
    State(app): State<ApiState>,
    cookies: Cookies,
) -> Redirect {
    tracing::debug!("params = {:?}", &params);
    if let Some(code) = params.code.as_ref() {
        if let Some(param_state) = params.state.as_ref() {
            let nonce_opt = app.provider_nonce_set.lock().await.take(param_state);
            tracing::debug!("nonce = {:?}", &nonce_opt);
            if nonce_opt.is_some() {
                match app.provider_client.as_ref() {
                    Some(client) => {
                        match client.get_hash_cid_uppercase_from_code(&code).await {
                            Ok(Some(hash_cid)) => {
                                match login::get_user_from_hash_cid(&hash_cid, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra()).await {
                                    Ok(user_opt) => {
                                        if let Some(user) = user_opt {
                                            // reset failed tp 0
                                            if user.failed.unwrap_or_default() > 0 {
                                                if let Err(e) = config::insert_dup_failed(0, &user.loginname, &app.db_pool, &app.kphis_extra()).await {
                                                    tracing::error!("Error insert_dup_failed: {}", e.message);
                                                }
                                            }
                                            // generate new state_id and refresh token
                                            let Ulid(state_id) = Ulid::generate();
                                            match gen_token_public(
                                                state_id,
                                                (app.refresh_limit(), app.refresh_limit()),
                                                None, // generate new rexp
                                                TokenType::Refresh,
                                                &app.paseto.secret,
                                            ) {
                                                Ok(refresh_token) => {
                                                    // create cookie of refresh token
                                                    let cookie = Cookie::build((COOKIE_TOKEN_NAME, refresh_token)).path("/").http_only(true).same_site(SameSite::Strict).secure(true);
                                                    cookies.add(cookie.into());

                                                    // roles
                                                    let roles = login::get_user_roles(&user.loginname, &app.db_pool, &app.hosxp(), &app.kphis()).await.unwrap_or_default();
                                                    let mut role_names: HashSet<String> = HashSet::new();
                                                    {
                                                        let all_roles = app.roles.lock().await;
                                                        for role in roles.iter() {
                                                            if let Some(with_parent) = all_roles.get(&role.role) {
                                                                role_names.extend(with_parent.clone());
                                                            }
                                                        }
                                                    }
                                                    let mut permissions_names: HashSet<Permission> = HashSet::new();
                                                    {
                                                        let all_roles_permissions = app.roles_permissions.lock().await;
                                                        for role_name in role_names {
                                                            if let Some(rp) = all_roles_permissions.get(&role_name) {
                                                                permissions_names.extend(rp.clone());
                                                            }
                                                        }
                                                    }
                                                    let permissions = permissions_names.into_iter().collect::<Vec<Permission>>();

                                                    // set backend users state
                                                    app.online_add(state_id, &user, &roles, &permissions, real_addr).await;

                                                    tracing::info!("User {} ({}) Log-in from {} by Provider ID", &user.name, &user.loginname, real_addr.to_string());
                                                    Redirect::to("/#/info")
                                                }
                                                Err(e) => {
                                                    tracing::error!("{}", e.message);
                                                    Redirect::to("/#/index")
                                                }
                                            }
                                        } else {
                                            tracing::warn!("User not found");
                                            Redirect::to("/#/index")
                                        }
                                    }
                                    Err(e) => {
                                        tracing::error!("{}", e.message);
                                        Redirect::to("/#/index")
                                    }
                                }
                            }
                            Ok(None) => {
                                tracing::warn!("Nothing returned");
                                Redirect::to("/#/index")
                            }
                            Err(e) => {
                                tracing::error!("{e}");
                                Redirect::to("/#/index")
                            }
                        }
                    }
                    None => {
                        tracing::debug!("Provider ID is not enable");
                        Redirect::to("/#/index")
                    }
                }
            } else {
                tracing::warn!("Nonce has been used / Not found");
                Redirect::to("/#/index")
            }
        } else {
            tracing::warn!("No state");
            Redirect::to("/#/index")
        }
    } else {
        tracing::warn!("No Code returned");
        Redirect::to("/#/index")
    }
}
