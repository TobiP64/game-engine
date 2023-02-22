RESULTS=
rustup --version
rustc --version
cargo --version
cargo $1 --verbose --color=always --all-features --package $2  $3
# --message-format=json | rust-ci > $JB_SPACE_FILE_SHARE_PATH/results/$JB_SPACE_EXECUTION_NUMBER/$1