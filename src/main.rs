use std::fmt::format;

use client::HttpClient;
use clap::Parser;


mod client;
mod status;
mod Errors;



#[derive(Parser,Debug)]
#[command(name="nt-client")]
struct Cli{
    #[arg(long)]
    host:String,
    
    #[arg(long)]
    port:u16,

    #[arg(long,default_value_t=false)]
    https:bool,

    #[arg(long)]
    path:String,

    #[arg(long, default_value = "GET")]
    method:String,

    #[arg(long, default_value="")]
    body:String,
}

fn main() {

   let args = Cli::parse();

   let host_url:String = if args.https {
       format!("https://{}",args.host)
   }else{
       format!("http://{}",args.host)
   };
        
    let client = HttpClient::new(&host_url);
    let _ = match  args.method.to_lowercase().as_str() {
        "get"=>client.get(args.path.as_str()),
        _=>{
            eprintln!("Unsupported Http Methode :{}",args.method);
            return;
        }
    };
}  
