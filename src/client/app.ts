window.createQRCode = async (input: string) => {
  try {
    const QRCode = await import("qrcode");
    const output = await QRCode.toDataURL(input);

    return output;
  } catch {
    return "";
  }
};

window.createSHA = async (type: 1 | 256 | 512, input: string) => {
  if (!input) {
    return "";
  }

  const { sha } = await import("@/functions/sha");
  return await sha(type, input);
};

window.copyText = async (value: string) => {
  await navigator.clipboard.writeText(value);

  const { nofity } = await import("@/functions/notify");

  try {
    // console.log('Text copied to clipboard!');
    await nofity("success", "Copied");
  } catch (err) {
    // console.error('Failed to copy: ', err);
    await nofity("error", "Failed");
  }
};
