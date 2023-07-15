use std::fs;
//TODO NO COMPLETE
pub  fn create_file_json(){

    let _path="../../deploy-smart-constract/build/constract/";
    let paths = fs::read_dir(_path).unwrap();
    

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}
