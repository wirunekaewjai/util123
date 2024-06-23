export function dispatchEvent(type: string, eventInitDict?: CustomEventInit<unknown> | undefined) {
  return document.dispatchEvent(new CustomEvent(type, eventInitDict));
}