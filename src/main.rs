use client::HttpClient;

mod client;


fn main() {
    let client = HttpClient::new("http://jsonplaceholder.typicode.com");
    client.get("/posts/1").unwrap();

    

    println!("{:?}",client);
}  
