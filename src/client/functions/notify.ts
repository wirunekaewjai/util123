import { querySelector } from "@/functions/common/query-selector";
import { Notyf } from "notyf";

let notyf: Notyf | null = null;

export async function nofity(type: "success" | "error", message: string) {
  if (!notyf || !querySelector(document, ".notyf")) {
    notyf = new Notyf();
  }

  return notyf[type](message);
}