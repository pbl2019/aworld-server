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
