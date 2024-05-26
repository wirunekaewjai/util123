// AUTO GENERATED
import { tsx_map } from "@wirunekaewjai/tiny-tsx/macro";

export interface UtilityListItem {
  id: string;
  icon: string;
  name: string;
}

export const $utility_list = (items: UtilityListItem[]) => `<div class="space-y-2 divide-y" hx-boost="true">${tsx_map(items, (item) => `<a class="hover:text-blue-400 grid grid-cols-[16px_1fr] gap-x-4 px-4 py-2 items-center" href="/utils/${item.id}">${item.icon}${item.name}</a>`)}</div>`;

/*
interface Item {
  id: string;
  icon: string;
  name: string;
}

(items: Item[]) => (
  <div
    class="space-y-2 divide-y"
    hx-boost="true"
  >
    {map(items, (item) => (
      <a
        class="hover:text-blue-400 grid grid-cols-[16px_1fr] gap-x-4 px-4 py-2 items-center"
        href={`/utils/${item.id}`}
      >
        {item.icon}
        {item.name}
      </a>
    ))}
  </div>
);
*/
