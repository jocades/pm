// const err: string = 1

const server = Bun.serve({
  development: false,
  port: 8000,
  fetch(req) {
    const url = new URL(req.url)
    console.log('->', url.pathname)

    if (url.pathname === '/error') {
      throw new Error('An error occurred')
    }

    return new Response('Hello World!')
  },
})

console.log(`Server running at ${server.url}`)
