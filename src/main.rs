use example_indexer::start_server;

#[tokio::main(worker_threads = 8)]
async fn main() {
    start_server().await.unwrap()
}
