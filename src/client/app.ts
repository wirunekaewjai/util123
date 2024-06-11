async function createQRCode(input: string) {
  try {
    const QRCode = await import("qrcode");
    const output = await QRCode.toDataURL(input);

    return output;
  } catch {
    return "";
  }
}

async function createSHA(type: 1 | 256 | 512, input: string) {
  if (!input) {
    return "";
  }

  const { sha } = await import("@/functions/sha");
  return await sha(type, input);
}

async function copyText(value: string) {
  await navigator.clipboard.writeText(value);

  const { nofity } = await import("@/functions/notify");

  try {
    // console.log('Text copied to clipboard!');
    await nofity("success", "Copied");
  } catch (err) {
    // console.error('Failed to copy: ', err);
    await nofity("error", "Failed");
  }
}

function captureTextArea(element: HTMLTextAreaElement, onChange: (value: string) => void | Promise<void>) {
  const onInput = async () => {
    await onChange(element.value);
  };

  onInput();
  element.addEventListener("input", onInput);

  return () => {
    element.removeEventListener("input", onInput);
  };
}

function componentQRCode(element: HTMLElement) {
  const txt = element.querySelector("textarea")!;
  const img = element.querySelector("img")!;

  const onChange = async (value: string) => {
    const qrcode = await createQRCode(value);

    img.src = qrcode;
    img.classList.toggle("hidden", !qrcode);
  };

  return captureTextArea(txt, onChange);
}

function componentSHA1(element: HTMLElement) {
  const textareas = element.querySelectorAll("textarea");

  const txt1 = textareas.item(0);
  const txt2 = textareas.item(1);

  const onChange = async (value: string) => {
    txt2.value = await createSHA(1, value);
  };

  return captureTextArea(txt1, onChange);
}

function componentSHA256(element: HTMLElement) {
  const textareas = element.querySelectorAll("textarea");

  const txt1 = textareas.item(0);
  const txt2 = textareas.item(1);

  const onChange = async (value: string) => {
    txt2.value = await createSHA(256, value);
  };

  return captureTextArea(txt1, onChange);
}

function componentSHA512(element: HTMLElement) {
  const textareas = element.querySelectorAll("textarea");

  const txt1 = textareas.item(0);
  const txt2 = textareas.item(1);

  const onChange = async (value: string) => {
    txt2.value = await createSHA(512, value);
  };

  return captureTextArea(txt1, onChange);
}

function componentBase64Encode(element: HTMLElement) {
  const textareas = element.querySelectorAll("textarea");

  const txt1 = textareas.item(0);
  const txt2 = textareas.item(1);

  const onChange = (value: string) => {
    txt2.value = btoa(value);
  };

  return captureTextArea(txt1, onChange);
}

function componentBase64Decode(element: HTMLElement) {
  const textareas = element.querySelectorAll("textarea");

  const txt1 = textareas.item(0);
  const txt2 = textareas.item(1);

  const onChange = (value: string) => {
    txt2.value = atob(value);
  };

  return captureTextArea(txt1, onChange);
}

function componentCopy(element: HTMLElement) {
  const onCopy = async () => {
    if (element instanceof HTMLTextAreaElement) {
      await copyText(element.value);
    } else if (element.textContent) {
      await copyText(element.textContent);
    }
  };

  element.addEventListener("click", onCopy);

  return () => {
    element.removeEventListener("click", onCopy);
  };
}

document.addEventListener("DOMContentLoaded", () => {
  const componentAttributeKey = "x-component";
  const componentAttributeSelector = `[${componentAttributeKey}]`;
  const componentMaps = new Map<string, (element: HTMLElement) => (() => void)>([
    ["qrcode", componentQRCode],
    ["sha1", componentSHA1],
    ["sha256", componentSHA256],
    ["sha512", componentSHA512],
    ["base64-encode", componentBase64Encode],
    ["base64-decode", componentBase64Decode],
    ["copy", componentCopy],
  ]);

  const elementMaps = new Map<string, Set<HTMLElement>>();
  const elementUnsubscribeMaps = new Map<HTMLElement, () => void>();

  const add = (element: HTMLElement) => {
    const name = element.getAttribute(componentAttributeKey)!;
    const component = componentMaps.get(name);

    if (!component) {
      // console.debug("what is", name, "?");
      return;
    }

    if (!elementMaps.has(name)) {
      elementMaps.set(name, new Set());
    }

    if (!elementMaps.get(name)?.has(element)) {
      elementMaps.get(name)?.add(element);
      elementUnsubscribeMaps.set(element, component(element));

      // console.log("add", element);
    }
  };

  const remove = (element: HTMLElement) => {
    const name = element.getAttribute(componentAttributeKey)!;

    elementMaps.get(name)?.delete(element);

    elementUnsubscribeMaps.get(element)?.();
    elementUnsubscribeMaps.delete(element);

    // console.log("remove", element);
  };

  const addOrRemove = (element: Node, fn: (element: HTMLElement) => void) => {
    if (element instanceof HTMLElement) {
      if (element.hasAttribute(componentAttributeKey)) {
        fn(element);
      }

      element.querySelectorAll<HTMLElement>(componentAttributeSelector).forEach(fn);
    }
  };

  const mutationObserver = new MutationObserver((records) => {
    records.forEach((record) => {
      record.removedNodes.forEach((element) => {
        addOrRemove(element, remove);
      });

      record.addedNodes.forEach((element) => {
        addOrRemove(element, add);
      });
    });
  });

  mutationObserver.observe(document.body, {
    childList: true,
    subtree: true,
  });

  addOrRemove(document.body, add);
});