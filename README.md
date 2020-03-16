# Parallax Background with SDL and Rust

This is a small excerpt from the source code of the game I'm developing in Rust, turned into a feature demo.
widgets.rs is meant to contain other widget data structures and traits, but here you only get to see one; font use was removed as well.

Video
-----

If you just want to see the application running, there's a small sample video in the base directory.

Building
--------

To build this software either on Linux or Windows, setup your Rust environment according to these instructions:
https://www.rust-lang.org/tools/install

Additionally, if you're using Linux, you have to install SDL2 and SDL2-image development versions in your distribution.

On Ubuntu:
```
sudo apt-get install libsdl2-dev libsdl2-image-dev
```

On Fedora (as root):
```
dnf install SDL2-devel SDL2_image-devel
```

When you're done with that, clone this repository and use `cargo build` on the base directory to build the application.
The compiler will inform you if you're missing dependencies.

I have only tested this on 64-bit systems, but the files are there to allow it to build on 32-bit systems as well.

Testing
-------

Just run `cargo test` on the base directory to run all the unit tests.

Running the application
-----------------------

Use `cargo run` on the base directory and follow the instructions at the header of the window
to see a horrifyingly ugly but very efficient parallax background drawn into the screen canvas.

I haven't built this on a Mac so yeah, there's that. Have a Knights of Sidonia opening instead:

Knights of Sidonia Opening 1
----------------------------

誰(た)がために　我は征く
誰(た)がために　散り征くなら
何故に

打ち砕け　時　満ちて　生きるため　解き放て

宇宙(そら)の航路は風に消え　安住は幾億光年先
突き進むなら惑うな　いざ征かん　騎士よ

打ち砕け　KNIGHTS OF SIDONIA　時　満ちて　KNIGHTS OF SIDONIA
生きるため　KNIGHTS OF SIDONIA　解き放て　活路はこの手に

誰(た)がために　我は征く
誰(た)がために　散り征くなら

誓い立てる間もなく　この身を投げ出せ
重責と困憊と　運命(さだめ)には負けじと

終焉は何処(いずこ)と憂えるな　天上　そこはただ果ての果て
過ぎし幾多の交戦に　散った友を胸に

打ち砕け　KNIGHTS OF SIDONIA　時　満ちて　KNIGHTS OF SIDONIA
生きるため　KNIGHTS OF SIDONIA　解き放て　活路はこの手に

誰(た)がために　我は征く
誰(た)がために　散り征くなら

誓い立てる間もなく　この身を投げ出せ
重責と困憊と　運命(さだめ)には負けじと

破壊の向こう側で　星が白く笑む
されど打開の道　遠く　遠く　遠く

打ち砕け　KNIGHTS OF SIDONIA　時　満ちて　KNIGHTS OF SIDONIA
生きるため　KNIGHTS OF SIDONIA　解き放て　活路はこの手に

打ち砕け　KNIGHTS OF SIDONIA　時　満ちて　KNIGHTS OF SIDONIA
生きるため　KNIGHTS OF SIDONIA　解き放て　活路はこの手に
