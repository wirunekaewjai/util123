import { EVENT_LOCATION_CHANGE } from "@/constants/common/event";
import { addEventListener } from "@/functions/common/add-event-listener";
import { createComponent } from "@/functions/common/create-component";
import { dispatchEvent } from "@/functions/common/dispatch-event";
import { pushState } from "@/functions/common/push-state";
import { removeEventListener } from "@/functions/common/remove-event-listener";

export const Link = createComponent("link", (a: HTMLAnchorElement) => {
  const onClick = async (e: Event) => {
    const url = new URL(a.href);

    if (url.origin === window.location.origin) {
      e.preventDefault();

      pushState(url.pathname + url.search);
      dispatchEvent(EVENT_LOCATION_CHANGE);
    }
  };

  addEventListener(a, "click", onClick);

  return () => {
    removeEventListener(a, "click", onClick);
  };
});