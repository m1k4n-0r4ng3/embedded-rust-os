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

# ボード上での実行

`monitor reset init`を用いてボードをリセットしないと、最初から実行してくれない。

# semihosting

ビルドエラーは起きないが、`.rodata`と`.bss`セクションは設定する必要がある。

また、資料通りにコードを書くと、`static mut`に対して共有参照を取っているため、warningが出る。生のポインタとして扱うことで警告は出なくなる。