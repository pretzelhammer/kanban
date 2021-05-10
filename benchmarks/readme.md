## Benchmarks

This is the source code for the benchmarks run in the article [Kanban Backend in Rust](#) on my [Rust blog](https://github.com/pretzelhammer/rust-blog).

Prerequisites:

Have docker installed, and also:

```bash
brew update && brew install vegeta
python3 -m install psutil
nvm use 16.0.0 # only necessary to run the node benchmarks
cargo install just
rustup override set nightly
```

Example commands:

```bash
bash run_bench.sh ../diesel-rocket 60 40 reads
bash run_bench.sh ../diesel-rocket 60 40 mixed

bash run_bench.sh ../sqlx-actix-web 60 40 reads
bash run_bench.sh ../sqlx-actix-web 60 40 mixed

bash run_bench.sh ../pgp-express 60 40 reads
bash run_bench.sh ../pgp-express 60 40 mixed

bash run_bench.sh ../pgp-express-multi 60 40 reads
bash run_bench.sh ../pgp-express-multi 60 40 mixed
```
