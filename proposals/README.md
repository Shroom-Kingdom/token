# How to run proposals

## Change config

```json
export NEAR_ENV=mainnet
near login

# create proposal on SputnikDAO
ARGS=`cat ./policy.json`
near call shrm.sputnik-dao.near add_proposal '{
  "proposal": {
    "description": "DAO policy change. This proposal adds community members",
    "kind": {
      "ChangePolicy": {
        "policy": $ARGS
      }
    }
  }
}' --accountId $ACCOUNT_ID --amount 1
```

## SHRM Token Genesis

```json
export NEAR_ENV=mainnet
near login
near create-account token.shrm.near --masterAccount shrm.near --initialBalance 100

# build and deploy
./build.sh
near deploy --wasmFile res/shrm_token.wasm --accountId token.shrm.near

# create proposal on SputnikDAO
ARGS=`cat ./token.json | base64 -w 0`
near call shrm.sputnik-dao.near add_proposal '{
  "proposal": {
    "description": "SHRM token genesis",
    "kind": {
      "FunctionCall": {
        "receiver_id": "token.shrm.near",
        "actions": [
          {
            "method_name": "new",
            "args": "$ARGS",
            "deposit": "0",
            "gas": "20000000000000"
          }
        ]
      }
    }
  }
}' --accountId $ACCOUNT_ID --amount 1
```

## SHRM token airdrop

```json
# create proposal on SputnikDAO
near call shrm.sputnik-dao.near add_proposal '{
  "proposal": {
    "description": "Launch SHRM token airdrop",
    "kind": "Vote"
  }
}' --accountId $ACCOUNT_ID --amount 1
```

After proposal succeeds, run airdrop manually:

```sh
# fetch eligible accounts
curl 'https://stats.shroomkingdom.net/mainnet/score-query?offset=0&limit=100000&minlevel=3&stake=1&account_id_like=%25.near&account_id_not_like=%25.%25.near&created_before=1632096000000000000' | jq 'map({ account_id: .account_id, amount: (.level | tostring + "000000000000000000") })' > proposals/airdrop.json
#curl 'https://stats-testnet.shroomkingdom.net/testnet/score-query?offset=0&limit=100000&minlevel=1' | jq 'map({ account_id: .account_id, amount: (.level | tostring + "000000000000000000") })' > proposals/airdrop.json

# generate input for airdrop
cargo test

# run airdrop
search_dir='proposals/airdrop'
for entry in "$search_dir"/*
do
  ARGS=$(cat $entry)
  near call token.shrm.near airdrop --base64 $ARGS --accountId token.shrm.near --gas 300000000000000
done
```
