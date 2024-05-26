(data_url: string) => (
  <div
    class="w-40 h-40 border rounded-sm flex"
    id="qrcode"
  >
    <img
      alt="qrcode"
      class="w-full h-full"
      src={data_url}
    />
  </div>
);