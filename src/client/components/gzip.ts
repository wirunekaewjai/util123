import { addEventListener } from "@/functions/common/add-event-listener";
import { base64ToBlob } from "@/functions/common/base64-to-blob";
import { blobToBase64 } from "@/functions/common/blob-to-base64";
import { createComponent } from "@/functions/common/create-component";
import { formatNumber } from "@/functions/common/format-number";
import { getAttribute } from "@/functions/common/get-attribute";
import { gzipCompress } from "@/functions/common/gzip-compress";
import { gzipDecompress } from "@/functions/common/gzip-decompress";
import { querySelectorAll } from "@/functions/common/query-selector-all";
import { removeEventListener } from "@/functions/common/remove-event-listener";

const PREFIX = "data:application/gzip;base64,";

export const Gzip = createComponent("gzip", (element: HTMLElement) => {
  const fn = getAttribute(element, "x-function") as "compress" | "decompress";
  const textareas = querySelectorAll<HTMLTextAreaElement>(element, "textarea");
  const counts = querySelectorAll<HTMLSpanElement>(element, "span[name=count]");

  const txt1 = textareas.item(0);
  const txt2 = textareas.item(1);

  const count1 = counts.item(0);
  const count2 = counts.item(1);

  const compress = async (text: string) => {
    if (!text) {
      return "";
    }

    const buffer = new TextEncoder().encode(text);
    const blob = await gzipCompress(buffer);
    const base64 = await blobToBase64(blob);
    return base64.slice(PREFIX.length);
  };

  const decompress = async (base64: string) => {
    if (!base64) {
      return "";
    }

    const blob = await base64ToBlob(`${PREFIX}${base64}`);
    const buffer = await gzipDecompress(blob);
    const text = new TextDecoder().decode(buffer);
    return text;
  };

  const onInput = async () => {
    count1.textContent = formatNumber(txt1.value.length, 0);

    if (fn === "compress") {
      txt2.value = await compress(txt1.value);
    } else {
      txt2.value = await decompress(txt1.value);
    }

    count2.textContent = formatNumber(txt2.value.length, 0);
  };

  onInput();
  addEventListener(txt1, "input", onInput);

  return () => {
    removeEventListener(txt1, "input", onInput);
  };
});