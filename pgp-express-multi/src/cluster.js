const cluster = require('cluster');

if (cluster.isPrimary) {
    console.log(`primary ${process.pid} is running`);

    const numCPUs = require('physical-cpu-count');
    // console.log({ numCPUs });

    // fork workers
    for (let i = 0; i < numCPUs; i++) {
        cluster.fork();
    }

    cluster.on('online', (worker) => {
        console.log(`worker ${worker.process.pid} is running`);
    });

    cluster.on('exit', (worker, code, signal) => {
        console.log(`worker ${worker.process.pid} exited with code ${code} on signal ${signal}`);
    });
} else {
    require('./main');
}
