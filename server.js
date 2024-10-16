// Require the framework and instantiate it
const fastify = require('fastify')({ logger: true })

// Declare a route
fastify.get('/', function handler (request, reply) {
  reply
        .code(200)
        .send('hello client!');
});

// Run the server!
fastify.listen({ port: 80, host: '0.0.0.0' }, (err, address) => {
  if (err) {
    fastify.log.error(err);
    process.exit(1);
  }
});