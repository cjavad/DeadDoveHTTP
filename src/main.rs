use async_std::task;
mod http_base;
mod http_handler;
mod server;


fn run() {
    let fut = server::accept_loop("127.0.0.1:8080");
    task::block_on(fut);
}

fn main() {
    run();
}