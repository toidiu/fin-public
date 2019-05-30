var config = {};


// config.default_stuff =  ['red','green','blue','apple','yellow','orange','politics'];
// config.twitter.user_name = process.env.TWITTER_USER || 'username';
// config.twitter.password=  process.env.TWITTER_PASSWORD || 'password';
//
// config.server.port = 'http://localhost:8000';

config.api = {
    host: process.env.HOST || 'localhost',
    port: process.env.PORT || 4000,
    timeoutMs: 20000
};
// config.redis.host = 'hostname';
// config.redis.port = 6379;
// config.web.port = process.env.WEB_PORT || 9980;

module.exports = config;
