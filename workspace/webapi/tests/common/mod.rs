
use sqlx::PgPool;
use std::net::TcpListener;
use testcontainers::{clients, images, images::generic::WaitFor, Docker};
use webapi::server::run;

#[derive(Debug)]
pub struct ServiceUnderTest {
    base_endpoint_url: String,
    connection_pool: PgPool,
}

impl ServiceUnderTest {
    pub fn endpoint(self, path: &str) -> String {
        format!("{}{}", self.base_endpoint_url, path)
    }
}

pub async fn spawn_app() -> ServiceUnderTest {
    let docker = clients::Cli::default();

    let db = "trenako-test";
    let user = "postgres-user-test";
    let password = "postgres-password-test";

    let postgres_image =
        images::generic::GenericImage::new("postgres:13.1-alpine")
            .with_wait_for(WaitFor::message_on_stderr(
                "database system is ready to accept connections",
            ))
            .with_env_var("POSTGRES_DB", db)
            .with_env_var("POSTGRES_USER", user)
            .with_env_var("POSTGRES_PASSWORD", password);
    let node = docker.run(postgres_image);
    let port = node.get_host_port(5432).unwrap_or(5432);

    let connection_string =
        &format!("postgres://{}:{}@localhost:{}/{}", user, password, port, db);

    let connection_pool = PgPool::connect(connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server =
        run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    ServiceUnderTest {
        base_endpoint_url: format!("http://127.0.0.1:{}", port),
        connection_pool,
    }
}