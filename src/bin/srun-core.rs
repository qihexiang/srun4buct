use srun::core::Core;

#[tokio::main]
async fn main() {
    let core = Core::load().await;
    core.init_daemon().await.unwrap();
}
