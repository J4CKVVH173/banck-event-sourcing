mod structs;

use std::{net::SocketAddr, sync::Arc};

use axum::{
    Json, Router,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use crate::application::{Storage, use_cases::UseCases};


pub struct Server<T: Storage + Send + Sync + 'static> {
    use_cases: Arc<Mutex<UseCases<T>>>,
}

impl<T: Storage + Send + Sync + 'static> Server<T> {
    pub fn new(use_cases: UseCases<T>) -> Self {
        Self {
            use_cases: Arc::new(Mutex::new(use_cases)),
        }
    }

    /// Запускает axum-сервер на указанном адресе.
    pub async fn run(self, addr: &str) {
        let app = Router::new()
            .route("/add", post(Self::handle_add))
            .route("/remove", post(Self::handle_remove))
            .route("/total", get(Self::handle_total))
            .with_state(self.use_cases);

        let socket_addr: SocketAddr = addr.parse().expect("Invalid socket address");

        let listener = TcpListener::bind(socket_addr)
            .await
            .expect("Failed to bind to address");
        println!("Server running on {}", socket_addr);

        axum::serve(listener, app).await.expect("Server failed");
    }

    async fn handle_add(
        State(use_cases): State<Arc<Mutex<UseCases<T>>>>,
        Json(payload): Json<structs::AddPayload>,
    ) -> impl IntoResponse {
        let uc = use_cases.lock().await;
        uc.add(payload.amount).unwrap();
        format!("Added: {}", payload.amount)
    }

    async fn handle_remove(
        State(use_cases): State<Arc<Mutex<UseCases<T>>>>,
        Json(payload): Json<structs::RemovePayload>,
    ) -> impl IntoResponse {
        let uc = use_cases.lock().await;
        uc.remove(payload.amount).unwrap();
        format!("Removed: {}", payload.amount)
    }

    async fn handle_total(State(use_cases): State<Arc<Mutex<UseCases<T>>>>) -> impl IntoResponse {
        let total = {
            let uc = use_cases.lock().await;
            uc.get_amount().unwrap()
        };
        format!("Total: {}", total)
    }
}
