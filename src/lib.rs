// Empty library for packaging WIT and adapter modules

pub const WASM_RPC_WIT: &str = include_str!("../wit/deps/golem-rpc/wasm-rpc.wit");
pub const WASI_POLL_WIT: &str = include_str!("../wit/deps/io/poll.wit");
pub const WASI_WALL_CLOCKS_WIT: &str = include_str!("../wit/deps/clocks/wall-clock.wit");
