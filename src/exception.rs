use crate::exception::RpcError::{AclError, BizError, InvalidArgs, Timeout, UnknownError};
use std::io::{Error, ErrorKind};
use time::Duration;

#[derive(thiserror::Error, Debug)]
pub enum RpcError {
    #[error("input is invalid")]
    InvalidArgs,

    #[error("rpc timeout with {0:?}")]
    Timeout(Duration),

    #[error("request from {0} is not allowed")]
    AclError(String),

    #[error("load balance fail")]
    LoadbalanceError,

    #[error("system error: {0:?}")]
    SysError(#[from] std::io::Error),

    #[error("business error:{0}")]
    BizError(u32),

    #[error("unknown error:{0}")]
    UnknownError(String),
}

pub fn mock_one_way_rpc(mock_arg: i32) -> anyhow::Result<(), RpcError> {
    if mock_arg < 0 || mock_arg > 5 {
        return Ok(());
    }

    if mock_arg == 0 {
        return Err(InvalidArgs);
    }

    if mock_arg == 1 {
        return Err(Timeout(Duration::seconds(1)));
    }

    if mock_arg == 2 {
        return Err(AclError("unknown".to_owned()));
    }

    if mock_arg == 3 {
        return Err(RpcError::SysError(Error::new(
            ErrorKind::ConnectionAborted,
            "connect aborted",
        )));
    }

    if mock_arg == 4 {
        return Err(BizError(1024));
    }

    if mock_arg == 5 {
        return Err(UnknownError("system error".to_string()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::exception::mock_one_way_rpc;

    #[test]
    fn mock_one_way_rpc_test() {
        let r = -1..6;

        r.for_each(|arg| println!("response {:?}", mock_one_way_rpc(arg)))
    }
}
