use single_threaded_server::SingleThreadServer;

fn main() {
    let single_threaded_server = SingleThreadServer::new();
    single_threaded_server.start_listening();
}
