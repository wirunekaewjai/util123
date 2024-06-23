export function querySelectorAll<T extends Element>(element: Element | Document, selector: string) {
  return element.querySelectorAll<T>(selector);
}