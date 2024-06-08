use dotenv::var;
use html_to_string_macro::html;
use serde_json::Value;

use crate::functions::get_asset_path;

pub fn doc(asset_map: &Value, title: &str, children: Vec<String>) -> String {
    let fa_kit_id = var("FONT_AWESOME_KIT_ID").expect("FONT_AWESOME_KIT_ID must be set");

    return html!(
        <>
            {"<!DOCTYPE html>"}
            <html lang="en">
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />

                    <link rel="icon" sizes="any" type="image/x-icon" href="/favicon.ico" />
                    <link rel="stylesheet" href={get_asset_path(asset_map, "/assets/style.css")} />

                    <title>{title}</title>

                    <script src="https://unpkg.com/htmx.org@1.9.12" integrity="sha384-ujb1lZYygJmzgSwoxRggbCHcjc0rB2XoQrxeTUQyRjrOnlCoYta87iKBWq3EsdM2" crossorigin="anonymous"></script>

                    <script src={get_asset_path(asset_map, "/assets/app.js")} type="module"></script>
                    <script src={format!("https://kit.fontawesome.com/{fa_kit_id}.js")} crossorigin="anonymous"></script>
                </head>
                <body>
                    {children.join("")}
                </body>
            </html>
        </>
    );
}
