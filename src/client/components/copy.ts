import { addEventListener } from "@/functions/common/add-event-listener";
import { createComponent } from "@/functions/common/create-component";
import { removeEventListener } from "@/functions/common/remove-event-listener";

export const Copy = createComponent("copy", (element: HTMLElement) => {
  const copyText = async (value: string) => {
    await navigator.clipboard.writeText(value);

    const { nofity } = await import("@/functions/notify");

    try {
      // console.log('Text copied to clipboard!');
      await nofity("success", "Copied");
    } catch (err) {
      // console.error('Failed to copy: ', err);
      await nofity("error", "Failed");
    }
  };

  const onCopy = async () => {
    if (element instanceof HTMLTextAreaElement) {
      await copyText(element.value);
    } else if (element.textContent) {
      await copyText(element.textContent);
    }
  };

  addEventListener(element, "click", onCopy);

  return () => {
    removeEventListener(element, "click", onCopy);
  };
});