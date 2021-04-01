const { isAppWebView, facilities, appendSearch } = require('@schacker/wasm-utils')
const isAppWebViewJS = require('./jsversion')
const appendSearchJS = require('./testAppend')

// const path = require('path').join(__dirname, 'wasm_utils_bg.wasm');
// const bytes = require('fs').readFileSync(path);

// const wasmModule = new WebAssembly.Module(bytes);
// const wasmInstance = new WebAssembly.Instance(wasmModule, {__wbindgen_placeholder__: module.exports});
// console.log(wasmInstance.exports)
// return
function fac(n) {
  if (n <= 0) {
    return 1;
  }
  return n * fac(n - 1);
}


const times = 10**2
console.time("js")
for (let i = 0;i < times; i++) {
  // isAppWebViewJS('lianjia/homelink/1.0', "lianjia")
  // const s = fac(50)
  const s = appendSearchJS('/a/b/c', "?a=1&b=2&c=3")
  // console.log(s)

  // ['lianjia/homelink/1.0', 
  //   'vrStudio/1.0', 
  //   'homelink/3.1', 
  //   'lianjia/alliance/2.1', 
  //   'lianjia/lianjiabaichuan/1.2', 
  //   'lianjiabeike/1.1',
  //   'lianjiaatom/1.2'].map((ua) => {
  //   return isAppWebViewJS(ua, "lianjia")
  // })
}
console.timeEnd("js")

console.time("wasm")
for (let i = 0;i < times; i++) {
  // isAppWebView('lianjia/homelink/1.0', "lianjia")
  // const s = facilities(50)
  const s = appendSearch('/a/b/c', "?a=1&b=2&c=3")
  // console.log(s)

  // ['lianjia/homelink/1.0', 
  //   'vrStudio/1.0', 
  //   'homelink/3.1', 
  //   'lianjia/alliance/2.1', 
  //   'lianjia/lianjiabaichuan/1.2', 
  //   'lianjiabeike/1.1',
  //   'lianjiaatom/1.2'].map((ua) => {
  //   return isAppWebView(ua, "lianjia")
  // })
}
console.timeEnd("wasm")



