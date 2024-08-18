function log(msg) {
  console.log(`[${new Date().toISOString()}] ${msg}`);
}

const server = Bun.serve({
  development: false,
  port: 8000,
  fetch(req) {
    const url = new URL(req.url);
    log(`${req.method} ${url.pathname}`);

    if (url.pathname === "/error") {
      throw new Error("An error occurred");
    }

    return new Response("Hello World!");
  },
});

log(`Server running at ${server.url}`);
