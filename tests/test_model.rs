#![allow(unused_imports)]

use std::sync::OnceLock;

use rbatis::{rbdc::db, RBatis};

pub fn get_pool() -> &'static RBatis {
    static RB: OnceLock<RBatis> = OnceLock::new();
    RB.get_or_init(|| {
        dotenv::dotenv().ok();
        let url = std::env::var("DATABASE_URL").unwrap();
        // let url = include_str!("../.env").replace("DATABASE_URL=", "");
        let rb = RBatis::new();
        rb.init(rbdc_mysql::Driver {}, &url).unwrap();
        rb
    })
}

#[tokio::test]
async fn proxy_test() {
    // let rb = get_pool();
    // let cm = PMerchant::select_by_column(rb, "name", "test").await;
    // use rbatis::plugin::page::PageRequest;
    // let cm = PProxy::select_page_by_name(&rb, &PageRequest::new(1, 1), "admin").await;
    // println!("{:?}", cm);
}
