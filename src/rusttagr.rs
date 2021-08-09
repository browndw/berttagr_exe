extern crate anyhow;

use std;
use crate::pos_tagging;
use crate::pos_tagging::POSModel;

fn try_tag(input: Vec<String>) -> anyhow::Result<std::vec::Vec<std::vec::Vec<pos_tagging::POSTag>>> {
  let mut formatted : String = "".to_owned();
  for single in input {
    formatted.push_str(&format!("{} ", single));
  }
  let format_vec = [formatted.as_str()]; 
  //    Set-up model
  let pos_model = POSModel::new(Default::default())?;
  //    Run model
  Ok(pos_model.predict(&format_vec))
} 

#[no_mangle]
pub fn rust_tag_r(input: Vec<String>) -> String {
  let output = match try_tag(input) {
    Ok(x) => x,
    Err(x) => panic!("{}", x)
  };

  let mut str_out : String = "".to_owned();
  for pos_tag in output {
    str_out.push_str(&format!("{:?}", pos_tag));
  }
  str_out
}
