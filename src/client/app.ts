import { copyText } from "@/functions/copy-text";
import { listenCustomEvent } from "@/functions/listen-custom-event";
import { parseHxQuery } from "@/functions/parse-hx-query";
import { sendHxResponse } from "@/functions/send-hx-response";
import { sha } from "@/functions/sha";
import type { HtmxBeforeRequestEventDetail } from "@/types/htmx";
import { qrcode } from "@/views/qrcode";

const beforeRequests = new Map<string, (e: CustomEvent) => Promise<void> | void>();

beforeRequests.set("/@qrcode", async (e: CustomEvent<HtmxBeforeRequestEventDetail>) => {
  e.preventDefault();

  const { requestConfig, xhr } = e.detail;
  const [, search] = requestConfig.path.split("?");

  try {
    const query = parseHxQuery(search, requestConfig.parameters) as {
      input: string;
    };

    const QRCode = await import("qrcode");
    const output = await QRCode.toDataURL(query.input);

    return sendHxResponse(xhr, {
      body: qrcode(output),
      url: requestConfig.path,
    });
  } catch {
    return sendHxResponse(xhr, {
      body: "",
      url: requestConfig.path,
    });
  }
});

beforeRequests.set("/@sha", async (e: CustomEvent<HtmxBeforeRequestEventDetail>) => {
  e.preventDefault();

  const { requestConfig, xhr } = e.detail;
  const [, search] = requestConfig.path.split("?");

  const query = parseHxQuery(search, requestConfig.parameters) as {
    id: string;
    type: 1 | 256 | 512;
  } & {
    [key: string]: string;
  };

  const input = query[query.id];
  const type = query.type;

  return sendHxResponse(xhr, {
    body: await sha(type, input),
    url: requestConfig.path,
  });
});

beforeRequests.set("/@base64-encode", (e: CustomEvent<HtmxBeforeRequestEventDetail>) => {
  e.preventDefault();

  const { requestConfig, xhr } = e.detail;
  const [, search] = requestConfig.path.split("?");

  const query = parseHxQuery(search, requestConfig.parameters) as {
    input1: string;
  };

  return sendHxResponse(xhr, {
    body: btoa(query.input1),
    url: requestConfig.path,
  });
});

beforeRequests.set("/@base64-decode", (e: CustomEvent<HtmxBeforeRequestEventDetail>) => {
  e.preventDefault();

  const { requestConfig, xhr } = e.detail;
  const [, search] = requestConfig.path.split("?");

  const query = parseHxQuery(search, requestConfig.parameters) as {
    input2: string;
  };

  return sendHxResponse(xhr, {
    body: atob(query.input2),
    url: requestConfig.path,
  });
});

beforeRequests.set("/@copy", async (e: CustomEvent<HtmxBeforeRequestEventDetail>) => {
  e.preventDefault();

  const { requestConfig, xhr } = e.detail;
  const [, search] = requestConfig.path.split("?");

  const query = parseHxQuery(search, requestConfig.parameters) as {
    id: string;
  };

  const el = document.getElementById(query.id);
  const txt = el?.textContent;

  if (!txt) {
    return sendHxResponse(xhr, {
      body: "",
      url: requestConfig.path,
    });
  }

  await copyText(txt);

  return sendHxResponse(xhr, {
    body: "",
    url: requestConfig.path,
  });
});

const onBeforeRequest = async (e: CustomEvent<HtmxBeforeRequestEventDetail>) => {
  const [pathname] = e.detail.requestConfig.path.split("?");
  await beforeRequests.get(pathname)?.(e);
};

listenCustomEvent("htmx:beforeRequest", onBeforeRequest);