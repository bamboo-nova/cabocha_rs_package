use cabocha::parser::Parser;

fn main() {
  let mut parser = Parser::new("");
  let sentence = "スパイスの粉感もあまり感じられないので飲みやすいチャイだと思う。";

  let tree = parser.parse_to_tree(sentence);

  println!("{}", tree.to_string(cabocha::consts::CABOCHA_FORMAT::LATTICE));
  let texts = tree.to_string(cabocha::consts::CABOCHA_FORMAT::LATTICE);
  let text_array: Vec<&str> = texts.split('\n').collect();

  let mut depends: Vec<Vec<String>> = vec![];

  for text in text_array {
    let feature_vec: Vec<&str> = text.split(' ').collect();
    if text {
        let dep_1: &str = feature_vec[1];
        let dep_2: String = feature_vec[2].replace("D", "");
        let depend: Vec<String> = vec![dep_1.to_string(), dep_2];
        depends.push(depend);
    } else {
        let feature_vec: Vec<&str> = text.split('\t').collect();
        if feature_vec.len() > 1 {
            let parts: Vec<&str> = feature_vec[1].split(',').collect();
            if parts.len() > 1 && parts[0] == "名詞" && parts[1] == "一般" {
                println!("{}", feature_vec[0]);
            }
        }
    }
  }
  for depend in &depends {
    println!("{:?}", depend);
  }
}
