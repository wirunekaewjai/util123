// AUTO GENERATED
pub fn heading(icon: &str, content: &str) -> String {
    return format!(
        r#"<div class="grid grid-cols-[16px_1fr] gap-x-4 px-4 py-2 items-center">{}<h1 class="font-bold text-xl">{}</h1></div>"#,
        icon, content
    );
}

/*
(icon: string, content: string) => (
  <div class="grid grid-cols-[16px_1fr] gap-x-4 px-4 py-2 items-center">
    {icon}
    <h1 class="font-bold text-xl">
      {content}
    </h1>
  </div>
);
*/
