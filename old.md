Code id: 6613

Contract address: orai1094ellehzmlsqf6uax7lhlayggd6wf8vu2ejtlzfgk7e8d4pjgssglxvsf

# Find bug

- Come from the fn generate_random_string
  => replace with env.block.time for commit id

# Use Cosmwasm IDE extension instead of CLI

# CLI

## Upload wasm and init contract

```
yarn oraicli wasm upload /home/asus/Workspace/oraichain/orai-decal-contract/artifacts/orai_decal_nft.wasm --fees 2500orai

yarn oraicli wasm instantiate --code-id 6613 --input '{"symbol": "Orai Decal Contract", "minter": "orai1yzsegvns6vmvf5q29uv26p3th4fd2kzmsq3h6m"}' --label "orai decal"
```

## Execute

### Mint

```
yarn oraicli wasm execute
orai1094ellehzmlsqf6uax7lhlayggd6wf8vu2ejtlzfgk7e8d4pjgssglxvsf --input '{"mint":{"token_id":"1","owner":"orai1yzsegvns6vmvf5q29uv26p3th4fd2kzmsq3h6m","name":"3D T-shirt Decal","image":"This is image","prompt":"A cat"}}'
```

### Commit

```
{"commit": {"token_id": "2", "prompt": "a dog"}}
```

### Approve Commit

```
{"approve_commit": {"token_id": "2", "commit_id": "1705118448"}}
```

#### Before approve commit

```
{
  "data": {
    "token_info": {
      "owner": "IKGUMnDTNsTQCi8YrQYrvVLVWFs=",
      "owner_human_addr": "orai1yzsegvns6vmvf5q29uv26p3th4fd2kzmsq3h6m",
      "approvals": [],
      "name": "3D T-shirt Decal",
      "description": "",
      "image": "This is image",
      "prompt": "A cat",
      "contributors": [
        "orai1yzsegvns6vmvf5q29uv26p3th4fd2kzmsq3h6m",
        "orai1e2a2l2903dzxeh7ct7u8ahs3pz8g30qn8tcd8k"
      ],
      "commits": [
        {
          "owner": "orai1yzsegvns6vmvf5q29uv26p3th4fd2kzmsq3h6m",
          "id": "1705117333",
          "prompt": "A cat",
          "is_approved": true,
          "created_at": 1705117333
        },
        {
          "owner": "orai1e2a2l2903dzxeh7ct7u8ahs3pz8g30qn8tcd8k",
          "id": "1705118448",
          "prompt": "a dog",
          "is_approved": false,
          "created_at": 1705118448
        }
      ],
      "status": "Minted"
    }
  }
}
```

#### After approve commit

```
{
  "data": {
    "token_info": {
      "owner": "IKGUMnDTNsTQCi8YrQYrvVLVWFs=",
      "owner_human_addr": "orai1yzsegvns6vmvf5q29uv26p3th4fd2kzmsq3h6m",
      "approvals": [],
      "name": "3D T-shirt Decal",
      "description": "",
      "image": "This is image",
      "prompt": "a dog",
      "contributors": [
        "orai1yzsegvns6vmvf5q29uv26p3th4fd2kzmsq3h6m",
        "orai1e2a2l2903dzxeh7ct7u8ahs3pz8g30qn8tcd8k"
      ],
      "commits": [
        {
          "owner": "orai1yzsegvns6vmvf5q29uv26p3th4fd2kzmsq3h6m",
          "id": "1705117333",
          "prompt": "A cat",
          "is_approved": true,
          "created_at": 1705117333
        },
        {
          "owner": "orai1e2a2l2903dzxeh7ct7u8ahs3pz8g30qn8tcd8k",
          "id": "1705118448",
          "prompt": "a dog",
          "is_approved": true,
          "created_at": 1705118448
        }
      ],
      "status": "Minted"
    }
  }
}
```

## Query

```
{"token_info": {"token_id": "2"}}
```

```
{
  "data": {
    "token_info": {
      "owner": "IKGUMnDTNsTQCi8YrQYrvVLVWFs=",
      "owner_human_addr": "orai1yzsegvns6vmvf5q29uv26p3th4fd2kzmsq3h6m",
      "approvals": [],
      "name": "3D T-shirt Decal",
      "description": "",
      "image": "This is image",
      "prompt": "A cat",
      "contributors": [
        "orai1yzsegvns6vmvf5q29uv26p3th4fd2kzmsq3h6m"
      ],
      "commits": [
        {
          "owner": "orai1yzsegvns6vmvf5q29uv26p3th4fd2kzmsq3h6m",
          "id": "1705117333",
          "prompt": "A cat",
          "is_approved": true,
          "created_at": 1705117333
        }
      ],
      "status": "Minted"
    }
  }
}
```
