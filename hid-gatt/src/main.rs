use core::panic;
use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    io::{BufReader, BufWriter},
    ops::Add,
    os::windows::process,
    rc::{Rc, Weak},
};

use bt_only_headers::{
    hcimanager::{AppMsg, HciManager, MsgProcessor},
    messages::H4Packet,
    socket::{self, MockSocket, Socket},
};

fn main() {
    let mut socket = MockSocket::new(VecDeque::new());
    let mut queue: VecDeque<AppMsg> = vec![].into();
    let mut mgr = HciManager::new().unwrap();
    while let Some(packet) = socket.read().unwrap() {
        queue.push_front(AppMsg::Recv(packet));
        while let Some(msg) = queue.pop_front() {
            // Process the message
            queue.append(&mut mgr.process(msg.clone()).unwrap().into());

            // Handle the message in main
            match msg {
                AppMsg::Send(packet) => {
                    socket.write(packet).unwrap();
                }
                AppMsg::Recv(packet) => {
                    panic!("Unexpected packet: {:?}", packet);
                }
                _ => {}
            }
        }
    }
}
