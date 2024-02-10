use std::fs;
// handling json files
use serde::{Deserialize , Serialize};
#[derive(Serialize, Deserialize,Debug)]
struct Pages
{
    pages : Vec<PageDetails>,

}

#[derive(Serialize, Deserialize,Debug)]
struct PageDetails 
{
    title : String ,
    link : bool,
    path : String ,
    content: String ,
    template :bool,
    index :i32
}
fn main(){
    let jStr = fs::read_to_string("map.json").expect("error loading json");
    println!("{:?}",jStr);
    let pageObj = serde_json::from_str::<Pages>(&jStr).expect("error structuring json ");
}