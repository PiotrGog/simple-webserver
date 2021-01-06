use multi_threaded_server::MultiThreadServer;
// use single_threaded_server::SingleThreadServer;

fn main() {
    // let single_threaded_server = SingleThreadServer::new();
    // single_threaded_server.start_listening();
    let multi_threaded_server = MultiThreadServer::new();
    multi_threaded_server.start_listening();
}
