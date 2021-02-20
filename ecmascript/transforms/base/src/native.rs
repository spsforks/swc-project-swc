//! Module to check if an identifier is native word.

use phf::phf_set;
use swc_atoms::{js_word, JsWord};

macro_rules! native {
    (
        $(
            $i:tt
        ),*
    ) => {
        pub fn is_native(sym: &str) -> bool {
            static SET: phf::Set<&'static str> = phf_set! {
                $(
                    $i,
                )*
            };

            SET.contains(sym)
        }

        /// Faster
        pub fn is_native_word(sym: &JsWord) -> bool {
            match *sym{
                $(
                    js_word!($i) => true,
                )*
                _ => false
            }
        }
    };
}

native!(
    "Array",
    "ArrayBuffer",
    "Atomics",
    "BigInt",
    "BigInt64Array",
    "BigUint64Array",
    "Boolean",
    "DataView",
    "Date",
    "Error",
    "EvalError",
    "Float32Array",
    "Float64Array",
    "Function",
    "Infinity",
    "Int16Array",
    "Int32Array",
    "Int8Array",
    "JSON",
    "Map",
    "Math",
    "NaN",
    "Number",
    "Object",
    "Promise",
    "Proxy",
    "RangeError",
    "ReferenceError",
    "Reflect",
    "RegExp",
    "Set",
    "SharedArrayBuffer",
    "String",
    "Symbol",
    "SyntaxError",
    "TypeError",
    "Uint16Array",
    "Uint32Array",
    "Uint8Array",
    "Uint8ClampedArray",
    "URIError",
    "WeakMap",
    "WeakSet",
    "AbortController",
    "AbortSignal",
    "AnalyserNode",
    "Animation",
    "AnimationEffectReadOnly",
    "AnimationEffectTiming",
    "AnimationEffectTimingReadOnly",
    "AnimationEvent",
    "AnimationPlaybackEvent",
    "AnimationTimeline",
    "ApplicationCache",
    "ApplicationCacheErrorEvent",
    "Attr",
    "Audio",
    "AudioBuffer",
    "AudioBufferSourceNode",
    "AudioContext",
    "AudioDestinationNode",
    "AudioListener",
    "AudioNode",
    "AudioParam",
    "AudioProcessingEvent",
    "AudioScheduledSourceNode",
    "AudioWorkletGlobalScope",
    "AudioWorkletNode",
    "AudioWorkletProcessor",
    "BarProp",
    "BaseAudioContext",
    "BatteryManager",
    "BeforeUnloadEvent",
    "BiquadFilterNode",
    "Blob",
    "BlobEvent",
    "BroadcastChannel",
    "BudgetService",
    "ByteLengthQueuingStrategy",
    "Cache",
    "CacheStorage",
    "CanvasCaptureMediaStreamTrack",
    "CanvasGradient",
    "CanvasPattern",
    "CanvasRenderingContext2D",
    "ChannelMergerNode",
    "ChannelSplitterNode",
    "CharacterData",
    "ClipboardEvent",
    "CloseEvent",
    "Comment",
    "CompositionEvent",
    "ConstantSourceNode",
    "ConvolverNode",
    "CountQueuingStrategy",
    "Credential",
    "CredentialsContainer",
    "Crypto",
    "CryptoKey",
    "CSS",
    "CSSConditionRule",
    "CSSFontFaceRule",
    "CSSGroupingRule",
    "CSSImportRule",
    "CSSKeyframeRule",
    "CSSKeyframesRule",
    "CSSMediaRule",
    "CSSNamespaceRule",
    "CSSPageRule",
    "CSSRule",
    "CSSRuleList",
    "CSSStyleDeclaration",
    "CSSStyleRule",
    "CSSStyleSheet",
    "CSSSupportsRule",
    "CustomElementRegistry",
    "CustomEvent",
    "DataTransfer",
    "DataTransferItem",
    "DataTransferItemList",
    "DelayNode",
    "DeviceMotionEvent",
    "DeviceOrientationEvent",
    "Document",
    "DocumentFragment",
    "DocumentType",
    "DOMError",
    "DOMException",
    "DOMImplementation",
    "DOMMatrix",
    "DOMMatrixReadOnly",
    "DOMParser",
    "DOMPoint",
    "DOMPointReadOnly",
    "DOMQuad",
    "DOMRect",
    "DOMRectReadOnly",
    "DOMStringList",
    "DOMStringMap",
    "DOMTokenList",
    "DragEvent",
    "DynamicsCompressorNode",
    "Element",
    "ErrorEvent",
    "Event",
    "EventSource",
    "EventTarget",
    "File",
    "FileList",
    "FileReader",
    "FocusEvent",
    "FontFace",
    "FontFaceSetLoadEvent",
    "FormData",
    "GainNode",
    "Gamepad",
    "GamepadButton",
    "GamepadEvent",
    "HashChangeEvent",
    "Headers",
    "History",
    "HTMLAllCollection",
    "HTMLAnchorElement",
    "HTMLAreaElement",
    "HTMLAudioElement",
    "HTMLBaseElement",
    "HTMLBodyElement",
    "HTMLBRElement",
    "HTMLButtonElement",
    "HTMLCanvasElement",
    "HTMLCollection",
    "HTMLContentElement",
    "HTMLDataElement",
    "HTMLDataListElement",
    "HTMLDetailsElement",
    "HTMLDialogElement",
    "HTMLDirectoryElement",
    "HTMLDivElement",
    "HTMLDListElement",
    "HTMLDocument",
    "HTMLElement",
    "HTMLEmbedElement",
    "HTMLFieldSetElement",
    "HTMLFontElement",
    "HTMLFormControlsCollection",
    "HTMLFormElement",
    "HTMLFrameElement",
    "HTMLFrameSetElement",
    "HTMLHeadElement",
    "HTMLHeadingElement",
    "HTMLHRElement",
    "HTMLHtmlElement",
    "HTMLIFrameElement",
    "HTMLImageElement",
    "HTMLInputElement",
    "HTMLLabelElement",
    "HTMLLegendElement",
    "HTMLLIElement",
    "HTMLLinkElement",
    "HTMLMapElement",
    "HTMLMarqueeElement",
    "HTMLMediaElement",
    "HTMLMenuElement",
    "HTMLMetaElement",
    "HTMLMeterElement",
    "HTMLModElement",
    "HTMLObjectElement",
    "HTMLOListElement",
    "HTMLOptGroupElement",
    "HTMLOptionElement",
    "HTMLOptionsCollection",
    "HTMLOutputElement",
    "HTMLParagraphElement",
    "HTMLParamElement",
    "HTMLPictureElement",
    "HTMLPreElement",
    "HTMLProgressElement",
    "HTMLQuoteElement",
    "HTMLScriptElement",
    "HTMLSelectElement",
    "HTMLShadowElement",
    "HTMLSlotElement",
    "HTMLSourceElement",
    "HTMLSpanElement",
    "HTMLStyleElement",
    "HTMLTableCaptionElement",
    "HTMLTableCellElement",
    "HTMLTableColElement",
    "HTMLTableElement",
    "HTMLTableRowElement",
    "HTMLTableSectionElement",
    "HTMLTemplateElement",
    "HTMLTextAreaElement",
    "HTMLTimeElement",
    "HTMLTitleElement",
    "HTMLTrackElement",
    "HTMLUListElement",
    "HTMLUnknownElement",
    "HTMLVideoElement",
    "IDBCursor",
    "IDBCursorWithValue",
    "IDBDatabase",
    "IDBFactory",
    "IDBIndex",
    "IDBKeyRange",
    "IDBObjectStore",
    "IDBOpenDBRequest",
    "IDBRequest",
    "IDBTransaction",
    "IDBVersionChangeEvent",
    "IdleDeadline",
    "IIRFilterNode",
    "Image",
    "ImageBitmap",
    "ImageBitmapRenderingContext",
    "ImageCapture",
    "ImageData",
    "InputEvent",
    "IntersectionObserver",
    "IntersectionObserverEntry",
    "Intl",
    "KeyboardEvent",
    "KeyframeEffect",
    "KeyframeEffectReadOnly",
    "Location",
    "MediaDeviceInfo",
    "MediaDevices",
    "MediaElementAudioSourceNode",
    "MediaEncryptedEvent",
    "MediaError",
    "MediaKeyMessageEvent",
    "MediaKeySession",
    "MediaKeyStatusMap",
    "MediaKeySystemAccess",
    "MediaList",
    "MediaQueryList",
    "MediaQueryListEvent",
    "MediaRecorder",
    "MediaSettingsRange",
    "MediaSource",
    "MediaStream",
    "MediaStreamAudioDestinationNode",
    "MediaStreamAudioSourceNode",
    "MediaStreamEvent",
    "MediaStreamTrack",
    "MediaStreamTrackEvent",
    "MessageChannel",
    "MessageEvent",
    "MessagePort",
    "MIDIAccess",
    "MIDIConnectionEvent",
    "MIDIInput",
    "MIDIInputMap",
    "MIDIMessageEvent",
    "MIDIOutput",
    "MIDIOutputMap",
    "MIDIPort",
    "MimeType",
    "MimeTypeArray",
    "MouseEvent",
    "MutationEvent",
    "MutationObserver",
    "MutationRecord",
    "NamedNodeMap",
    "NavigationPreloadManager",
    "Navigator",
    "NetworkInformation",
    "Node",
    "NodeFilter",
    "NodeIterator",
    "NodeList",
    "Notification",
    "OfflineAudioCompletionEvent",
    "OfflineAudioContext",
    "OffscreenCanvas",
    "Option",
    "OscillatorNode",
    "PageTransitionEvent",
    "PannerNode",
    "Path2D",
    "PaymentAddress",
    "PaymentRequest",
    "PaymentRequestUpdateEvent",
    "PaymentResponse",
    "Performance",
    "PerformanceEntry",
    "PerformanceLongTaskTiming",
    "PerformanceMark",
    "PerformanceMeasure",
    "PerformanceNavigation",
    "PerformanceNavigationTiming",
    "PerformanceObserver",
    "PerformanceObserverEntryList",
    "PerformancePaintTiming",
    "PerformanceResourceTiming",
    "PerformanceTiming",
    "PeriodicWave",
    "Permissions",
    "PermissionStatus",
    "PhotoCapabilities",
    "Plugin",
    "PluginArray",
    "PointerEvent",
    "PopStateEvent",
    "Presentation",
    "PresentationAvailability",
    "PresentationConnection",
    "PresentationConnectionAvailableEvent",
    "PresentationConnectionCloseEvent",
    "PresentationConnectionList",
    "PresentationReceiver",
    "PresentationRequest",
    "ProcessingInstruction",
    "ProgressEvent",
    "PromiseRejectionEvent",
    "PushManager",
    "PushSubscription",
    "PushSubscriptionOptions",
    "RadioNodeList",
    "Range",
    "ReadableStream",
    "RemotePlayback",
    "Request",
    "ResizeObserver",
    "ResizeObserverEntry",
    "Response",
    "RTCCertificate",
    "RTCDataChannel",
    "RTCDataChannelEvent",
    "RTCDtlsTransport",
    "RTCIceCandidate",
    "RTCIceGatherer",
    "RTCIceTransport",
    "RTCPeerConnection",
    "RTCPeerConnectionIceEvent",
    "RTCRtpContributingSource",
    "RTCRtpReceiver",
    "RTCRtpSender",
    "RTCSctpTransport",
    "RTCSessionDescription",
    "RTCStatsReport",
    "RTCTrackEvent",
    "Screen",
    "ScreenOrientation",
    "ScriptProcessorNode",
    "SecurityPolicyViolationEvent",
    "Selection",
    "ServiceWorker",
    "ServiceWorkerContainer",
    "ServiceWorkerRegistration",
    "ShadowRoot",
    "SharedWorker",
    "SourceBuffer",
    "SourceBufferList",
    "SpeechSynthesisEvent",
    "SpeechSynthesisUtterance",
    "StaticRange",
    "StereoPannerNode",
    "Storage",
    "StorageEvent",
    "StorageManager",
    "StyleSheet",
    "StyleSheetList",
    "SubtleCrypto",
    "SVGAElement",
    "SVGAngle",
    "SVGAnimatedAngle",
    "SVGAnimatedBoolean",
    "SVGAnimatedEnumeration",
    "SVGAnimatedInteger",
    "SVGAnimatedLength",
    "SVGAnimatedLengthList",
    "SVGAnimatedNumber",
    "SVGAnimatedNumberList",
    "SVGAnimatedPreserveAspectRatio",
    "SVGAnimatedRect",
    "SVGAnimatedString",
    "SVGAnimatedTransformList",
    "SVGAnimateElement",
    "SVGAnimateMotionElement",
    "SVGAnimateTransformElement",
    "SVGAnimationElement",
    "SVGCircleElement",
    "SVGClipPathElement",
    "SVGComponentTransferFunctionElement",
    "SVGDefsElement",
    "SVGDescElement",
    "SVGDiscardElement",
    "SVGElement",
    "SVGEllipseElement",
    "SVGFEBlendElement",
    "SVGFEColorMatrixElement",
    "SVGFEComponentTransferElement",
    "SVGFECompositeElement",
    "SVGFEConvolveMatrixElement",
    "SVGFEDiffuseLightingElement",
    "SVGFEDisplacementMapElement",
    "SVGFEDistantLightElement",
    "SVGFEDropShadowElement",
    "SVGFEFloodElement",
    "SVGFEFuncAElement",
    "SVGFEFuncBElement",
    "SVGFEFuncGElement",
    "SVGFEFuncRElement",
    "SVGFEGaussianBlurElement",
    "SVGFEImageElement",
    "SVGFEMergeElement",
    "SVGFEMergeNodeElement",
    "SVGFEMorphologyElement",
    "SVGFEOffsetElement",
    "SVGFEPointLightElement",
    "SVGFESpecularLightingElement",
    "SVGFESpotLightElement",
    "SVGFETileElement",
    "SVGFETurbulenceElement",
    "SVGFilterElement",
    "SVGForeignObjectElement",
    "SVGGElement",
    "SVGGeometryElement",
    "SVGGradientElement",
    "SVGGraphicsElement",
    "SVGImageElement",
    "SVGLength",
    "SVGLengthList",
    "SVGLinearGradientElement",
    "SVGLineElement",
    "SVGMarkerElement",
    "SVGMaskElement",
    "SVGMatrix",
    "SVGMetadataElement",
    "SVGMPathElement",
    "SVGNumber",
    "SVGNumberList",
    "SVGPathElement",
    "SVGPatternElement",
    "SVGPoint",
    "SVGPointList",
    "SVGPolygonElement",
    "SVGPolylineElement",
    "SVGPreserveAspectRatio",
    "SVGRadialGradientElement",
    "SVGRect",
    "SVGRectElement",
    "SVGScriptElement",
    "SVGSetElement",
    "SVGStopElement",
    "SVGStringList",
    "SVGStyleElement",
    "SVGSVGElement",
    "SVGSwitchElement",
    "SVGSymbolElement",
    "SVGTextContentElement",
    "SVGTextElement",
    "SVGTextPathElement",
    "SVGTextPositioningElement",
    "SVGTitleElement",
    "SVGTransform",
    "SVGTransformList",
    "SVGTSpanElement",
    "SVGUnitTypes",
    "SVGUseElement",
    "SVGViewElement",
    "TaskAttributionTiming",
    "Text",
    "TextDecoder",
    "TextEncoder",
    "TextEvent",
    "TextMetrics",
    "TextTrack",
    "TextTrackCue",
    "TextTrackCueList",
    "TextTrackList",
    "TimeRanges",
    "Touch",
    "TouchEvent",
    "TouchList",
    "TrackEvent",
    "TransitionEvent",
    "TreeWalker",
    "UIEvent",
    "URL",
    "URLSearchParams",
    "ValidityState",
    "VisualViewport",
    "VTTCue",
    "WaveShaperNode",
    "WebAssembly",
    "WebGL2RenderingContext",
    "WebGLActiveInfo",
    "WebGLBuffer",
    "WebGLContextEvent",
    "WebGLFramebuffer",
    "WebGLProgram",
    "WebGLQuery",
    "WebGLRenderbuffer",
    "WebGLRenderingContext",
    "WebGLSampler",
    "WebGLShader",
    "WebGLShaderPrecisionFormat",
    "WebGLSync",
    "WebGLTexture",
    "WebGLTransformFeedback",
    "WebGLUniformLocation",
    "WebGLVertexArrayObject",
    "WebSocket",
    "WheelEvent",
    "Window",
    "Worker",
    "WritableStream",
    "XMLDocument",
    "XMLHttpRequest",
    "XMLHttpRequestEventTarget",
    "XMLHttpRequestUpload",
    "XMLSerializer",
    "XPathEvaluator",
    "XPathExpression",
    "XPathResult",
    "XSLTProcessor"
);
