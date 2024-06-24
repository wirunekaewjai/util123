import { EVENT_LOCATION_CHANGE } from "@/constants/common/event";
import { addEventListener } from "@/functions/common/add-event-listener";
import { createComponent } from "@/functions/common/create-component";
import { getAttribute } from "@/functions/common/get-attribute";
import { morphHTML } from "@/functions/common/morph-html";
import { removeEventListener } from "@/functions/common/remove-event-listener";
import { replaceState } from "@/functions/common/replace-state";

export const Router = createComponent("router", (router: HTMLElement) => {
  let currentPathname = location.pathname;

  const index = getAttribute(router, "x-index") || "0";
  // const fade = hasAttribute(router, "x-fade");

  const partIndex = Number(index) + 1;

  const controller = new AbortController();
  const caches = new Map<string, string>();

  const getHTML = async (path: string) => {
    const cache = caches.get(path);

    if (typeof cache === "string") {
      return cache;
    }

    const res = await fetch(path, {
      signal: controller.signal,
      headers: {
        "X-Fragment": "1",
      },
    });

    const html = await res.text();

    caches.set(path, html);
    return html;
  };

  const onChange = async () => {
    const parts1 = currentPathname.split("/");
    const parts2 = location.pathname.split("/");

    for (let i = 1; i < partIndex; i++) {
      if (parts1[i] !== parts2[i]) {
        // parent-level change
        return;
      }
    }

    if (parts1[partIndex] === parts2[partIndex]) {
      // nothing change on target level
      return;
    }

    currentPathname = location.pathname;

    // if (fade) {
    //   router.innerHTML = "";
    // }

    const html = await getHTML(location.pathname);

    morphHTML(router, html);
    replaceState(location.pathname);
  };

  addEventListener(document, EVENT_LOCATION_CHANGE, onChange);

  return () => {
    removeEventListener(document, EVENT_LOCATION_CHANGE, onChange);
    controller.abort();
  };
});