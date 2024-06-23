import { querySelector } from "@/functions/common/query-selector";
import { removeAttribute } from "@/functions/common/remove-attribute";
import { setAttribute } from "@/functions/common/set-attribute";

const HEAD_TAGS = new Set([
  "META",
  "TITLE",
]);

export function morphPage(html: string) {
  const root = document.createElement("html");

  root.innerHTML = html;

  const head = querySelector(root, "head");
  const body = querySelector(root, "body");

  if (head) {
    const incomings: Element[] = [];
    const incomingHTMLs: string[] = [];

    const currentHTMLs: string[] = [];

    const removes: Element[] = [];

    for (let i = 0; i < head.children.length; i++) {
      const element = head.children.item(i)!;

      if (!HEAD_TAGS.has(element.tagName)) {
        continue;
      }

      incomings.push(element);
      incomingHTMLs.push(element.outerHTML);
    }

    for (let i = 0; i < document.head.children.length; i++) {
      const element = document.head.children.item(i)!;
      const elementHTML = element.outerHTML;

      if (!HEAD_TAGS.has(element.tagName)) {
        continue;
      }

      if (incomingHTMLs.includes(elementHTML)) {
        currentHTMLs.push(elementHTML);

        // still alive !!!
        continue;
      }

      removes.push(element);
    }

    for (const element of removes) {
      element.remove();
    }

    for (const element of incomings) {
      if (!HEAD_TAGS.has(element.tagName)) {
        continue;
      }

      if (currentHTMLs.includes(element.outerHTML)) {
        // this element is exists !!!
        continue;
      }

      document.head.append(element);
    }
  }

  if (body) {
    const removes: string[] = [];

    for (let i = 0; i < document.body.attributes.length; i++) {
      const attr = document.body.attributes.item(i)!;
      const newer = body.attributes.getNamedItem(attr.name);

      if (newer) {
        setAttribute(document.body, attr.name, newer.value);
      } else {
        removes.push(attr.name);
      }
    }

    for (const remove of removes) {
      removeAttribute(document.body, remove);
    }

    for (let i = 0; i < body.attributes.length; i++) {
      const attr = body.attributes.item(i)!;
      setAttribute(document.body, attr.name, attr.value);
    }

    document.body.innerHTML = body.innerHTML;
  }
}