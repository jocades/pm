async function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

let i = 0
while (true) {
  console.log(`[${new Date().toISOString()}] - ${i}`)
  await sleep(5000)
  i++
}
