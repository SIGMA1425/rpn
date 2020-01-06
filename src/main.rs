fn main() {
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";
    let ans = rpn(exp);

    //デバッグビルド時のみ答えが正しいかチェック
    debug_assert_eq!("26.2840", format!("{:.4}", ans));
    //formatでansを小数点以下4桁までぼ値を文字列に変換

    println!("{} = {:.4}", exp, ans);
}

fn rpn(exp: &str) -> f64{
    //変数stackを空のスタックに束縛
    let mut stack = Vec::new();

    //expの要素をスペースで分割し、tokenをそれらに順に束縛
    //要素がなくなるまで繰り返す
    for token in exp.split_whitespace(){
        if let Ok(num) = token.parse::<f64>(){
            stack.push(num);
        }
        else{
            //tokenが数値でないのなら演算子なのか調べる
            match token{
                //tokenが演算子ならapply2関数で計算
                //|x, y| x+yはクロージャ
                //引数x, yをとり、x+yを計算して答えを返す
                "+" => apply2(&mut stack, |x, y| x + y),
                "-" => apply2(&mut stack, |x, y| x - y),
                "*" => apply2(&mut stack, |x, y| x * y),
                "/" => apply2(&mut stack, |x, y| x / y),

                //tokenが演算子でなければエラーを起こして終了する
                _ => panic!("Unknown operator: {}", token)
            }
        }
    }
    stack.pop().expect("Stack underflow")
}

fn apply2<F>(stack: &mut Vec<f64>, fun: F)
// F型のトレイト境界
where
    F: Fn(f64, f64) -> f64,
{
    //変数yとxをスタックの最後の2要素に束縛する
    if let (Some(y), Some(x)) = (stack.pop(), stack.pop()){
        //クロージャfunで計算し、その結果に変数zを束縛する
        let z = fun(x, y);
        //変数zの値をzに積む
        stack.push(z);
    }
    else{
        //スタックから要素が取り出せなかったときはエラーを起こして終了する
        panic!("Stack underflow");
    }
}
