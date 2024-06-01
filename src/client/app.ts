import { copyText } from "@/client/functions/copy-text";
import { sha } from "@/client/functions/sha";
import { qrcode } from "@/client/views/qrcode";
import { onHxGet } from "@wirunekaewjai/jetpack";
import QRCode from "qrcode";

onHxGet(async (path, query) => {
  if (path === "/@qrcode") {
    try {
      const input = query.get("input")!;
      const output = await QRCode.toDataURL(input);

      return qrcode(output);
    } catch {
      return "";
    }
  }

  if (path === "/@sha") {
    const id = query.get("id")!;
    const input = query.get(id)!;
    const type = Number(query.get("type")) as 1 | 256 | 512;

    return await sha(type, input);
  }

  if (path === "/@base64-encode") {
    const input = query.get("input1")!;
    const output = btoa(input);

    return output;
  }

  if (path === "/@base64-decode") {
    const input = query.get("input2")!;
    const output = atob(input);

    return output;
  }

  if (path === "/@copy") {
    const el = document.getElementById(query.get("id")!);
    const txt = el?.textContent;

    if (!txt) {
      return "";
    }

    await copyText(txt);
    return "";
  }
});
