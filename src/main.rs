//first terminal run command cargo run
//second and thrid terminal command --- telnet localhost 8080 then start the chat -- mult client can easy chat with each other

use tokio::{
    io::{AsyncBufReadExt,  AsyncWriteExt, BufReader},
    net::TcpListener, sync::broadcast,
};

// #[tokio::main]
// async fn main() {
//     println!("hello");
//     let listener= TcpListener::bind("localhost:8080").await.unwrap();
//  loop{
//     //mult client
//     let(mut socket,_addr)=listener.accept().await.unwrap();

//     tokio::spawn(async move { //handle multiple client every one write and read //mult theard handle

//     let (reader, mut writer)=socket.split();
//     let mut reader= BufReader::new(reader);
//     let mut line= String::new();

//     loop{
//    // let mut buffer = [0u8;1024];
//     //signal client
//     let bytes_read= reader.read_line(&mut line).await.unwrap();
//     if bytes_read==0{
//         break;
//     }

//     writer.write_all(line.as_bytes()).await.unwrap();
//     line.clear();
//     }
// });
//  }
// }


//---------------------------------------------------------------------//
//                     chat server                                    //
//--------------------------------------------------------------------//
#[tokio::main]
async fn main() {
    println!("hello");
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, rx)=broadcast::channel(10);
    loop {
        //mult client
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx=tx.clone();
        let mut rx= tx.subscribe();

        tokio::spawn(async move {
            //handle multiple client every one write and read //mult theard handle

            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                // let mut buffer = [0u8;1024];
                //signal client
                tokio::select! {
                result = reader.read_line(&mut line)=>{
                    if result.unwrap() == 0 {
                        break;
                    }
                    tx.send((line.clone(), addr)).unwrap();
                    line.clear();
                }
                
                result=rx.recv()=>{
                    let (msg,other_addr)=result.unwrap();
                    if addr!=other_addr{
                        writer.write_all(msg.as_bytes()).await.unwrap();

                    }

                }

            }
            }
        });
    }
}
