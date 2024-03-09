type Id = number;
type Value = unknown;
type SendMessage = (msg: string) => void;

function WSDOMStartWebSocket(wsUrl: string | URL, wsProtocols?: string | string[]) {
	const ws = new WebSocket(wsUrl, wsProtocols);
	const wsdom = new WSDOM((msg: string) => {
		ws.send(msg);
	});
	ws.onopen = () => {
		console.debug("WSDOM WebSocket connection open!");
		console.debug("WebSocket object", ws);
		console.debug("WSDOM object", wsdom);
	}
	ws.onmessage = (msg: MessageEvent<string>) => {
		wsdom.handleIncomingMessage(msg.data);
	};
	ws.onclose = (ev: CloseEvent) => {
		console.debug("WSDOM WebSocket closed", ev);
	}
	ws.onerror = (ev: Event) => {
		console.warn("WSDOM WebSocket errored", ev);
	}
}
class WSDOM {
	private internal: WSDOMInternal;
	constructor(sendMessage: SendMessage) {
		this.internal = new WSDOMInternal(sendMessage);
	}
	public handleIncomingMessage(msg: string) {
		const fn = new Function('_w', msg);
		fn(this.internal);
	}
}
class WSDOMInternal {
	private sender: SendMessage;
	private values: Map<Id, Value>;
	constructor(sender: SendMessage) {
		this.sender = sender;
		this.values = new Map();
	}
	public g = (id: Id): Value => {
		return this.values.get(id);
	}
	public s = (id: Id, value: Value) => {
		this.values.set(id, value);
	}
	public d = (id: Id) => {
		this.values.delete(id);
	}
	public r = (id: Id, val: Value) => {
		const valJson = JSON.stringify(val);
		(this.sender)(`${id}:${valJson}`);
	}
}
