use serde_json::Value;
use socketioxide::{
    extract::{Data, SocketRef},
    layer::SocketIoLayer,
    SocketIo,
};

fn on_connect(socket: SocketRef) {
    socket.on("message", |socket: SocketRef, Data::<Value>(data)| {
        println!("received message {data}");
        socket.broadcast().emit("message-back", &data).ok();
    });
}

pub fn new_layer() -> SocketIoLayer {
    let (socket_layer, io) = SocketIo::new_layer();
    io.ns("/", on_connect);
    socket_layer
}
