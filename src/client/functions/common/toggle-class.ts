export function toggleClass(element: Element, token: string, force?: boolean) {
  element.classList.toggle(token, force);
}