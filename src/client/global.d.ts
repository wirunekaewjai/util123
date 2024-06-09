export declare global {
  var createQRCode: (input: string) => Promise<string>;
  var createSHA: (type: 1 | 256 | 512, input: string) => Promise<string>;
  var copyText: (value: string) => Promise<void>;
}