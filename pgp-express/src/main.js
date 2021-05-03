require('dotenv').config();

const express = require('express');
const app = express();
const port = 8000;
const apiRoutes = require('./routes');

app.get('/', (_req, res) => {
    res.send('Hello, world!');
});

app.use('/api', apiRoutes);

app.listen(port, () => {
    console.log(`server listening at http://localhost:${port}`)
});