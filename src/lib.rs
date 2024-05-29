pub mod header;


use header::*;

use async_std::prelude::*;
use async_std::fs;

use async_std::net::TcpStream;
use async_native_tls::TlsStream;

use async_std::io::BufReader;





pub async fn handle_client(mut stream: TlsStream<TcpStream>) {
    
    loop {
        let mut buf_reader = BufReader::new(&mut stream);

         let mut request = Vec::new();

         //"Reading request"
         loop {
            let mut request_line = String::new();
            buf_reader.read_line(&mut request_line).await.unwrap();

            if request_line == "\r\n"  {
                break;
            }
            else {

                request.push(request_line.replace("\r\n",""));
            }
         }

        let mut header = Header::new();


        
        header.process_header(&request);
        dbg!(&request);

        if header.failure {
            break;
        }


        let status_line = "HTTP/1.1 200 OK";

        //"Reading content to file"
        let contents = fs::read_to_string("templates/index.html").await.unwrap();
        let length = contents.len();


        let response =format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        //"Sending Contents"
        stream.write_all(response.as_bytes()).await.unwrap();
        //"Response sent !"
        
    }
}
