export function toJson(value: any, indent?: number) {
  return JSON.stringify(value, null, indent);
}