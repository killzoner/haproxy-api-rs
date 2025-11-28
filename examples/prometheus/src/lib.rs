use haproxy_api::{Action, Core, ServiceMode, Txn};
use mlua::prelude::*;
use prometheus::{
    register_counter_vec_with_registry, register_histogram_vec_with_registry,
    register_int_gauge_vec_with_registry, Registry, TextEncoder,
};
use std::{
    borrow::Cow,
    collections::HashMap,
    ops::Sub,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub const DEFAULT_HISTOGRAM_BUCKETS_SECONDS: [f64; 9] =
    [0.1, 0.25, 0.5, 0.75, 1.0, 2.5, 5.0, 7.5, 10.0];

#[mlua::lua_module(skip_memory_check)]
fn haproxy_prometheus_module(lua: &Lua) -> LuaResult<bool> {
    let core = Core::new(lua)?;
    let registry = init_registry()?;

    let foo_call_total = register_counter_vec_with_registry!(
        "foo_http_requests_total",
        "Number of HTTP requests made with foo query param.",
        &["param"],
        registry
    )
    .map_err(|e| e.into_lua_err())?;

    let bar_call_total = register_counter_vec_with_registry!(
        "bar_http_requests_total",
        "Number of HTTP requests made with bar query param.",
        &["param"],
        registry
    )
    .map_err(|e| e.into_lua_err())?;

    let response_time = register_histogram_vec_with_registry!(
        "total_response_time_seconds",
        "Response time.",
        &["foo", "bar"],
        registry
    )
    .map_err(|e| e.into_lua_err())?;

    // process request items only available in this hook
    // and save them in txn vars if needed
    core.register_action("rust_req", &[Action::HttpReq], 0, |_lua, txn: Txn| {
        println!("rust_req BEGIN");

        for kv in txn.http()?.req_get_headers()?.pairs() {
            let (k, v): (String, Vec<String>) = kv?;
            println!("request header {k}: {v:?}");
        }

        // get x-request-uri set by previous
        // haproxy set-header capture directive
        let req_uri = txn
            .http()?
            .req_get_headers()?
            .get_first::<String>("x-request-uri")?
            .unwrap_or_default();
        let req_uri = req_uri.strip_prefix("/?").unwrap_or(&req_uri);

        println!("req_uri: {req_uri:?}");
        txn.set_var("txn.req_uri", req_uri)?;

        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.into_lua_err())?
            .as_millis();

        println!("start_time: {start_time:?}");
        txn.set_var("txn.start_time", start_time)?;

        println!("rust_req END");
        Ok(())
    })?;

    // process response items only available in this hook
    // plus saved txn vars and update prometheus metrics
    core.register_action(
        "rust_resp",
        &[Action::HttpAfterRes],
        0,
        move |_lua, txn: Txn| {
            println!("rust_resp BEGIN");

            for kv in txn.http()?.res_get_headers()?.pairs() {
                let (k, v): (String, Vec<String>) = kv?;
                println!("response header {k}: {v:?}");
            }

            let req_uri = txn.get_var::<String>("txn.req_uri")?;
            let query_params: HashMap<Cow<'_, str>, Cow<'_, str>> =
                form_urlencoded::parse(req_uri.as_bytes()).collect();
            println!("query_params: {query_params:?}");

            let foo_param = query_params.get("foo");
            if let Some(param) = foo_param {
                foo_call_total.with_label_values(&[param]).inc();
            }

            let bar_param = query_params.get("bar");
            if let Some(param) = bar_param {
                bar_call_total.with_label_values(&[param]).inc();
            }

            // Tt and others are not available
            let method = txn.f.get::<String>("method", ())?;
            println!("method {method:?}");
            let status = txn.f.get::<String>("status", ())?;
            println!("status {status:?}");

            let start_time = txn
                .get_var::<u128>("txn.start_time")
                .map(|e| Duration::from_millis(e as u64))?;

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| e.into_lua_err())?;

            let process_time = now.sub(start_time);
            println!("computed Tt {process_time:?}");

            let foo_value = foo_param.cloned().unwrap_or_default();
            let bar_value = bar_param.cloned().unwrap_or_default();

            response_time
                .with_label_values(&[foo_value, bar_value])
                .observe(process_time.as_secs_f64());

            println!("rust_resp END");
            Ok(())
        },
    )?;

    let get_metrics = LuaFunction::wrap(move || render_metrics(&registry));

    let code = mlua::chunk! {
        local applet = ...
        local response, err = $get_metrics()

        applet:set_status(200)
        applet:add_header("content-length", string.len(response))
        applet:add_header("content-type", "application/octet-stream")
        applet:start_response()
        applet:send(response)
    };
    core.register_lua_service("serve_metrics", ServiceMode::Http, code)?;

    Ok(true)
}

fn init_registry() -> LuaResult<Registry> {
    let registry = prometheus::Registry::new();

    let constant_gauge = register_int_gauge_vec_with_registry!(
        "app_name",
        "Metric with a constant '1' value labeled with app name",
        &["app",],
        registry,
    )
    .map_err(|e| e.into_lua_err())?;

    constant_gauge
        .with_label_values(&["haproxy_prometheus_module"])
        .set(1);

    Ok(registry)
}

fn render_metrics(registry: &Registry) -> LuaResult<String> {
    let mut output: String = String::new();
    TextEncoder::new()
        .encode_utf8(&registry.gather(), &mut output)
        .map_err(|e| e.into_lua_err())?;

    Ok(output)
}
