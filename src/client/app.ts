import "./prelude";

import { copyText } from "@/client/functions/copy-text";
import { sha } from "@/client/functions/sha";
import { qrcode } from "@/client/views/qrcode";
import { hxQuery, hxResponse } from "@wirunekaewjai/jetpack";

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
    return detail.elt.setAttribute("name", pathname.split(".")[0].split("/")[2]);
  }
};

const onBeforeRequest = async (e: CustomEvent) => {
  const xhr = e.detail.xhr as XMLHttpRequest;
  const conf = e.detail.requestConfig as {
    path: string;
    parameters: Record<string, any>;
  };

  const [pathname, search] = conf.path.split("?");

  if (pathname === "/@qrcode") {
    e.preventDefault();

    try {
      const query = hxQuery(search, conf.parameters) as {
        input: string;
      };

      const QRCode = await import("qrcode");
      const output = await QRCode.toDataURL(query.input);

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

    const query = hxQuery(search, conf.parameters) as {
      id: string;
      type: 1 | 256 | 512;
    } & {
      [key: string]: string;
    };

    const input = query[query.id];
    const type = query.type;

    return hxResponse(xhr, {
      body: await sha(type, input),
      url: conf.path,
    });
  }

  if (pathname === "/@base64-encode") {
    e.preventDefault();

    const query = hxQuery(search, conf.parameters) as {
      input1: string;
    };

    return hxResponse(xhr, {
      body: btoa(query.input1),
      url: conf.path,
    });
  }

  if (pathname === "/@base64-decode") {
    e.preventDefault();

    const query = hxQuery(search, conf.parameters) as {
      input2: string;
    };

    return hxResponse(xhr, {
      body: atob(query.input2),
      url: conf.path,
    });
  }

  if (pathname === "/@copy") {
    e.preventDefault();

    const query = hxQuery(search, conf.parameters) as {
      id: string;
    };

    const el = document.getElementById(query.id);
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