use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    io::{BufReader, BufWriter},
    ops::Add,
    os::windows::process,
    rc::{Rc, Weak},
};

pub mod hcimanager;
pub mod pairinghandler;
pub mod socket;

fn main() {
    let mut socket = Box::new(socket::MockSocket::new());
    let mut mgr = hcimanager::HciManager::new(socket).unwrap();
    mgr.execute().unwrap();
    mgr.process().unwrap();
}
