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
    let j_str = fs::read_to_string("map.json").expect("error loading json");
    let page_obj = serde_json::from_str::<Pages>(&j_str).expect("error structuring json ");
    let mut template_str : String = "".to_string();
    for e in 0..page_obj.pages.len()
    {
        let page_details = &page_obj.pages[e];
        let temp = page_details.template;
        if temp== true
        {
            let temp_path = &page_obj.pages[e].content;
            template_str = fs::read_to_string(temp_path).expect("error loading html template");
            break;
        }

    }
    println!("{template_str:?}");
}