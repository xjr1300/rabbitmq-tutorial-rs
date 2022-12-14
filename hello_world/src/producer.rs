use lapin::{options::BasicPublishOptions, BasicProperties};
use tracing::info;

use common::{connect, set_default_logging_env};

use hello_world::{declare_queue, QUEUE_NAME};

fn main() {
    set_default_logging_env();

    tracing_subscriber::fmt::init();

    async_global_executor::block_on(async {
        // RabbitMQに接続
        let conn = connect().await;
        info!("connected");

        // チャネルを作成
        let channel = conn.create_channel().await.expect("create channel error");
        info!(state=?conn.status().state());

        // キューを定義
        let queue = declare_queue(&channel).await;
        info!(state=?conn.status().state());
        info!(?queue, "declared queue");

        info!("publish");
        let payload = b"Hello world!";
        let _confirm = channel
            .basic_publish(
                "",         // デフォルトエクスチェンジ
                QUEUE_NAME, // ルーティングキー: デフォルトエクスチェンジの場合、ルーティングキーと同じ名前のキューに発行
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await
            .expect("basic publish error");
    });
}
