import { Notyf } from "notyf";

let notyf: Notyf | null = null;

export async function nofity(type: "success" | "error", message: string) {
  if (!notyf || !document.querySelector(".notyf")) {
    notyf = new Notyf();
  }

  return notyf[type](message);
}