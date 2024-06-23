export function querySelector<T extends Element>(element: Element | Document, selector: string) {
  return element.querySelector(selector) as T | null;
}