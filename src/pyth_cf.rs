#[path = "./pyth.rs"]
pub mod pyth;
use pyth_client::PriceConf;

use pyth::web3::pubkey::Pubkey;
use pyth::web3::Cluster;
use pyth::*;

use std::str::FromStr;

// Fetch and cache products list at Cloudflare KV store
// pub async fn fetch_pyth_products_n_cache_cf_by_symbol(
//     cluster: &ClusterUrl,
//     symbol: &str,
// ) -> HashMap<String, Pubkey> {
//     // TODO: Read all the mapping product's accounts and cache to Cloudflare KV store.

//     let product_account = fetch_pyth_product_account_by_symbol(cluster, &symbol).await;
//     product_account
// }

pub async fn fetch_cf_cached_products_n_pyth_price_by_symbol(
    cluster: &Cluster,
    symbol: &str,
) -> Option<PriceConf> {
    // TODO: Read the mapping product's account from Cloudflare KV store.
    // Mocked SOL/USD
    let address = Pubkey::from_str("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix").unwrap();

    // Fetch price from pyth
    let product_account = fetch_pyth_price_by_pubkey(cluster, &address).await;
    product_account
}

static mut STATE: &'static str = "";

#[no_mangle]
pub extern "C" fn add_product_list(product_list: &'static str) {
    unsafe { STATE = &product_list.clone() };
}

#[no_mangle]
pub extern "C" fn get_product_list() -> &'static str {
    unsafe { STATE }
}
