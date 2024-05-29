use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::os::unix::net::UnixStream;
use std::io::BufReader;
use std::io::Write;
use std::io::BufRead;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub jsonrpc: String,
    pub method: String,
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub params: Option<HashMap<String, Value>>,
}

fn main() -> std::io::Result<()> {
    let s = Request {
        jsonrpc: "2.0".to_string(),
        method: "rpc_get_methods".to_string(),
        id: 1,
        params: None,
    };

//    let mut reqs:Vec<Request> = Vec::new();
//    reqs.push(s);


//    let req = format!("Request: {}", serde_json::to_string(&reqs).unwrap());
//    println!("Request: {}", req);

//    let req = r###"{
//  "jsonrpc": "2.0",
//  "method": "rpc_get_methods",
//  "id": 1
//} 
//"###;

    let mut stream = UnixStream::connect("/var/tmp/spdk.sock")?;
    //stream.write_all(req.as_bytes())?;
    stream.write_all(serde_json::to_string(&s).unwrap().as_bytes())?;
    let stream = BufReader::new(stream);
    for line in stream.lines() {
        println!("{}", line.unwrap());
        break;
    }

    Ok(())
}
