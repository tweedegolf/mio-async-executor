mod executor;

fn main() {
    let (executor, spawner) = executor::new_executor_spawner();
    spawner.spawn(async_main());
    // Drop this spawner, so that the `run` method can stop as soon as all other
    // spawners (stored within tasks) are dropped
    drop(spawner);

    executor.run();
}

async fn async_main() {
    let socket = executor::UdpSocket::bind("127.0.0.1:8000").unwrap();

    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    let mut buf = [0; 10];
    let (amt, src) = socket.recv_from(&mut buf).await.unwrap();

    // Redeclare `buf` as slice of the received data and send reverse data back to origin.
    let buf = &mut buf[..amt];
    buf.reverse();
    socket.send_to(buf, src).await.unwrap();
}
