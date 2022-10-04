#![allow(dead_code)]

use std::time::Duration;

// header typs (bitflags)
pub const UDX_HEADER_DATA: u32 = 1;
pub const UDX_HEADER_END: u32 = 2;
pub const UDX_HEADER_SACK: u32 = 4;
pub const UDX_HEADER_MESSAGE: u32 = 8;
pub const UDX_HEADER_DESTROY: u32 = 16;

/// Max transmit attempts per packet
pub const UDX_MAX_TRANSMITS: u8 = 5;
pub const UDX_CLOCK_GRANULARITY_MS: Duration = Duration::from_millis(20);

pub const UDX_HEADER_SIZE: usize = 20;
pub const UDX_MAX_DATA_SIZE: usize = 1380;
pub const UDX_MAGIC_BYTE: u8 = 255;
pub const UDX_VERSION: u8 = 1;

pub const UDX_DEFAULT_TTL: u32 = 64;
pub const UDX_MTU: usize = 1400;
pub const UDX_SOCKET_RECEIVING: u32 = 1;
pub const UDX_SOCKET_BOUND: u32 = 2;
pub const UDX_SOCKET_CLOSING: u32 = 4;
pub const UDX_SOCKET_CLOSING_HANDLES: u32 = 8;

pub const UDX_PACKET_STREAM_STATE: u32 = 1;
pub const UDX_PACKET_STREAM_WRITE: u32 = 2;
pub const UDX_PACKET_STREAM_SEND: u32 = 4;
pub const UDX_PACKET_STREAM_DESTROY: u32 = 8;
pub const UDX_PACKET_SEND: u32 = 16;

pub const UDX_PACKET_CALLBACK: u32 =
    UDX_PACKET_STREAM_SEND | UDX_PACKET_STREAM_DESTROY | UDX_PACKET_SEND;
pub const UDX_PACKET_FREE_ON_SEND: u32 = UDX_PACKET_STREAM_STATE | UDX_PACKET_STREAM_DESTROY;

pub const UDX_HEADER_DATA_OR_END: u32 = UDX_HEADER_DATA | UDX_HEADER_END;

pub const UDX_STREAM_CONNECTED: u32 = 1;
pub const UDX_STREAM_RECEIVING: u32 = 2;
pub const UDX_STREAM_READING: u32 = 4;
pub const UDX_STREAM_ENDING: u32 = 8;
pub const UDX_STREAM_ENDING_REMOTE: u32 = 16;
pub const UDX_STREAM_ENDED: u32 = 32;
pub const UDX_STREAM_ENDED_REMOTE: u32 = 64;
pub const UDX_STREAM_DESTROYING: u32 = 128;
pub const UDX_STREAM_DESTROYED: u32 = 256;
pub const UDX_STREAM_DESTROYED_REMOTE: u32 = 512;
pub const UDX_STREAM_CLOSED: u32 = 1024;

pub const UDX_STREAM_ALL_DESTROYED: u32 = UDX_STREAM_DESTROYED | UDX_STREAM_DESTROYED_REMOTE;
pub const UDX_STREAM_ALL_ENDED: u32 = UDX_STREAM_ENDED | UDX_STREAM_ENDED_REMOTE;
pub const UDX_STREAM_DEAD: u32 =
    UDX_STREAM_ALL_DESTROYED | UDX_STREAM_DESTROYING | UDX_STREAM_CLOSED;
pub const UDX_STREAM_SHOULD_READ: u32 = UDX_STREAM_ENDED_REMOTE | UDX_STREAM_DEAD;
pub const UDX_STREAM_READ: u32 = 0;
pub const UDX_STREAM_SHOULD_END: u32 = UDX_STREAM_ENDING | UDX_STREAM_ENDED | UDX_STREAM_DEAD;
pub const UDX_STREAM_END: u32 = UDX_STREAM_ENDING;
pub const UDX_STREAM_SHOULD_END_REMOTE: u32 =
    UDX_STREAM_ENDED_REMOTE | UDX_STREAM_DEAD | UDX_STREAM_ENDING_REMOTE;
pub const UDX_STREAM_END_REMOTE: u32 = UDX_STREAM_ENDING_REMOTE;
