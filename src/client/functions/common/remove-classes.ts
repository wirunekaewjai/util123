export function removeClasses(element: Element, ...tokens: string[]) {
  element.classList.remove(...tokens);
}