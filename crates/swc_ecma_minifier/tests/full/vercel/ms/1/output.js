export default function($,c){try{if("string"==typeof $&&$.length>0)return function(e){if((e=String(e)).length>100)throw Error("Value exceeds the maximum length of 100 characters.");const s=/^(-?(?:\d+)?\.?\d+) *(milliseconds?|msecs?|ms|seconds?|secs?|s|minutes?|mins?|m|hours?|hrs?|h|days?|d|weeks?|w|years?|yrs?|y)?$/i.exec(e);if(!s)return NaN;const $=parseFloat(s[1]),n=(s[2]||"ms").toLowerCase();switch(n){case"years":case"year":case"yrs":case"yr":case"y":return 315576e5*$;case"weeks":case"week":case"w":return 6048e5*$;case"days":case"day":case"d":return 864e5*$;case"hours":case"hour":case"hrs":case"hr":case"h":return 36e5*$;case"minutes":case"minute":case"mins":case"min":case"m":return 6e4*$;case"seconds":case"second":case"secs":case"sec":case"s":return 1e3*$;case"milliseconds":case"millisecond":case"msecs":case"msec":case"ms":return $;default:throw Error(`The unit ${n} was matched, but no matching case exists.`)}}($);if("number"==typeof $&&isFinite($))return c?.long?s($):e($);throw Error("Value is not a string or number.")}catch(r){const a=n(r)?`${r.message}. value=${JSON.stringify($)}`:"An unknown error has occured.";throw Error(a)}};function e(e){const s=Math.abs(e);return s>=864e5?`${Math.round(e/864e5)}d`:s>=36e5?`${Math.round(e/36e5)}h`:s>=6e4?`${Math.round(e/6e4)}m`:s>=1e3?`${Math.round(e/1e3)}s`:`${e}ms`}function s(e){const s=Math.abs(e);return s>=864e5?$(e,s,864e5,"day"):s>=36e5?$(e,s,36e5,"hour"):s>=6e4?$(e,s,6e4,"minute"):s>=1e3?$(e,s,1e3,"second"):`${e} ms`}function $(e,s,$,n){return`${Math.round(e/$)} ${n}${s>=1.5*$?"s":""}`}function n(e){return"object"==typeof e&&null!==e&&"message"in e}
