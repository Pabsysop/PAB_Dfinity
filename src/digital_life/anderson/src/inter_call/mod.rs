use candid::Principal;
use ic_cdk::api;

pub async fn leave(board_canister: &Principal, room_id: String) {
    let _ret: () = api::call::call(
        board_canister.clone(),
        "LeaveRoom", 
        (room_id,)
    ).await.unwrap_or(());
}

pub async fn listen(board_canister: &Principal, room_id: String, ticket: Option<String>) -> Result<String, String> {
    let (session,): (String,) = match api::call::call(
        board_canister.clone(),
        "JoinRoom", 
        (ticket, room_id,)
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

pub async fn speak(board_canister: &Principal, room_id: String) {
    let _ret: () = api::call::call(
        board_canister.clone(),
        "Speak", 
        (room_id,)
    ).await
    .unwrap_or_else(|(code, msg)| 
        ic_cdk::trap(format!("An error happened during the call: {}: {}",code as u8, msg).as_str())
    );
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
            ic_cdk::println!("An error happened during the call: {}: {}", code as u8, msg);
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
            ic_cdk::println!("An error happened during the call: {}: {}", code as u8, msg);
            return None;
        }
    };

    Some(board_can_id)
}

pub async fn open_room_call(board_canister: &Principal, title: String, cover: Option<String>) -> Option<String>{
    let (room_id,): (String,) = match api::call::call(
        board_canister.clone(),
        "OpenRoom",
        (title, cover,)
    ).await 
    {
        Ok(x) => x,
        Err((code, msg)) => {
            ic_cdk::println!("An error happened during the call: {}: {}", code as u8, msg);
            return None;
        }
    };

    Some(room_id)
}

pub async fn follow(life_canister: &Principal){
    let (_ret,): ((),) = match api::call::call(
        life_canister.clone(),
        "FollowMe",
        ()
    ).await 
    {
        Ok(x) => x,
        Err((code, msg)) => {
            ic_cdk::println!("An error happened during the call: {}: {}", code as u8, msg);
            ((),)
        }
    };
}
