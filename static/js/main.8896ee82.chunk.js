(this["webpackJsonpcontent-aware-image-resizer"]=this["webpackJsonpcontent-aware-image-resizer"]||[]).push([[0],{7:function(e,t,n){},8:function(e,t,n){"use strict";n.r(t);var r=n(5),c=n(3),i=n(4);var a=n(1);function s(e,t){(function(e){var t=new Image;return t.crossOrigin="Anonymous",t.src=e,new Promise((function(e,n){t.onload=function(){e(t)},t.onerror=function(e){n(e)}}))})(e).then((function(e){t.width=e.width,t.height=e.height;var n=t.getContext("2d");n.drawImage(e,0,0,e.width,e.height,0,0,t.width,t.height);var r=n.getImageData(0,0,t.width,t.height);console.log(r)}))}var o=function(e){var t=Object(c.c)(null);return Object(c.a)((function(){t&&s(e.src,t.current)}),[t]),Object(a.jsx)("canvas",{id:e.id,ref:t})};var d=function(e){var t=e.currentDisplay,n="SOURCE"===t?"":"hidden",r="TARGET"===t?"":"hidden";return Object(a.jsxs)("div",{className:"border-8 border-gray-600 border-opacity-5",children:[Object(a.jsx)("div",{className:n,children:Object(a.jsx)(o,{id:"canvas-source",src:e.src})}),Object(a.jsx)("div",{className:r,children:Object(a.jsx)("canvas",{id:"canvas-target"})})]})};var h,l=function(e){var t=e.handleResize;return Object(a.jsx)("div",{className:"Controls",children:Object(a.jsx)("button",{className:"bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",onClick:t,children:"Resize to 50%"})})},u=(n(7),{display:"SOURCE"});function j(e,t){switch(t.type){case"RESIZE":return{display:"TARGET"};default:throw new Error("Unknown action type in reducer.")}}function b(){var e=Object(c.b)(j,u),t=Object(r.a)(e,2),i=t[0],s=t[1];return Object(a.jsxs)("div",{className:"App flex flex-col h-screen",children:[Object(a.jsx)("div",{className:"flex-grow",children:Object(a.jsx)("div",{className:"flex items-center\tjustify-center h-full",children:Object(a.jsx)(d,{src:"https://source.unsplash.com/yRjLihK35Yw/800x450",currentDisplay:i.display})})}),Object(a.jsx)("div",{className:"border-t border-gray-150 p-10 bg-white",children:Object(a.jsx)(l,{handleResize:function(){var e=document.getElementById("canvas-source"),t=document.getElementById("canvas-target");t.width=e.width,t.height=e.height;var r=e.width-2,c=e.height;!function(e,t,r,c,i){var a=e.source,s=e.target;!function(){for(var e=arguments.length,t=new Array(e),r=0;r<e;r++)t[r]=arguments[r];n.e(3).then(n.bind(null,12)).then((function(e){return e.resize.apply(e,t)})).catch(console.error)}(a.getContext("2d"),s.getContext("2d"),t,r,c,i)}({source:e,target:t},e.width,e.height,r,c),s({type:"RESIZE"})}})})]})}h=function(){i.a.render(Object(a.jsx)(b,{}),document.getElementById("app"))},"loading"!=document.readyState?h():document.addEventListener("DOMContentLoaded",h)}},[[8,1,2]]]);
//# sourceMappingURL=main.8896ee82.chunk.js.map