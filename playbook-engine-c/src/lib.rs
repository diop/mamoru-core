mod error;
mod parse;
mod playbook_dto;

use crate::error::ParseError;
use crate::parse::{parse_confirmations, parse_playbook, parse_trigger};
use chrono::{DateTime, Utc};
use playbook_engine::Engine;
use safer_ffi::prelude::*;
use std::error::Error;
use std::sync::{Mutex, MutexGuard, OnceLock};

static ENGINE: OnceLock<Mutex<Engine>> = OnceLock::new();

fn engine<'a>() -> MutexGuard<'a, Engine<'static>> {
    ENGINE
        .get_or_init(|| Mutex::new(Engine::new()))
        .lock()
        .unwrap()
}

#[derive_ReprC]
#[repr(C)]
pub struct FfiJsonResult {
    is_error: bool,
    error_message: Option<char_p::Box>,
    result_json: Option<char_p::Box>,
}

impl<T, E> From<Result<T, E>> for FfiJsonResult
where
    T: serde::Serialize,
    E: Error,
{
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(value) => Self::ok(value),
            Err(err) => Self::error(err),
        }
    }
}

impl FfiJsonResult {
    fn error<E: Error>(err: E) -> Self {
        Self {
            is_error: true,
            error_message: Some(format!("{}", err).try_into().unwrap()),
            result_json: None,
        }
    }

    fn ok<T: serde::Serialize>(data: T) -> Self {
        Self {
            is_error: false,
            error_message: None,
            result_json: Some(serde_json::to_string(&data).unwrap().try_into().unwrap()),
        }
    }
}

/// Validates a playbook
#[ffi_export]
fn ffi_validate_playbook<'a>(playbook_json: char_p::Ref<'a>) -> FfiJsonResult {
    let playbook_json = playbook_json.to_str();

    match parse_playbook(playbook_json) {
        Ok(_) => FfiJsonResult::ok(()),
        Err(err) => FfiJsonResult::error(err),
    }
}

/// Starts a playbook
/// Arguments are passed as JSON strings
/// Date is RFC3339
#[ffi_export]
fn ffi_playbook_start<'a>(
    datetime: char_p::Ref<'a>,
    run_id: char_p::Ref<'a>,
    playbook_json: char_p::Ref<'a>,
    trigger_json: char_p::Ref<'a>,
) -> FfiJsonResult {
    let datetime = datetime.to_str();
    let run_id = run_id.to_str();
    let playbook_json = playbook_json.to_str();
    let trigger_json = trigger_json.to_str();

    let now = match parse_datetime(datetime) {
        Ok(now) => now,
        Err(err) => return FfiJsonResult::error(err),
    };

    let playbook = match parse_playbook(playbook_json) {
        Ok(playbook) => playbook,
        Err(err) => return FfiJsonResult::error(err),
    };

    let trigger = match parse_trigger(trigger_json) {
        Ok(trigger) => trigger,
        Err(err) => return FfiJsonResult::error(err),
    };

    let mut engine = engine();

    let result = engine.start_playbook(now, run_id.to_string(), playbook, trigger);

    result.into()
}

/// Resumes a playbook
/// Arguments are passed as JSON strings
/// Date is RFC3339
#[ffi_export]
fn ffi_playbook_resume<'a>(
    datetime: char_p::Ref<'a>,
    run_id: char_p::Ref<'a>,
    confirmations_json: char_p::Ref<'a>,
) -> FfiJsonResult {
    let datetime = datetime.to_str();
    let run_id = run_id.to_str();
    let confirmations_json = confirmations_json.to_str();

    let now = match parse_datetime(datetime) {
        Ok(now) => now,
        Err(err) => return FfiJsonResult::error(err),
    };
    let confirmations = match parse_confirmations(confirmations_json) {
        Ok(confirmations) => confirmations,
        Err(err) => return FfiJsonResult::error(err),
    };

    let mut engine = engine();

    let result = engine.resume_playbook(now, run_id.to_string(), confirmations);

    result.into()
}

fn parse_datetime(datetime_rfc3339: &str) -> Result<DateTime<Utc>, ParseError> {
    let datetime = DateTime::parse_from_rfc3339(datetime_rfc3339)
        .map_err(|_| ParseError::DateTimeParse(datetime_rfc3339.to_string()))?;

    Ok(datetime.with_timezone(&Utc))
}

#[safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    safer_ffi::headers::builder()
        .to_file("headers.h")?
        .generate()
}
