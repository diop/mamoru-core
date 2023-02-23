use super::{Incident, WasmEnv};
use crate::daemon::sql::SqlExecutor;
use as_ffi_bindings::StringPtr;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use tokio::runtime::Handle;
use tracing::error;
use wasmer::{imports, AsStoreMut, Function, FunctionEnv, FunctionEnvMut, Imports};

const HTTP_DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

pub(crate) fn all(store: &mut impl AsStoreMut, env: &FunctionEnv<WasmEnv>) -> Imports {
    imports! {
        "env" => {
            "abort" => Function::new_typed_with_env(store,  env, abort),
        },
        "mamoru" => {
            "query" => Function::new_typed_with_env(store, env, query),
            "report" => Function::new_typed_with_env(store, env, report),
            "http" => Function::new_typed_with_env(store, env, http),
        }
    }
}

fn abort(
    ctx: FunctionEnvMut<WasmEnv>,
    message: StringPtr,
    filename: StringPtr,
    line: u32,
    col: u32,
) -> Result<(), wasmer::RuntimeError> {
    let env = ctx.data();
    let message = env.read_string_ptr(&message, &ctx)?;
    let filename = env.read_string_ptr(&filename, &ctx)?;

    error!("Error: {} at {}:{} col: {}", message, filename, line, col);

    Ok(())
}

#[tracing::instrument(skip_all, level = "trace")]
fn query(
    mut ctx: FunctionEnvMut<WasmEnv>,
    query: StringPtr,
) -> Result<StringPtr, wasmer::RuntimeError> {
    runtime_error_ctx(|| {
        let env = ctx.data();
        let query = env.read_string_ptr(&query, &ctx)?;
        let sql_executor = SqlExecutor::new(&query)?;

        let outputs =
            Handle::current().block_on(async move { sql_executor.query(&env.data_ctx).await })?;

        let serialized = serde_json::to_string(&outputs)?;
        let ptr = WasmEnv::alloc_string_ptr(env.bindings_env.clone(), serialized, &mut ctx)?;

        Ok(ptr)
    })
}

#[tracing::instrument(skip_all, level = "trace")]
fn report(ctx: FunctionEnvMut<WasmEnv>) -> Result<(), wasmer::RuntimeError> {
    let tx = &ctx.data().incidents_tx;

    runtime_error_ctx(|| {
        tx.try_send(Incident)?;

        Ok(())
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum HttpMethod {
    Post,
    Get,
    Put,
    Patch,
    Delete,
}

impl From<HttpMethod> for reqwest::Method {
    fn from(value: HttpMethod) -> Self {
        match value {
            HttpMethod::Post => Self::POST,
            HttpMethod::Get => Self::GET,
            HttpMethod::Put => Self::PUT,
            HttpMethod::Patch => Self::PATCH,
            HttpMethod::Delete => Self::DELETE,
        }
    }
}

#[derive(Deserialize)]
struct HttpRequest {
    method: HttpMethod,
    url: String,
    #[serde(default)]
    headers: HashMap<String, String>,
    body: Option<String>,
}

#[derive(Serialize)]
struct HttpResponse {
    status: u16,
    error: Option<String>,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl HttpResponse {
    async fn from_reqwest(value: reqwest::Response) -> Self {
        Self {
            error: None,
            status: value.status().as_u16(),
            headers: value
                .headers()
                .iter()
                .filter_map(|(k, v)| {
                    let key = k.to_string();

                    match v.to_str() {
                        Ok(value) => Some((key, value.to_string())),
                        Err(err) => {
                            error!(error = ?err, ?key, "Failed to read header");

                            None
                        }
                    }
                })
                .collect(),
            body: match value.bytes().await {
                Ok(body) => Some(body.to_vec()),
                Err(err) => {
                    error!(error = ?err, "Failed to read response body");

                    None
                }
            },
        }
    }
}

#[tracing::instrument(skip_all, level = "trace")]
fn http(
    mut ctx: FunctionEnvMut<WasmEnv>,
    request_json_ptr: StringPtr,
) -> Result<StringPtr, wasmer::RuntimeError> {
    runtime_error_ctx(|| {
        let env = ctx.data();
        let request_json = env.read_string_ptr(&request_json_ptr, &ctx)?;

        let http_request: HttpRequest = serde_json::from_str(&request_json)?;

        let client = reqwest::Client::builder()
            .timeout(HTTP_DEFAULT_TIMEOUT)
            .build()?;

        let mut request_builder = client.request(http_request.method.into(), http_request.url);

        if let Some(body) = http_request.body {
            request_builder = request_builder.body(body);
        }

        for (key, value) in http_request.headers {
            request_builder = request_builder.header(key, value);
        }

        let response = Handle::current().block_on(async move {
            match request_builder.send().await {
                Ok(res) => HttpResponse::from_reqwest(res).await,
                Err(err) => HttpResponse {
                    error: Some(err.to_string()),
                    status: 0,
                    headers: Default::default(),
                    body: None,
                },
            }
        });

        let serialized = serde_json::to_string(&response)?;
        let ptr = WasmEnv::alloc_string_ptr(env.bindings_env.clone(), serialized, &mut ctx)?;

        Ok(ptr)
    })
}

fn runtime_error_ctx<F, T>(fun: F) -> Result<T, wasmer::RuntimeError>
where
    F: FnOnce() -> Result<T, Box<dyn Error>>,
{
    fun().map_err(|err| wasmer::RuntimeError::new(err.to_string()))
}
