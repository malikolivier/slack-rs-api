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

#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use crate::async_impl::SlackWebRequestSender;
pub use crate::mod_types::reactions_types::*;

/// Adds a reaction to an item.
///
/// Wraps https://api.slack.com/methods/reactions.add

pub async fn add<R>(client: &R, request: &AddRequest) -> Result<AddResponse, AddError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("channel", request.channel.to_string())),
        Some(("name", request.name.to_string())),
        Some(("timestamp", request.timestamp.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/reactions.add");
    client
        .get(&url, &params[..])
        .await
        .map_err(AddError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddResponse>(&result)
                .map_err(|e| AddError::MalformedResponse(result, e))
        })
}
/// Gets reactions for an item.
///
/// Wraps https://api.slack.com/methods/reactions.get

pub async fn get<R>(client: &R, request: &GetRequest) -> Result<GetResponse, GetError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.to_string())),
        request.file.as_ref().map(|file| ("file", file.to_string())),
        request
            .file_comment
            .as_ref()
            .map(|file_comment| ("file_comment", file_comment.to_string())),
        request.full.as_ref().map(|full| ("full", full.to_string())),
        request
            .timestamp
            .as_ref()
            .map(|timestamp| ("timestamp", timestamp.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/reactions.get");
    client
        .get(&url, &params[..])
        .await
        .map_err(GetError::Client)
        .and_then(|result| {
            serde_json::from_str::<GetResponse>(&result)
                .map_err(|e| GetError::MalformedResponse(result, e))
        })
}
/// Lists reactions made by a user.
///
/// Wraps https://api.slack.com/methods/reactions.list

pub async fn list<R>(client: &R, request: &ListRequest) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .count
            .as_ref()
            .map(|count| ("count", count.to_string())),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.to_string())),
        request.full.as_ref().map(|full| ("full", full.to_string())),
        request
            .limit
            .as_ref()
            .map(|limit| ("limit", limit.to_string())),
        request.page.as_ref().map(|page| ("page", page.to_string())),
        request.user.as_ref().map(|user| ("user", user.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/reactions.list");
    client
        .get(&url, &params[..])
        .await
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
}
/// Removes a reaction from an item.
///
/// Wraps https://api.slack.com/methods/reactions.remove

pub async fn remove<R>(
    client: &R,
    request: &RemoveRequest,
) -> Result<RemoveResponse, RemoveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.to_string())),
        request.file.as_ref().map(|file| ("file", file.to_string())),
        request
            .file_comment
            .as_ref()
            .map(|file_comment| ("file_comment", file_comment.to_string())),
        Some(("name", request.name.to_string())),
        request
            .timestamp
            .as_ref()
            .map(|timestamp| ("timestamp", timestamp.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/reactions.remove");
    client
        .get(&url, &params[..])
        .await
        .map_err(RemoveError::Client)
        .and_then(|result| {
            serde_json::from_str::<RemoveResponse>(&result)
                .map_err(|e| RemoveError::MalformedResponse(result, e))
        })
}
