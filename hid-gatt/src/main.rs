mod hcimanager;
mod socket;

fn main() {
    let mut socket = Box::new(socket::MockSocket::new());
    let mut mgr = hcimanager::HciManager::new(socket).unwrap();
    mgr.execute().unwrap();
    mgr.process().unwrap();
}
