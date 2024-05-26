// AUTO GENERATED
use tiny_tsx::tsx_join;

pub fn doc(title: &str, children: Vec<String>) -> String {
    return format!(
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><link rel="icon" sizes="any" type="image/x-icon" href="/favicon.ico"><link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/notyf@3/notyf.min.css"><link rel="stylesheet" href="/assets/style.css"><title>{}</title><script defer src="https://unpkg.com/htmx.org@1.9.12/dist/htmx.min.js"></script><script defer type="module" src="/assets/app.js"></script></head><body>{}<script src="https://cdn.jsdelivr.net/npm/notyf@3/notyf.min.js"></script></body></html>"#,
        title,
        tsx_join(&children)
    );
}

/*
(title: string, children: string[]) => (
  <>
    {"<!DOCTYPE html>"}
    <html lang="en">
      <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <link rel="icon" sizes="any" type="image/x-icon" href="/favicon.ico" />
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/notyf@3/notyf.min.css" />
        <link rel="stylesheet" href="/assets/style.css" />

        <title>{title}</title>

        <script defer src="https://unpkg.com/htmx.org@1.9.12/dist/htmx.min.js"></script>
        <script defer type="module" src="/assets/app.js"></script>
      </head>
      <body>
        {join(children)}
        <script src="https://cdn.jsdelivr.net/npm/notyf@3/notyf.min.js"></script>
      </body>
    </html>
  </>
);
*/
