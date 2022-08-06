# RabbitMQ Tutorial by Rust

## RabbitMQコンテナ

- rabbitmq:3.10.7-management
  - バージョン3.10.7
  - 管理プラグイン付き

### RabbitMQコンテナ起動

```bash
docker-compose up -d
```

### RabbitMQ管理ページの表示

- URL: http://localhost:15672/
- ユーザー名: guest
- パスワード: guest

## RabbitMQとメッセージブローカー

`RabbitMQ`はメッセージブローカーである。メッセージブローカーはメッセージを受け取り、送信する。
`RabbitMQ`のメッセージは単なるバイナリデータであり、`RabbitMQ`はメッセージの内容を解釈しない。

`プロデューサー`はメッセージを送信（生産）して、`コンシューマー`がメッセージを処理（消費）する。

`エクスチェンジ`は、`プロデューサー`と`コンシューマー`の間でメッセージを仲介する。
`エクスチェンジ`は、`プロデューサー`から受信したメッセージを`キュー`に蓄積する。

`コンシューマー`は、`キュー`に接続して、キューイングされたメッセージを処理する。

`エクスチェンジ`は、メッセージが`コンシューマー`によって確実に処理されたか管理する。
`エクスチェンジ`は、メッセージが処理されていないと判断した場合は、そのメッセージを再キューイングする。

`RabbitMQ`は、以下の場合メッセージを再キューイングする。

* 肯定応答を受診する前にコンシューマーと切断されてチャネルが閉じられたとき
* コンシューマーが再キューイングを指定して`reject`や`nack`で応答したとき

## 1. Hello World

プロデューサーが、 `デフォルトエクスチェンジ `にルーティングキー`hello`を使用してメッセージを送信することで、`hello`キューに、メッセージがキューイングされる。

コンシューマーは、`hello`キューにキューイングされたメッセージを取得して処理する。
コンシューマーが処理に成功した場合は、`RabbitMQ`に肯定応答を返却して、`RabbitMQ`にメッセージが正常に処理したことを伝える。
これにより`RabbitMQ`はこのメッセージを削除する。

`デフォルトエクスチェンジ`は、`メッセージブローカー（RabbitMQ)`によってい事前に準備された、名前を持たない`ダイレクトエクスチェンジ`である。

`デフォルトエクスチェンジ`にメッセージを送信したとき、そのメッセージの`ルーティングキー`と同じ名前の`キュー`にメッセージがキューイングされる。

![Hello World](https://www.rabbitmq.com/img/tutorials/python-one.png)

```bash
# コンシューマーを起動
cargo run --package hello_world --bin consumer
# プロデューサーがメッセージを発行
cargo run --package hello_world --bin producer
```
コンシューマーは、`Ctrl + C`で強制終了する。
