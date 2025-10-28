use async_udx::UdxSocket;
use std::io;

#[tokio::test]
async fn socket_dgrams() -> io::Result<()> {
    let socka = UdxSocket::bind("127.0.0.1:0")?;
    let sockb = UdxSocket::bind("127.0.0.1:0")?;

    let msg = "hi!".as_bytes();
    socka.send(sockb.local_addr()?, msg);
    let (_from, buf) = sockb.recv().await?;
    assert_eq!(&buf, msg);

    Ok(())
}
