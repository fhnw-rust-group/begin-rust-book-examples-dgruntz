# 01

Übungen zu Kapitel 1 (Hello World)

Das Projekt habe ich angelegt mit

    cargo new --bin begin-rust-01

und das generierte Verzeichnis habe ich dann in `01` umbenannt.
Das Projekt kann gebaut und ausgeführt werden mit dem Befehl

    cargo run

Das binary wird im Verzeichnis /target/debug abgelegt und hat eine Grösse von 154'624 Bytes.

Mit dem Befehl

    cargo build --release

kann ein Release gebaut werden. Ein Release-Build ist jedoch nicht viel kleiner: 151'040 Bytes.
