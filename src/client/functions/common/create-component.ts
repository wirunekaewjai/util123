import type { Component } from "@/types/common/component";

export function createComponent(name: string, component: Component): [string, Component] {
  return [name, component];
}