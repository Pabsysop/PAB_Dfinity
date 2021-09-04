pub mod types;

use ic_cdk::api;
use candid::Principal;
use nft::*;

pub async fn listen_to(to: Principal, args: String) -> Result<(), String> {
    match api::call::call(to, "Listen", (args,)).await
    {
        Ok(x) => x,
        Err((code, msg)) => {
            return Err(format!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            ))
        }
    };
    Ok(())
}
pub async fn send_nft_call(to: Principal, args: ()) -> Result<(), String> {
    match api::call::call(to, "RcvNft", (args,)).await
    {
        Ok(x) => x,
        Err((code, msg)) => {
            return Err(format!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            ))
        }
    };
    Ok(())
}
pub async fn mint_visa_nft_call(nft_can: Principal, egg: NftEgg) -> Result<NFT, String>{
    match api::call::call(nft_can,"mint", (egg,)).await
    {
        Ok(x) => x,
        Err((code, msg)) => {
            return Err(format!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            ))
        }
    };
    Err(format!("mint fail"))
}
