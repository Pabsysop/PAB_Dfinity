pub mod types;

use ic_cdk::api;
use candid::Principal;
use nft::*;

pub async fn listen_to(to: Principal, args: String) {
    match api::call::call(to, "Listen", (args,)).await
    {
        Ok(x) => x,
        Err((code, msg)) => {
            println!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            );
        }
    };
}

pub async fn mint_visa_nft_call(nft_can: Principal, owner: Principal) -> Result<String, String>{
    let egg = NftEgg {
        payload: NFTPayload::Payload(vec![0x00]),
        content_type: "text".to_string(),
        owner,
        properties: vec![Property{
            name : String::from("board visa"),
            value : Value::Empty,
            immutable : true
        }],
        is_private: true
    };

    let (nft_id,): (String,) = match api::call::call(
        nft_can,
        "mint", 
        (egg,)).await
    {
        Ok(x) => x,
        Err((code, msg)) => {
            return Err(format!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            ))
        }
    };
 
    Ok(nft_id)
}
