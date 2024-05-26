// AUTO GENERATED
use tiny_tsx::tsx_map;

pub struct UtilityListItem {
    pub id: String,
    pub icon: String,
    pub name: String,
}

pub fn utility_list(items: Vec<UtilityListItem>) -> String {
    return format!(
        r#"<div class="space-y-2 divide-y" hx-boost="true">{}</div>"#,
        tsx_map(&items, &|item| format!(
            r#"<a class="hover:text-blue-400 grid grid-cols-[16px_1fr] gap-x-4 px-4 py-2 items-center" href="/utils/{}">{}{}</a>"#,
            item.id, item.icon, item.name
        ))
    );
}

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
