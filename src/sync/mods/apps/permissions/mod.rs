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

pub mod resources;
pub mod scopes;
pub mod users;

pub use crate::mod_types::apps::permissions::*;
use crate::sync::SlackWebRequestSender;

/// Returns list of permissions this app has on a team.
///
/// Wraps https://api.slack.com/methods/apps.permissions.info

pub fn info<R>(
    client: &R,
    token: Option<&str>,
    _request: &InfoRequest,
) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![token.map(|token| ("token", token.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/apps.permissions.info");
    client
        .get(&url, &params[..])
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result)
                .map_err(|e| InfoError::MalformedResponse(result, e))
        })
}
/// Allows an app to request additional scopes
///
/// Wraps https://api.slack.com/methods/apps.permissions.request

pub fn request<R>(
    client: &R,
    token: &str,
    request: &RequestRequest,
) -> Result<RequestResponse, RequestError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token.to_string())),
        Some(("scopes", request.scopes.to_string())),
        Some(("trigger_id", request.trigger_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/apps.permissions.request");
    client
        .get(&url, &params[..])
        .map_err(RequestError::Client)
        .and_then(|result| {
            serde_json::from_str::<RequestResponse>(&result)
                .map_err(|e| RequestError::MalformedResponse(result, e))
        })
}
