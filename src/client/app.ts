import { copyText } from "@/client/functions/copy-text";
import { sha } from "@/client/functions/sha";
import { qrcode } from "@/client/views/qrcode";
import { hxResponse } from "@wirunekaewjai/jetpack";

const onAfterSwap = (e: CustomEvent) => {
  const detail = e.detail as {
    elt: HTMLElement;
    pathInfo: {
      requestPath: string;
    };
  };

  const [pathname] = detail.pathInfo.requestPath.split("?");

  // bind icon name in svg element for preserve svg
  if (pathname.startsWith("/icons/")) {
    return detail.elt.setAttribute("name", pathname.split("/")[2]);
  }
};

const onBeforeRequest = async (e: CustomEvent) => {
  const xhr = e.detail.xhr as XMLHttpRequest;
  const conf = e.detail.requestConfig as {
    path: string;
    parameters: Record<string, any>;
  };

  const [pathname, search] = conf.path.split("?");

  const hxVals = conf.parameters;
  const query = new URLSearchParams(search);

  if (pathname === "/@qrcode") {
    e.preventDefault();

    try {
      const input = query.get("input") ?? hxVals.input;

      const QRCode = await import("qrcode");
      const output = await QRCode.toDataURL(input);

      return hxResponse(xhr, {
        body: qrcode(output),
        url: conf.path,
      });
    } catch {
      return hxResponse(xhr, {
        body: "",
        url: conf.path,
      });
    }
  }

  if (pathname === "/@sha") {
    e.preventDefault();

    const id = query.get("id") ?? hxVals.id;
    const input = query.get(id) ?? hxVals[id];
    const type = Number(query.get("type") ?? hxVals.type) as 1 | 256 | 512;

    return hxResponse(xhr, {
      body: await sha(type, input),
      url: conf.path,
    });
  }

  if (pathname === "/@base64-encode") {
    e.preventDefault();

    const input = query.get("input1") ?? hxVals.input1;

    return hxResponse(xhr, {
      body: btoa(input),
      url: conf.path,
    });
  }

  if (pathname === "/@base64-decode") {
    e.preventDefault();

    const input = query.get("input2") ?? hxVals.input2;

    return hxResponse(xhr, {
      body: atob(input),
      url: conf.path,
    });
  }

  if (pathname === "/@copy") {
    e.preventDefault();

    const id = query.get("id") ?? hxVals.id;
    const el = document.getElementById(id);
    const txt = el?.textContent;

    if (!txt) {
      return hxResponse(xhr, {
        body: "",
        url: conf.path,
      });
    }

    await copyText(txt);

    return hxResponse(xhr, {
      body: "",
      url: conf.path,
    });
  }
};

htmx.on("htmx:afterSwap", (e) => {
  return onAfterSwap(e as CustomEvent);
});

// wrapped async function with normal function because htmx not supported async function
htmx.on("htmx:beforeRequest", (e) => {
  return onBeforeRequest(e as CustomEvent);
});