extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

struct NodeStyle {
	pub data?: String,

}
struct NodeProps {
	
}
struct EventList {
	pub onClick?: String,
	pub onChange?: String,
	pub onInput?: String,
	pub onFocus?: String,
	pub onBlur?: String,
}

struct Config {
	pub key: u32,
	pub parentKey: u32,
	pub nodeName: String,
	nodeStyle: NodeStyle,
	nodeProps: NodeProps,
	parentPath: String,
	eventList?: EventList,
	children: Config[]
}

#[wasm_bindgen]
extern {
	pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
	alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn createDOM(config: &Config) {
	
}