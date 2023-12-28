interface EventListenerOptions {
    capture?: boolean;
    "haha": "lol"
}

interface AddEventListenerOptions extends EventListenerOptions {
    once?: boolean;
    passive?: boolean;
    signal?: AbortSignal;
}

interface AbortSignal {

}