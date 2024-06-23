import { getAttribute } from "@/functions/common/get-attribute";
import { querySelector } from "@/functions/common/query-selector";
import { removeAttribute } from "@/functions/common/remove-attribute";
import { setAttribute } from "@/functions/common/set-attribute";

export function morphHTML(target: HTMLElement, next: string) {
  const html = document.createElement("html");

  html.innerHTML = next;

  const head = querySelector(html, "head");
  const body = querySelector(html, "body");

  if (head) {
    for (let i = 0; i < head.children.length; i++) {
      const incoming = head.children.item(i);

      if (incoming?.tagName === "TITLE") {
        document.title = incoming.textContent ?? "";
      }

      else if (incoming?.tagName === "META") {
        const name = getAttribute(incoming, "name");
        const content = getAttribute(incoming, "content");

        if (name) {
          const meta = querySelector(document.head, `meta[name='${name}']`);

          if (meta && getAttribute(meta, "content") !== content) {
            if (content) {
              setAttribute(meta, "content", content);
            } else {
              removeAttribute(meta, "content");
            }
          }
        }
      }
    }
  }

  if (body) {
    target.innerHTML = body.innerHTML;
  }
}