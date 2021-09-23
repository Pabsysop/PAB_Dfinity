use candid::Principal;
use ic_cdk::api;

pub async fn listen(board_canister: &Principal, room_id: String) -> Result<String, String> {
    let (session,): (String,) = match api::call::call(
        board_canister.clone(),
        "Join", 
        (room_id,)
    ).await 
    {
        Ok(x) => x,
        Err((code, msg)) => {
            return Err(format!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            ))
        }
    };

    Ok(session)
}

pub async fn request_invite_code(nais_canister: &Principal) -> Option<Vec<String>> {
    let (codes,): (Vec<String>,) = match api::call::call(
        nais_canister.clone(),
        "GetInviteCodes",
        ()
    ).await 
    {
        Ok(x) => x,
        Err((code, msg)) => {
            println!("An error happened during the call: {}: {}", code as u8, msg);
            return None;
        }
    };

    Some(codes)
}

pub async fn create_board_call(nais_canister: &Principal, chairman: Principal) -> Option<Principal>{
    let (board_can_id,): (Principal,) = match api::call::call(
        nais_canister.clone(),
        "BuildCivilization",
        (chairman,)
    ).await 
    {
        Ok(x) => x,
        Err((code, msg)) => {
            println!("An error happened during the call: {}: {}", code as u8, msg);
            return None;
        }
    };

    Some(board_can_id)
}
