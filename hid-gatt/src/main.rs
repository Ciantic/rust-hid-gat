use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    io::{BufReader, BufWriter},
    ops::Add,
    os::windows::process,
    rc::{Rc, Weak},
};

use bt_only_headers::{hcimanager, messages::H4Packet, socket};

fn main() {
    let mut socket = Box::new(socket::MockSocket::new(VecDeque::new()));
    let packets: VecDeque<H4Packet> = vec![].into();
    let mut mgr = hcimanager::HciManager::new(packets, socket).unwrap();
    mgr.execute().unwrap();
    mgr.process().unwrap();
}
