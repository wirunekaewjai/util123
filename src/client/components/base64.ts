import { addEventListener } from "@/functions/common/add-event-listener";
import { createComponent } from "@/functions/common/create-component";
import { getAttribute } from "@/functions/common/get-attribute";
import { querySelectorAll } from "@/functions/common/query-selector-all";
import { removeEventListener } from "@/functions/common/remove-event-listener";

export const Base64 = createComponent("base64", (element: HTMLElement) => {
  const fn = getAttribute(element, "x-function") as "atob" | "btoa";
  const textareas = querySelectorAll<HTMLTextAreaElement>(element, "textarea");

  const txt1 = textareas.item(0);
  const txt2 = textareas.item(1);

  const onInput = () => {
    txt2.value = window[fn](txt1.value);
  };

  onInput();
  addEventListener(txt1, "input", onInput);

  return () => {
    removeEventListener(txt1, "input", onInput);
  };
});