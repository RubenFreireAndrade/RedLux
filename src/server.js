const express = require('express');
const http = require('http');
const mongoose = require('mongoose');
//import ts from 'ts-node'

const { config } = require('./app/utils/env_config.ts');

const router = express();

mongoose.connect(config.mongo.url)
.then(() => { 
  console.log("Connected to MongoDB!");
  startServer();
})
.catch(error => { console.log(error) });

const startServer = () => {
  router.use((req, res, next) => {
    res.on('finish', () => {
      console.log(`URL: ${req.url} - ${res.statusCode}`);
    })

    next();
  })
}