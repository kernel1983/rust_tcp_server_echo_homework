// 从 https://doc.rust-lang.org/std/net/struct.TcpListener.html 抄的代码并加以修改

use std::io::prelude::*; //这一行不加上就没有read write，不知道为什么
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?; //绑定IP和端口号

    for stream in listener.incoming() { //是个迭代器，返回stream但是没有addr，够用
        match stream {
            Ok(mut stream) => {//这里要加mut，试了好久，不然编译不过
                println!("new client!"); //一条log
                let mut input:[u8; 128] = [0; 128]; //可以修改的uint8，超出128输入会溢出
                while( !((input[0] == 'q' as u8) && (input[1] == '\r' as u8)) ) { //q回车退出，在linux下测试ok，win下可能不行
                    input = [0; 128]; // C语言方式的覆盖内存
                    stream.read(&mut input)?; //从tcp读
                    stream.write(&input)?; //写回，实现echo
                }
            }
            Err(_e) => { /* connection failed */ }
        }
    }
    Ok(())
}
