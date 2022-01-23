#[path = "../../pyth_cf.rs"]
mod pyth_cf;
use pyth_cf::*;

#[path = "../../pyth.rs"]
mod pyth;

use crate::{
    cfkv::WorkersKv,
    discord::interaction::{
        InteractionApplicationCommandCallbackData, InteractionResponse, InteractionResponseType,
    },
};

pub(crate) async fn hello(kv: &WorkersKv) -> InteractionResponse {
    // let product_hashmap = "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix";
    // get
    let product_hashmap = kv
        .get_text("Crypto.SOL/USD")
        .await
        .unwrap_or_default()
        .unwrap_or_default();

    let price_conf = fetch_cf_cached_products_n_pyth_price_by_symbol(
        &pyth_cf::pyth::web3::Cluster::Devnet,
        "Crypto.SOL/USD",
    )
    .await;

    let price = price_conf.unwrap().price.to_string();
    let content = format!("SOLUSD2: {}:{}", product_hashmap, price);

    InteractionResponse {
        ty: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionApplicationCommandCallbackData { content: content }),
    }
}
