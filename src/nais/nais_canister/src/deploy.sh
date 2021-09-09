dfx canister --no-wallet install nais --mode="reinstall"
dfx canister --no-wallet call nais "Initialize" "()"
./nais_admin --pem identity.pem update --candid nais_canister.did "rrkah-fqaaa-aaaaa-aaaaq-cai" "uploadNftWasm"
./nais_admin --pem identity.pem update --candid nais_canister.did "rrkah-fqaaa-aaaaa-aaaaq-cai" "uploadAndersonWasm"
