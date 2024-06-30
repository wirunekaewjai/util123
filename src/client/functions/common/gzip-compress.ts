export async function gzipCompress(buffer: Uint8Array) {
  const input = new ReadableStream({
    start(controller) {
      controller.enqueue(buffer);
      controller.close();
    },
  });

  const compressedStream = input.pipeThrough(new CompressionStream("gzip"));
  const reader = compressedStream.getReader();

  const chunks = [];

  while (true) {
    const { value, done } = await reader.read();

    if (done) {
      break;
    }

    chunks.push(value);
  }

  return new Blob(chunks, { type: "application/gzip" });
}
