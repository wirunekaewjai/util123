export function addEventListener(target: Element | Document | Window, type: string, listener: EventListenerOrEventListenerObject, options?: boolean | AddEventListenerOptions) {
  target.addEventListener(type, listener, options);
}