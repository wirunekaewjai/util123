export function pushState(url: string, data?: any) {
  history.pushState(data, "", url);
}