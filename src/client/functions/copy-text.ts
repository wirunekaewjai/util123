let notyf: null | {
  success: (msg: string) => void;
  error: (msg: string) => void;
} = null;

export async function copyText(value: string) {
  if (!notyf) {
    notyf = new window.Notyf();
  }

  try {
    await navigator.clipboard.writeText(value);
    // console.log('Text copied to clipboard!');
    notyf?.success("Copied");
  } catch (err) {
    // console.error('Failed to copy: ', err);
    notyf?.error("Failed");
  }
}