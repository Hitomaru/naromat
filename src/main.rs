mod entities;
use crate::entities::chapter::Chapter;

fn main() {
    let text = r"
    こんにちは。
    私は[ボブ:ぼぶ]です。
    「こういう[こと:.]もあるし」
    あるいは「こんな」場合？もあるでしょう！";


    let chap = Chapter::new(text);
    chap.print();
}
