function t(){}function e(t){return t()}function n(){return Object.create(null)}function o(t){t.forEach(e)}function r(t){return"function"==typeof t}function c(t,e){return t!=t?e==e:t!==e||t&&"object"==typeof t||"function"==typeof t}function s(e){return e&&r(e.destroy)?e.destroy:t}function u(t,e){t.appendChild(e)}function l(t,e,n){t.insertBefore(e,n||null)}function i(t){t.parentNode.removeChild(t)}function a(t){return document.createElement(t)}function f(t){return document.createTextNode(t)}function d(){return f(" ")}function h(t,e,n,o){return t.addEventListener(e,n,o),()=>t.removeEventListener(e,n,o)}function p(t,e,n){null==n?t.removeAttribute(e):t.getAttribute(e)!==n&&t.setAttribute(e,n)}function m(t,e){e=""+e,t.wholeText!==e&&(t.data=e)}function g(t,e,n){t.classList[n?"add":"remove"](e)}let $;function _(t){$=t}const v=[],y=[],b=[],x=[],w=Promise.resolve();let k=!1;function E(t){b.push(t)}const N=new Set;let A=0;function L(){const t=$;do{for(;A<v.length;){const t=v[A];A++,_(t),j(t.$$)}for(_(null),v.length=0,A=0;y.length;)y.pop()();for(let t=0;t<b.length;t+=1){const e=b[t];N.has(e)||(N.add(e),e())}b.length=0}while(v.length);for(;x.length;)x.pop()();k=!1,N.clear(),_(t)}function j(t){if(null!==t.fragment){t.update(),o(t.before_update);const e=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,e),t.after_update.forEach(E)}}const S=new Set;function B(t,e){-1===t.$$.dirty[0]&&(v.push(t),k||(k=!0,w.then(L)),t.$$.dirty.fill(0)),t.$$.dirty[e/31|0]|=1<<e%31}function O(c,s,u,l,a,f,d,h=[-1]){const p=$;_(c);const m=c.$$={fragment:null,ctx:null,props:f,update:t,not_equal:a,bound:n(),on_mount:[],on_destroy:[],on_disconnect:[],before_update:[],after_update:[],context:new Map(s.context||(p?p.$$.context:[])),callbacks:n(),dirty:h,skip_bound:!1,root:s.target||p.$$.root};d&&d(m.root);let g=!1;if(m.ctx=u?u(c,s.props||{},((t,e,...n)=>{const o=n.length?n[0]:e;return m.ctx&&a(m.ctx[t],m.ctx[t]=o)&&(!m.skip_bound&&m.bound[t]&&m.bound[t](o),g&&B(c,t)),e})):[],m.update(),g=!0,o(m.before_update),m.fragment=!!l&&l(m.ctx),s.target){if(s.hydrate){const t=function(t){return Array.from(t.childNodes)}(s.target);m.fragment&&m.fragment.l(t),t.forEach(i)}else m.fragment&&m.fragment.c();s.intro&&((v=c.$$.fragment)&&v.i&&(S.delete(v),v.i(y))),function(t,n,c,s){const{fragment:u,on_mount:l,on_destroy:i,after_update:a}=t.$$;u&&u.m(n,c),s||E((()=>{const n=l.map(e).filter(r);i?i.push(...n):o(n),t.$$.on_mount=[]})),a.forEach(E)}(c,s.target,s.anchor,s.customElement),L()}var v,y;_(p)}function T(t,e,n){const o=t.slice();return o[11]=e[n],o[13]=n,o}function q(t){let e,n,r,c,f,m,g=t[0],$=[];for(let e=0;e<g.length;e+=1)$[e]=C(T(t,g,e));return{c(){e=a("div"),n=a("div"),r=a("input"),c=d();for(let t=0;t<$.length;t+=1)$[t].c();p(r,"class","search__field"),p(r,"type","search"),p(r,"placeholder","Search composers by last name"),p(n,"class","search"),p(e,"class","search-wrapper")},m(o,i){l(o,e,i),u(e,n),u(n,r),u(n,c);for(let t=0;t<$.length;t+=1)$[t].m(n,null);f||(m=[h(r,"input",t[6]),s(P.call(null,r)),s(D.call(null,n,t[4])),h(e,"keydown",t[5])],f=!0)},p(t,e){if(133&e){let o;for(g=t[0],o=0;o<g.length;o+=1){const r=T(t,g,o);$[o]?$[o].p(r,e):($[o]=C(r),$[o].c(),$[o].m(n,null))}for(;o<$.length;o+=1)$[o].d(1);$.length=g.length}},d(t){t&&i(e),function(t,e){for(let n=0;n<t.length;n+=1)t[n]&&t[n].d(e)}($,t),f=!1,o(m)}}}function C(t){let e,n,o,r,c,s,$,_,v,y=t[11].lastName+"",b=t[11].firstName+"";function x(){return t[8](t[13])}return{c(){e=a("a"),n=a("div"),o=f(y),r=f(", "),c=f(b),s=d(),p(n,"class","search__result"),g(n,"search__result_selected",t[2]===t[13]),p(e,"href",$="/composer/"+t[11].slug)},m(t,i){l(t,e,i),u(e,n),u(n,o),u(n,r),u(n,c),u(e,s),_||(v=h(e,"mouseenter",x),_=!0)},p(r,s){t=r,1&s&&y!==(y=t[11].lastName+"")&&m(o,y),1&s&&b!==(b=t[11].firstName+"")&&m(c,b),4&s&&g(n,"search__result_selected",t[2]===t[13]),1&s&&$!==($="/composer/"+t[11].slug)&&p(e,"href",$)},d(t){t&&i(e),_=!1,v()}}}function M(e){let n,o,r,c,s,u=e[1]&&q(e);return{c(){n=a("div"),n.innerHTML='<img src="/static/img/search-icon.svg" alt="Search"/>',o=d(),u&&u.c(),r=f(""),p(n,"class","search-button")},m(t,i){l(t,n,i),l(t,o,i),u&&u.m(t,i),l(t,r,i),c||(s=h(n,"click",e[3]),c=!0)},p(t,[e]){t[1]?u?u.p(t,e):(u=q(t),u.c(),u.m(r.parentNode,r)):u&&(u.d(1),u=null)},i:t,o:t,d(t){t&&i(n),t&&i(o),u&&u.d(t),t&&i(r),c=!1,s()}}}function P(t){t.focus()}function D(t,e){const n=n=>t&&!t.contains(n.target)&&!n.defaultPrevented&&e();return document.addEventListener("click",n,!0),{destroy(){document.removeEventListener("click",n,!0)}}}function H(t,e,n){let o=[],r=[void 0,void 0],c=!1,s=0;async function u(){void 0!==r[0]?(n(0,o=await async function(t){try{const e=await fetch(`/api/search?q=${t}`);return e.ok?await e.json():[]}catch(t){return console.log(t),[]}}(r[0])),void 0!==r[1]?(r[0]=r[1],r[1]=void 0,await u()):r[0]=void 0):n(0,o=[])}function l(){n(0,o=[]),n(1,c=!1)}function i(t){n(2,s=t)}return[o,c,s,function(){n(1,c=!0)},l,function(t){"ArrowUp"===t.code&&o.length>0?n(2,s=s>0?s-1:o.length-1):"ArrowDown"===t.code?n(2,s=s<o.length-1?s+1:n(2,s=0)):"Escape"===t.code?l():"Enter"===t.code&&o.length>0&&(location.pathname=`/composer/${o[s].slug}`)},function(t){const e=t.target.value||void 0;void 0===r[0]?(r[0]=e,u()):r[1]=e},i,t=>i(t)]}const I=document.getElementById("searchBlock"),U=I?new class extends class{$destroy(){!function(t,e){const n=t.$$;null!==n.fragment&&(o(n.on_destroy),n.fragment&&n.fragment.d(e),n.on_destroy=n.fragment=null,n.ctx=[])}(this,1),this.$destroy=t}$on(t,e){const n=this.$$.callbacks[t]||(this.$$.callbacks[t]=[]);return n.push(e),()=>{const t=n.indexOf(e);-1!==t&&n.splice(t,1)}}$set(t){var e;this.$$set&&(e=t,0!==Object.keys(e).length)&&(this.$$.skip_bound=!0,this.$$set(t),this.$$.skip_bound=!1)}}{constructor(t){super(),O(this,t,H,M,c,{})}}({target:I}):void 0;export{U as default};
//# sourceMappingURL=bundle.js.map
