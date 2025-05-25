# Piko

本项目是基于 [Milky 协议](https://milky.ntqqrev.org/) 构建的 Rust 异步 Bot 框架，采用 [`tokio`](https://tokio.rs/) 作为异步运行时。

## 项目结构

```
piko
├── piko-milky
```

* `piko-milky` 是 [Milky 协议](https://milky.ntqqrev.org/) 的 Rust 客户端实现。
* 当前实现已完整覆盖协议定义，但尚未进行充分测试。

### 模块结构

```
piko-milky
├── common      # 通用工具模块
├── entity      # 协议结构体定义，对应 milky 文档中的实体
├── event       # 事件定义，对应 milky 文档中的事件部分
├── proto       # API 定义，对应 milky 文档中的接口
│   ├── file
│   ├── friend
│   ├── group
│   ├── message
│   ├── request
│   └── system
└── route       # 事件路由器，简化事件监听的业务开发
```

### API 设计示例（以 group\_files 模块为例）

```rust
pub mod group_files {
    pub struct Get { /* ... */ }
    pub struct GetResponse { /* ... */ }
    impl_request! { "get_group_files" | Get, GetResponse }

    pub struct Move { /* ... */ }
    impl_request! { "move_group_files" | Move }

    pub struct Rename { /* ... */ }
    impl_request! { "rename_group_files" | Rename }

    pub struct Delete { /* ... */ }
    impl_request! { "delete_group_files" | Delete }
}
```

所有请求类型均实现 `piko_milky::proto::Request` 特征，具备 `send` 方法：

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bot = Bot::connect("localhost:1234".into());

    let res = group_files::Get {
        group_id: 12345678,
        parent_folder_id: None,
    }.send(&bot).await?;
}
```

## 路由器使用示例

```rust
use std::sync::Arc;
use tokio::sync::Mutex;
use piko_milky::{
    handler, kind, Bot, Events, Meta, Router, RouterTrait,
    GroupJoinRequest, MessageReceive,
};

#[derive(Default)]
struct MyBot {
    // 自定义状态
}

// 基于锁实现的状态封装
// 如果你不需要访问 MyBot 的可变引用，则不需要加锁
// 一般来说你需要为共享状态实现内部可变性
type MyBotState = Arc<Mutex<MyBot>>;

// 一个处理器
async fn handle_message_receive(
    // 元数据包含了 bot，自定义 state 等重要信息，其泛型参数表示 state 的类型
    Meta { bot, state, .. }: Meta<MyBotState>,
    // 事件对象，即 event 下定义的事件实体
    event: MessageReceive,
) {
    // 处理消息接收事件
}

// 一个处理器
async fn handle_group_join_request(
    Meta { bot, state, .. }: Meta<MyBotState>,
    event: GroupJoinRequest,
) {
    // 处理加群请求事件
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化共享状态
    let my_bot = MyBot::default();
    let my_bot_state = Arc::new(Mutex::new(my_bot));

    // 通过 route 方法设置路由，kind 与事件实体是一一对应的, 而 handler(handle_message_receive) 
    // 则是将上述处理器函数 handle_message_receive 包装成一个真正的处理器。最后的 with_state(my_bot_state) 
    // 是设置全局共享状态，所有处理器都能在元数据的 state 字段获得该对象的实例，这也意味着一个路由器中所有的
    // 处理器的 state 的类型必须一致。
    let app = Router::new()
        .route(kind::MessageReceive, handler(handle_message_receive))
        .route(kind::GroupJoinRequest, handler(handle_group_join_request))
        .with_state(my_bot_state);

    // 连接到 milky bot 服务端
    let bot = Bot::connect("localhost:1234".into());

    // 监听 bot 事件
    let events = Events::listen(&bot, app).await?;

    // 监听 ctrl_c 事件
    tokio::signal::ctrl_c().await?;

    // 停止监听 bot 事件
    events.close().await;

    Ok(())
}
```

---

## License

This project is licensed under the MIT License.


