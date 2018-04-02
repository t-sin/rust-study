(((深夜1時からRust始めるチャレンジ)))
====

###### @t-sin, 2018-03-30, Rust勉強会#1 

----


# 君の名は： t-sin

- いまこれからRustに入門するニンゲン
- Common Lisp使い
- この世からLispが消え去ったときはNimかPythonを使いたい

----

# この発表では

- Rustの機能をさっと撫で、
- プログラムを書いてみて、
- その解説をしてみます。

----

# ここでのRust勉強の方針

- 先にマニュアル参照しつつコードを書く
- それに使ったものの詳細を調査する

理由： **時間がないから**

----

# では、はじまります。

----

# Rustとは

> Rustは速度、安全性、並行性の3つのゴールにフォーカスしたシステムプログラミング言語です。
> --- https://www.rust-lang.org/ja-JP/

具体的な特徴はというと…

----

# Rustの特徴

- ゼロコスト抽象化 
- ムーブセマンティクス
- 保証されたメモリ安全性
- データ競合のないスレッド
- トレイトによるジェネリクス
- パターンマッチング
- 型推論
- 最小限のランタイム
- 効率的なCバインディング

----

# Rustの特徴

- ゼロコスト抽象化
	- 抽象化の機能は全てコンパイル時に処理される
- ムーブセマンティクス
	- 値に対する束縛はひとつ
- 保証されたメモリ安全性
	- 所有権、借用、ライフタイムによる
- データ競合のないスレッド
- トレイトによるジェネリクス
	- Javaでいうインターフェース的な？
- パターンマッチング
- 型推論
- 最小限のランタイム
- 効率的なCバインディング

----


# インストール & Hello World

なにはともあれ…

```rust
$ curl https://sh.rustup.rs -sSf | sh
...
$ rustc --version
rustc 1.25.0 (84203cac6 2018-03-25)
$ mkdir hello-world && cd hello-world
$ cat <<EOS > main.rs
fn main() {
  println!("hello world");
}
EOS
$ rustc ./main.rs && ./main
hello world
```

導入はとっても簡単！

----

# ちょこっとコード書いてみる

----

# 文字列ストリームを実装してみる

- 標準入力から文字列を読み、
- 文字列と読み込み位置を格納して、
- そこから関数で文字を覗き見してみる。

----

# 文字列ストリームの仕様

- ストリームはタプル: `(もじれつ, ポインタ)`
- 読み込み関数はふたつ
	- ポインタの文字を読むけどポインタを更新しない`peek_char()`
	- ポインタの文字を読んでポインタを進める`read_char()`

----

# 文字列ストリームの実装

まずは`peek_char()`を実装

```rust
use std::io;

fn peek_char(s: (&str, usize)) -> Option<char> {
    match s.0.chars().nth(s.1) {
        None => None,
        Some(ch) => Some(ch)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);

    println!("{:?}", peek_char((&*input, 1)).unwrap());
    println!("{:?}", peek_char((&*input, 2)).unwrap());
}
```

----

# 実行結果

```sh
$ rustc stream.rs
warning: unused `std::result::Result` which must be used
  --> stream.rs:12:5
   |
12 |     io::stdin().read_line(&mut input);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default

$ ./stream
12345  # 入力
'2'
'3'
```

----

# コードの解説（１） - `peek_char()`の型

```rust
fn peek_char(s: (&str, usize)) -> Option<char> {
    ...
}
```

- 文字列ストリームは文字列(`&str`)と位置(`uzise`)の組で表現
- 文字列ストリームを受けとって、文字(`char`)を返す
	- `Option`は失敗するかしれないことを表す型
	- Haskellの`Maybe`みたいなやつ
- `&`は参照を表す
	- `&str`は文字列(`str`)への参照

----

# コードの解説（2） - `peek_char()`の処理

```rust
fn peek_char(s: (&str, usize)) -> Option<char> {
    match s.0.chars().nth(s.1) {
        None => None,
        Some(ch) => Some(ch)
    }
}
```

- タプル要素のアクセス (`s.0`)
- 文字列の文字へのアクセス (`str.chars().nth(idx)`)
- `match`はパターンマッチ
	- `nth()`が`None`のとき、(失敗なので)`None`を返す
	- 成功したときは、`Some()`に包んで`ch`を返す

----

# コードの解説(3) - `main()`関数

```rust
use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);

    println!("{:?}", peek_char((&*input, 1)).unwrap());
    println!("{:?}", peek_char((&*input, 2)).unwrap());
}
```

- `std::io`は標準ライブラリのクレート（Rustのパッケージ)
- ミュータブルな文字列として`input`を作成
- 文字列の出力
	- `println`はマクロなので`!`をつける
	- 書式文字列は`{}`で、この`{`と`}`の間に指定子を書く

----

# コードの解説(4) - `peek_char()`呼び出し

```rust
fn peek_char(s: (&str, usize)) -> Option<char> {...}
fn main() {
    ...
    println!("{:?}", peek_char((&*input, 1)).unwrap());
}
```

- `String`を`&str`に変換するため`&*`をつける
	- `&`のついた型は参照
	- `String`はヒープにつくられた`str`の参照の型なので…
	- まず`input`に`*`をつけて参照を解決する (`str`が得られる)
	- その参照をまた得るために`&`をつける (`&str`が得られる)
- 返り値は`Option`(`Maybe`みたいなやつ)なので`.unwrap()`で取り出す
	- 失敗を補足してないので、範囲外を参照すると落ちる😎

----

# `read_char()`の実装

`&mut`の前にあえなく敗北。だめだった…

----

# Rustをまず触ってみて得た学び

- ミュータブルにする際のつらみ
	- `&mut`地獄🔥
	- ライフタイムでエラー
- 中途半端な理解でコードを書くと火傷する
	- まずは"the book"(TRPL)を読め
	- *所有権*、*借用*、*ライフタイム*の章は3回読め
- 