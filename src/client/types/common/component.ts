import type { Unsubscribe } from "@/types/common/unsubscribe";

export type Component = (element: any) => Unsubscribe | void;