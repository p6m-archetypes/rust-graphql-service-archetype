pub mod settings;

use anyhow::Result;
use async_graphql::{EmptySubscription, Schema, http::GraphiQLSource};
use axum::{
    Router,
    extract::State,
    http::{header, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
};
use settings::CoreSettings;
use {{ prefix_name }}_{{ suffix_name }}_schema::{
    {{ PrefixName }}{{ SuffixName }}Schema, MutationRoot, QueryRoot,
};
{% if persistence ~= 'None' %}use {{ prefix_name }}_{{ suffix_name }}_persistence::PersistencePool;
{% endif %}{% if cache ~= 'None' %}use {{ prefix_name }}_{{ suffix_name }}_cache::CachePool;
{% endif %}{% if messaging ~= 'None' %}use {{ prefix_name }}_{{ suffix_name }}_messaging::MessagingClient;
{% endif %}
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
    pub fn management_router() -> Router {
        Router::new()
            .route("/health/readiness", get(health_readiness))
            .route("/health/liveness", get(health_liveness))
            .route("/metrics", get(metrics_handler))
    }
}

pub struct Builder {
{% if persistence ~= 'None' %}    db: PersistencePool,
{% endif %}{% if cache ~= 'None' %}    cache: Option<CachePool>,
{% endif %}{% if messaging ~= 'None' %}    messaging: Option<MessagingClient>,
{% endif %}    settings: CoreSettings,
}

impl Builder {
    pub fn new({% if persistence ~= 'None' %}db: PersistencePool{% endif %}) -> Self {
        Self {
{% if persistence ~= 'None' %}            db,
{% endif %}{% if cache ~= 'None' %}            cache: None,
{% endif %}{% if messaging ~= 'None' %}            messaging: None,
{% endif %}            settings: CoreSettings::default(),
        }
    }

    pub fn with_settings(mut self, settings: &CoreSettings) -> Self {
        self.settings = settings.clone();
        self
    }

{% if cache ~= 'None' %}    pub fn with_cache(mut self, cache: CachePool) -> Self {
        self.cache = Some(cache);
        self
    }

{% endif %}{% if messaging ~= 'None' %}    pub fn with_messaging(mut self, messaging: MessagingClient) -> Self {
        self.messaging = Some(messaging);
        self
    }

{% endif %}    pub async fn build(self) -> Result<{{ PrefixName }}{{ SuffixName }}Core> {
        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
{% if persistence ~= 'None' %}            .data(self.db)
{% endif %}{% if cache ~= 'None' %}            .data(self.cache.expect("cache must be initialized with with_cache()"))
{% endif %}{% if messaging ~= 'None' %}            .data(self.messaging.expect("messaging must be initialized with with_messaging()"))
{% endif %}            .finish();

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

/// Prometheus metrics endpoint.
async fn metrics_handler() -> impl IntoResponse {
    // TODO: wire up metrics-exporter-prometheus handle and return rendered text.
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")],
        "# Prometheus metrics\n",
    )
}
