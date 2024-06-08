import type { Unsubscribe } from "@/types/unsubscribe";

export function listenCustomEvent(type: string, callback: (e: CustomEvent) => void | Promise<void>): Unsubscribe {
  const listener = (e: Event) => {
    return callback(e as CustomEvent);
  };

  document.addEventListener(type, listener);
  return () => document.removeEventListener(type, listener);
}