use uuid::v1::{Timestamp, Context};
use uuid::Uuid;
use std::time::SystemTime;
use crate::errors::AppError;

pub fn gen_uuid_v1() -> Result<Uuid, uuid::Error> {
    let context = Context::new(42);
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let ts = Timestamp::from_unix(&context, now.as_secs(), now.subsec_nanos());
    let uuid = Uuid::new_v1(ts, &[7, 7, 7, 7, 7, 7])?;
    Ok(uuid)
}