#[path = "../../pyth_cf.rs"]
mod pyth_cf;
use pyth_cf::*;
use pyth_client::PriceConf;

#[path = "../../pyth.rs"]
mod pyth;

use crate::discord::interaction::{
    InteractionApplicationCommandCallbackData, InteractionResponse, InteractionResponseType,
};

pub(crate) async fn hello() -> InteractionResponse {
    let product_hashmap = "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix";
    let price_conf = fetch_cf_cached_products_n_pyth_price_by_symbol(
        &pyth_cf::pyth::web3::Cluster::Devnet,
        "Crypto.SOL/USD",
    )
    .await;

    let price = price_conf.unwrap().price.to_string();
    let content = format!("SOLUSD: {}:{}", product_hashmap, price);

    InteractionResponse {
        ty: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionApplicationCommandCallbackData { content: content }),
    }
}
