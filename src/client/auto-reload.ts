(async function () {
  const DELAY = 1_000;

  async function sleep(ms: number) {
    await new Promise((resolve) => setTimeout(resolve, ms));
  }

  let version = "";

  while (true) {
    try {
      const res = await fetch(`/api/version?v=${version}`, {
        method: "POST",
        cache: "no-cache",
      });

      if (!res.ok) {
        await sleep(DELAY);
        continue;
      }

      const value = await res.text();

      if (version && version !== value) {
        window.location.reload();
        return;
      }

      version = value;
    } catch {
      await sleep(DELAY);
    }
  }
})();