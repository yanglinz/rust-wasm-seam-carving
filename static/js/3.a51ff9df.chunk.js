(this["webpackJsonprust-wasm-seam-carving"]=this["webpackJsonprust-wasm-seam-carving"]||[]).push([[3],{1:function(e,t,n){"use strict";n.r(t);var r=n(10),a=n(7),i=n(13),c=n(8),o=n(4);var s=n(6),l=n(9);function u(){var e,t=(e={},Object(l.a)(e,"yRjLihK35Yw",{alt:"Balloons in the sky"}),Object(l.a)(e,"e-S-Pe2EmrE",{alt:"Birds in the sky"}),Object(l.a)(e,"F6XKjhMNB14",{alt:"Waves on a beach"}),Object(l.a)(e,"KGwK6n7rxSg",{alt:"Hot balloons"}),Object(l.a)(e,"C9XgrB8hqBI",{alt:"Top down shot of beach"}),Object(l.a)(e,"pVr6wvUneMk",{alt:"Desert landscape"}),Object(l.a)(e,"Pn6iimgM-wo",{alt:"Light house at night"}),Object(l.a)(e,"4Oi1756LtF4",{alt:"Castle"}),e),n=Math.round(.8*window.innerWidth),r=Math.min(1e3,n),a=r/2;return Object.keys(t).map((function(e,n){t[e]=Object(s.a)(Object(s.a)({},t[e]),{},{url:"https://source.unsplash.com/".concat(e,"/").concat(r,"x").concat(a),previewUrl:"https://source.unsplash.com/".concat(e,"/200x200")})})),t}var d=n(18),h=n(3),f=["children","intent"];var m=function(e){var t=e.children,n=e.intent,r=Object(d.a)(e,f);return"PRIMARY"===n?Object(h.jsx)("button",Object(s.a)(Object(s.a)({className:"drop-shadow\tpx-5 font-large py-3 text-white text-base bg-blue-500 hover:bg-blue-600 border border-gray-300 rounded-md focus:outline-none disabled:opacity-50 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"},r),{},{children:t})):"SECONDARY"===n?Object(h.jsx)("button",Object(s.a)(Object(s.a)({className:"drop-shadow\tpx-5 font-large py-3 text-gray-700 text-base hover:bg-gray-50 bg-white border border-gray-300 rounded-md focus:outline-none disabled:opacity-50 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"},r),{},{children:t})):null};function g(e){return c.a.createPortal(e.children,function(){var e=arguments.length>0&&void 0!==arguments[0]?arguments[0]:"dialog-mount",t=document.getElementById(e);return t||((t=document.createElement("div")).id=e,document.body.appendChild(t)),t}())}function b(e){return Object(h.jsx)(g,{children:Object(h.jsx)("div",{className:"fixed z-10 inset-0 overflow-y-auto",role:"dialog","aria-modal":"true",children:Object(h.jsxs)("div",{className:"flex items-end justify-center pb-20 pt-4 px-4 min-h-screen text-center sm:block sm:p-0",children:[Object(h.jsx)("div",{className:"fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity","aria-hidden":"true"}),Object(h.jsx)("span",{className:"hidden sm:inline-block sm:align-middle sm:h-screen","aria-hidden":"true",children:"\u200b"}),Object(h.jsx)("div",{className:"inline-block align-bottom text-left bg-white rounded-sm shadow-xl overflow-hidden transform transition-all sm:align-middle sm:my-8 sm:w-full sm:max-w-lg",children:e.children})]})})})}var j=function(e){var t=e.globalState,n=e.handleImageSelect,r=e.handleImageUpload,a=e.handleClose;if("IMAGE_SELECT"!==t.control.state)return null;var i=u();return Object(h.jsxs)(b,{children:[Object(h.jsx)("div",{className:"pb-4 pt-5 px-4 bg-white sm:p-6 sm:pb-4",children:Object(h.jsxs)("div",{className:"flex flex-col content-center items-center",children:[Object(h.jsx)("h3",{className:"my-3 text-gray-900 text-lg font-medium leading-6",children:"Select or Upload Image"}),Object(h.jsx)("div",{className:"grid gap-1 grid-cols-4 overflow-hidden",children:Object.keys(i).map((function(e,t){var r=i[e];return Object(h.jsx)("img",{className:"inline-block w-20 h-20",onClick:function(){return n(r.url)},alt:r.alt,src:r.previewUrl})}))}),Object(h.jsx)("div",{className:"my-3",children:Object(h.jsx)("input",{className:"inline-block",type:"file",accept:"image/png, image/jpeg",onChange:r})})]})}),Object(h.jsx)("div",{className:"px-4 py-3 bg-gray-50 sm:flex sm:flex-row-reverse sm:px-6",children:Object(h.jsx)(m,{intent:"SECONDARY",onClick:a,children:"Cancel"})})]})};function v(){return{source:document.getElementById("canvas-source"),target:document.getElementById("canvas-target")}}var p=function(e){var t=e.globalState.selectedImage,n="SOURCE"===t.state?"":"hidden",r="TARGET"===t.state?"":"hidden";return Object(h.jsxs)("div",{className:"border-8 border-gray-600 border-opacity-5",children:[Object(h.jsx)("div",{className:n,children:Object(h.jsx)("canvas",{id:"canvas-source"})}),Object(h.jsx)("div",{className:r,children:Object(h.jsx)("canvas",{id:"canvas-target"})})]})};function O(e){var t=e.minWidth,n=e.maxWidth,i=e.disabled,c=e.onChange,o=Object(a.c)(n),l=Object(r.a)(o,2),u=l[0],d=l[1],f={disabled:!0};return i||(f={min:t,max:n,value:u,onChange:function(e){var t=parseInt(e.target.value);d(t),c(t)}}),Object(h.jsx)("div",{className:"flex justify-center",children:Object(h.jsxs)("div",{className:"align-center flex w-11/12 sm:w-8/12",children:[Object(h.jsx)("input",Object(s.a)({id:"resize",name:"resize",type:"range",className:"flex-grow"},f)),Object(h.jsxs)("label",{className:"w-20 text-right text-gray-700 font-mono",for:"resize",children:[i?0:u,"px"]})]})})}var x,y=function(e){var t=e.globalState,n=e.handleResize,i=e.handleOpenImageSelect,c=t.selectedImage,o=Object(a.c)(0),s=Object(r.a)(o,2),l=s[0],u=s[1],d="SOURCE"===c.state&&0!==l&&l!==c.width;return Object(h.jsxs)("div",{className:"Controls",children:[Object(h.jsx)("div",{className:"pb-3",children:Object(h.jsx)(O,{disabled:"SOURCE"!==c.state,minWidth:20,maxWidth:c.width,onChange:u},c.url)}),Object(h.jsx)("div",{className:"flex justify-center",children:Object(h.jsxs)("div",{className:"flex",children:[Object(h.jsx)(m,{intent:"PRIMARY",disabled:!d,onClick:function(){return n(l)},children:"Resize Image"}),Object(h.jsx)("div",{className:"px-1"}),Object(h.jsx)(m,{intent:"SECONDARY",onClick:i,children:"Try Another Image"})]})})]})},w=(n(12),{selectedImage:{state:"INITIAL",width:0,height:0},control:{state:"INITIAL"}}),E={IMAGE_LOADED:function(e,t){var n=t.payload,r=n.width,a=n.height,i=n.url;e.selectedImage.state="SOURCE",e.selectedImage.width=r,e.selectedImage.height=a,e.selectedImage.url=i},RESIZE_INITIALIZED:function(e,t){e.selectedImage.state="TARGET"},IMAGE_SELECT_OPENED:function(e,t){e.control.state="IMAGE_SELECT"},IMAGE_SELECT_CLOSED:function(e,t){e.control.state="INITIAL"}};function I(e,t){var n=E[t.type];if(!n)throw new Error("Unknown action type in reducer.");return Object(i.a)(e,(function(e){return n(e,t)}))}function _(e,t){var n=t.image,r=v().source;r.width=n.width,r.height=n.height,r.getContext("2d").drawImage(n,0,0,n.width,n.height,0,0,r.width,r.height)}function N(e,t){var n=t.imageUrl,r=document.createElement("img");r.src=n,r.crossOrigin="Anonymous",r.onload=function(){_(0,{image:r}),e({type:"IMAGE_LOADED",payload:{width:r.width,height:r.height,url:r.src}})}}function k(){var e=Object(a.b)(I,w),t=Object(r.a)(e,2),n=t[0],i=t[1];return Object(a.a)((function(){return N(i,{imageUrl:u().KGwK6n7rxSg.url})}),[]),Object(h.jsxs)("div",{className:"App flex flex-col h-screen",children:[Object(h.jsx)("div",{className:"flex-grow",children:Object(h.jsx)("div",{className:"items-center\tjustify-center flex h-full",children:Object(h.jsx)(p,{globalState:n})})}),Object(h.jsx)("div",{className:"border-gray-150 p-10 bg-white border-t",children:Object(h.jsx)(y,{globalState:n,handleResize:function(e){return function(e,t){var n=t.resizedWidth,r=v(),a=r.source,i=r.target,c=o.a.from_canvas(a.getContext("2d"),a.width,a.height);function s(){var e=new Uint8ClampedArray(Object(o.k)().buffer,c.image_data_ptr(),c.width*c.height*4);i.width=c.width,i.height=c.height;var t=new ImageData(e,c.width,c.height);i.getContext("2d").putImageData(t,0,0)}var l=a.width-n;requestAnimationFrame((function e(){l<=0||(c.mark_seam(),s(),requestAnimationFrame((function(){c.delete_seam(),s(),l-=1,requestAnimationFrame(e)})))})),e({type:"RESIZE_INITIALIZED"})}(i,{resizedWidth:e})},handleOpenImageSelect:function(){return i({type:"IMAGE_SELECT_OPENED"})}})}),Object(h.jsx)(j,{globalState:n,handleImageSelect:function(e){N(i,{imageUrl:e}),i({type:"IMAGE_SELECT_CLOSED"})},handleImageUpload:function(e){return function(e,t){var n=t.inputEvent.target.files[0],r=document.createElement("img");r.src=window.URL.createObjectURL(n),r.onload=function(){_(0,{image:r}),e({type:"IMAGE_LOADED",payload:{width:r.width,height:r.height,url:r.src}})}}(i,{inputEvent:e})},handleClose:function(){return i({type:"IMAGE_SELECT_CLOSED"})}})]})}x=function(){c.a.render(Object(h.jsx)(k,{}),document.getElementById("app"))},"loading"!=document.readyState?x():document.addEventListener("DOMContentLoaded",x)},12:function(e,t,n){},17:function(e,t,n){"use strict";var r=n.w[e.i];e.exports=r;n(4);r.s()},4:function(e,t,n){"use strict";(function(e){n.d(t,"k",(function(){return E})),n.d(t,"a",(function(){return _})),n.d(t,"i",(function(){return N})),n.d(t,"d",(function(){return k})),n.d(t,"b",(function(){return S})),n.d(t,"e",(function(){return A})),n.d(t,"f",(function(){return C})),n.d(t,"c",(function(){return L})),n.d(t,"g",(function(){return D})),n.d(t,"j",(function(){return T})),n.d(t,"h",(function(){return R}));var r=n(15),a=n(16),i=n(17),c=new Array(32).fill(void 0);function o(e){return c[e]}c.push(void 0,null,!0,!1);var s=c.length;function l(e){var t=o(e);return function(e){e<36||(c[e]=s,s=e)}(e),t}function u(e){var t=typeof e;if("number"==t||"boolean"==t||null==e)return"".concat(e);if("string"==t)return'"'.concat(e,'"');if("symbol"==t){var n=e.description;return null==n?"Symbol":"Symbol(".concat(n,")")}if("function"==t){var r=e.name;return"string"==typeof r&&r.length>0?"Function(".concat(r,")"):"Function"}if(Array.isArray(e)){var a=e.length,i="[";a>0&&(i+=u(e[0]));for(var c=1;c<a;c++)i+=", "+u(e[c]);return i+="]"}var o,s=/\[object ([^\]]+)\]/.exec(toString.call(e));if(!(s.length>1))return toString.call(e);if("Object"==(o=s[1]))try{return"Object("+JSON.stringify(e)+")"}catch(l){return"Object"}return e instanceof Error?"".concat(e.name,": ").concat(e.message,"\n").concat(e.stack):o}var d=0,h=null;function f(){return null!==h&&h.buffer===i.k.buffer||(h=new Uint8Array(i.k.buffer)),h}var m=new("undefined"===typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8"),g="function"===typeof m.encodeInto?function(e,t){return m.encodeInto(e,t)}:function(e,t){var n=m.encode(e);return t.set(n),{read:e.length,written:n.length}};function b(e,t,n){if(void 0===n){var r=m.encode(e),a=t(r.length);return f().subarray(a,a+r.length).set(r),d=r.length,a}for(var i=e.length,c=t(i),o=f(),s=0;s<i;s++){var l=e.charCodeAt(s);if(l>127)break;o[c+s]=l}if(s!==i){0!==s&&(e=e.slice(s)),c=n(c,i,i=s+3*e.length);var u=f().subarray(c+s,c+i);s+=g(e,u).written}return d=s,c}var j=null;function v(){return null!==j&&j.buffer===i.k.buffer||(j=new Int32Array(i.k.buffer)),j}var p=new("undefined"===typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});function O(e,t){return p.decode(f().subarray(e,e+t))}function x(e){s===c.length&&c.push(c.length+1);var t=s;return s=c[t],c[t]=e,t}p.decode();var y=32;function w(e,t){var n=t(1*e.length);return f().set(e,n/1),d=e.length,n}function E(){return l(i.r())}function I(e,t){try{return e.apply(this,t)}catch(n){i.g(x(n))}}var _=function(){function e(){Object(r.a)(this,e)}return Object(a.a)(e,[{key:"__destroy_into_raw",value:function(){var e=this.ptr;return this.ptr=0,e}},{key:"free",value:function(){var e=this.__destroy_into_raw();i.c(e)}},{key:"width",get:function(){return i.b(this.ptr)>>>0},set:function(e){i.e(this.ptr,e)}},{key:"height",get:function(){return i.a(this.ptr)>>>0},set:function(e){i.d(this.ptr,e)}},{key:"mark_seam",value:function(){i.q(this.ptr)}},{key:"delete_seam",value:function(){i.l(this.ptr)}},{key:"image_data_ptr",value:function(){return i.o(this.ptr)}},{key:"image_data_vec",value:function(){try{var e=i.f(-16);i.p(e,this.ptr);var t=v()[e/4+0],n=v()[e/4+1],r=(a=t,c=n,f().subarray(a/1,a/1+c)).slice();return i.h(t,1*n),r}finally{i.f(16)}var a,c}}],[{key:"__wrap",value:function(t){var n=Object.create(e.prototype);return n.ptr=t,n}},{key:"from_canvas",value:function(t,n,r){try{var a=i.m(function(e){if(1==y)throw new Error("out of js stack");return c[--y]=e,y}(t),n,r);return e.__wrap(a)}finally{c[y++]=void 0}}},{key:"from_vec",value:function(t,n,r){var a=w(t,i.i),c=d,o=i.n(a,c,n,r);return e.__wrap(o)}}]),e}();function N(e){l(e)}function k(){return I((function(e,t,n,r,a){return x(o(e).getImageData(t,n,r,a))}),arguments)}function S(e,t){var n=w(o(t).data,i.i),r=d;v()[e/4+1]=r,v()[e/4+0]=n}function A(){return x(new Error)}function C(e,t){var n=b(o(t).stack,i.i,i.j),r=d;v()[e/4+1]=r,v()[e/4+0]=n}function L(e,t){try{console.error(O(e,t))}finally{i.h(e,t)}}function D(e,t){var n=b(u(o(t)),i.i,i.j),r=d;v()[e/4+1]=r,v()[e/4+0]=n}function T(e,t){throw new Error(O(e,t))}function R(){return x(i.k)}}).call(this,n(14)(e))}}]);
//# sourceMappingURL=3.a51ff9df.chunk.js.map