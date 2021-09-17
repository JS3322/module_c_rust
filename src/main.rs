use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
//use std::thread;
//use std::time::Duration;
use module_rust::ThreadPool;

fn main() {
    //1024보다 작은 포트 or 다른 프로세스에 포트 연결 시도시 Err 반환 > 종료 : unwrap
    let listener = TcpListener::bind("127.0.0.1:15220").unwrap();
    let pool = ThreadPool::new(3);

    //incommig : TcpStream 반복자를 반환하며 멀티 쓰레드처럼 데이터를 전송
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
            println!("connection");
        });
    }
}

//TcpStream 내부 읽고 저장하기 때문에 가변성 적용
fn handle_connection(mut stream: TcpStream) {
    //1024bytes 버터 생성 0으로 채움
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    //println!("Reauest: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents,);

    //TcpStream 요청에 대한 응답시 byte로 변환하여 보냄, write 매개변수 자료형이 &[u8]
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
