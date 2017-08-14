// Bloom
//
// HTTP REST API caching middleware
// Copyright: 2017, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::net::SocketAddr;

pub struct Config {
    pub server: ConfigServer,
    pub control: ConfigControl,
    pub proxy: ConfigProxy,
    pub cache: ConfigCache,
    pub redis: ConfigRedis,
}

pub struct ConfigServer {
    pub inet: SocketAddr,
}

pub struct ConfigControl {
    pub inet: SocketAddr,
    pub tcp_timeout: u64,
}

pub struct ConfigProxy {
    pub shard: u8,
    pub inet: SocketAddr,
    pub tunnel_threads: usize,
}

pub struct ConfigCache {
    pub ttl_default: usize,
}

pub struct ConfigRedis {
    pub inet: SocketAddr,
    pub database: u8,
    pub max_key_size: usize,
    pub max_key_expiration: usize,
}
