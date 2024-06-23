export function debounce(fn: Function, delay: number = 300) {
  let t: number | undefined;

  return () => {
    if (t) {
      clearTimeout(t);
    }

    t = setTimeout(fn, delay);
  };
}