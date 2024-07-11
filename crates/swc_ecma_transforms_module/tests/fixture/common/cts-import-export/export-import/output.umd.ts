(function(global, factory) {
    if (typeof module === "object" && typeof module.exports === "object") factory(exports);
    else if (typeof define === "function" && define.amd) define([
        "exports"
    ], factory);
    else if (global = typeof globalThis !== "undefined" ? globalThis : global || self) factory(global.input = {});
})(this, function(exports1) {
    "use strict";
    Object.defineProperty(exports1, "__esModule", {
        value: true
    });
    const foo = exports.foo = require("foo");
});
