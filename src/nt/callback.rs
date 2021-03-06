use crate::EntryData;
use std::net::SocketAddr;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum CallbackType {
    Add,
    Delete,
    Update,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ConnectionCallbackType {
    ClientConnected,
    ClientDisconnected,
}

pub type ConnectionAction = dyn FnMut(&SocketAddr) + Send + 'static;

pub type Action = dyn FnMut(&EntryData) + Send + 'static;
