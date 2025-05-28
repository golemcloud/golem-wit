// Empty library for packaging WIT and adapter modules

pub const WASM_RPC_WIT: &str = include_str!("../wit/deps/golem-rpc/wasm-rpc.wit");

pub const WASI_IO_ERROR_WIT: &str = include_str!("../wit/deps/io/error.wit");
pub const WASI_IO_POLL_WIT: &str = include_str!("../wit/deps/io/poll.wit");
pub const WASI_IO_STREAMS_WIT: &str = include_str!("../wit/deps/io/streams.wit");
pub const WASI_IO_WORLD_WIT: &str = include_str!("../wit/deps/io/world.wit");

pub const WASI_IO: &[(&str, &str)] = &[
    ("error.wit", WASI_IO_ERROR_WIT),
    ("poll.wit", WASI_IO_POLL_WIT),
    ("streams.wit", WASI_IO_STREAMS_WIT),
    ("world.wit", WASI_IO_WORLD_WIT),
];

pub const WASI_CLOCKS_WALL_CLOCK_WIT: &str = include_str!("../wit/deps/clocks/wall-clock.wit");
pub const WASI_CLOCKS_MONOTONIC_CLOCK_WIT: &str =
    include_str!("../wit/deps/clocks/monotonic-clock.wit");
pub const WASI_CLOCKS_TIMEZONE_WIT: &str = include_str!("../wit/deps/clocks/timezone.wit");
pub const WASI_CLOCKS_WORLD_WIT: &str = include_str!("../wit/deps/clocks/world.wit");

pub const WASI_CLOCKS: &[(&str, &str)] = &[
    ("wall-clock.wit", WASI_CLOCKS_WALL_CLOCK_WIT),
    ("monotonic-clock.wit", WASI_CLOCKS_MONOTONIC_CLOCK_WIT),
    ("timezone.wit", WASI_CLOCKS_TIMEZONE_WIT),
    ("world.wit", WASI_CLOCKS_WORLD_WIT),
];
