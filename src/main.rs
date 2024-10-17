use my_project::list_parser;
  
pub fn main() {
      print!("OK!");
      assert_eq!(list_parser::list("[1,1,2,3,5,8]"), Ok(vec![1, 1, 2, 3, 5, 8]));
      print!("{:?}", list_parser::list("[1,1,2,3,5,8]"));
    }