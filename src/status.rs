use std::{arch::x86_64::_mm_sll_epi16, os::linux::raw};

use crate::Errors::HttpResponseErrors;
use colored::*;


#[derive(Debug)]
pub enum StatusCodes{
    Ok,
    Created,
    NoContent,
    MovedPermanently,
    Found,
    BadRequest,
    Unauthorized,
    Forbiden,
    NotFound,
    InternalServerError,
    BadGetway,
    ServiceUnavailabe,
    Informational(u16),
    Redirection(u16),
    ClientError(u16),
    ServerError(u16),
    Unknown(u16),
}


impl StatusCodes{
    pub fn from_u16(code:u16)->Self{
        match code{
        200=>StatusCodes::Ok,
        201=>StatusCodes::Created,
        204=>StatusCodes::NoContent,
        301=>StatusCodes::MovedPermanently,
        302=>StatusCodes::Found,
        400=>StatusCodes::BadRequest,
        401=>StatusCodes::Unauthorized,
        403=>StatusCodes::Forbiden,
        404=>StatusCodes::NotFound,
        500=>StatusCodes::InternalServerError,
        502=>StatusCodes::BadGetway,
        503=>StatusCodes::ServiceUnavailabe,
        100..=199=>StatusCodes::Informational(code),
        300..=399=>StatusCodes::Redirection(code),
        400..=499=>StatusCodes::ClientError(code),
        500..=599=>StatusCodes::ServerError(code),
        _=>StatusCodes::Unknown(code)
        }
    }


    pub fn description(&self)->ColoredString{
        match self{
            StatusCodes::Ok => "OK - The request has succeeded".green(),
            StatusCodes::Created => "Created - The resource was created".green(),
            StatusCodes::NoContent => "No Content - Nothing to return".yellow(),
            StatusCodes::MovedPermanently => "Moved Permanently".yellow(),
            StatusCodes::Found => "Found (Temporary Redirect)".yellow(),
            StatusCodes::BadRequest => "Bad Request".red(),
            StatusCodes::Unauthorized => "Unauthorized".red(),
            StatusCodes::Forbiden => "Forbidden".red(),
            StatusCodes::NotFound => "Not Found".red(),
            StatusCodes::InternalServerError => "Internal Server Error".red(),
            StatusCodes::BadGetway => "Bad Gateway".yellow(),
            StatusCodes::ServiceUnavailabe => "Service Unavailable".yellow(),
            StatusCodes::Informational(_) => "Informational response".yellow(),
            StatusCodes::Redirection(_) => "Redirection".yellow(),
            StatusCodes::ClientError(_) => "Client Error".red(),
            StatusCodes::ServerError(_) => "Server Error".red(),
            StatusCodes::Unknown(_) => "Unknown Status Code".yellow(),
        }
    }
}


#[derive(Debug)]
pub struct HttpResponse{
    pub code:StatusCodes,
    pub raw:String
}

impl HttpResponse{
    pub fn from_raw(raw:String)->Option<Self>{
        let line= raw.lines().next()?;
        let code = line.split_whitespace().nth(1)?.parse::<u16>().unwrap();
        Some(Self{
            code:StatusCodes::from_u16(code),
            raw

        })

    }

    pub fn print_limited(self){
        print!("Response : ");
        for line in self.raw.lines().take(4){
            println!("{}",line);
        }

        let (header,body) = self.raw.split_once("\r\n\r\n").unwrap();
        println!("{}",body);
    }

    pub fn print_Summary(self){
        println!("Response Status : {}",self.code.description());

        self.print_limited();
        
    }



}




