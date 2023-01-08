use grpc::RLServer;
use leash::{AnyhowResultFuture, Component};
use proto::nova::ratelimit::ratelimiter::ratelimiter_server::RatelimiterServer;
use redis::aio::MultiplexedConnection;
use redis_global_local_bucket_ratelimiter::RedisGlobalLocalBucketRatelimiter;
use shared::config::Settings;
use std::future::Future;
use std::{net::ToSocketAddrs, pin::Pin};
use tokio::sync::oneshot;
use tonic::transport::Server;

mod grpc;
mod redis_global_local_bucket_ratelimiter;

pub struct RatelimiterServerComponent {}
impl Component for RatelimiterServerComponent {
    type Config = ();
    const SERVICE_NAME: &'static str = "ratelimiter";

    fn start(
        &self,
        settings: Settings<Self::Config>,
        stop: oneshot::Receiver<()>,
    ) -> AnyhowResultFuture<()> {
        Box::pin(async move {
            let redis = Into::<
                Pin<Box<dyn Future<Output = anyhow::Result<MultiplexedConnection>> + Send>>,
            >::into(settings.redis)
            .await?;

            let server = RLServer::new(RedisGlobalLocalBucketRatelimiter::new(redis));

            Server::builder()
                .add_service(RatelimiterServer::new(server))
                .serve_with_shutdown(
                    "0.0.0.0:8093".to_socket_addrs().unwrap().next().unwrap(),
                    async move {
                        let _ = stop.await;
                    },
                )
                .await?;

            Ok(())
        })
    }

    fn new() -> Self {
        Self {}
    }
}
