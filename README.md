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
