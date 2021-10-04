# How to run proposals

## Change config

```sh
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

```sh
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
