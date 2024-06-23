import { addEventListener } from "@/functions/common/add-event-listener";
import { createComponent } from "@/functions/common/create-component";
import { querySelector } from "@/functions/common/query-selector";
import { removeEventListener } from "@/functions/common/remove-event-listener";

export const QRCode = createComponent("qrcode", (element: HTMLElement) => {
  const txt = querySelector<HTMLTextAreaElement>(element, "textarea")!;
  const img = querySelector<HTMLImageElement>(element, "img")!;

  const createQRCode = async (input: string) => {
    try {
      const QRCode = await import("qrcode");
      const output = await QRCode.toDataURL(input);

      return output;
    } catch {
      return "";
    }
  };

  const onInput = async () => {
    const qrcode = await createQRCode(txt.value);

    img.src = qrcode;
    img.classList.toggle("hidden", !qrcode);
  };

  onInput();
  addEventListener(txt, "input", onInput);

  return () => {
    removeEventListener(txt, "input", onInput);
  };
});