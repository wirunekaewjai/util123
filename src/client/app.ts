import { Base64 } from "@/components/base64";
import { Link } from "@/components/common/link";
import { Router } from "@/components/common/router";
import { Copy } from "@/components/copy";
import { Gzip } from "@/components/gzip";
import { QRCode } from "@/components/qrcode";
import { Sha } from "@/components/sha";
import { addEventListener } from "@/functions/common/add-event-listener";
import { registerComponents } from "@/functions/common/register-components";

addEventListener(document, "DOMContentLoaded", () => {
  registerComponents(
    /* common */
    Link,
    Router,

    /* */
    Base64,
    Copy,
    Gzip,
    QRCode,
    Sha,
  );
});
