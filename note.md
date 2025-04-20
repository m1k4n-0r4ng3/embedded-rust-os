# NOTE

詰まった場所についてのメモ

# ボードをWSLに接続する

```bash
usbipd list # BUSIDを確認
usbipd bind --busid <busid> # ここだけ管理者権限が必要
usbipd attach --wsl --busid <busid> # wslに接続
usbpid detach --busid <busid> # 物理的に抜いてもOK
```

# ボードのメモリマップ

[データシート](https://www.st.com/resource/en/datasheet/stm32f401re.pdf)を参考に確認する。

今回は以下だった。

- SRAM(96KiB)は0x2000_0000-0x2001_7FFF
- Flash(256KiB)は0x0800_0000-0x0807_FFFF

# openocd.cfg

参考資料で使用するものをそのままは使えなかった。targetを自分のボードに合わせて変更する。

```
source [find target/stm32f4x.cfg]
```

# rust-toolchain

使用するRustのバージョンを`rust-toolchain.toml`で指定可能。

```
[toolchain]
channel = "stable"
channel = "nightly-2024-12-01"
components = ["rustfmt", "clippy"]
targets = ...
```

# ボード上での実行

`monitor reset init`を用いてボードをリセットしないと、最初から実行してくれない。

# semihosting

ビルドエラーは起きないが、`.rodata`と`.bss`セクションは設定する必要がある。

また、資料通りにコードを書くと、`static mut`に対して共有参照を取っているため、warningが出る。生のポインタとして扱うことで警告は出なくなる。

# llvm_asm

`llvm_asm!`はRust 1.59で廃止されたので、`asm!`を使うようにする。

# 名前付きラベル

[参考](https://doc.rust-lang.org/nightly/rust-by-example/unsafe/asm.html#labels)

インラインアセンブリでは名前付きラベルは異常な動作を引き起こす可能性があるため、使用できない。(詳細は参考ページ)

そのため、ラベルは番号で指定することにする。前方に飛ぶのか後方に飛ぶのかで`b`と`f`を使い分ける。

# volatile

[参考](https://users.rust-lang.org/t/volatile-option-in-new-asm-macro/44289/1)

`asm!`マクロでは`volatile`がデフォルトになっているので、指定する必要がない。最適化を行っていい場合は、`options(pure)`を付けることになる。

# repr

`[repr(C)]`はC言語のABIに沿ったメモリレイアウトを強制するもの。これによって、以下のことが保証される。

- フィールド順にメモリに配置される
- C互換のアライメント/パディングが行われる
- ABI互換で、Cからポインタを渡しても正しいフィールドにアクセスできる

# レジスタの退避

インラインアセンブリで変更される可能性があるレジスタを伝える時は、`out("r4") _`というようにする。
これによってコンパイラがアセンブリの実行前にレジスタに格納されていた値を保存する。

なお、`r6`と`r7`に関しては指定できない。これはLLVMが内部処理用に予約しているレジスタだから。

# SVCallのプロローグ/エピローグを抑制する

特権から非特権への遷移は成功するものの、非特権から特権への遷移が失敗してしまった。
原因を調査したところ、`SVCall()`の先頭でプロローグ処理が入っていることが原因だとわかった。

プロローグ処理を行わないようにするため、`naked`属性を付与した。

```
#![feature(naked_functions)]
...
use core::arch::naked_asm;
...
#[naked]
#[no_mangle]
pub unsafe extern "C" fn SVCall() {
    naked_asm!(
        "cmp lr, #0xfffffff9",
```
