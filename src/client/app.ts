import "@wirunekaewjai/ts/htmx-interceptor/prelude";

import { copyText } from "@/client/functions/copy-text";
import { sha } from "@/client/functions/sha512";
import { $qrcode } from "@/client/views/qrcode";
import QRCode from "qrcode";

interceptor.add("/@generate-qrcode", async ({ query }) => {
  try {
    const input = query.input;
    const output = await QRCode.toDataURL(input);

    if (output) {
      return $qrcode(output);
    }

    return "";
  } catch {
    return "";
  }
});

interceptor.add("/@sha1", async ({ query }) => {
  const id = query.id;
  const input = query[id];
  const output = await sha(1, input);

  return output;
});

interceptor.add("/@sha256", async ({ query }) => {
  const id = query.id;
  const input = query[id];
  const output = await sha(256, input);

  return output;
});

interceptor.add("/@sha512", async ({ query }) => {
  const id = query.id;
  const input = query[id];
  const output = await sha(512, input);

  return output;
});

interceptor.add("/@base64-encode-change", async ({ query }) => {
  const input = query.input1;
  const output = btoa(input);

  return output;
});

interceptor.add("/@base64-decode-change", async ({ query }) => {
  const input = query.input2;
  const output = atob(input);

  return output;
});

interceptor.add("/@copy", async ({ query }) => {
  const el = document.getElementById(query.id);
  const txt = el?.textContent;

  if (!txt) {
    return "";
  }

  await copyText(txt);
  return "";
});