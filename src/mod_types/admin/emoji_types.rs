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
#![allow(dead_code)]

use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct AddRequest {
    /// The name of the emoji to be removed. Colons (`:myemoji:`) around the value are not required, although they may be included.
    pub name: String,
    /// Authentication token. Requires scope: `admin.teams:write`
    pub token: String,
    /// The URL of a file to use as an image for the emoji. Square images under 128KB and with transparent backgrounds work best.
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<AddResponse, AddError<E>>> for AddResponse {
    fn into(self) -> Result<AddResponse, AddError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(AddError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum AddError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for AddError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => AddError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for AddError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AddError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            AddError::Unknown(ref s) => write!(f, "{}", s),
            AddError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for AddError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            AddError::MalformedResponse(_, ref e) => Some(e),
            AddError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct AddAliasRequest {
    /// The alias of the emoji.
    pub alias_for: String,
    /// The name of the emoji to be aliased. Colons (`:myemoji:`) around the value are not required, although they may be included.
    pub name: String,
    /// Authentication token. Requires scope: `admin.teams:write`
    pub token: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddAliasResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<AddAliasResponse, AddAliasError<E>>> for AddAliasResponse {
    fn into(self) -> Result<AddAliasResponse, AddAliasError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(AddAliasError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum AddAliasError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for AddAliasError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => AddAliasError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for AddAliasError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AddAliasError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            AddAliasError::Unknown(ref s) => write!(f, "{}", s),
            AddAliasError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for AddAliasError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            AddAliasError::MalformedResponse(_, ref e) => Some(e),
            AddAliasError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct ListRequest {
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page
    pub cursor: Option<String>,
    /// The maximum number of items to return. Must be between 1 - 1000 both inclusive.
    pub limit: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<ListResponse, ListError<E>>> for ListResponse {
    fn into(self) -> Result<ListResponse, ListError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(ListError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum ListError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for ListError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => ListError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ListError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ListError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            ListError::Unknown(ref s) => write!(f, "{}", s),
            ListError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for ListError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ListError::MalformedResponse(_, ref e) => Some(e),
            ListError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct RemoveRequest {
    /// The name of the emoji to be removed. Colons (`:myemoji:`) around the value are not required, although they may be included.
    pub name: String,
    /// Authentication token. Requires scope: `admin.teams:write`
    pub token: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RemoveResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<RemoveResponse, RemoveError<E>>> for RemoveResponse {
    fn into(self) -> Result<RemoveResponse, RemoveError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(RemoveError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum RemoveError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for RemoveError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => RemoveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RemoveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RemoveError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            RemoveError::Unknown(ref s) => write!(f, "{}", s),
            RemoveError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for RemoveError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            RemoveError::MalformedResponse(_, ref e) => Some(e),
            RemoveError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct RenameRequest {
    /// The name of the emoji to be renamed. Colons (`:myemoji:`) around the value are not required, although they may be included.
    pub name: String,
    /// The new name of the emoji.
    pub new_name: String,
    /// Authentication token. Requires scope: `admin.teams:write`
    pub token: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<RenameResponse, RenameError<E>>> for RenameResponse {
    fn into(self) -> Result<RenameResponse, RenameError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(RenameError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum RenameError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for RenameError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => RenameError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RenameError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RenameError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            RenameError::Unknown(ref s) => write!(f, "{}", s),
            RenameError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for RenameError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            RenameError::MalformedResponse(_, ref e) => Some(e),
            RenameError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}