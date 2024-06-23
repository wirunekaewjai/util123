import { getAttribute } from "@/functions/common/get-attribute";
import { hasAttribute } from "@/functions/common/has-attribute";
import { querySelectorAll } from "@/functions/common/query-selector-all";
import type { Component } from "@/types/common/component";
import type { Unsubscribe } from "@/types/common/unsubscribe";

export function registerComponents(...components: [string, Component][]) {
  const componentAttributeKey = "x-component";
  const componentAttributeSelector = `[${componentAttributeKey}]`;
  const componentMap = new Map<string, Component>(components);

  const registrations = new Map<string, Set<HTMLElement>>();
  const unsubscribes = new Map<HTMLElement, Unsubscribe>();

  const add = (element: HTMLElement) => {
    const names = getAttribute(element, componentAttributeKey)?.split(",") ?? [];

    for (const name of names) {
      const component = componentMap.get(name);

      if (!component) {
        console.debug("unknown component:", name);
        continue;
      }

      if (!registrations.has(name)) {
        registrations.set(name, new Set());
      }

      if (!registrations.get(name)?.has(element)) {
        registrations.get(name)?.add(element);

        const unsubscribe = component(element);

        if (unsubscribe) {
          unsubscribes.set(element, unsubscribe);
        }

        // console.log("added", element);
      }
    }
  };

  const remove = (element: HTMLElement) => {
    const names = getAttribute(element, componentAttributeKey)?.split(",") ?? [];

    for (const name of names) {
      registrations.get(name)?.delete(element);

      unsubscribes.get(element)?.();
      unsubscribes.delete(element);

      // console.log("removed", element);
    }
  };

  const execute = (fn: (element: HTMLElement) => void, element: Node) => {
    if (element instanceof HTMLElement) {
      if (hasAttribute(element, componentAttributeKey)) {
        fn(element);
      }

      querySelectorAll<HTMLElement>(element, componentAttributeSelector).forEach(fn);
    }
  };

  const mutationObserver = new MutationObserver((records) => {
    records.forEach((record) => {
      record.removedNodes.forEach((element) => {
        execute(remove, element);
      });

      record.addedNodes.forEach((element) => {
        execute(add, element);
      });
    });
  });

  const doc = document.documentElement;

  mutationObserver.observe(doc, {
    childList: true,
    subtree: true,
  });

  execute(add, doc);
}