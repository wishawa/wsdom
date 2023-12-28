type Id = number;
type Value = unknown;
type SendMessage = (msg: string) => void;

function startWebSocketWRMI(wsUrl: string | URL, wsProtocols?: string | string[]) {
	const ws = new WebSocket(wsUrl, wsProtocols);
	const wsdom = new WRMI((msg: string) => {
		ws.send(msg);
	});
	ws.onopen = () => {
		console.debug("WRMI WS connection open!");
		console.debug("WebSocket object", ws);
		console.debug("WRMI object", wsdom);
	}
	ws.onmessage = (msg: MessageEvent<string>) => {
		wsdom.handleIncomingMessage(msg.data);
	};
}
class WRMI {
	private internal: WRMIInternal;
	constructor(sendMessage: SendMessage) {
		this.internal = new WRMIInternal(sendMessage);
	}
	public handleIncomingMessage(msg: string) {
		const fn = new Function('_w', msg);
		fn(this.internal);
	}
}
class WRMIInternal {
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
