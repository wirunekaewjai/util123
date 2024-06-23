import { Base64 } from "@/components/base64";
import { Copy } from "@/components/copy";
import { QRCode } from "@/components/qrcode";
import { Sha } from "@/components/sha";
import { addEventListener } from "@/functions/common/add-event-listener";
import { registerComponents } from "@/functions/common/register-components";

addEventListener(document, "DOMContentLoaded", () => {
  registerComponents(
    Base64,
    Copy,
    QRCode,
    Sha,
  );
});
