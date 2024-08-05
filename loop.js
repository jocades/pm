async function sleep(s) {
  return new Promise((resolve) => setTimeout(resolve, s * 1000))
}

let i = 0
while (true) {
  console.log(`[${new Date().toISOString()}] - ${i}`)
  await sleep(1)
  i++
}
