import { addEventListener } from "@/functions/common/add-event-listener";
import { createComponent } from "@/functions/common/create-component";
import { getAttribute } from "@/functions/common/get-attribute";
import { querySelectorAll } from "@/functions/common/query-selector-all";
import { removeEventListener } from "@/functions/common/remove-event-listener";

export const Sha = createComponent("sha", (element: HTMLElement) => {
  const type = Number(getAttribute(element, "x-type")) as 1 | 256 | 512;
  const textareas = querySelectorAll<HTMLTextAreaElement>(element, "textarea");

  const txt1 = textareas.item(0);
  const txt2 = textareas.item(1);

  const createSHA = async (input: string) => {
    if (!input) {
      return "";
    }

    const { sha } = await import("@/functions/sha");
    return await sha(type, input);
  };

  const onInput = async () => {
    txt2.value = await createSHA(txt1.value);
  };

  onInput();
  addEventListener(txt1, "input", onInput);

  return () => {
    removeEventListener(txt1, "input", onInput);
  };
});