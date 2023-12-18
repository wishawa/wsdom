/**
 * Allows manipulation of the browser session history, that is the pages visited in the tab or frame that the current page is loaded in.
 *
 * [MDN Reference](https://developer.mozilla.org/docs/Web/API/History)
 */
interface History {
    /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/History/length) */
    readonly length: number;
    /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/History/scrollRestoration) */
    scrollRestoration: ScrollRestoration;
    /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/History/state) */
    readonly state: any;
    /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/History/back) */
    back(): void;
    /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/History/forward) */
    forward(): void;
    /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/History/go) */
    go(delta?: number): void;
    /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/History/pushState) */
    pushState(data: any, unused: string, url?: string | URL | null): void;
    /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/History/replaceState) */
    replaceState(data: any, unused: string, url?: string | URL | null): void;
}

declare var History: {
    prototype: History;
    new(): History;
};