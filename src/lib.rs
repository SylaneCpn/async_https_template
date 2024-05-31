pub mod header;


use header::*;

use async_std::prelude::*;
use async_std::fs;

use async_std::net::TcpStream;
use async_native_tls::TlsStream;

use async_std::io::BufReader;





pub async fn handle_client(mut stream: TlsStream<TcpStream>) {

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
         }//end header extract loop


        let mut header = Header::new();


        header.process_header(&request);
        //println!("header : {} ",&header).await;

        let mut body : Vec<u8> = Vec::new();

        if header.length != 0 {

            body = vec![0;header.length];
            buf_reader.read_exact(&mut body[..]).await.unwrap();
            //println!("body length : {}",body.len()).await;
            //println!("body contents : {:?}",&body).await;
            //println!("body contents as str : {}", String::from_utf8_lossy(&body)).await;

        }

        //println!("request : {:?} ",&request);



        let status_line = "HTTP/1.1 200 OK";

        //"Reading content to file"
        if &header.request_uri == "/" {


                let contents = fs::read_to_string("templates/index.html").await.unwrap();
                let length = contents.len();


                let response =format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

                //"Sending Contents"
                stream.write_all(response.as_bytes()).await.unwrap();
                //"Response sent !"
        }

        else {
                let contents = fs::read_to_string("templates/test.html").await.unwrap();
                let length = contents.len();


                let response =format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

                //"Sending Contents"
                stream.write_all(response.as_bytes()).await.unwrap();
                //"Response sent !"
        }

}
