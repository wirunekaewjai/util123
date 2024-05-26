// AUTO GENERATED
pub fn qrcode(data_url: &str) -> String {
    return format!(
        r#"<div class="w-40 h-40 border rounded-sm flex" id="qrcode"><img alt="qrcode" class="w-full h-full" src="{}"></div>"#,
        data_url
    );
}

/*
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
*/
