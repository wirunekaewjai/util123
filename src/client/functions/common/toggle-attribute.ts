export function toggleAttribute(element: Element, qualifiedName: string, force?: boolean) {
  return element.toggleAttribute(qualifiedName, force);
}