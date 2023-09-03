import http from 'http';
import express from 'express';
import mongoose from 'mongoose';

import { envConfig } from './config/env_config';

const router = express();

mongoose.connect(envConfig.mongo.url)
.then(() => {
    console.log("Connected to MongoDB");
}).catch(error => {
    console.log(error);
});