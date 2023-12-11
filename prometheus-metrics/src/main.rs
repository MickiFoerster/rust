use axum::{
    extract::MatchedPath, http::Request, middleware::Next, response::IntoResponse, routing::get,
    Router,
};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::{
    future::ready,
    net::SocketAddr,
    time::{Duration, Instant},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn metrics_app() -> Router {
    let recorder_handle = setup_metrics_recorder();
    Router::new().route(
        "/metrics",
        get(move || {
            tracing::info!(
                "metrics endpoint was called: rendering metrics with prometheus recorder",
            );
            ready(recorder_handle.render())
        }),
    )
}

fn main_app() -> Router {
    tokio::spawn(async move {
        loop {
            tracing::trace!("concurrent thread for incrementing counter ...");
            tokio::time::sleep(Duration::from_millis(500)).await;
            metrics::increment_counter!("counter_all_fivehundret_ms_total", &[("key", "value")]);
        }
    });
    tokio::spawn(async move {
        tracing::trace!("concurrent thread for gauge ...");

        metrics::gauge!("gauge_with_seven_values", 2.);
        tokio::time::sleep(Duration::from_millis(500)).await;

        metrics::gauge!("gauge_with_seven_values", 13.);
        tokio::time::sleep(Duration::from_millis(500)).await;

        metrics::gauge!("gauge_with_seven_values", 5.);
        tokio::time::sleep(Duration::from_millis(500)).await;

        metrics::gauge!("gauge_with_seven_values", 17.);
        tokio::time::sleep(Duration::from_millis(500)).await;

        metrics::gauge!("gauge_with_seven_values", 7.);
        tokio::time::sleep(Duration::from_millis(500)).await;

        metrics::gauge!("gauge_with_seven_values", 3.);
        tokio::time::sleep(Duration::from_millis(500)).await;

        metrics::gauge!("gauge_with_seven_values", 11.);
        tokio::time::sleep(Duration::from_millis(500)).await;

        tracing::trace!("gauge values written");
    });

    Router::new()
        .route(
            "/fast",
            get(|| async {
                tracing::info!("increase counter my_testing_counter_total");
                metrics::increment_counter!("my_testing_counter_total", &[("route", "fast")]);
            }),
        )
        .route(
            "/slow",
            get(|| async {
                tokio::time::sleep(Duration::from_secs(1)).await;
                metrics::increment_counter!("my_testing_counter_total", &[("route", "slow")]);
            }),
        )
        .route_layer(middleware::from_fn(track_metrics))
}

async fn start_main_server() {
    let app = main_app();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("establishing server failed")
}

async fn start_metrics_server() {
    let app = metrics_app();

    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("establishing server failed")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // The `/metrics` endpoint should not be publicly available. If behind a reverse proxy, this
    // can be achieved by rejecting requests to `/metrics`. In this example, a second server is
    // started on another port to expose `/metrics`.
    let (_main_server, _metrics_server) = tokio::join!(start_main_server(), start_metrics_server());
}

fn setup_metrics_recorder() -> PrometheusHandle {
    // const EXPONENTIAL_SECONDS: &[f64] = &[
    //     0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    // ];

    PrometheusBuilder::new()
        // .set_buckets_for_metric(
        //     Matcher::Full("http_requests_duration_seconds".to_string()),
        //     EXPONENTIAL_SECONDS,
        // )
        // .unwrap()
        .install_recorder()
        .unwrap()
}

async fn track_metrics<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    metrics::increment_counter!("http_requests_total", &labels);
    metrics::histogram!("http_requests_duration_seconds", latency, &labels);

    response
}
