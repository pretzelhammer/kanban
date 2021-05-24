bash run_bench.sh ../diesel-rocket 60 40 reads
sleep 5
bash run_bench.sh ../diesel-rocket 60 40 mixed
sleep 5

bash run_bench.sh ../sqlx-actix-web 60 40 reads
sleep 5
bash run_bench.sh ../sqlx-actix-web 60 40 mixed
sleep 5

bash run_bench.sh ../pgp-express 60 40 reads
sleep 5
bash run_bench.sh ../pgp-express 60 40 mixed
sleep 5

bash run_bench.sh ../pgp-express-multi 60 40 reads
sleep 5
bash run_bench.sh ../pgp-express-multi 60 40 mixed
sleep 5

