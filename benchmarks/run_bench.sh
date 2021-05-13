# exit script on any errors
set -e

# node -v && exit 3

echo_usage() {
    echo "usage: bash run_bench.sh <project directory> <duration> <workers> <targets>"
}

int_re=^[1-9][0-9]*$

# check if first arg is present and is a directory
[[ -d $1 ]] || (echo "arg 1: $1 is not a project directory" && echo_usage && exit 1)
project_path=$1
project_name=$(basename $project_path)

# echo $project_path $project_name && exit 1

# check if second arg is present and is a positive integer
[[ $2 =~ $int_re ]] || (echo "arg 2: $2 is not a positive integer" && echo_usage && exit 2)
duration=$2

# check if third arg is present and is a positive integer
[[ $3 =~ $int_re ]] || (echo "arg 3: $3 is not a positive integer" && echo_usage && exit 3)
workers=$3

# check if fourth arg is present and is a targets file in current working directory
[[ -f $4.txt ]] || (echo "arg 4: $4 is not a valid targets file" && echo_usage && exit 4)
targets=$4
targets_file=$4.txt

echo "project name: ${project_name} - duration: ${duration} - workers: ${workers} - targets: ${targets}"

# change to project path
cd $project_path

if [[ -f Cargo.toml ]]
then
    # launch db & run migrations
    just start-dev
    # build with optimizations
    just build-native
    # run in bg
    target/release/kanban &
else
    # set node version
    # nvm use # doesn't work in bash script for whatever reason, set manually beforehand
    # launch db & run migrations
    npm run start-dev
    # run node in background
    node src/cluster.js &
fi

# get pid of last process started in background
server_pid=$!

echo "server running with pid $server_pid"

sleep 5

# switch back to benchmarks dir
cd ../benchmarks

project_csv="${project_name}-${duration}s-${workers}w-${targets}.csv"
project_bin="${project_name}-${duration}s-${workers}w-${targets}.bin"
project_report="${project_name}-${duration}s-${workers}w-${targets}.report"

python3 monitor_process.py $duration $server_pid > data/$project_csv &
# warm up
cat $targets_file | vegeta attack -http2 -duration=10s -rate=0 -max-workers=${workers} > /dev/null
# the real loadtest
cat $targets_file | vegeta attack -http2 -duration=${duration}s -rate=0 -max-workers=${workers} > data/$project_bin
cat data/$project_bin | vegeta report > data/$project_report
rm data/$project_bin

sleep 1

echo "about to kill $server_pid"

kill $server_pid

# switch back to project
cd $project_path

if [[ -f Cargo.toml ]]
then
    # shutdown db
    just stop-dev
else
    # shutdown db
    npm run stop-dev
fi

# switch back to benchmarks
cd ../benchmarks
echo "LATEST CPU & MEMORY INFO"
cat data/$project_csv
echo "LATEST THROUGHPUT INFO"
cat data/$project_report
echo "BEST THROUGHPUT INFO"
cat best/$project_report
