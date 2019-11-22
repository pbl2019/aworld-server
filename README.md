# Control Server

> プレイヤー毎のコントローラー状態を管理

## Usage

▼ Mac ユーザー向け
```sh
# ローカル内
$ cd docker; make up
$ make bash

# コンテナ内
$ cargo run
# ビルド版
$ cargo run --release
```

## port

- コントロールサーバのport
  - UDPを受け付ける: 34255 (クライアントはこのportに向かってudpで送信する必要)
  - データサーバーへのUDPで送信するために必要なport: 34250 (適当な値でok)
- データサーバーのport
  - UDPを受け付ける: 34254 (コントロールサーバーはこのportに向かってudpで送信する必要)

## Note

## データの持ち方

```
charaId: {
  ArrowUp: True
  ArrowDown: False
  ・
  ・
}
```

### AIようにコマンドを作成しておく
TradeStart
TradeAccept
TradeDeny
TradeItem: Itemの選択
TradeEnd

### データの流れ

クライアント → コントロールサーバー → データサーバ → データサーバ

通信: udp

## ncコマンドによるテスト

```sh
apt-get update && apt-get install -y nc
nc -u localhost 34254
{"addr": "firefox", "kind": "forward", "payload": {"speed": 10}}
```

## portが使われてるよって言われる場合

```sh
apt-get update && apt-get install -y lsof
lsof -i
COMMAND   PID USER   FD   TYPE DEVICE SIZE/OFF NODE NAME
aworld-se  12 root    3u  IPv4  19049      0t0  UDP localhost:34255 
aworld-se  12 root    4u  IPv4  19050      0t0  UDP localhost:34250 
aworld-se  12 root    5u  IPv4  19049      0t0  UDP localhost:34255 

apt-get update && apt-get install -y kill
kill -9 hogehogepid # ここのhogehogepidは、lsof -iで確認したPIDを入れる
```
