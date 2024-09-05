// #![allow(unused_braces)]
// #![cfg_attr(debug_assertions, allow(dead_code,
//      unused_imports, unused_variables, unused_mut))]

use std::{sync::OnceLock, time::Duration};

pub use ::rbatis::*;

static RB: OnceLock<RBatis> = OnceLock::new();

pub fn get_pool() -> &'static RBatis {
    RB.get().unwrap()
}

pub async fn init_db_pool(url: impl AsRef<str>, max_conns: u32) -> Result<()> {
    let rb = RBatis::new();
    rb.init(rbdc_mysql::Driver {}, url.as_ref())?;
    let pool = rb.get_pool()?;
    pool.set_timeout(Some(Duration::from_secs(60))).await;
    pool.set_max_open_conns(max_conns as u64).await;
    // 获取一个连接,检查是否成功
    pool.get_timeout(Duration::from_secs(10)).await?;

    assert!(RB.set(rb).is_ok());

    Ok(())
}
