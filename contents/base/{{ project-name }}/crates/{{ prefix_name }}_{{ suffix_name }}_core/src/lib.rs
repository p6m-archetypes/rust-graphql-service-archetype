pub mod settings;

use anyhow::Result;
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use metrics_exporter_prometheus::PrometheusBuilder;
use settings::CoreSettings;

{% if cache ~= 'None' %}
use {{ prefix_name }}_{{ suffix_name }}_cache::CachePool;
{% endif %}
{% if messaging ~= 'None' %}
use {{ prefix_name }}_{{ suffix_name }}_messaging::MessagingClient;
{% endif %}
{% if persistence ~= 'None' %}
use {{ prefix_name }}_{{ suffix_name }}_persistence::PersistencePool;
{% endif %}
use {{ prefix_name }}_{{ suffix_name }}_schema::{{ "{" }}{{ PrefixName }}{{ SuffixName }}Schema, MutationRoot, QueryRoot, Store};

pub struct {{ PrefixName }}{{ SuffixName }}Core {
    schema: {{ PrefixName }}{{ SuffixName }}Schema,
    #[allow(dead_code)]
    settings: CoreSettings,
}

impl {{ PrefixName }}{{ SuffixName }}Core {
    pub fn builder({% if persistence ~= 'None' %}db: PersistencePool{% endif %}) -> Builder {
        Builder::new({% if persistence ~= 'None' %}db{% endif %})
    }

    /// GraphQL service router — binds to service_port.
    pub fn router(&self) -> Router {
        Router::new()
            .route("/graphql", post(graphql_handler).get(graphql_playground))
            .with_state(self.schema.clone())
    }

    /// Management router — health probes and metrics on management_port.
    /// Separate from the service router so Kubernetes network policy can restrict
    /// probe traffic independently from GraphQL traffic.
    ///
    /// Installs the process-global Prometheus recorder (call once, at startup).
    pub fn management_router() -> Router {
        let metrics_handle = PrometheusBuilder::new()
            .install_recorder()
            .expect("failed to install Prometheus metrics recorder");

        // Seed a build-info family so /metrics is meaningful from the first scrape.
        metrics::gauge!(
            "{{ prefix_name }}_{{ suffix_name }}_build_info",
            "version" => env!("CARGO_PKG_VERSION")
        )
        .set(1.0);

        Router::new()
            .route("/health/readiness", get(health_readiness))
            .route("/health/liveness", get(health_liveness))
            .route(
                "/metrics",
                get(move || {
                    let handle = metrics_handle.clone();
                    async move { metrics_handler(handle).await }
                }),
            )
    }
}

pub struct Builder {
{% if persistence ~= 'None' %}
    db: PersistencePool,
{% endif %}
{% if cache ~= 'None' %}
    cache: Option<CachePool>,
{% endif %}
{% if messaging ~= 'None' %}
    messaging: Option<MessagingClient>,
{% endif %}
    settings: CoreSettings,
}

impl Builder {
    #[allow(clippy::new_without_default)]
    pub fn new({% if persistence ~= 'None' %}db: PersistencePool{% endif %}) -> Self {
        Self {
{% if persistence ~= 'None' %}
            db,
{% endif %}
{% if cache ~= 'None' %}
            cache: None,
{% endif %}
{% if messaging ~= 'None' %}
            messaging: None,
{% endif %}
            settings: CoreSettings::default(),
        }
    }

    pub fn with_settings(mut self, settings: &CoreSettings) -> Self {
        self.settings = settings.clone();
        self
    }

{% if cache ~= 'None' %}
    pub fn with_cache(mut self, cache: CachePool) -> Self {
        self.cache = Some(cache);
        self
    }

{% endif %}
{% if messaging ~= 'None' %}
    pub fn with_messaging(mut self, messaging: MessagingClient) -> Self {
        self.messaging = Some(messaging);
        self
    }

{% endif %}
    pub async fn build(self) -> Result<{{ PrefixName }}{{ SuffixName }}Core> {
        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
{% if persistence ~= 'None' %}
            .data(Store::new(self.db))
{% else %}
            .data(Store::default())
{% endif %}
{% if cache ~= 'None' %}
            .data(self.cache.expect("cache must be initialized with with_cache()"))
{% endif %}
{% if messaging ~= 'None' %}
            .data(self.messaging.expect("messaging must be initialized with with_messaging()"))
{% endif %}
            .finish();

        Ok({{ PrefixName }}{{ SuffixName }}Core {
            schema,
            settings: self.settings,
        })
    }
}

async fn graphql_handler(
    State(schema): State<{{ PrefixName }}{{ SuffixName }}Schema>,
    req: axum::Json<async_graphql::Request>,
) -> axum::Json<async_graphql::Response> {
    axum::Json(schema.execute(req.0).await)
}

async fn graphql_playground() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

async fn health_readiness() -> impl IntoResponse {
    (StatusCode::OK, axum::Json(serde_json::json!({"status": "ok"})))
}

async fn health_liveness() -> impl IntoResponse {
    (StatusCode::OK, axum::Json(serde_json::json!({"status": "ok"})))
}

/// Prometheus metrics endpoint: renders everything the installed recorder has collected.
async fn metrics_handler(handle: metrics_exporter_prometheus::PrometheusHandle) -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")],
        handle.render(),
    )
}
