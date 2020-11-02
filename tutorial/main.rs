/*
    Rustのチュートリアル
    参考にしたサイト : 
        http://www.tohoho-web.com/ex/rust.html
*/

fn main() {
    // 変数
    let x = 100;     // <- イミュータブル(変更できない)
    let mut y = 200; // <- ミュータブル(変更できる)

    const PI: f32 = 3.14; // 定数

    // 型変換
    let x: i32 = 123;
    let y: i64 = x as i64;
}
