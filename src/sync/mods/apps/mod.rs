//=============================================================================
//
//                    WARNING: This file is AUTO-GENERATED
//
// Do not make changes directly to this file.
//
// If you would like to make a change to the library, please update the schema
// definitions at https://github.com/slack-rs/slack-api-schemas
//
// If you would like to make a change how the library was generated,
// please edit https://github.com/slack-rs/slack-rs-api/tree/master/codegen
//
//=============================================================================

#![allow(unused_imports)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::blacklisted_name)]

pub mod event;
pub mod permissions;

pub use crate::mod_types::apps::*;
use crate::sync::SlackWebRequestSender;

/// Uninstalls your app from a workspace.
///
/// Wraps https://api.slack.com/methods/apps.uninstall

pub fn uninstall<R>(
    client: &R,
    token: Option<&str>,
    request: &UninstallRequest,
) -> Result<UninstallResponse, UninstallError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        token.map(|token| ("token", token.to_string())),
        request
            .client_id
            .as_ref()
            .map(|client_id| ("client_id", client_id.to_string())),
        request
            .client_secret
            .as_ref()
            .map(|client_secret| ("client_secret", client_secret.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/apps.uninstall");
    client
        .get(&url, &params[..])
        .map_err(UninstallError::Client)
        .and_then(|result| {
            serde_json::from_str::<UninstallResponse>(&result)
                .map_err(|e| UninstallError::MalformedResponse(result, e))
        })
}
