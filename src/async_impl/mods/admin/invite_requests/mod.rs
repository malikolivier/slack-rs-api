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

pub mod approved;
pub mod denied;

use crate::async_impl::SlackWebRequestSender;
pub use crate::mod_types::admin::invite_requests::*;

/// Approve a workspace invite request.
///
/// Wraps https://api.slack.com/methods/admin.inviteRequests.approve

pub async fn approve<R>(
    client: &R,
    token: &str,
    request: &ApproveRequest,
) -> Result<ApproveResponse, ApproveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("invite_request_id", request.invite_request_id.to_string())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.inviteRequests.approve");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .await
        .map_err(ApproveError::Client)
        .and_then(|result| {
            serde_json::from_str::<ApproveResponse>(&result)
                .map_err(|e| ApproveError::MalformedResponse(result, e))
        })
}
/// Deny a workspace invite request.
///
/// Wraps https://api.slack.com/methods/admin.inviteRequests.deny

pub async fn deny<R>(
    client: &R,
    token: &str,
    request: &DenyRequest,
) -> Result<DenyResponse, DenyError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("invite_request_id", request.invite_request_id.to_string())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.inviteRequests.deny");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .await
        .map_err(DenyError::Client)
        .and_then(|result| {
            serde_json::from_str::<DenyResponse>(&result)
                .map_err(|e| DenyError::MalformedResponse(result, e))
        })
}
/// List all pending workspace invite requests.
///
/// Wraps https://api.slack.com/methods/admin.inviteRequests.list

pub async fn list<R>(
    client: &R,
    token: &str,
    request: &ListRequest,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token.to_string())),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.to_string())),
        request
            .limit
            .as_ref()
            .map(|limit| ("limit", limit.to_string())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.inviteRequests.list");
    client
        .get(&url, &params[..])
        .await
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
}
