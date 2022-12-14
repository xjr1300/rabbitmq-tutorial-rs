use lapin::{options::BasicPublishOptions, BasicProperties};
use tracing::info;

use common::{connect, set_default_logging_env, PERSISTENT_DELIVERY_MODE};

use work_queues::{declare_queue, QUEUE_NAME};

fn main() {
    set_default_logging_env();

    tracing_subscriber::fmt::init();

    async_global_executor::block_on(async {
        let conn = connect().await;
        info!("connected");

        let channel = conn.create_channel().await.expect("create channel error");
        info!(state=?conn.status().state());

        // 永続するキューを定義
        let queue = declare_queue(&channel).await;
        info!(state=?conn.status().state());
        info!(?queue, "declare queue");

        info!("publish");
        let properties = BasicProperties::default().with_delivery_mode(PERSISTENT_DELIVERY_MODE);
        let payload = b"Hello world!";
        let _confirm = channel
            .basic_publish(
                "",         // デフォルトエクスチェンジに送信
                QUEUE_NAME, // ルーティングキー: デフォルトエクスチェンジに送信するため、ルーティングキーと同じ名前のキューにメッセージを`キューイング
                BasicPublishOptions::default(),
                payload,
                properties,
            )
            .await
            .expect("basic publish error");
    });
}
