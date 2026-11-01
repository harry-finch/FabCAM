//! Local-only HTTP origin admission and bounded worker lifetime.
use axum::{
    body::{Body, Bytes},
    extract::{Request, State},
    http::{HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Router,
};
use futures::Stream;
use ivac_core::cancel::CancelToken;
use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

#[derive(Clone)]
pub struct Policy {
    pub origins: Vec<HeaderValue>,
    permits: Arc<Semaphore>,
}
impl Default for Policy {
    fn default() -> Self {
        Self::new(
            vec![
                "http://localhost:5173".parse().unwrap(),
                "http://127.0.0.1:5173".parse().unwrap(),
                "http://localhost:1420".parse().unwrap(),
                "http://127.0.0.1:1420".parse().unwrap(),
                "tauri://localhost".parse().unwrap(),
            ],
            2,
        )
    }
}
impl Policy {
    pub fn new(origins: Vec<HeaderValue>, limit: usize) -> Self {
        Self {
            origins,
            permits: Arc::new(Semaphore::new(limit)),
        }
    }
    pub fn configured(host: &str, port: u16) -> anyhow::Result<Self> {
        let ip: std::net::IpAddr = host.parse()?;
        anyhow::ensure!(ip.is_loopback(),"Unauthenticated HTTP server requires a loopback bind; LAN mode needs a separately reviewed authentication policy");
        let mut p = Self::default();
        if let Ok(raw) = std::env::var("IVAC_CORS_ORIGINS") {
            if !raw.trim().is_empty() {
                p.origins=raw.split(',').map(|s| {
                    let s=s.trim();let uri:axum::http::Uri=s.parse()?;
                    anyhow::ensure!(s!="null" && uri.authority().is_some() && matches!(uri.scheme_str(),Some("http"|"https"|"tauri")) && !s.contains('@') && uri.path()=="/" && uri.query().is_none(),"Origins must be explicit scheme/authority values; wildcard and null are unavailable");
                    Ok(HeaderValue::from_str(s)?)
                }).collect::<anyhow::Result<Vec<_>>>()?;
            }
        }
        p.origins.push(
            format!(
                "http://{}:{port}",
                if ip.is_ipv6() {
                    format!("[{host}]")
                } else {
                    host.into()
                }
            )
            .parse()?,
        );
        Ok(p)
    }
}
struct Lease {
    _permit: OwnedSemaphorePermit,
}
#[derive(Clone)]
pub struct Job {
    _lease: Arc<Lease>,
    pub cancel: CancelToken,
}
impl Job {
    pub fn spawn<F, T>(&self, f: F) -> tokio::task::JoinHandle<T>
    where
        F: FnOnce(CancelToken) -> T + Send + 'static,
        T: Send + 'static,
    {
        let job = self.clone();
        tokio::task::spawn_blocking(move || {
            let result = f(job.cancel.clone());
            drop(job);
            result
        })
    }
}
struct CancelOnDrop(Job);
impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        self.0.cancel.cancel();
    }
}
struct GuardedBody {
    inner: Pin<Box<dyn Stream<Item = Result<Bytes, axum::Error>> + Send>>,
    _guard: CancelOnDrop,
}
impl Stream for GuardedBody {
    type Item = Result<Bytes, axum::Error>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.as_mut().poll_next(cx)
    }
}
async fn admit(State(policy): State<Policy>, mut req: Request, next: Next) -> Response {
    let origins = req
        .headers()
        .get_all(axum::http::header::ORIGIN)
        .iter()
        .collect::<Vec<_>>();
    if origins.is_empty()
        && req
            .headers()
            .get("sec-fetch-site")
            .is_some_and(|v| v == "cross-site")
    {
        return (
            StatusCode::FORBIDDEN,
            "Cross-site browser request requires an authorized Origin",
        )
            .into_response();
    }
    if origins.len() > 1 || origins.first().is_some_and(|v| !policy.origins.contains(v)) {
        return (StatusCode::FORBIDDEN, "Browser Origin is not authorized").into_response();
    }
    if matches!(req.uri().path(), "/healthz" | "/version")
        || req.uri().path().starts_with("/generate/cancel/")
        || req.method() == axum::http::Method::OPTIONS
    {
        return next.run(req).await;
    }
    let Ok(permit) = policy.permits.clone().try_acquire_owned() else {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            "Local worker budget is busy; retry when the current job finishes",
        )
            .into_response();
    };
    let job = Job {
        _lease: Arc::new(Lease { _permit: permit }),
        cancel: CancelToken::new(),
    };
    let guard = CancelOnDrop(job.clone());
    req.extensions_mut().insert(job);
    // Also bound incomplete admitted request bodies. Dropping the service
    // future signals cancellation; its worker clone retains the permit until exit.
    let Ok(response) =
        tokio::time::timeout(std::time::Duration::from_secs(120), next.run(req)).await
    else {
        return (
            StatusCode::REQUEST_TIMEOUT,
            "Local request deadline exceeded",
        )
            .into_response();
    };
    let (parts, body) = response.into_parts();
    Response::from_parts(
        parts,
        Body::from_stream(GuardedBody {
            inner: Box::pin(body.into_data_stream()),
            _guard: guard,
        }),
    )
}
pub fn protect<S: Clone + Send + Sync + 'static>(app: Router<S>, policy: Policy) -> Router<S> {
    app.layer(axum::middleware::from_fn_with_state(policy, admit))
}
#[cfg(test)]
pub fn test_router<S: Clone + Send + Sync + 'static>(app: Router<S>) -> Router<S> {
    protect(app, Policy::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        http::Request as HttpRequest,
        routing::{get, post},
        Extension,
    };
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tower::ServiceExt;
    fn request(uri: &str, origin: Option<&str>) -> HttpRequest<Body> {
        let mut req = HttpRequest::builder().uri(uri).method("POST");
        if let Some(origin) = origin {
            req = req.header("origin", origin);
        }
        req.body(Body::empty()).unwrap()
    }
    #[tokio::test]
    async fn origin_refusal_precedes_multipart_and_worker_admission() {
        let app = protect(
            Router::new().route("/import", post(|| async { StatusCode::NO_CONTENT })),
            Policy::default(),
        );
        for (origin, status) in [
            (None, 204),
            (Some("http://localhost:5173"), 204),
            (Some("null"), 403),
            (Some("https://foreign.invalid"), 403),
        ] {
            let response = app
                .clone()
                .oneshot(request("/import", origin))
                .await
                .unwrap();
            assert_eq!(response.status().as_u16(), status);
        }
        let mut cross_site = request("/import", None);
        cross_site
            .headers_mut()
            .insert("sec-fetch-site", "cross-site".parse().unwrap());
        assert_eq!(
            app.oneshot(cross_site).await.unwrap().status(),
            StatusCode::FORBIDDEN
        );
        assert!(Policy::configured("0.0.0.0", 8766).is_err());
    }
    #[tokio::test]
    async fn in_flight_request_cancel_signals_worker_and_holds_permit_until_exit() {
        let started = Arc::new(AtomicUsize::new(0));
        let finished = Arc::new(AtomicUsize::new(0));
        let a = started.clone();
        let b = finished.clone();
        let app = protect(
            Router::new()
                .route(
                    "/work",
                    post(move |Extension(job): Extension<Job>| {
                        let a = a.clone();
                        let b = b.clone();
                        async move {
                            job.spawn(move |cancel| {
                                a.fetch_add(1, Ordering::SeqCst);
                                let until =
                                    std::time::Instant::now() + std::time::Duration::from_secs(2);
                                while !cancel.is_cancelled() && std::time::Instant::now() < until {
                                    std::thread::sleep(std::time::Duration::from_millis(1));
                                }
                                assert!(cancel.is_cancelled());
                                b.fetch_add(1, Ordering::SeqCst);
                            })
                            .await
                            .unwrap();
                            StatusCode::OK
                        }
                    }),
                )
                .route("/normal", post(|| async { StatusCode::OK }))
                .route("/healthz", get(|| async { StatusCode::OK })),
            Policy::new(vec![], 1),
        );
        let task = tokio::spawn(app.clone().oneshot(request("/work", None)));
        tokio::time::timeout(std::time::Duration::from_secs(1), async {
            while started.load(Ordering::SeqCst) == 0 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        assert_eq!(
            app.clone()
                .oneshot(request("/work", None))
                .await
                .unwrap()
                .status(),
            StatusCode::TOO_MANY_REQUESTS
        );
        assert_eq!(started.load(Ordering::SeqCst), 1);
        task.abort();
        let _ = task.await;
        tokio::time::timeout(std::time::Duration::from_secs(1), async {
            while finished.load(Ordering::SeqCst) == 0 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        assert_eq!(
            app.oneshot(request("/normal", None))
                .await
                .unwrap()
                .status(),
            StatusCode::OK
        );
    }
}
