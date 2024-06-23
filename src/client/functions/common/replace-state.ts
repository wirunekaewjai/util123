export function replaceState(url: string, data?: any) {
  history.replaceState(data, "", url);
}