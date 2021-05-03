## benchmarks

reqs

```bash
python3 -m install psutil
nvm use 16.0.0
cargo install just
rustup override set nightly
```

commands

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