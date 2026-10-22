#[global_allocator]
static GLOBAL: rustfs_mimalloc::MiMalloc = rustfs_mimalloc::MiMalloc;

#[tokio::main]
async fn main() {
    println!("Hello from kani!");
}
