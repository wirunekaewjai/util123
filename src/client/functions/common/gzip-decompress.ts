export async function gzipDecompress(blob: Blob) {
  const decompressionStream = new DecompressionStream("gzip");
  const decompressedStream = blob.stream().pipeThrough(decompressionStream);

  const reader = decompressedStream.getReader();
  const chunks = [];

  while (true) {
    const { value, done } = await reader.read();

    if (done) {
      break;
    }

    chunks.push(value);
  }

  const buffer = await new Blob(chunks).arrayBuffer();
  return new Uint8Array(buffer);
}
