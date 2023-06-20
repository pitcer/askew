pub mod client;
pub mod server;

type Status = u8;

const STATUS_EMPTY: Status = 0;
const STATUS_INFO: Status = 1;
const STATUS_ERROR: Status = 2;
