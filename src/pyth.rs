use pyth_client::{load_mapping, load_price, load_product, PriceConf};

#[path = "./web3.rs"]
pub mod web3;
use std::collections::HashMap;
use std::str::FromStr;
use web3::pubkey::Pubkey;
use web3::*;

#[warn(dead_code)]
fn get_pyth_mapping_account(target: &Cluster) -> &'static str {
    match target {
        Cluster::MainnetBeta => "AHtgzX45WTKfkPG53L6WYhGEXwQkN1BVknET3sVsLL8J",
        _ => "BmA9Z6FjioHJPpjT39QazZyhDRUdZy2ezwx4GiDdE2u2",
    }
}

#[warn(dead_code)]
fn get_attr_str<'a, T>(ite: &mut T) -> String
where
    T: Iterator<Item = &'a u8>,
{
    let mut len = *ite.next().unwrap() as usize;
    let mut val = String::with_capacity(len);
    while len > 0 {
        val.push(*ite.next().unwrap() as char);
        len -= 1;
    }
    return val;
}

pub async fn fetch_pyth_product_account_by_symbol(
    cluster: &Cluster,
    symbol: &str,
) -> Option<Pubkey> {
    let product_accounts = fetch_pyth_product_accounts(cluster, Some(symbol)).await;
    Some(*product_accounts.get(symbol).unwrap())
}

pub async fn fetch_pyth_product_accounts(
    cluster: &Cluster,
    symbol: Option<&str>,
) -> HashMap<String, Pubkey> {
    let addr = get_pyth_mapping_account(cluster);
    let mut akey = Pubkey::from_str(&addr).unwrap();

    let mut product_accounts = HashMap::new();

    loop {
        // get Mapping account from key
        let map_data: &[u8] = &get_account_data(&cluster, &akey).await;
        let map_acct = load_mapping(&map_data).unwrap();

        for prod_akey in &map_acct.products {
            let prod_pkey = Pubkey::new(&prod_akey.val);
            let prod_data: &[u8] = &get_account_data(&cluster, &prod_pkey).await;
            let prod_acct = match load_product(&prod_data) {
                Ok(prod_acct) => prod_acct,
                Err(_) => break,
            };

            // print key and reference data for this Product
            // println!("prod_pkey .. {:?}", prod_pkey);
            let mut pit = (&prod_acct.attr[..]).iter();

            let _ = get_attr_str(&mut pit);
            let val = get_attr_str(&mut pit);
            // println!("  {:.<16} {}", key, val);

            // Valid?
            if prod_acct.px_acc.is_valid() {
                // Then keep it
                product_accounts.insert(val.clone(), Pubkey::new(&prod_acct.px_acc.val));
            }

            // Found specific symbol?
            if symbol.is_some() && val == symbol.unwrap().to_string() {
                // Found specific symbol
                break;
            }
        }

        // go to next Mapping account in list
        if !map_acct.next.is_valid() {
            break;
        }
        akey = Pubkey::new(&map_acct.next.val);
    }

    product_accounts
}

pub async fn fetch_pyth_price_by_symbol(cluster: &Cluster, symbol: &str) -> Option<PriceConf> {
    // Get product account
    let px_pkeys = fetch_pyth_product_accounts(&cluster, Some(symbol)).await;

    // Guard none px_pkey
    if !px_pkeys.contains_key(symbol) {
        return None;
    }

    let px_pkey = *px_pkeys.get(symbol).unwrap();

    // Get price
    fetch_pyth_price_by_pubkey(&cluster, &px_pkey).await
}

pub async fn fetch_pyth_price_by_pubkey(cluster: &Cluster, px_pkey: &Pubkey) -> Option<PriceConf> {
    let mut current_price;
    let mut px_pkey = *px_pkey;
    loop {
        let pd: &[u8] = &get_account_data(&cluster, &px_pkey).await;
        let pa = load_price(&pd).unwrap();

        current_price = pa.get_current_price();

        // go to next price account in list
        if pa.next.is_valid() {
            px_pkey = Pubkey::new(&pa.next.val);
        } else {
            break;
        }
    }

    current_price
}

#[cfg(test)]
#[tokio::test]
async fn test_fetch_pyth_product_accounts() {
    let cluster = Cluster::Devnet;
    let product_accounts = fetch_pyth_product_accounts(&cluster, None).await;

    println!("product_accounts: {:?}", product_accounts);
    assert_eq!(product_accounts.is_empty(), false);
    assert!(product_accounts.capacity() > 1);
}

#[cfg(test)]
#[tokio::test]
async fn test_fetch_pyth_product_account_by_symbol() {
    let cluster = Cluster::Devnet;
    let symbol = "Crypto.SOL/USD";
    let product_account = fetch_pyth_product_account_by_symbol(&cluster, symbol).await;

    println!("product_account: {:?}", product_account);
    assert_eq!(
        product_account.unwrap().to_string(),
        "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix".to_string()
    );
}

#[cfg(test)]
#[tokio::test]
async fn test_fetch_pyth_price_by_symbol() {
    let cluster = Cluster::Devnet;
    let symbol = "Crypto.SOL/USD";
    let current_price = fetch_pyth_price_by_symbol(&cluster, symbol).await;

    println!("current_price: {:?}", current_price);
    assert_ne!(current_price, None);
}

#[cfg(test)]
#[tokio::test]
async fn test_fetch_pyth_price_by_pubkey() {
    let cluster = Cluster::Devnet;
    // Mocked SOL/USD
    let address = Pubkey::from_str("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix").unwrap();

    // Fetch price from pyth
    let product_account = fetch_pyth_price_by_pubkey(&cluster, &address).await;

    println!("product_account: {:?}", product_account);
    assert_ne!(product_account, None);
}
