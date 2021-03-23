use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let addr = format!("127.0.0.1:9001"); // 服务器绑定ip和端口

    // 监听地址
    match TcpListener::bind(addr.as_str()) {
        Ok(listener) => {
            // 获取tcpStream
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                //处理tcpStream
                handle_stream(stream);
            }
        },
        Err(_err) => {
            // 打印错误
            println!("bind {} error, exit", addr.as_str());
        }
    };
}

// 处理tcpStream
fn handle_stream(mut stream: TcpStream) {

    let mut seq = 1; // 消息次数
    // 循环接受数据
    loop {
        // 声明缓冲区保存数据
        let mut buffer = vec![0; 512];
        // 读取到缓冲区
        stream.read(&mut buffer).unwrap();
        // 将读取的数据转换成string
        let message = String::from_utf8(buffer).expect("invalid utf8 message");
        // 去掉字符串中的空白符号
        let message = message.trim_matches(char::from(0));
        println!("message from client: {}", message);
        // 返回给客户端的消息
        let res_message = format!("#{seq:?} server received: {message:?}\n", seq = seq, message = message);
        // 给客服端返回数据
        stream.write(res_message.as_bytes()).unwrap();
        // 立即刷新stream缓冲区
        stream.flush().unwrap();
        seq += 1;
    }
}