export async function base64ToBlob(base64: string) {
  const res = await fetch(base64);
  const blob = await res.blob();
  return blob;
}