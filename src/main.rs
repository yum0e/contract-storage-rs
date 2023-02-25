mod error;

use error::CustomError;
use ethers::{
    abi::{Abi, Address},
    providers::{Http, JsonRpcClient, Middleware, Provider},
    types::{BigEndianHash, BlockId, H256, U256},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};

#[derive(Serialize, Deserialize, Debug)]
struct AbiFromFile {
    abi: Abi,
}

fn uint_to_hex(nb: u32) -> H256 {
    H256::from_uint(&U256::from(nb))
}

async fn get_storage<T: JsonRpcClient>(
    provider: &Provider<T>,
    contract_address: &str,
    max_slot: u32,
) -> Result<HashMap<H256, H256>, CustomError> {
    let mut storages: HashMap<H256, H256> = HashMap::new();
    let last_block_number = provider.get_block_number().await?;
    let addr = contract_address.parse::<Address>()?;

    for slot in 0u32..max_slot {
        let location = uint_to_hex(slot);
        let storage = provider
            .get_storage_at(addr, location, Some(BlockId::from(last_block_number)))
            .await?;
        {
            storages.insert(location, storage);
        };
    }
    Ok(storages)
}

#[tokio::main]
async fn main() {
    let rpc_endpoint =
        env::var("RPC_ENDPOINT").expect("You should add an RPC endpoint in your env vars");
    let provider: Provider<Http> =
        Provider::<Http>::try_from(rpc_endpoint).expect("could not instantiate HTTP Provider");

    let max_slot: u32 = 10;
    let storages = match get_storage(
        &provider,
        "0xe77eb6fb5037bCb11db10b9Ae478A7D01354Ae01",
        max_slot,
    )
    .await
    {
        Ok(storages) => storages,
        Err(e) => {
            match e {
                CustomError::ProviderError => eprintln!("Provider Error: {}", e),
                CustomError::ParseHexError => eprintln!("ParseHexError: {}", e),
            }
            return;
        }
    };
    for key in 0u32..max_slot {
        if let Some(value) = storages.get(&uint_to_hex(key)) {
            println!("{}", format!(r"[{}]: [{:?}]", &uint_to_hex(key), value));
        };
    }
}
