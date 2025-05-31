use std::io::{Read, Write};

use crate::status::{self, HttpResponse};

#[derive(Debug)]
pub struct HttpClient{
    host:String,
    port:u16,
    use_https:bool
}


impl HttpClient {
   pub fn new(url:&str)->Self{
        let result = url::Url::parse(url);

       let parsed_url=  match result{ 
            Ok(val)=>val,
            Err(e)=>{
                println!("Invalid Url Formar");
                url::Url::parse("https://default.comnput").unwrap()
            }
        };

       Self{
            host:parsed_url.host_str().unwrap().to_string(),
            port:parsed_url.port_or_known_default().unwrap(),
            use_https:parsed_url.scheme() == "https",
       }
   }


    pub fn get(&self,path:&str)->std::io::Result<()>{
            let addr = format!("{}:{}",self.host,self.port);
            let mut stream =std::net::TcpStream::connect(addr)?;
                
            
            let request = format!("GET {} HTTP/1.1\r\nHost:{}\r\nConnection:close\r\n\r\n",path,self.host);
            stream.write_all(request.as_bytes())?;
            stream.flush()?;
            let mut response = String::new();
            let _ = stream.read_to_string(&mut response)?;
        
            if let Some(http_response) = HttpResponse::from_raw(response){
                http_response.print_Summary();
            }else{
                print!("Error occured while parsing the response");
            }

            Ok(())

    }
        
         
}


