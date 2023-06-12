mod cabochalib;
use cabochalib::morph::Morph;

use cabocha::parser::Parser;

fn main() {
  let mut parser = Parser::new("");
  let sentence = "スパイスの粉感もあまり感じられないため、非常に飲みやすいチャイだと思う。";

  let tree = parser.parse_to_tree(sentence);

  // println!("{}", tree.to_string(cabocha::consts::CABOCHA_FORMAT::LATTICE));
  let texts = tree.to_string(cabocha::consts::CABOCHA_FORMAT::LATTICE);
  let text_array: Vec<&str> = texts.split('\n').collect();

  let tokenizer = match Morph::parse(text_array) {
    Ok(guard) => guard,
    Err(err) => {
        eprintln!("{}", err);
        // 例外的な値やデフォルト値などを設定する場合は、ここでそれを返す
        return; // または panic!() を使用してプログラムを終了させる
    },
  };
  println!("{:?}", tokenizer);
}