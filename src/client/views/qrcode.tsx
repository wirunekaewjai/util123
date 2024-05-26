export function qrcode(dataUrl: string) {
  return (
    <img
      alt="qrcode"
      class="w-full h-full"
      src={dataUrl}
    />
  );
}
