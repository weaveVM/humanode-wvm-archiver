use crate::utils::archive_block::archive;
use crate::utils::planetscale::{ps_archive_block, ps_get_latest_block_id};
use crate::utils::schema::Network;
use crate::utils::server_handlers::{handle_block, handle_info, handle_weave_gm};
use axum::{routing::get, Router};
use std::thread;
use std::time::Duration;
use tokio::task;

use crate::utils::transaction::decode_wvm_tx_data;

mod utils;
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let network = Network::config();
    let block_time = network.block_time;
    let ps_latest_archived_block = ps_get_latest_block_id().await;
    // it defaults to network.start_block if planestcale fails
    let mut start_block = ps_latest_archived_block;

    println!("\n{:#?}\n\n", network);
    // server routes
    let router = Router::new()
        .route("/", get(handle_weave_gm))
        .route("/info", get(handle_info))
        .route("/block/:id", get(handle_block));

        decode_wvm_tx_data("0xf94e26b121fc94ae54360985b465c401a4dff3432d3894f5056508c6f89c676c").await;

    // poll blocks & archive in parallel
    // task::spawn(async move {
    //     loop {
    //         println!("\n{}", "#".repeat(100));
    //         println!(
    //             "\nARCHIVING BLOCK #{} of Network {} -- ChainId: {}\n",
    //             start_block, network.name, network.network_chain_id
    //         );
    //         let archive_txid = archive(Some(start_block)).await.unwrap();
    //         let _ = ps_archive_block(&start_block, &archive_txid).await;
    //         start_block += 1;
    //         println!("\n{}", "#".repeat(100));
    //         thread::sleep(Duration::from_secs(block_time.into()));
    //     }
    // });

    Ok(router.into())
}
