export function removeEventListener(target: Element | Document | Window, type: string, listener: EventListenerOrEventListenerObject, options?: boolean | EventListenerOptions) {
  target.removeEventListener(type, listener, options);
}