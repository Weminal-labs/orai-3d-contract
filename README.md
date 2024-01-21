Code id: 6677

Contract address: orai1n27agkvx9jx58qdce5ezcp7cx3zuefhzt75gg0f6kvw5a9ucujuqygykf4

## Upload wasm and init contract

```
yarn oraicli wasm upload /home/asus/Workspace/oraichain/orai-decal-contract/artifacts/orai_3d.wasm --fees 2500orai

yarn oraicli wasm instantiate --code-id 6677 --input '{"symbol": "Orai 3D", "minter": "orai1yzsegvns6vmvf5q29uv26p3th4fd2kzmsq3h6m"}' --label "Orai 3D"
```

## Execute

### Mint

#### T-Shirt

```
yarn oraicli wasm execute
 --input '{"mint":{"token_id":"1","owner":"orai1yzsegvns6vmvf5q29uv26p3th4fd2kzmsq3h6m","name":"T-Shirt","image":"https://i.ebayimg.com/images/g/4IsAAOSwpddgqEwp/s-l1200.jpg","prompt":""}}'
```

#### Cup

```
yarn oraicli wasm execute
 --input '{"mint":{"token_id":"2","owner":"orai1yzsegvns6vmvf5q29uv26p3th4fd2kzmsq3h6m","name":"Cup","image":"https://www.shutterstock.com/image-vector/paper-cup-filled-black-coffee-600nw-1801429321.jpg","prompt":""}}'
```

#### Shoes

```
yarn oraicli wasm execute
 --input '{"mint":{"token_id":"3","owner":"orai1yzsegvns6vmvf5q29uv26p3th4fd2kzmsq3h6m","name":"Shoes","image":"https://static.nike.com/a/images/c_limit,w_592,f_auto/t_product_v1/b14aba9a-f828-45d3-9607-b687b884aa7d/revolution-7-easyon-road-running-shoes-nNqdwt.png","prompt":""}}'
```

### Commit

```
{"commit": {"token_id": "1", "prompt": "A radiant sun rises over a dark, serene landscape, casting an ethereal glow.", "eueno_url": "https://node1-gateway-ipfs.eueno.io/ipfs/QmegwaTzbVg3LDzuCG18m7BSzdcdrndfvboueraUZS7jG6"}}
```

```
{"commit": {"token_id": "1", "prompt": "Dark-light gradient, purple-pink-black, white stars, vivid light streaks, evokes wonder and vast space.", "eueno_url": "https://node1-gateway-ipfs.eueno.io/ipfs/QmQ62LKjFSWJTc1QmHfJe4mvHCFkDJHyVXiuoATtE9vN5Y"}}
```

### Approve Commit

```
{"approve_commit": {"token_id": "1", "commit_id": ""}}
```

```
{"approve_commit": {"token_id": "1", "commit_id": ""}}
```

## Query

```
{"token_info": {"token_id": "1"}}
```