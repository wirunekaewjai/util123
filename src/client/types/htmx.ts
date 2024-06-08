export interface HtmxAfterSwapEventDetail {
  elt: HTMLElement;
  pathInfo: {
    requestPath: string;
  };
}

export interface HtmxBeforeRequestEventDetail {
  elt: HTMLElement;
  requestConfig: HtmxRequestConfig;
  target: HTMLElement;
  xhr: XMLHttpRequest;
}

export interface HtmxRequestConfig {
  path: string;
  parameters: Record<string, any>;
}