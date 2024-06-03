export async function copyText(value: string) {
  const { nofity } = await import("./notify");

  try {
    await navigator.clipboard.writeText(value);
    // console.log('Text copied to clipboard!');
    await nofity("success", "Copied");
  } catch (err) {
    // console.error('Failed to copy: ', err);
    await nofity("error", "Failed");
  }
}