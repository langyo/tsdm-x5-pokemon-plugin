"use components";
import { environment, exit as exit$1, stderr, stdin, stdout, terminalInput, terminalOutput, terminalStderr, terminalStdin, terminalStdout } from "@bytecodealliance/preview2-shim/cli";
import { monotonicClock } from "@bytecodealliance/preview2-shim/clocks";
import { error, poll as poll$1, streams } from "@bytecodealliance/preview2-shim/io";
import { insecureSeed as insecureSeed$1 } from "@bytecodealliance/preview2-shim/random";
import { getPropertyValue, setProperty } from "tairitsu-browser:full/css-style-declaration";
import { createElement, createTextNode, getBody } from "tairitsu-browser:full/document";
import { add, contains, remove } from "tairitsu-browser:full/dom-token-list";
import { getAttribute, getBoundingClientRect, getClassList, getClientHeight, getScrollHeight, getScrollTop, removeAttribute, setAttribute, setInnerHtml, setScrollTop } from "tairitsu-browser:full/element";
import { getStyle } from "tairitsu-browser:full/element-css-inline-style";
import { getCurrentTarget, getTarget } from "tairitsu-browser:full/event";
import { addEventListener, preventDefault, removeEventListener, stopPropagation } from "tairitsu-browser:full/event-target";
import { getValue } from "tairitsu-browser:full/html-input-element";
import { getAltKey, getButton, getButtons, getClientX, getClientY, getCtrlKey, getMetaKey, getOffsetX, getOffsetY, getPageX, getPageY, getScreenX, getScreenY, getShiftKey } from "tairitsu-browser:full/mouse-event";
import { appendChild, getChildNodes, getFirstChild, getParentElement, insertBefore, removeChild, setTextContent } from "tairitsu-browser:full/node";
import { getLength, item } from "tairitsu-browser:full/node-list";
import { getElementById } from "tairitsu-browser:full/non-element-parent-node";
import { querySelector } from "tairitsu-browser:full/parent-node";
import { cancelAnimationFrame, clearInterval, clearTimeout, fetchPromise, requestAnimationFrame, setInterval, setTimeout } from "tairitsu-browser:full/platform-helpers";
import { getHeight, getIsPrimary, getPointerId, getPointerType, getPressure, getTangentialPressure, getTiltX, getTiltY, getTwist, getWidth } from "tairitsu-browser:full/pointer-event";
import { getDeltaMode, getDeltaX, getDeltaY, getDeltaZ } from "tairitsu-browser:full/wheel-event";
import { getComputedStyle, getInnerWidth } from "tairitsu-browser:full/window";
const { getEnvironment } = environment;

if (getEnvironment=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getEnvironment', was 'getEnvironment' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { exit } = exit$1;

if (exit=== undefined) {
  const err = new Error("unexpectedly undefined local import 'exit', was 'exit' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { getStderr } = stderr;

if (getStderr=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getStderr', was 'getStderr' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { getStdin } = stdin;

if (getStdin=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getStdin', was 'getStdin' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { getStdout } = stdout;

if (getStdout=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getStdout', was 'getStdout' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { TerminalInput } = terminalInput;

if (TerminalInput=== undefined) {
  const err = new Error("unexpectedly undefined local import 'TerminalInput', was 'TerminalInput' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { TerminalOutput } = terminalOutput;

if (TerminalOutput=== undefined) {
  const err = new Error("unexpectedly undefined local import 'TerminalOutput', was 'TerminalOutput' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { getTerminalStderr } = terminalStderr;

if (getTerminalStderr=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getTerminalStderr', was 'getTerminalStderr' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { getTerminalStdin } = terminalStdin;

if (getTerminalStdin=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getTerminalStdin', was 'getTerminalStdin' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { getTerminalStdout } = terminalStdout;

if (getTerminalStdout=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getTerminalStdout', was 'getTerminalStdout' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { subscribeDuration } = monotonicClock;

if (subscribeDuration=== undefined) {
  const err = new Error("unexpectedly undefined local import 'subscribeDuration', was 'subscribeDuration' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { Error: Error$1 } = error;

if (Error$1=== undefined) {
  const err = new Error("unexpectedly undefined local import 'Error$1', was 'Error' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { Pollable,
  poll } = poll$1;

if (Pollable=== undefined) {
  const err = new Error("unexpectedly undefined local import 'Pollable', was 'Pollable' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}


if (poll=== undefined) {
  const err = new Error("unexpectedly undefined local import 'poll', was 'poll' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { InputStream,
  OutputStream } = streams;

if (InputStream=== undefined) {
  const err = new Error("unexpectedly undefined local import 'InputStream', was 'InputStream' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}


if (OutputStream=== undefined) {
  const err = new Error("unexpectedly undefined local import 'OutputStream', was 'OutputStream' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { insecureSeed } = insecureSeed$1;

if (insecureSeed=== undefined) {
  const err = new Error("unexpectedly undefined local import 'insecureSeed', was 'insecureSeed' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}


let dv = new DataView(new ArrayBuffer());
const dataView = mem => dv.buffer === mem.buffer ? dv : dv = new DataView(mem.buffer);

function toUint64(val) {
  const converted = BigInt(val)
  
  return BigInt.asUintN(64, converted);
}


function toInt16(val) {
  
  val >>>= 0;
  val %= 2 ** 16;
  if (val >= 2 ** 15) {
    val -= 2 ** 16;
  }
  return val;
}


function toInt32(val) {
  
  return val >> 0;
}


function toUint16(val) {
  
  val >>>= 0;
  val %= 2 ** 16;
  return val;
}


function toUint32(val) {
  
  return val >>> 0;
}


function toUint8(val) {
  
  val >>>= 0;
  val %= 2 ** 8;
  return val;
}


function _isValidNumericPrimitive(ty, v) {
  if (v === undefined || v === null) { return false; }
  switch (ty) {
    case 'bool':
    return v === 0 || v === 1;
    break;
    case 'u8':
    return typeof v === 'number' && Number.isInteger(v) && v >= 0 && v <= 255;
    break;
    case 's8':
    return typeof v === 'number' && Number.isInteger(v) && v >= -128 && v <= 127;
    break;
    case 'u16':
    return typeof v === 'number' && Number.isInteger(v) && v >= 0 && v <= 65535;
    break;
    case 's16':
    return typeof v === 'number' && Number.isInteger(v) && v >= -32768 && v <= 32767;
    case 'u32':
    return typeof v === 'number' && Number.isInteger(v) && v >= 0 && v <= 4_294_967_295;
    case 's32':
    return typeof v === 'number' && Number.isInteger(v) && v >= -2_147_483_648 && v <= 2_147_483_647;
    case 'u64':
    return typeof v === 'bigint' && v >= 0 && v <= 18_446_744_073_709_551_615n;
    case 's64':
    return typeof v === 'bigint' && v >= -9223372036854775808n && v <= 9223372036854775807n;
    break;
    case 'f32':
    case 'f64': return typeof v === 'number';
    default:
    return false;
  }
  return true;
}

function _requireValidNumericPrimitive(ty, v) {
  if (v === undefined  || v === null || !_isValidNumericPrimitive(ty, v)) {
    throw new TypeError(`invalid ${ty} value [${v}]`);
  }
  return true;
}
const utf16Decoder = new TextDecoder('utf-16', { fatal: true, ignoreBOM: true });

const isLE = new Uint8Array(new Uint16Array([1]).buffer)[0] === 1;

function _utf16AllocateAndEncode(str, realloc, memory) {
  if (typeof str !== 'string') {
    throw new TypeError('expected a string, received [' + typeof str + ']');
  }
  const len = str.length;
  const ptr = realloc(0, 0, 2, len * 2);
  const out = new Uint16Array(memory.buffer, ptr, len);
  const put = isLE
  ? (i, ch) => { out[i] = ch; }
  : (i, ch) => { out[i] = (ch & 0xff) << 8 | ch >>> 8; };
  for (let i = 0; i < len; i++) {
    let ch = str.charCodeAt(i);
    if ((ch & 0xf800) === 0xd800) {
      if (ch < 0xdc00 && i + 1 < len && (str.charCodeAt(i + 1) & 0xfc00) === 0xdc00) {
        put(i++, ch);
        ch = str.charCodeAt(i);
      } else {
        // Unpaired surrogates are replaced, as when converting to a `USVString`
        ch = 0xfffd;
      }
    }
    put(i, ch);
  }
  return { ptr, len, codepoints: [...str].length };
}


function _compactUtf16Decode(memory, ptr, taggedLength) {
  ptr >>>= 0;
  taggedLength >>>= 0;
  if (ptr % 2 !== 0) { throw new RangeError('unaligned compact UTF-16 string'); }
  if (taggedLength & 0x80000000) {
    return utf16Decoder.decode(new Uint8Array(memory.buffer, ptr, (taggedLength & 0x7fffffff) * 2));
  }
  const bytes = new Uint8Array(memory.buffer, ptr, taggedLength);
  let value = '';
  // TextDecoder('latin1') uses Windows-1252, which changes
  // bytes 0x80..0x9f. The ABI maps every byte directly.
  for (let i = 0; i < bytes.length; i += 0x8000) {
    value += String.fromCharCode(...bytes.subarray(i, i + 0x8000));
  }
  return value;
}


function _compactUtf16AllocateAndEncode(str, realloc, memory) {
  if (typeof str !== 'string') { throw new TypeError('expected a string'); }
  if (str.length >= 0x80000000) { throw new RangeError('compact UTF-16 string is too long'); }
  for (let i = 0; i < str.length; i++) {
    if (str.charCodeAt(i) > 0xff) {
      const encoded = _utf16AllocateAndEncode(str, realloc, memory);
      return { ...encoded, len: (encoded.len | 0x80000000) >>> 0 };
    }
  }
  const ptr = realloc(0, 0, 2, str.length) >>> 0;
  if (ptr % 2 !== 0) { throw new RangeError('unaligned compact UTF-16 allocation'); }
  const bytes = new Uint8Array(memory.buffer, ptr, str.length);
  for (let i = 0; i < str.length; i++) { bytes[i] = str.charCodeAt(i); }
  return { ptr, len: str.length };
}

const TEXT_DECODER_UTF8 = new TextDecoder('utf-8', { fatal: true, ignoreBOM: true });
const TEXT_ENCODER_UTF8 = new TextEncoder();

function _utf8AllocateAndEncode(s, realloc, memory) {
  if (typeof s !== 'string') {
    throw new TypeError('expected a string, received [' + typeof s + ']');
  }
  if (s.length === 0) { return { ptr: 1, len: 0 }; }
  // Compute the exact allocation size up front. Some older preview1
  // adapters only support an initial allocation, not a subsequent shrink.
  let len = 0;
  let codepoints = 0;
  for (let i = 0; i < s.length; i++) {
    const ch = s.charCodeAt(i);
    codepoints++;
    if (ch < 0x80) { len += 1; }
    else if (ch < 0x800) { len += 2; }
    else if (ch >= 0xd800 && ch <= 0xdbff &&
    i + 1 < s.length &&
    (s.charCodeAt(i + 1) & 0xfc00) === 0xdc00) {
      len += 4;
      i++;
    } else { len += 3; }
  }
  const ptr = realloc(0, 0, 1, len);
  const { read, written } = TEXT_ENCODER_UTF8.encodeInto(
  s,
  new Uint8Array(memory.buffer, ptr, len),
  );
  if (read !== s.length || written !== len) {
    throw new Error('failed to encode whole string');
  }
  const res = { ptr, len, codepoints };
  return res;
}

const T_FLAG = 1 << 30;
const T_OWN = 2 ** 32;
const HANDLE_INDEX_SPACES = new Map();

function getHandleIndexSpace(key) {
  let space = HANDLE_INDEX_SPACES.get(key);
  if (space === undefined) {
    // One index space per component instance (Canonical ABI
    // `inst.handles`): resource handles, waitables, waitable sets and
    // error contexts share it. Freed indices are reused LIFO, as
    // `Table.add` does. Index 0 is never a handle.
    space = {
      next: 1,
      free: [],
      alloc() { return this.free.length > 0 ? this.free.pop() : this.next++; },
      release(idx) { this.free.push(idx); },
    };
    HANDLE_INDEX_SPACES.set(key, space);
  }
  return space;
}


function rscTableCreateOwn(table, rep) {
  // Reps are u32: one from a core function arrives as a signed i32
  rep >>>= 0;
  table._createdReps.add(rep);
  const handle = getHandleIndexSpace(table._componentIdx ?? table).alloc();
  table[handle << 1] = 0;
  table[(handle << 1) + 1] = rep + T_OWN;
  return handle;
}

const RESOURCE_SCOPE_TASKS = new Map();
const WebAssemblyRuntimeError = WebAssembly.RuntimeError;

function rscTableRemove(table, handle) {
  const scope = table[handle << 1];
  const val = table[(handle << 1) + 1];
  const own = val >= T_OWN;
  const rep = own ? val - T_OWN: val;
  // A borrowed entry can store zero without being freed.
  if (val === undefined || (scope & T_FLAG) !== 0) {
    throw new WebAssemblyRuntimeError(`unknown handle index ${handle}`);
  }
  if (own && scope !== 0) {
    throw new WebAssemblyRuntimeError('cannot remove owned resource while borrowed');
  }
  const borrowTask = own ? undefined : RESOURCE_SCOPE_TASKS.get(scope);
  // Mark the slot freed and give its index back to the instance's space.
  table[handle << 1] = T_FLAG;
  table[(handle << 1) + 1] = 0;
  getHandleIndexSpace(table._componentIdx ?? table).release(handle);
  borrowTask?.removeBorrowedHandle();
  return { rep, scope, own };
}

let RESOURCE_SCOPE_ID = 0;

let curResourceBorrows = [];
const ASYNC_TASKS_BY_COMPONENT_IDX = new Map();
const ASYNC_CURRENT_COMPONENT_IDXS = [];
const CURRENT_TASK_META = {};

function getCurrentTask(componentIdx, taskID) {
  let usedGlobal = false;
  if (componentIdx === undefined || componentIdx === null) {
    throw new Error('missing component idx'); // TODO(fix)
    // componentIdx = ASYNC_CURRENT_COMPONENT_IDXS.at(-1);
    // usedGlobal = true;
  }
  
  const taskMetas = ASYNC_TASKS_BY_COMPONENT_IDX.get(componentIdx);
  if (taskMetas === undefined || taskMetas.length === 0) { return undefined; }
  
  if (taskID) {
    return taskMetas.find(meta => meta.task.id() === taskID);
  }
  
  // The task whose guest code is running is the one the metadata
  // wrappers record for the component (and restore when a suspended
  // stack resumes). The component's most recently created task may
  // be another one, e.g. a prepared callee that has not entered yet
  // while an earlier task of the component is suspended.
  const current = CURRENT_TASK_META[componentIdx];
  if (current && current.taskID !== undefined) {
    const currentMeta = taskMetas.find(meta => meta.task.id() === current.taskID);
    if (currentMeta) { return currentMeta; }
  }
  
  const taskMeta = taskMetas[taskMetas.length - 1];
  if (!taskMeta || !taskMeta.task) { return undefined; }
  
  return taskMeta;
}
const ASYNC_STATE = new Map();

function promiseWithResolvers() {
  if (Promise.withResolvers) {
    return Promise.withResolvers();
  } else {
    let resolve;
    let reject;
    const promise = new Promise((res, rej) => {
      resolve = res;
      reject = rej;
    });
    return { promise, resolve, reject };
  }
}

const _debugLog = (...args) => {
  if (!globalThis?.process?.env?.JCO_DEBUG) { return; }
  console.debug(...args);
};

class Waitable {
  #componentIdx;
  
  #pendingEventFn = null;
  
  #promise;
  #resolve;
  #reject;
  
  #waitableSet = null;
  
  #hasSyncWaiter = false;
  
  #idx = null; // to component-global waitables
  
  target;
  
  constructor(args) {
    const { componentIdx, target } = args;
    this.#componentIdx = componentIdx;
    this.target = args.target;
    this.#resetPromise();
  }
  
  componentIdx() { return this.#componentIdx; }
  isInSet() { return this.#waitableSet !== null; }
  
  idx() { return this.#idx; }
  setIdx(idx) {
    if (idx === 0) { throw new Error("waitable idx cannot be zero"); }
    this.#idx = idx;
  }
  
  setTarget(tgt) { this.target = tgt; }
  
  #resetPromise() {
    const { promise, resolve, reject } = promiseWithResolvers()
    this.#promise = promise;
    this.#resolve = resolve;
    this.#reject = reject;
  }
  
  resolve() { this.#resolve(); }
  reject(err) { this.#reject(err); }
  promise() { return this.#promise; }
  
  hasPendingEvent() {
    // _debugLog('[Waitable#hasPendingEvent()]', {
      //     componentIdx: this.#componentIdx,
      //     waitable: this,
      //     waitableSet: this.#waitableSet,
      //     hasPendingEvent: this.#pendingEventFn !== null,
      // });
      return this.#pendingEventFn !== null;
    }
    
    setPendingEvent(fn) {
      _debugLog('[Waitable#setPendingEvent()] args', {
        waitable: this,
        inSet: this.#waitableSet,
      });
      this.#pendingEventFn = fn;
    }
    
    getPendingEvent() {
      _debugLog('[Waitable#getPendingEvent()] args', {
        waitable: this,
        inSet: this.#waitableSet,
        hasPendingEvent: this.#pendingEventFn !== null,
      });
      if (this.#pendingEventFn === null) { return null; }
      const eventFn = this.#pendingEventFn;
      this.#pendingEventFn = null;
      const e = eventFn();
      this.#resetPromise();
      return e;
    }
    
    join(waitableSet) {
      _debugLog('[Waitable#join()] args', {
        waitable: this,
        waitableSet: waitableSet,
        isRemoval: waitableSet === null,
      });
      
      if (this.#waitableSet === undefined) {
        throw new TypeError('waitable set must be not be undefined');
      }
      
      if (this.#waitableSet) {
        this.#waitableSet.removeWaitable(this);
      }
      
      this.#waitableSet = waitableSet;
      
      if (waitableSet) {
        this.#waitableSet.addWaitable(this);
      }
    }
    
    drop() {
      _debugLog('[Waitable#drop()] args', {
        componentIdx: this.#componentIdx,
        waitable: this,
      });
      this.join(null);
    }
    
    async waitForPendingEvent(args) {
      const { cstate } = args;
      if (!cstate) { throw new TypeError('missing component state'); }
      
      if (this.#waitableSet !== null || this.#hasSyncWaiter) {
        throw new Error("waitable is already in a set/has a sync waiter");
      }
      this.#hasSyncWaiter = true;
      await cstate.waitUntil({
        cancellable: false,
        readyFn: () => this.hasPendingEvent(),
      });
      this.#hasSyncWaiter = false;
    }
    
  }
  const INSTANCE_FLAGS = new Map();
  const STORE_TRAP = { error: null };
  const STORE_ASYNC_STATE = { deadlockCheck: null, pendingHostOperations: 0 };
  
  function _checkForDeadlock() {
    if (STORE_ASYNC_STATE.deadlockCheck !== null || STORE_TRAP.error !== null) { return; }
    STORE_ASYNC_STATE.deadlockCheck = setTimeout(() => {
      STORE_ASYNC_STATE.deadlockCheck = null;
      if (STORE_TRAP.error !== null || STORE_ASYNC_STATE.pendingHostOperations > 0) { return; }
      
      const suspendedTasks = new Set();
      for (const state of ASYNC_STATE.values()) {
        if (state.hasPendingSchedulerWork()) {
          state.runTickLoop();
          return;
        }
        for (const meta of state.suspendedTaskMetas()) {
          suspendedTasks.add(meta.task);
        }
      }
      
      const unresolvedRoots = new Set();
      for (const task of suspendedTasks) {
        const root = task.getRootTask();
        if (!root.isResolvedState()) { unresolvedRoots.add(root); }
      }
      if (unresolvedRoots.size === 0) { return; }
      
      const err = new WebAssemblyRuntimeError('wasm trap: deadlock detected: event loop cannot make further progress');
      // Which tasks were waiting, and on whose behalf. The message stays
      // exactly what the Canonical ABI calls for, so this rides alongside
      // it: a deadlock reported from a real program is otherwise a bare
      // sentence, and the state that produced it is gone by the time
      // anyone reads the failure.
      err.deadlockDetail = {
        pendingHostOperations: STORE_ASYNC_STATE.pendingHostOperations,
        suspendedTasks: [...suspendedTasks].map((task) => ({
          taskID: task.id(),
          componentIdx: task.componentIdx(),
          state: task.taskState(),
          rootTaskID: task.getRootTask().id(),
        })),
        unresolvedRootTaskIDs: [...unresolvedRoots].map((root) => root.id()),
      };
      STORE_TRAP.error = err;
      for (const root of unresolvedRoots) {
        root.setErrored(err);
        root.reject(err);
      }
      for (const task of suspendedTasks) {
        if (!task.isResolvedState() && unresolvedRoots.has(task.getRootTask())) {
          task.setErrored(err);
          task.reject(err);
        }
      }
      // A deadlock is a trap of the whole store: no instance may be entered again.
      for (const state of ASYNC_STATE.values()) { state.markTrapped(err); }
      for (const state of ASYNC_STATE.values()) { state.abandonLockWaiters(); }
      for (const state of ASYNC_STATE.values()) { state.abandonAdmissionWaiters(); }
      for (const state of ASYNC_STATE.values()) { state.runTickLoop(); }
    }, 0);
  }
  
  const CORE_TRAP_MESSAGES = new Map([
  ['unreachable', "wasm trap: wasm `unreachable` instruction executed"],
  ['memory access out of bounds', "wasm trap: out of bounds memory access"],
  ['divide by zero', "wasm trap: integer divide by zero"],
  ['remainder by zero', "wasm trap: integer divide by zero"],
  ['divide result unrepresentable', "wasm trap: integer overflow"],
  ['float unrepresentable in integer range', "wasm trap: invalid conversion to integer"],
  ['table index is out of bounds', "wasm trap: undefined element: out of bounds table access"],
  ['function signature mismatch', "wasm trap: indirect call type mismatch"],
  ['call stack exhausted', "wasm trap: call stack exhausted"],
  ]);
  function _normalizeCoreTrap(err) {
    if (!(err instanceof WebAssemblyRuntimeError)) { return err; }
    const message = CORE_TRAP_MESSAGES.get(err.message);
    if (message !== undefined) { err.message = message; }
    return err;
  }
  
  class RepTable {
    // Sentinel marking a freed slot; the freelist link for a freed slot
    // lives in the odd cell. This keeps get()/contains()/remove() on freed
    // reps well-defined (previously they returned/corrupted freelist links).
    static FREE = Symbol('RepTable.free');
    
    #data = [0, null];
    #size = 0;
    #target;
    // An index space shared with the other handle kinds of a component
    // instance (see `getHandleIndexSpace`); without one, this table keeps
    // its own free list.
    #space = null;
    
    constructor(args) {
      this.target = args?.target;
      this.#space = args?.space ?? null;
    }
    
    data() { return this.#data; }
    
    insert(val) {
      if (this.#space !== null) {
        const rep = this.#space.alloc();
        this.#data[rep << 1] = val;
        this.#data[(rep << 1) + 1] = null;
        this.#size += 1;
        return rep;
      }
      _debugLog('[RepTable#insert()] args', { val, target: this.target });
      const freeIdx = this.#data[0];
      if (freeIdx === 0) {
        this.#data.push(val);
        this.#data.push(null);
        const rep = (this.#data.length >> 1) - 1;
        _debugLog('[RepTable#insert()] inserted', { val, target: this.target, rep });
        this.#size += 1;
        return rep;
      }
      const placementIdx = freeIdx << 1;
      if (this.#data[placementIdx] !== RepTable.FREE) {
        throw new Error('corrupt rep table freelist: head does not point at a freed slot');
      }
      this.#data[0] = this.#data[placementIdx + 1];
      this.#data[placementIdx] = val;
      this.#data[placementIdx + 1] = null;
      _debugLog('[RepTable#insert()] inserted', { val, target: this.target, rep: freeIdx });
      this.#size += 1;
      return freeIdx;
    }
    
    get(rep) {
      _debugLog('[RepTable#get()] args', { rep, target: this.target });
      if (rep === 0) { throw new WebAssemblyRuntimeError('invalid resource rep during get, (cannot be 0)'); }
      
      const baseIdx = rep << 1;
      const val = this.#data[baseIdx];
      if (val === RepTable.FREE) { return undefined; }
      return val;
    }
    
    contains(rep) {
      _debugLog('[RepTable#contains()] args', { rep, target: this.target });
      if (rep === 0) { throw new Error('invalid resource rep during contains, (cannot be 0)'); }
      
      const baseIdx = rep << 1;
      const val = this.#data[baseIdx];
      return val !== RepTable.FREE && !!val;
    }
    
    remove(rep) {
      _debugLog('[RepTable#remove()] args', { rep, target: this.target });
      if (rep === 0) { throw new Error('invalid resource rep during remove, (cannot be 0)'); }
      if (this.#space !== null) {
        const baseIdx = rep << 1;
        const val = this.#data[baseIdx];
        if (val === undefined || val === RepTable.FREE) {
          throw new WebAssemblyRuntimeError(`unknown handle index ${rep}`);
        }
        this.#data[baseIdx] = RepTable.FREE;
        this.#data[baseIdx + 1] = null;
        this.#size -= 1;
        this.#space.release(rep);
        return val;
      }
      if (this.#data.length === 2) { throw new Error('invalid'); }
      
      const baseIdx = rep << 1;
      if (baseIdx >= this.#data.length) {
        throw new Error(`invalid rep [${rep}] during remove, out of range`);
      }
      const val = this.#data[baseIdx];
      if (val === RepTable.FREE) {
        throw new WebAssemblyRuntimeError(`double removal of rep [${rep}] (already freed)`);
      }
      
      this.#data[baseIdx] = RepTable.FREE;
      this.#data[baseIdx + 1] = this.#data[0];
      this.#data[0] = rep;
      this.#size -= 1;
      
      return val;
    }
    
    size() { return this.#size; }
    
    clear() {
      _debugLog('[RepTable#clear()] args', { rep, target: this.target });
      this.#data = [0, null];
    }
  }
  
  class ComponentAsyncState {
    static EVENT_HANDLER_EVENTS = [ 'backpressure-change' ];
    
    static TickResult = {
      // no suspended tasks remain
      DONE: 'done',
      // a suspended task was resumed (more may be ready)
      RESUMED: 'resumed',
      // suspended tasks remain but none were ready
      IDLE: 'idle',
    };
    
    #componentIdx;
    #callingAsyncImport = false;
    #syncImportWait = promiseWithResolvers();
    #lockHolderTaskID = null;
    #lockWaiters = [];
    #admissionQueue = [];
    #lockHandoffScheduled = false;
    #pendingTaskStarts = 0;
    #parkedTasks = new Map();
    // Suspended threads, oldest first (a Map iterates in insertion
    // order, so no separate queue is kept for them).
    #suspendedThreadsByRef = new Map();
    #errored = null;
    #trapped = false;
    #trapError = null;
    #backpressure = 0;
    #backpressureWaiters = 0n;
    
    #handlerMap = new Map();
    #nextHandlerID = 0n;
    
    #tickLoop = null;
    #tickLoopInterval = null;
    
    #onExclusiveReleaseHandlers = [];
    
    #mayLeave = true;
    
    handles;
    subtasks;
    threads;
    
    constructor(args) {
      this.#componentIdx = args.componentIdx;
      this.handles = new RepTable({
        target: `component [${this.#componentIdx}] handles (waitable objects)`,
        space: getHandleIndexSpace(this.#componentIdx),
      });
      this.subtasks = new RepTable({ target: `component [${this.#componentIdx}] subtasks` });
      this.threads = new RepTable({ target: `component [${this.#componentIdx}] threads` });
    };
    
    componentIdx() { return this.#componentIdx; }
    
    get mayLeave() {
      const flags = INSTANCE_FLAGS.get(this.#componentIdx);
      return flags === undefined ? this.#mayLeave : flags.value === 1;
    }
    set mayLeave(value) {
      if (typeof value !== 'boolean') { throw new TypeError('mayLeave must be a boolean'); }
      this.#mayLeave = value;
      const flags = INSTANCE_FLAGS.get(this.#componentIdx);
      if (flags !== undefined) { flags.value = value ? 1 : 0; }
    }
    
    errored() { return this.#errored !== null; }
    setErrored(err) {
      _debugLog('[ComponentAsyncState#setErrored()] component errored', { err, componentIdx: this.#componentIdx });
      if (this.#errored) { return; }
      if (!err) {
        err = new Error('error elswehere (see other component instance error)')
        err.componentIdx = this.#componentIdx;
      }
      this.#errored = err;
    }
    
    markTrapped(err) {
      if (!(err instanceof WebAssemblyRuntimeError)) {
        return false;
      }
      err = _normalizeCoreTrap(err);
      this.#trapped = true;
      if (this.#trapError === null) { this.#trapError = err; }
      _debugLog('[ComponentAsyncState#markTrapped()] component trapped', { err, componentIdx: this.#componentIdx });
      if (STORE_TRAP.error === null) { STORE_TRAP.error = err; }
      return true;
    }
    
    isTrapped() { return this.#trapped; }
    // For work that was already under way or queued when the instance
    // trapped: it fails with that trap (nothing runs after a trap).
    throwIfInterrupted() {
      if (this.#trapped) {
        throw this.#trapError ?? new WebAssemblyRuntimeError("wasm trap: cannot enter component instance");
      }
    }
    throwIfTrapped() {
      if (this.#trapped) {
        throw new WebAssemblyRuntimeError("wasm trap: cannot enter component instance");
      }
    }
    
    callingSyncImport(val) {
      if (val === undefined) { return this.#callingAsyncImport; }
      if (typeof val !== 'boolean') { throw new TypeError('invalid setting for async import'); }
      const prev = this.#callingAsyncImport;
      this.#callingAsyncImport = val;
      if (prev === true && this.#callingAsyncImport === false) {
        this.#notifySyncImportEnd();
      }
    }
    
    #notifySyncImportEnd() {
      const existing = this.#syncImportWait;
      this.#syncImportWait = promiseWithResolvers();
      existing.resolve();
    }
    
    async waitForSyncImportCallEnd() {
      await this.#syncImportWait.promise;
    }
    
    setBackpressure(v) {
      this.#backpressure = v;
      return this.#backpressure
    }
    getBackpressure() { return this.#backpressure; }
    
    incrementBackpressure() {
      const current = this.#backpressure;
      if (current < 0 || current > 2**16) {
        throw new Error(`invalid current backpressure value [${current}]`);
      }
      const newValue = this.getBackpressure() + 1;
      if (newValue >= 2**16) {
        throw new Error(`invalid new backpressure value [${newValue}], overflow`);
      }
      return this.setBackpressure(newValue);
    }
    
    decrementBackpressure() {
      const current = this.#backpressure;
      if (current < 0 || current > 2**16) {
        throw new Error(`invalid current backpressure value [${current}]`);
      }
      // Canonical ABI `backpressure.dec`: trap if the counter would go
      // below zero.
      const newValue = current - 1;
      if (newValue < 0) {
        throw new WebAssemblyRuntimeError('backpressure counter underflow');
      }
      return this.setBackpressure(newValue);
    }
    hasBackpressure() { return this.#backpressure > 0; }
    
    waitForBackpressure() {
      let backpressureCleared = false;
      const cstate = this;
      cstate.addBackpressureWaiter();
      const handlerID = this.registerHandler({
        event: 'backpressure-change',
        fn: (bp) => {
          if (bp === 0) {
            cstate.removeHandler(handlerID);
            backpressureCleared = true;
          }
        }
      });
      return new Promise((resolve) => {
        const interval = setInterval(() => {
          if (backpressureCleared) { return; }
          clearInterval(interval);
          cstate.removeBackpressureWaiter();
          resolve(null);
        }, 0);
      });
    }
    
    registerHandler(args) {
      const { event, fn } = args;
      if (!event) { throw new Error("missing handler event"); }
      if (!fn) { throw new Error("missing handler fn"); }
      
      if (!ComponentAsyncState.EVENT_HANDLER_EVENTS.includes(event)) {
        throw new Error(`unrecognized event handler [${event}]`);
      }
      
      const handlerID = this.#nextHandlerID++;
      let handlers = this.#handlerMap.get(event);
      if (!handlers) {
        handlers = [];
        this.#handlerMap.set(event, handlers)
      }
      
      handlers.push({ id: handlerID, fn, event });
      return handlerID;
    }
    
    removeHandler(args) {
      const { event, handlerID } = args;
      const registeredHandlers = this.#handlerMap.get(event);
      if (!registeredHandlers) { return; }
      const found = registeredHandlers.find(h => h.id === handlerID);
      if (!found) { return; }
      this.#handlerMap.set(event, this.#handlerMap.get(event).filter(h => h.id !== handlerID));
    }
    
    getBackpressureWaiters() { return this.#backpressureWaiters; }
    addBackpressureWaiter() { this.#backpressureWaiters++; }
    removeBackpressureWaiter() {
      this.#backpressureWaiters--;
      if (this.#backpressureWaiters < 0) {
        throw new Error("unexepctedly negative number of backpressure waiters");
      }
    }
    
    // The per-slice mutual-exclusion lock for guest execution in this
    // component instance. Guest slices (callback invocations and
    // sync-lifted bodies) must be atomic per component even across the
    // JSPI suspensions jco introduces for host imports: wit-bindgen's
    // executors publish per-task state in single linear-memory cells
    // (the wasip3-task pointer, context-local storage discipline) that
    // an interleaved slice of the same component corrupts
    //
    // The lock is *owned*: acquisition records the holder task and
    // release is a no-op for anyone else, so a task exiting can no
    // longer drop a hold it does not own (blind acquire/release-any
    // was the previous discipline). Contended acquisition queues
    // FIFO; release hands the lock to the next waiter directly.
    isExclusivelyLocked() { return this.#lockHolderTaskID !== null; }
    exclusivelyLockedBy(taskID) { return this.#lockHolderTaskID === taskID; }
    
    exclusiveLock(taskID) {
      _debugLog('[ComponentAsyncState#exclusiveLock()]', {
        holder: this.#lockHolderTaskID,
        requester: taskID,
        componentIdx: this.#componentIdx,
      });
      if (taskID === undefined || taskID === null) {
        throw new Error('exclusive lock requires the acquiring task id');
      }
      if (this.#lockHolderTaskID !== null) {
        throw new Error(`component [${this.#componentIdx}] exclusive lock held by task [${this.#lockHolderTaskID}], requested by [${taskID}]`);
      }
      this.#lockHolderTaskID = taskID;
    }
    
    // Awaitable acquisition: takes the lock immediately when free,
    // otherwise queues FIFO behind the current holder and earlier
    // waiters. The resolved promise implies ownership.
    acquireExclusiveLock(taskID) {
      if (taskID === undefined || taskID === null) {
        throw new Error('exclusive lock requires the acquiring task id');
      }
      if (this.#lockHolderTaskID === null) {
        this.#lockHolderTaskID = taskID;
        _debugLog('[ComponentAsyncState#acquireExclusiveLock()] acquired', {
          holder: taskID,
          componentIdx: this.#componentIdx,
        });
        return;
      }
      if (this.#lockHolderTaskID === taskID) {
        throw new Error(`task [${taskID}] already holds the lock for component [${this.#componentIdx}]`);
      }
      _debugLog('[ComponentAsyncState#acquireExclusiveLock()] waiting', {
        holder: this.#lockHolderTaskID,
        requester: taskID,
        componentIdx: this.#componentIdx,
        queued: this.#lockWaiters.length,
      });
      return new Promise((resolve) => {
        this.#lockWaiters.push({ taskID, resolve });
      });
    }
    
    // Release every queued entry without granting ownership, so that
    // calls waiting to enter a trapped store fail instead of hanging.
    // FIFO admission of tasks entering this instance (Canonical ABI
    // `inst.pending`): a task enters only after every task that asked to
    // enter before it has entered or given up, so calls enter in the order
    // they were made, whatever each waits for on the way.
    takeAdmissionTicket(taskID) {
      const ticket = { taskID, ...promiseWithResolvers() };
      this.#admissionQueue.push(ticket);
      return ticket;
    }
    hasPendingAdmissions() { return this.#admissionQueue.length > 0; }
    waitForAdmissionTurn(ticket) {
      if (this.#admissionQueue[0] === ticket) { return; }
      return ticket.promise;
    }
    releaseAdmissionTicket(ticket) {
      const idx = this.#admissionQueue.indexOf(ticket);
      if (idx === -1) { return; }
      this.#admissionQueue.splice(idx, 1);
      if (idx === 0 && this.#admissionQueue.length > 0) {
        this.#admissionQueue[0].resolve();
      }
    }
    // Release every queued entry (the instance trapped): each then fails
    // in `enter()` instead of waiting forever.
    abandonAdmissionWaiters() {
      const tickets = this.#admissionQueue.splice(0);
      for (const ticket of tickets) { ticket.resolve(); }
    }
    
    abandonLockWaiters() {
      const waiters = this.#lockWaiters.splice(0);
      for (const waiter of waiters) { waiter.resolve(); }
    }
    
    cancelExclusiveLockWaiter(taskID) {
      const idx = this.#lockWaiters.findIndex(waiter => waiter.taskID === taskID);
      if (idx === -1) { return false; }
      const [waiter] = this.#lockWaiters.splice(idx, 1);
      // Release the awaiting `enter()` continuation without granting
      // ownership. It observes the task's resolved cancellation state
      // before attempting to execute guest code.
      waiter.resolve();
      return true;
    }
    
    exclusiveRelease(taskID) {
      _debugLog('[ComponentAsyncState#exclusiveRelease()] args', {
        holder: this.#lockHolderTaskID,
        releaser: taskID,
        componentIdx: this.#componentIdx,
      });
      if (this.#lockHolderTaskID !== taskID) {
        // Ownerless releases were the historical behavior; a foreign
        // release now leaves the hold intact
        _debugLog('[ComponentAsyncState#exclusiveRelease()] ignoring foreign release', {
          holder: this.#lockHolderTaskID,
          releaser: taskID,
          componentIdx: this.#componentIdx,
        });
        return false;
      }
      
      // Make the release observable before handing the lock to the next
      // asynchronous guest slice.
      //
      // Release handlers may expose a lifted value whose consumer immediately
      // performs a synchronous call on the same component; that call must run
      // while the instance is genuinely unlocked, not via enterSync's
      // lock-free fallback code.
      this.#lockHolderTaskID = null;
      
      this.#onExclusiveReleaseHandlers = this.#onExclusiveReleaseHandlers.filter(v => !!v);
      for (const [idx, f] of this.#onExclusiveReleaseHandlers.entries()) {
        try {
          this.#onExclusiveReleaseHandlers[idx] = null;
          f();
        } catch (err) {
          _debugLog("error while executing handler for next exclusive release", err);
          throw err;
        }
      }
      this.#scheduleLockHandoff();
      return true;
    }
    
    #scheduleLockHandoff() {
      if (this.#lockHandoffScheduled || this.#lockWaiters.length === 0) { return; }
      this.#lockHandoffScheduled = true;
      queueMicrotask(() => {
        this.#lockHandoffScheduled = false;
        // A synchronous call triggered by a release handler gets the
        // first opportunity to use the unlocked component. Its own
        // release schedules the next handoff; re-queueing one here
        // would spin the microtask queue for as long as the holder
        // stays blocked (e.g. in a synchronous wait), starving every
        // timer, the deadlock detector included.
        if (this.#lockHolderTaskID !== null) { return; }
        const next = this.#lockWaiters.shift();
        if (!next) { return; }
        this.#lockHolderTaskID = next.taskID;
        next.resolve();
      });
    }
    
    onNextExclusiveRelease(fn) {
      _debugLog('[ComponentAsyncState#()onNextExclusiveRelease] registering');
      this.#onExclusiveReleaseHandlers.push(fn);
    }
    
    async waitForExclusiveRelease() {
      while (this.isExclusivelyLocked()) {
        await new Promise(resolve => this.onNextExclusiveRelease(resolve));
      }
    }
    
    // Callback drivers refer to the implicit thread by task ID.
    // Stackful built-ins pass their current thread directly.
    #getSuspendedTaskMeta(taskID) {
      const metas = [...this.#suspendedThreadsByRef.values()].filter(meta => meta.thread.taskID() === taskID);
      return metas.find(meta => meta.cancellable && this.suspendedThreadReady(meta.thread)) ?? metas[0];
    }
    
    suspendTask(args) {
      const { task, ...opts } = args;
      task.endFirstSlice();
      return this.suspendThread({ thread: task.defaultThread(), ...opts });
    }
    
    resumeTaskByID(taskID) {
      const meta = this.#getSuspendedTaskMeta(taskID);
      return meta ? this.resumeThread(meta.thread) : false;
    }
    
    suspendedTaskReady(taskID) {
      const meta = this.#getSuspendedTaskMeta(taskID);
      return meta ? this.suspendedThreadReady(meta.thread) : false;
    }
    
    suspendedTaskCancellable(taskID) {
      return !!this.#getSuspendedTaskMeta(taskID)?.cancellable;
    }
    
    isTaskSuspended(taskID) {
      return !!this.#getSuspendedTaskMeta(taskID);
    }
    
    *suspendedTaskMetas() {
      for (const meta of this.#suspendedThreadsByRef.values()) {
        yield { ...meta, task: meta.thread.task() };
      }
    }
    
    #removeSuspendedThreadMeta(thread) {
      const meta = this.#suspendedThreadsByRef.get(thread);
      this.#suspendedThreadsByRef.delete(thread);
      return meta;
    }
    
    suspendThread(args) {
      const { thread, readyFn, cancellable, onResume, suspended = false } = args;
      _debugLog('[ComponentAsyncState#suspendThread()]', {
        threadID: thread.id(),
        taskID: thread.taskID(),
        componentIdx: this.#componentIdx,
      });
      
      if (thread.componentIdx() !== this.#componentIdx) {
        throw new Error('assert: thread component idx should match async state');
      }
      if (this.#suspendedThreadsByRef.has(thread)) {
        throw new Error(`thread [${thread.id()}] already suspended`);
      }
      
      let promise;
      let resume;
      if (onResume) {
        resume = () => onResume(!thread.isCancelled() && !thread.isRejected() && !this.isTrapped());
      } else {
        const resolvers = promiseWithResolvers();
        promise = resolvers.promise;
        resume = () => resolvers.resolve(!thread.isCancelled() && !thread.isRejected() && !this.isTrapped());
      }
      this.#suspendedThreadsByRef.set(thread, { thread, cancellable, readyFn, resume, suspended });
      
      thread.notifyProgress();
      
      this.runTickLoop();
      _checkForDeadlock();
      
      return promise;
    }
    
    resumeThread(thread) {
      const meta = this.#removeSuspendedThreadMeta(thread);
      if (!meta) { return false; }
      meta.resume();
      return true;
    }
    
    markThreadReady(thread) {
      const meta = this.#suspendedThreadsByRef.get(thread);
      if (!meta || !meta.suspended) { return false; }
      meta.suspended = false;
      meta.readyFn = () => true;
      this.runTickLoop();
      return true;
    }
    
    isThreadSuspended(thread) {
      return this.#suspendedThreadsByRef.get(thread)?.suspended ?? false;
    }
    
    isThreadReady(thread) {
      const meta = this.#suspendedThreadsByRef.get(thread);
      return !!meta && !meta.suspended && (meta.thread.isRejected() || meta.readyFn());
    }
    
    suspendedThreadReady(thread) {
      const meta = this.#suspendedThreadsByRef.get(thread);
      if (!meta) { return false; }
      if (!meta.readyFn) {
        throw new Error(`suspended thread [${thread.id()}] is missing a readiness function`);
      }
      if (this.isTrapped() || meta.thread.isRejected()) { return true; }
      if (!meta.readyFn()) { return false; }
      const task = meta.thread.task();
      return !task.needsExclusiveLock()
      || !this.isExclusivelyLocked()
      || this.exclusivelyLockedBy(task.id());
    }
    
    addPendingTaskStart() { this.#pendingTaskStarts++; }
    removePendingTaskStart() { this.#pendingTaskStarts--; }
    
    hasPendingSchedulerWork() {
      if (this.#pendingTaskStarts > 0) { return true; }
      if (this.#lockHandoffScheduled) { return true; }
      for (const meta of this.#suspendedThreadsByRef.values()) {
        if (this.suspendedThreadReady(meta.thread)) { return true; }
      }
      return false;
    }
    
    async runTickLoop() {
      if (this.#tickLoop !== null) { return; }
      this.#tickLoop = 1;
      setTimeout(async () => {
        let result = this.tick();
        while (result !== ComponentAsyncState.TickResult.DONE) {
          if (result === ComponentAsyncState.TickResult.IDLE) {
            _checkForDeadlock();
          }
          // After resuming a task, re-tick as soon as the resumed
          // slice's microtask continuations have drained (timeout 0)
          // so queued sibling resumptions aren't charged the idle
          // polling interval; otherwise poll at the idle cadence.
          const delay = result === ComponentAsyncState.TickResult.RESUMED ? 0 : 10;
          await new Promise((resolve) => setTimeout(resolve, delay));
          result = this.tick();
        }
        this.#tickLoop = null;
      }, 10);
    }
    
    tick() {
      // (resuming returns immediately below, so the map is never
      // mutated while this loop is still iterating it)
      for (const [thread, meta] of this.#suspendedThreadsByRef) {
        if (!meta || !meta.readyFn) {
          throw new Error(`missing/invalid thread meta despite thread [${thread.id()}] being present`);
        }
        
        if (this.isTrapped() || meta.thread.isRejected()) {
          _debugLog('[ComponentAsyncState#tick()] detected thread rejection, leaving early', { meta });
          this.resumeThread(thread);
          return ComponentAsyncState.TickResult.RESUMED;
        }
        
        if (!this.suspendedThreadReady(thread)) { continue; }
        
        _debugLog('[ComponentAsyncState#tick()] resuming thread via tick', {
          threadID: thread.id(),
          componentIdx: this.#componentIdx,
        });
        this.resumeThread(thread);
        
        // Resume only one thread per tick so its current-thread
        // register window closes before another continuation runs.
        return ComponentAsyncState.TickResult.RESUMED;
      }
      
      const idle = this.#suspendedThreadsByRef.size > 0;
      return idle
      ? ComponentAsyncState.TickResult.IDLE
      : ComponentAsyncState.TickResult.DONE;
    }
    
    createWaitable(args) {
      return new Waitable({ target: args?.target, });
    }
  }
  
  function getOrCreateAsyncState(componentIdx, init) {
    if (!ASYNC_STATE.has(componentIdx)) {
      const newState = new ComponentAsyncState({ componentIdx });
      ASYNC_STATE.set(componentIdx, newState);
    }
    return ASYNC_STATE.get(componentIdx);
  }
  
  function getCurrentThread(componentIdx, threadID, taskID) {
    // Host (-1) tasks have no component instance, and so no thread table.
    if (componentIdx === -1) { return undefined; }
    if (threadID === undefined || threadID === null || threadID === 0) { return undefined; }
    
    const cstate = getOrCreateAsyncState(componentIdx);
    const thread = cstate.threads.get(threadID);
    if (!thread) { return undefined; }
    
    // Thread indices are recycled once a thread exits (the thread table
    // reuses freed slots) but task ids never are. A register that still
    // names an exited thread, e.g. during a post-return that runs after
    // its task exited, must not resolve to whichever newer thread took
    // the slot; callers then fall back as for a missing thread.
    if (taskID !== undefined && thread.taskID() !== taskID) { return undefined; }
    
    return { id: threadID, componentIdx, taskID: thread.taskID(), task: thread.task(), thread };
  }
  const ASYNC_CURRENT_TASK_IDS = [];
  
  function clearCurrentThread(componentIdx, taskID, opts = {}) {
    _debugLog('[clearCurrentThread()] args', { componentIdx, taskID });
    
    if (componentIdx === undefined || componentIdx === null) {
      throw new Error('missing/invalid component instance index while ending current task');
    }
    
    const tasks = ASYNC_TASKS_BY_COMPONENT_IDX.get(componentIdx);
    if (!tasks || !Array.isArray(tasks)) {
      throw new Error('missing/invalid tasks for component instance while ending task');
    }
    if (tasks.length == 0) {
      throw new Error(`no current tasks for component instance [${componentIdx}] while ending task`);
    }
    
    const taskIdx = taskID === undefined
    ? tasks.length - 1
    : tasks.findIndex(meta => meta.id === taskID);
    if (taskIdx === -1) { return; }
    
    const taskMeta = tasks[taskIdx];
    if (!opts.preserveTask) {
      tasks.splice(taskIdx, 1);
    }
    const globalTaskIdx = ASYNC_CURRENT_TASK_IDS.lastIndexOf(taskMeta.id);
    if (globalTaskIdx !== -1) {
      ASYNC_CURRENT_TASK_IDS.splice(globalTaskIdx, 1);
      ASYNC_CURRENT_COMPONENT_IDXS.splice(globalTaskIdx, 1);
    }
    return taskMeta.task;
  }
  const symbolDispose = Symbol.dispose || Symbol.for('dispose');
  
  // Dispose of a host-provided value that a guest discarded, using `disposeFn`
  // if provided, otherwise the value's `Symbol.asyncDispose` or `Symbol.dispose`.
  //
  // Disposal runs in a microtask, so host code never runs inside a canonical
  // built-in (where it could re-enter the component), and errors are reported
  // rather than surfaced to the guest.
  function _disposeHostValue(value, disposeFn) {
    if (!disposeFn) {
      if (value === null || (typeof value !== 'object' && typeof value !== 'function')) { return; }
      if (typeof Symbol.asyncDispose === 'symbol' && typeof value[Symbol.asyncDispose] === 'function') {
        disposeFn = value[Symbol.asyncDispose];
      } else if (typeof value[symbolDispose] === 'function') {
        disposeFn = value[symbolDispose];
      } else {
        return;
      }
    }
    const reportErr = (err) => console.error('[jco] error while disposing discarded host value', err);
    queueMicrotask(() => {
      try {
        const res = disposeFn.call(value);
        if (res && typeof res.then === 'function') { res.then(undefined, reportErr); }
      } catch (err) {
        reportErr(err);
      }
    });
  }
  
  const GLOBAL_COMPONENT_MEMORY_MAP = new Map();
  
  function lookupMemoriesForComponent(args) {
    const { componentIdx } = args ?? {};
    if (args.componentIdx === undefined) { throw new TypeError("missing component idx"); }
    
    const metas = GLOBAL_COMPONENT_MEMORY_MAP.get(componentIdx);
    if (!metas) { return []; }
    
    if (args.memoryIdx === undefined) {
      return Object.values(metas);
    }
    
    const meta = metas[args.memoryIdx];
    return meta?.memory;
  }
  
  class AsyncSubtask {
    static _ID = 0n;
    
    static State = {
      STARTING: 0,
      STARTED: 1,
      RETURNED: 2,
      CANCELLED_BEFORE_STARTED: 3,
      CANCELLED_BEFORE_RETURNED: 4,
    };
    
    #id;
    #state = AsyncSubtask.State.STARTING;
    #componentIdx;
    
    #parentTask;
    #childTask = null;
    
    #dropped = false;
    #cancelRequested = false;
    
    #memoryIdx = null;
    #lenders = null;
    
    #waitable = null;
    
    #callbackFn = null;
    #callbackFnName = null;
    
    #postReturnFn = null;
    #onProgressFn = null;
    #pendingEventFn = null;
    
    #callMetadata = {};
    
    #resolved = false;
    
    #onResolveHandlers = [];
    #onStartHandlers = [];
    
    #result = null;
    #resultSet = false;
    
    // Pending value returned by a host implementation of this (async import)
    // subtask, held so it can be disposed if the guest discards the call
    #hostPendingResult = null;
    
    fnName;
    target;
    isAsync;
    isManualAsync;
    // One execution slice awaited by the conditional cancel trampoline.
    cancelProgress = null;
    
    constructor(args) {
      if (typeof args.componentIdx !== 'number') {
        throw new Error('invalid componentIdx for subtask creation');
      }
      this.#componentIdx = args.componentIdx;
      
      this.#id = ++AsyncSubtask._ID;
      this.fnName = args.fnName;
      
      if (!args.parentTask) { throw new Error('missing parent task during subtask creation'); }
      this.#parentTask = args.parentTask;
      
      if (args.childTask) { this.#childTask = args.childTask; }
      
      if (args.memoryIdx) { this.#memoryIdx = args.memoryIdx; }
      
      if (!args.waitable) { throw new Error("missing/invalid waitable"); }
      this.#waitable = args.waitable;
      
      if (args.callMetadata) { this.#callMetadata = args.callMetadata; }
      
      this.#lenders = [];
      this.target = args.target;
      this.isAsync = args.isAsync;
      this.isManualAsync = args.isManualAsync;
    }
    
    id() { return this.#id; }
    parentTaskID() { return this.#parentTask?.id(); }
    childTaskID() { return this.#childTask?.id(); }
    state() { return this.#state; }
    
    getWaitable() { return this.#waitable; }
    waitableRep() { return this.#waitable.idx(); }
    // The handle index is allocated only when the guest gets to see the
    // subtask (an async lower that did not complete at once), and after the
    // lifted arguments have freed theirs (Canonical ABI `canon_lower`).
    ensureHandle() {
      const existing = this.#waitable.idx();
      if (existing !== null && existing !== undefined) { return existing; }
      const idx = this.#getComponentState().handles.insert(this);
      this.#waitable.setIdx(idx);
      this.#waitable.setTarget(`waitable for subtask (waitable id [${idx}], subtask internal ID [${this.id()}])`);
      return idx;
    }
    
    join() { return this.#waitable.join(...arguments); }
    getPendingEvent() { return this.#waitable.getPendingEvent(...arguments); }
    hasPendingEvent() { return this.#waitable.hasPendingEvent(...arguments); }
    setPendingEvent() { return this.#waitable.setPendingEvent(...arguments); }
    
    setTarget(tgt) { this.target = tgt; }
    
    getResult() {
      if (!this.#resultSet) { throw new Error("subtask result has not been set") }
      return this.#result;
    }
    setResult(v) {
      if (this.#resultSet) { throw new Error("subtask result has already been set"); }
      this.#result = v;
      this.#resultSet = true;
    }
    
    componentIdx() { return this.#componentIdx; }
    
    setChildTask(t) {
      if (!t) { throw new Error('cannot set missing/invalid child task on subtask'); }
      if (this.#childTask) { throw new Error('child task is already set on subtask'); }
      if (this.#parentTask === t) { throw new Error("parent cannot be child"); }
      this.#childTask = t;
    }
    getChildTask(t) { return this.#childTask; }
    
    getParentTask() { return this.#parentTask; }
    
    setCallbackFn(f, name) {
      if (!f) { return; }
      if (this.#callbackFn) { throw new Error('callback fn can only be set once'); }
      this.#callbackFn = f;
      this.#callbackFnName = name;
    }
    
    getCallbackFnName() {
      if (!this.#callbackFn) { return undefined; }
      return this.#callbackFn.name;
    }
    
    setPostReturnFn(f) {
      if (!f) { return; }
      if (this.#postReturnFn) { throw new Error('postReturn fn can only be set once'); }
      this.#postReturnFn = f;
    }
    
    setOnProgressFn(f) {
      if (this.#onProgressFn) { throw new Error('on progress fn can only be set once'); }
      this.#onProgressFn = f;
    }
    
    isNotStarted() {
      return this.#state == AsyncSubtask.State.STARTING;
    }
    
    cancellationRequested() { return this.#cancelRequested; }
    
    // Request cooperative cancellation of this subtask, on behalf of the
    // supertask (i.e. `canon subtask.cancel`).
    //
    // If the callee is another guest task, the request is delivered to it and
    // the callee confirms via `task.cancel` (or still resolves via `task.return`).
    //
    // If the callee is a host function, the pending call is treated as
    // immediately cancelled -- consistent with hosts being expected to resolve
    // cancellation promptly -- and any later host resolution is discarded
    // (see `AsyncTask#onResolve`). The host is notified of the discard by
    // disposing the pending value it returned (see `setHostPendingResult`).
    requestCancellation() {
      _debugLog('[AsyncSubtask#requestCancellation()] args', {
        componentIdx: this.#componentIdx,
        subtaskID: this.#id,
        state: this.#state,
        childTaskID: this.childTaskID(),
        fnName: this.fnName,
      });
      if (this.#cancelRequested) {
        throw new Error('cancellation has already been requested for this subtask');
      }
      this.#cancelRequested = true;
      
      if (this.#resolved) { return; }
      
      if (this.#childTask) {
        this.#childTask.requestCancellation();
        return;
      }
      
      this.onResolve(null);
      this.#disposeHostPendingResult();
    }
    
    // Record the value returned by the host implementation of this subtask.
    //
    // Host imports may return a thenable that implements `Symbol.asyncDispose`
    // or `Symbol.dispose`. If the guest cancels the call before its result is
    // delivered, the result is discarded and the thenable is disposed so the
    // host can stop pending work and release anything it produced.
    setHostPendingResult(v) {
      if (v === null || (typeof v !== 'object' && typeof v !== 'function') || typeof v.then !== 'function') {
        return;
      }
      this.#hostPendingResult = v;
      if (this.#resolved && this.#state !== AsyncSubtask.State.RETURNED) {
        this.#disposeHostPendingResult();
      }
    }
    
    #disposeHostPendingResult() {
      const pending = this.#hostPendingResult;
      if (!pending) { return; }
      this.#hostPendingResult = null;
      _disposeHostValue(pending);
    }
    
    registerOnStartHandler(f) {
      this.#onStartHandlers.push(f);
    }
    
    onStart(args) {
      _debugLog('[AsyncSubtask#onStart()] args', {
        componentIdx: this.#componentIdx,
        subtaskID: this.#id,
        parentTaskID: this.parentTaskID(),
        fnName: this.fnName,
        args,
      });
      
      if (this.#onProgressFn) { this.#onProgressFn(); }
      
      // Starting a nested operation is a task execution boundary.
      // In particular, a cancellation handler may synchronously
      // enter an import which then suspends in the host.  Wake a
      // supertask that is driving one cancellation slice so an
      // async `subtask.cancel` can report BLOCKED without waiting
      // for that nested operation to finish.
      this.#parentTask.notifyProgress();
      
      this.#state = AsyncSubtask.State.STARTED;
      
      let result;
      
      // If we have been provided a helper start function as a result of
      // component fusion performed by wasmtime tooling, then we can call that helper and lifts/lowers will
      // be performed for us.
      //
      // See also documentation on `HostIntrinsic::PrepareCall`
      //
      if (this.#callMetadata.startFn) {
        result = this.#callMetadata.startFn.apply(null, args?.startFnParams ?? []);
      }
      
      return result;
    }
    
    
    registerOnResolveHandler(f) {
      this.#onResolveHandlers.push(f);
    }
    
    reject(subtaskErr) {
      if (this.#resolved) { return; }
      
      if (this.#onProgressFn) { this.#onProgressFn(); }
      
      if (this.#state === AsyncSubtask.State.STARTING) {
        this.#state = AsyncSubtask.State.CANCELLED_BEFORE_STARTED;
      } else if (this.#state === AsyncSubtask.State.STARTED) {
        this.#state = AsyncSubtask.State.CANCELLED_BEFORE_RETURNED;
      } else {
        throw new Error('cannot reject a completed subtask');
      }
      
      this.#resolved = true;
      this.#hostPendingResult = null;
      this.#parentTask.removeSubtask(this);
      this.#parentTask.reject(subtaskErr);
    }
    
    onResolve(subtaskValue) {
      _debugLog('[AsyncSubtask#onResolve()] args', {
        componentIdx: this.#componentIdx,
        subtaskID: this.#id,
        isAsync: this.isAsync,
        childTaskID: this.childTaskID(),
        parentTaskID: this.parentTaskID(),
        parentTaskFnName: this.#parentTask?.entryFnName(),
        fnName: this.fnName,
      });
      
      if (this.#resolved) {
        throw new Error('subtask has already been resolved');
      }
      
      if (this.#onProgressFn) { this.#onProgressFn(); }
      
      if (subtaskValue === null && this.#cancelRequested) {
        if (this.#state === AsyncSubtask.State.STARTING) {
          this.#state = AsyncSubtask.State.CANCELLED_BEFORE_STARTED;
        } else {
          if (this.#state !== AsyncSubtask.State.STARTED) {
            throw new Error('resolved subtask must have been started before cancellation');
          }
          this.#state = AsyncSubtask.State.CANCELLED_BEFORE_RETURNED;
        }
      } else {
        if (this.#state !== AsyncSubtask.State.STARTED) {
          throw new Error('resolved subtask must have been started before completion');
        }
        this.#state = AsyncSubtask.State.RETURNED;
        // The host result is delivered to the guest, so it is no longer ours to dispose
        this.#hostPendingResult = null;
      }
      
      this.setResult(subtaskValue);
      
      for (const f of this.#onResolveHandlers) {
        try {
          f(subtaskValue);
        } catch (err) {
          console.error("error during subtask resolve handler", err);
          throw err;
        }
      }
      
      const callMetadata = this.getCallMetadata();
      
      // TODO(fix): we should be able to easily have the caller's meomry
      // to lower into here, but it's not present in PrepareCall
      const memory = callMetadata.memory ?? this.#parentTask?.getReturnMemory() ?? lookupMemoriesForComponent({ componentIdx: this.#parentTask?.componentIdx() })[0];
      // NOTE: cancelled resolutions carry no value, so nothing is lowered
      const returned = this.#state === AsyncSubtask.State.RETURNED;
      if (returned && callMetadata && !callMetadata.returnFn && (this.isAsync || callMetadata.funcTypeIsAsync) && callMetadata.resultPtr && memory) {
        const { resultPtr, realloc } = callMetadata;
        const lowers = callMetadata.lowers; // may have been updated in task.return of the child
        if (lowers && lowers.length > 0) {
          lowers[0]({
            componentIdx: this.#componentIdx,
            memory,
            realloc,
            vals: [subtaskValue],
            storagePtr: resultPtr,
            stringEncoding: callMetadata.stringEncoding,
          });
        }
      }
      
      this.#resolved = true;
      this.#parentTask.removeSubtask(this);
      
      if (!this.isAsync) {
        this.deliverResolve();
        const rep = this.waitableRep();
        if (rep) {
          try {
            const removed = this.#getComponentState().handles.remove(rep);
            if (removed !== this) {
              throw new Error("unexpectedly received non-self Subtask from handle removal");
            }
            this.drop();
          } catch (err) {
            _debugLog('[AsyncSubtask#onResolve()] failed to remove subtask after sync subtask completion', err);
          }
        }
      }
    }
    
    getStateNumber() { return this.#state; }
    isReturned() { return this.#state === AsyncSubtask.State.RETURNED; }
    
    getCallMetadata() { return this.#callMetadata; }
    
    isResolved() {
      if (this.#state === AsyncSubtask.State.STARTING
      || this.#state === AsyncSubtask.State.STARTED) {
        return false;
      }
      if (this.#state === AsyncSubtask.State.RETURNED
      || this.#state === AsyncSubtask.State.CANCELLED_BEFORE_STARTED
      || this.#state === AsyncSubtask.State.CANCELLED_BEFORE_RETURNED) {
        return true;
      }
      throw new Error('unrecognized internal Subtask state [' + this.#state + ']');
    }
    
    addLender(handle) {
      _debugLog('[AsyncSubtask#addLender()] args', { handle });
      if (!Number.isNumber(handle)) { throw new Error('missing/invalid lender handle [' + handle + ']'); }
      
      if (this.#lenders.length === 0 || this.isResolved()) {
        throw new Error('subtask has no lendors or has already been resolved');
      }
      
      handle.lends++;
      this.#lenders.push(handle);
    }
    
    deliverResolve() {
      _debugLog('[AsyncSubtask#deliverResolve()] args', {
        lenders: this.#lenders,
        parentTaskID: this.parentTaskID(),
        subtaskID: this.#id,
        childTaskID: this.childTaskID(),
        resolved: this.isResolved(),
        resolveDelivered: this.resolveDelivered(),
      });
      
      const cannotDeliverResolve = this.resolveDelivered() || !this.isResolved();
      if (cannotDeliverResolve) {
        throw new Error('subtask cannot deliver resolution twice, and the subtask must be resolved');
      }
      
      for (const lender of this.#lenders) {
        lender.lends--;
      }
      
      this.#lenders = null;
    }
    
    resolveDelivered() {
      _debugLog('[AsyncSubtask#resolveDelivered()] args', { });
      if (this.#lenders === null && !this.isResolved()) {
        throw new Error('invalid subtask state, lenders missing and subtask has not been resolved');
      }
      return this.#lenders === null;
    }
    
    drop() {
      _debugLog('[AsyncSubtask#drop()] args', {
        componentIdx: this.#componentIdx,
        parentTaskID: this.#parentTask?.id(),
        parentTaskFnName: this.#parentTask?.entryFnName(),
        childTaskID: this.#childTask?.id(),
        childTaskFnName: this.#childTask?.entryFnName(),
        subtaskFnName: this.fnName,
      });
      if (!this.#waitable) { throw new Error('missing/invalid inner waitable'); }
      if (!this.resolveDelivered()) {
        throw new WebAssemblyRuntimeError('cannot drop a subtask which has not yet resolved');
      }
      if (this.#waitable) { this.#waitable.drop() }
      this.#dropped = true;
    }
    
    #getComponentState() {
      const state = getOrCreateAsyncState(this.#componentIdx);
      if (!state) {
        throw new Error('invalid/missing async state for component [' + componentIdx + ']');
      }
      return state;
    }
    
    getWaitableHandleIdx() {
      _debugLog('[AsyncSubtask#getWaitableHandleIdx()] args', { });
      if (!this.#waitable) { throw new Error('missing/invalid waitable'); }
      return this.waitableRep();
    }
  }
  
  class FutureValue {
    #start;
    #settled;
    #hideThen = 0;
    #thenFn;
    
    constructor(start) {
      if (typeof start !== 'function') {
        throw new TypeError('future start operation must be a function');
      }
      this.#start = start;
      this.#thenFn = this.#then.bind(this);
    }
    
    get then() {
      return this.#hideThen === 0 ? this.#thenFn : undefined;
    }
    
    #read() {
      if (!this.#settled) {
        // The start operation resolves to a non-thenable box so a
        // future-valued payload cannot be assimilated by this Promise.
        this.#settled = Promise.resolve().then(this.#start);
      }
      return this.#settled;
    }
    
    resolveAsValue(resolve) {
      this.#hideThen++;
      try {
        resolve(this);
      } finally {
        this.#hideThen--;
      }
    }
    
    #deliver(resolve, value) {
      if (value instanceof FutureValue) {
        // Promise resolution reads `then` synchronously. Hide it only
        // for that lookup so resolving this layer yields the inner
        // FutureValue instead of recursively awaiting it.
        value.resolveAsValue(resolve);
        return;
      }
      resolve(value);
    }
    
    #then(resolve, reject) {
      return this.#read().then(
      box => this.#deliver(resolve, box.value),
      reject,
      );
    }
  }
  
  function _postReturnTrap(componentIdx, err) {
    // Core exceptions and stack exhaustion are fatal during cleanup.
    if (!(err instanceof WebAssemblyRuntimeError)) {
      const message = /call stack|stack overflow|too much recursion/i.test(err?.message ?? '')
      ? "wasm trap: call stack exhausted" : "wasm trap: uncaught exception propagated out of component";
      err = new WebAssemblyRuntimeError(message, { cause: err });
    }
    getOrCreateAsyncState(componentIdx).markTrapped(err);
    return err;
  }
  
  function _recordTrap(componentIdx, err, convertsRangeErrors = false) {
    // A built-in reading or writing guest memory past its end fails with a
    // DataView RangeError: that is the Canonical ABI's out-of-bounds trap.
    if (convertsRangeErrors && err instanceof RangeError) {
      err = new WebAssemblyRuntimeError('out of bounds memory access');
    }
    if (err instanceof WebAssemblyRuntimeError) {
      // (a trampoline without a static instance records against the
      // component whose guest code is running)
      const idx = componentIdx ?? CURRENT_TASK_META.current?.componentIdx;
      if (idx !== undefined && idx !== null) {
        getOrCreateAsyncState(idx).markTrapped(err);
      }
    }
    return err;
  }
  const ASYNC_DETERMINISM = 'random';
  const _coinFlip = () => { return Math.random() > 0.5; };
  
  const ASYNC_EVENT_CODE = {
    NONE: 0,
    SUBTASK: 1,
    STREAM_READ: 2,
    STREAM_WRITE: 3,
    FUTURE_READ: 4,
    FUTURE_WRITE: 5,
    TASK_CANCELLED: 6,
  };
  
  function _getGlobalCurrentThreadMeta(componentIdx) {
    const v = componentIdx === undefined || componentIdx === null
    ? CURRENT_TASK_META.current
    : CURRENT_TASK_META[componentIdx];
    if (v === undefined || v === null) {
      return undefined;
    }
    return { ...v };
  }
  
  
  function _withGlobalCurrentThreadMeta(args) {
    _debugLog('[_withGlobalCurrentThreadMeta()] args', args);
    if (!args) { throw new TypeError('args missing'); }
    if (args.taskID === undefined) { throw new TypeError('missing task ID'); }
    if (args.threadID === undefined) { throw new TypeError('missing thread ID'); }
    if (args.componentIdx === undefined) { throw new TypeError('missing component idx'); }
    if (!args.fn) { throw new TypeError('missing fn'); }
    const { taskID, threadID, componentIdx, fn } = args;
    const previous = CURRENT_TASK_META[componentIdx] ?? null;
    const previousCurrent = CURRENT_TASK_META.current ?? null;
    
    try {
      CURRENT_TASK_META.current =
      CURRENT_TASK_META[componentIdx] = { threadID, taskID, componentIdx };
      return fn();
    } catch (err) {
      _debugLog("error while executing sync callee/callback", {
        ...args,
        err,
      });
      throw err;
    } finally {
      // Synchronous wrappers can nest without any intervening JS
      // scheduling. Restore the caller rather than clearing it so
      // helper core exports (for example fused return adapters) can
      // temporarily run under a different thread of the same component.
      CURRENT_TASK_META[componentIdx] = previous;
      CURRENT_TASK_META.current = previousCurrent;
    }
  }
  
  function _withGlobalCurrentThreadMetaAsync(args) {
    _debugLog('[_withGlobalCurrentThreadMetaAsync()] args', args);
    if (!args) { throw new TypeError('args missing'); }
    if (args.taskID === undefined) { throw new TypeError('missing task ID'); }
    if (args.threadID === undefined) { throw new TypeError('missing thread ID'); }
    if (args.componentIdx === undefined) { throw new TypeError('missing component idx'); }
    if (!args.fn) { throw new TypeError('missing fn'); }
    
    const { taskID, threadID, componentIdx, fn } = args;
    const previous = CURRENT_TASK_META[componentIdx] ?? null;
    const previousCurrent = CURRENT_TASK_META.current ?? null;
    
    let result;
    try {
      CURRENT_TASK_META.current =
      CURRENT_TASK_META[componentIdx] = { taskID, threadID, componentIdx };
      result = fn();
    } catch (err) {
      _debugLog("error while executing sync callee/callback", {
        ...args,
        err,
      });
      CURRENT_TASK_META[componentIdx] = previous;
      CURRENT_TASK_META.current = previousCurrent;
      throw err;
    }
    
    if (result === null ||
    (typeof result !== 'object' && typeof result !== 'function') ||
    typeof result.then !== 'function') {
      CURRENT_TASK_META[componentIdx] = previous;
      CURRENT_TASK_META.current = previousCurrent;
      return result;
    }
    
    return (async () => {
      try {
        return await result;
      } catch (err) {
        _debugLog("error while executing async callee/callback", {
          ...args,
          err,
        });
        throw err;
      } finally {
        if (CURRENT_TASK_META[componentIdx]?.threadID === threadID &&
        CURRENT_TASK_META[componentIdx]?.taskID === taskID) {
          CURRENT_TASK_META[componentIdx] = null;
        }
        if (CURRENT_TASK_META.current?.threadID === threadID &&
        CURRENT_TASK_META.current?.taskID === taskID) {
          CURRENT_TASK_META.current = null;
        }
      }
    })();
  }
  
  class AsyncTask {
    static _ID = 0n;
    // Ids for the implicit threads of host (-1) tasks, which have no
    // component instance thread table (see `addThread`).
    static _HOST_THREAD_ID = 0;
    
    static State = {
      INITIAL: 'initial',
      CANCELLED: 'cancelled',
      CANCEL_PENDING: 'cancel-pending',
      CANCEL_DELIVERED: 'cancel-delivered',
      RESOLVED: 'resolved',
    }
    
    static BlockResult = {
      CANCELLED: 'block.cancelled',
      NOT_CANCELLED: 'block.not-cancelled',
    }
    
    #id;
    #componentIdx;
    #state;
    #isAsync;
    #isManualAsync;
    #callingWasmExport = true;
    #lockFreeEntry = false;
    #preserveFutureResult;
    #entryFnName = null;
    
    #onResolveHandlers = [];
    #progressWaiters = [];
    #completionPromise = null;
    #completionValue;
    #completionReady = false;
    #settleCompletionPromise;
    #rejected = false;
    
    #exitPromise = null;
    #onExitHandlers = [];
    
    #memoryIdx = null;
    #memory = null;
    
    #callbackFn = null;
    #callbackFnName = null;
    
    #postReturnFn = null;
    
    #getCalleeParamsFn = null;
    #calleeIsAsync = null;
    #admissionTicket = null;
    #funcTypeIsAsync = false;
    #returnResultTypeIdx = null;
    
    #stringEncoding = null;
    
    #parentSubtask = null;
    
    #errHandling;
    
    #backpressurePromise;
    #backpressureWaiters = 0n;
    
    #returnLowerFns = null;
    
    #resourceScopeId;
    #resourceBorrowCount = 0;
    #resourceLenders = [];
    #resourceScopeExited = false;
    
    #subtasks = [];
    
    // Threads owned by this task, indexed by their task-scoped
    // thread id (see AsyncThread -- index 0 is always the task's
    // implicit thread, created alongside it).
    #threads = [];
    
    #entered = false;
    #exited = false;
    #errored = null;
    
    cancelled = false;
    cancelRequested = false;
    alwaysTaskReturn = false;
    
    returnCalls =  0;
    tmpRetI64HighBits = 0|0;
    
    constructor(opts) {
      this.#id = ++AsyncTask._ID;
      this.#resourceScopeId = ++RESOURCE_SCOPE_ID;
      RESOURCE_SCOPE_TASKS.set(this.#resourceScopeId, this);
      
      if (opts?.componentIdx === undefined) {
        throw new TypeError('missing component id during task creation');
      }
      this.#componentIdx = opts.componentIdx;
      
      this.#state = AsyncTask.State.INITIAL;
      this.#isAsync = opts?.isAsync ?? false;
      this.#isManualAsync = opts?.isManualAsync ?? false;
      this.#preserveFutureResult = opts?.preserveFutureResult ?? false;
      this.#entryFnName = opts.entryFnName;
      // Tasks that execute guest slices (export calls, fused
      // callees) default to true; import-handler tasks pass false
      // explicitly (they run host code nested inside the caller's
      // already-locked slice).
      this.#callingWasmExport = opts?.callingWasmExport !== false;
      
      const {
        promise: completionPromise,
        resolve: resolveCompletionPromise,
        reject: rejectCompletionPromise,
      } = promiseWithResolvers();
      this.#completionPromise = completionPromise;
      // A nested rejection can reach the root task while its Wasm
      // entrypoint is still suspended, before the export wrapper awaits
      // this promise. Mark it handled immediately while preserving the
      // original rejected promise for the eventual caller.
      completionPromise.catch(() => {});
      
      let completionSettled = false;
      const settleCompletionPromise = () => {
        if (completionSettled || !this.#completionReady) { return; }
        completionSettled = true;
        if (this.#errored !== null) {
          rejectCompletionPromise(this.#errored);
        } else if (this.#rejected) {
          rejectCompletionPromise(this.#completionValue);
        } else if (
        this.#preserveFutureResult
        && this.#completionValue instanceof FutureValue
        ) {
          this.#completionValue.resolveAsValue(resolveCompletionPromise);
        } else {
          resolveCompletionPromise(this.#completionValue);
        }
      };
      
      this.#settleCompletionPromise = settleCompletionPromise;
      this.#onResolveHandlers.push((results) => {
        if (this.#parentSubtask !== null) { return; }
        if (!this.#isAsync && !this.#isManualAsync) { return; }
        this.#completionValue = results;
        this.#completionReady = true;
        // Publish after the current guest slice returns, so a trap
        // in that slice can still reject the call. Do not wait for
        // task exit: detached work may require further host calls
        // or consumption of a returned resource's stream.
      });
      
      const {
        promise: exitPromise,
        resolve: resolveExitPromise,
        reject: rejectExitPromise,
      } = promiseWithResolvers();
      this.#exitPromise = exitPromise;
      
      this.#onExitHandlers.push(() => {
        if (this.#parentSubtask === null && (this.#isAsync || this.#isManualAsync)) {
          settleCompletionPromise();
        }
        resolveExitPromise();
      });
      
      if (opts.callbackFn) { this.#callbackFn = opts.callbackFn; }
      if (opts.callbackFnName) { this.#callbackFnName = opts.callbackFnName; }
      
      if (opts.getCalleeParamsFn) { this.#getCalleeParamsFn = opts.getCalleeParamsFn; }
      if (opts.stringEncoding) { this.#stringEncoding = opts.stringEncoding; }
      
      if (opts.parentSubtask) { this.#parentSubtask = opts.parentSubtask; }
      
      
      if (opts.errHandling) { this.#errHandling = opts.errHandling; }
    }
    
    taskState() { return this.#state; }
    id() { return this.#id; }
    componentIdx() { return this.#componentIdx; }
    entryFnName() { return this.#entryFnName; }
    
    resourceScopeId() { return this.#resourceScopeId; }
    
    addBorrowedHandle() {
      if (this.#resourceScopeExited) {
        throw new Error('cannot add a borrow to an exited resource scope');
      }
      this.#resourceBorrowCount++;
    }
    
    removeBorrowedHandle() {
      if (this.#resourceBorrowCount === 0) {
        throw new Error('resource borrow count underflow');
      }
      this.#resourceBorrowCount--;
    }
    
    addResourceLender(table, handle) {
      if (this.#resourceScopeExited) {
        throw new Error('cannot add a lender to an exited resource scope');
      }
      this.#resourceLenders.push({ table, handle });
    }
    
    validateResourceBorrowScope() {
      if (this.#resourceScopeExited) { return; }
      if (this.#resourceBorrowCount !== 0) {
        throw new WebAssemblyRuntimeError('borrow handles still remain at the end of the call');
      }
      for (const { table, handle } of this.#resourceLenders) {
        const idx = handle << 1;
        const lendCount = table[idx];
        if (!Number.isInteger(lendCount) || lendCount <= 0 || lendCount >= 2**30) {
          throw new Error('invalid resource lender state at scope exit');
        }
        table[idx] = lendCount - 1;
      }
      this.#resourceLenders = [];
      this.#resourceScopeExited = true;
      RESOURCE_SCOPE_TASKS.delete(this.#resourceScopeId);
    }
    
    completionPromise() { return this.#completionPromise; }
    settleCompletion() { this.#settleCompletionPromise(); }
    exitPromise() { return this.#exitPromise; }
    
    waitForProgress() {
      const { promise, resolve } = promiseWithResolvers();
      this.#progressWaiters.push(resolve);
      return promise;
    }
    
    notifyProgress() {
      const waiters = this.#progressWaiters;
      this.#progressWaiters = [];
      for (const resolve of waiters) { resolve(); }
    }
    
    isAsync() { return this.#isAsync; }
    isManualAsync() { return this.#isManualAsync; }
    isSync() { return !this.isAsync(); }
    
    getErrHandling() { return this.#errHandling; }
    
    hasCallback() { return this.#callbackFn !== null; }
    
    getReturnMemoryIdx() { return this.#memoryIdx; }
    setReturnMemoryIdx(idx) {
      if (idx === null) { return; }
      this.#memoryIdx = idx;
    }
    
    getReturnMemory() { return this.#memory; }
    setReturnMemory(m) {
      if (m === null) { return; }
      this.#memory = m;
    }
    
    setReturnLowerFns(fns) { this.#returnLowerFns = fns; }
    getReturnLowerFns() { return this.#returnLowerFns; }
    
    calleeIsAsync() { return this.#calleeIsAsync === true; }
    
    setCalleeIsAsync(value) {
      if (typeof value !== 'boolean') { throw new TypeError('callee async state must be a boolean'); }
      this.#calleeIsAsync = value;
    }
    
    setParentSubtask(subtask) {
      if (!subtask || !(subtask instanceof AsyncSubtask)) { return }
      if (this.#parentSubtask) { throw new Error('parent subtask can only be set once'); }
      this.#parentSubtask = subtask;
    }
    
    getParentSubtask() { return this.#parentSubtask; }
    
    // Registers a newly-created thread as belonging to this task
    // and assigns it its real, *component-instance-scoped* id
    // (`Task.register_thread` in the canonical ABI) -- threads
    // are addressed instance-wide (not per-task) so that a
    // thread of one task can target a thread of another, e.g.
    // via `thread.resume-later`.
    addThread(thread) {
      // Host (-1) tasks model host-side import handling: there is no
      // component instance behind them, so no instance thread table to
      // register with (and none may be materialised on their behalf).
      if (this.#componentIdx === -1) {
        thread.setId(++AsyncTask._HOST_THREAD_ID);
        this.#threads.push(thread);
        return thread;
      }
      const cstate = getOrCreateAsyncState(this.#componentIdx);
      thread.setId(cstate.threads.insert(thread));
      this.#threads.push(thread);
      return thread;
    }
    
    #checkThreadExit() {
      if (this.#threads.length === 1 && !this.isResolvedState() && !this.isRejected()) {
        const err = new WebAssemblyRuntimeError(`(component [${this.#componentIdx}]) task [${this.#id}] exited without resolution`);
        throw this.#componentIdx === -1 ? err : _recordTrap(this.#componentIdx, err);
      }
    }
    
    // Unregisters a thread that has finished running
    // (`Task.unregister_thread`), freeing its instance-scoped id.
    removeThread(thread) {
      this.#checkThreadExit();
      this.settleCompletion();
      if (this.#componentIdx !== -1) {
        const cstate = getOrCreateAsyncState(this.#componentIdx);
        cstate.threads.remove(thread.id());
      }
      this.#threads = this.#threads.filter(t => t !== thread);
      if (this.#exited && this.#threads.length === 0) {
        clearCurrentThread(this.#componentIdx, this.#id);
      }
    }
    
    // The thread created alongside this task (always the first
    // one registered). Used by task-level callback dispatch and
    // exit cleanup, which operate on the task rather than an
    // explicitly identified thread.
    defaultThread() { return this.#threads[0]; }
    
    // TODO(threads): this is very inefficient, we can pass along a root task,
    // and ideally do not need this once thread support is in place
    getRootTask() {
      let currentSubtask = this.getParentSubtask();
      let task = this;
      while (currentSubtask) {
        task = currentSubtask.getParentTask();
        currentSubtask = task.getParentSubtask();
      }
      return task;
    }
    
    setPostReturnFn(f) {
      if (!f) { return; }
      if (this.#postReturnFn) { throw new Error('postReturn fn can only be set once'); }
      this.#postReturnFn = f;
    }
    
    setCallbackFn(f, name) {
      if (!f) { return; }
      if (this.#callbackFn) { throw new Error('callback fn can only be set once'); }
      this.#callbackFn = f;
      this.#callbackFnName = name;
    }
    
    getCallbackFnName() {
      if (!this.#callbackFnName) { return undefined; }
      return this.#callbackFnName;
    }
    
    runCallbackFn(...args) {
      if (!this.#callbackFn) { throw new Error('no callback function has been set for task'); }
      if (this.#callbackFn._jcoMaySuspend === false) {
        return _withGlobalCurrentThreadMeta({
          taskID: this.#id,
          threadID: this.defaultThread().id(),
          componentIdx: this.#componentIdx,
          fn: () => this.#callbackFn.apply(null, args),
        });
      }
      return _withGlobalCurrentThreadMetaAsync({
        taskID: this.#id,
        threadID: this.defaultThread().id(),
        componentIdx: this.#componentIdx,
        fn: () => { return this.#callbackFn.apply(null, args); }
      });
    }
    
    getCalleeParams() {
      if (!this.#getCalleeParamsFn) { throw new Error('missing/invalid getCalleeParamsFn'); }
      return this.#getCalleeParamsFn();
    }
    
    // Legacy manually-async exports are sync-typed in the component
    // but use JSPI precisely so their guest stack may suspend.
    // Canonical ABI: only sync-typed callees may not block (`canon_lift`);
    // an async-lifted callee may, even when called through a sync lowering.
    mayBlock() { return this.isAsync() || this.isManualAsync() || this.isResolvedState() || this.#funcTypeIsAsync }
    // (an async-typed function may block however it was lifted)
    setFuncTypeIsAsync(v) { this.#funcTypeIsAsync = v; }
    // The interned result type of the task's function (checked by `task.return`)
    setReturnResultTypeIdx(v) { this.#returnResultTypeIdx = v; }
    returnResultTypeIdx() { return this.#returnResultTypeIdx; }
    
    mayEnter(task) {
      const cstate = getOrCreateAsyncState(this.#componentIdx);
      if (cstate.hasBackpressure()) {
        _debugLog('[AsyncTask#mayEnter()] disallowed due to backpressure', { taskID: this.#id });
        return false;
      }
      if (!cstate.callingSyncImport()) {
        _debugLog('[AsyncTask#mayEnter()] disallowed due to sync import call', { taskID: this.#id });
        return false;
      }
      const callingSyncExportWithSyncPending = cstate.callingSyncExport && !task.isAsync;
      if (!callingSyncExportWithSyncPending) {
        _debugLog('[AsyncTask#mayEnter()] disallowed due to sync export w/ sync pending', { taskID: this.#id });
        return false;
      }
      return true;
    }
    
    enterSync() {
      if (this.needsExclusiveLock()) {
        const cstate = getOrCreateAsyncState(this.#componentIdx);
        if (!cstate.isExclusivelyLocked()) {
          cstate.exclusiveLock(this.#id);
        } else {
          // A host-called sync export arriving while another
          // task's slice holds the lock: synchronous entry
          // cannot wait, and historically this entry silently
          // stole the hold. Run without the lock instead --
          // the holder's bookkeeping stays intact and its
          // release still pairs
          this.#lockFreeEntry = true;
          _debugLog('[AsyncTask#enterSync()] entering without exclusive lock', {
            taskID: this.#id,
            componentIdx: this.#componentIdx,
          });
        }
      }
      return true;
    }
    
    tryEnter() {
      if (this.#entered) {
        throw new Error(`task with ID [${this.#id}] should not be entered twice`);
      }
      
      if (this.deliverPendingCancel({ cancellable: true })) {
        this.cancel();
        return false;
      }
      
      const cstate = getOrCreateAsyncState(this.#componentIdx);
      if (this.isSync()) {
        this.#entered = true;
        return true;
      }
      if (cstate.hasBackpressure()) { return null; }
      // Entries already queued for this instance go first.
      if (cstate.hasPendingAdmissions()) { return null; }
      if (this.needsExclusiveLock()) {
        if (cstate.isExclusivelyLocked()) { return null; }
        cstate.exclusiveLock(this.#id);
      }
      
      if (this.deliverPendingCancel({ cancellable: true })) {
        cstate.exclusiveRelease(this.#id);
        this.cancel();
        return false;
      }
      
      this.#entered = true;
      return true;
    }
    
    enterHost() {
      if (this.#entered) {
        throw new Error(`task with ID [${this.#id}] should not be entered twice`);
      }
      
      // If cancellation was requested before the task was entered, resolve
      // as cancelled without ever running host code
      if (this.deliverPendingCancel({ cancellable: true })) {
        this.cancel();
        return false;
      }
      
      getOrCreateAsyncState(this.#componentIdx);
      
      this.#entered = true;
      // A guest task may be synchronously blocked in this
      // host entry. Propagate that boundary so an async
      // cancellation driver can yield BLOCKED while the host
      // operation is outstanding.
      const parentTask = this.#parentSubtask?.getParentTask();
      if (
      parentTask?.taskState() === AsyncTask.State.CANCEL_DELIVERED
      || (parentTask && !parentTask.hasCallback())
      ) {
        parentTask.notifyProgress();
      }
      return this.#entered;
    }
    
    async enter(opts) {
      _debugLog('[AsyncTask#enter()] args', {
        taskID: this.#id,
        componentIdx: this.#componentIdx,
        subtaskID: this.getParentSubtask()?.id(),
        args: opts,
        entryFnName: this.#entryFnName,
      });
      
      if (opts?.isHost) { return this.enterHost(); }
      
      if (this.#entered) {
        throw new Error(`task with ID [${this.#id}] should not be entered twice`);
      }
      
      // If cancellation was requested before the task was entered, resolve
      // as cancelled without ever running guest code
      if (this.deliverPendingCancel({ cancellable: true })) {
        this.cancel();
        return false;
      }
      
      const cstate = getOrCreateAsyncState(this.#componentIdx);
      
      // NOTE: concurrent task lifetimes within one component instance are
      // permitted by the Component Model: entry is governed by the
      // backpressure and exclusive-lock checks below (the lock is held per
      // execution slice, not for the task's lifetime).
      //
      // Serializing entire task lifetimes here (the former "execution slot" queue)
      // deadlocks pipelines where a parked long-lived task's progress depends on a
      // later entry into the same component.
      
      // If a task is synchronous then we can avoid component-relevant
      // tracking and immediately enter.
      if (this.isSync()) {
        this.#entered = true;
        
        // TODO(breaking): remove once manually-specifying async fns is removed
        // It is currently possible for an actually sync export to be specified
        // as async via JSPI
        if (this.#isManualAsync) {
          if (this.needsExclusiveLock()) { await cstate.acquireExclusiveLock(this.#id); }
        }
        
        return this.#entered;
      }
      
      // Enter in the order the calls were made: queue behind every
      // earlier entry into this instance, whatever it waits for.
      const ticket = cstate.takeAdmissionTicket(this.#id);
      this.#admissionTicket = ticket;
      let entered = false;
      try {
        entered = await this.#enterQueued(cstate, ticket);
        return entered;
      } finally {
        // An entered task keeps the slot until its first slice ends
        // (`endFirstSlice`): the next call is admitted once this one
        // returned, blocked or trapped, as `Store.invoke` runs a call.
        if (!entered) { this.endFirstSlice(); }
      }
    }
    
    // The first slice of this task ended (it returned, blocked or trapped):
    // the next entry into the instance may be admitted (Canonical ABI
    // `Store.invoke` runs a call until it first blocks or returns).
    endFirstSlice() {
      const ticket = this.#admissionTicket;
      if (ticket === null) { return; }
      this.#admissionTicket = null;
      getOrCreateAsyncState(this.#componentIdx).releaseAdmissionTicket(ticket);
    }
    
    async #enterQueued(cstate, ticket) {
      await cstate.waitForAdmissionTurn(ticket);
      // Cancelled before it started while it was queued: the task was
      // retired already and must not park in a wait as a dead entry.
      if (this.isResolvedState() || this.isCancelled()) { return false; }
      
      // Wait until there is no backpressure *and* the exclusive lock (if
      // needed) is ours. Backpressure can be set while we wait for the
      // lock (by the task holding it), so it's checked again once the lock
      // is acquired, as the Canonical ABI's `enter_implicit_thread` waits
      // for both together.
      while (true) {
        if (cstate.hasBackpressure()) {
          cstate.addBackpressureWaiter();
          
          const result = await this.waitUntil({
            readyFn: () => {
              return !cstate.hasBackpressure();
            },
            cancellable: true,
          });
          
          cstate.removeBackpressureWaiter();
          
          if (!result || this.isCancelled()) {
            if (!this.isResolvedState()) { this.cancel(); }
            return false;
          }
        }
        
        // Acquire the per-slice exclusive lock (FIFO-queued when
        // contended); the first slice runs under this hold and the
        // driver loop releases/re-acquires it per slice thereafter.
        if (this.needsExclusiveLock()) {
          await cstate.acquireExclusiveLock(this.#id);
          if (cstate.hasBackpressure()) {
            cstate.exclusiveRelease(this.#id);
            continue;
          }
        }
        break;
      }
      
      // The store may have trapped (e.g. a deadlock was detected) while
      // this task waited to enter: it must not run, and its call fails
      // with that trap.
      try {
        cstate.throwIfInterrupted();
      } catch (err) {
        cstate.exclusiveRelease(this.#id);
        this.setErrored(err);
        return false;
      }
      
      // Cancellation-before-start may resolve this task while its
      // queued lock acquisition is still pending. Acquiring the lock
      // does not make the already-resolved task runnable again.
      if (this.isResolvedState() || this.isCancelled()) {
        cstate.exclusiveRelease(this.#id);
        return false;
      }
      
      // Cancellation can be requested while entry is waiting for
      // backpressure or its exclusive lock. Do not execute the guest
      // after acquiring a lock for a task that should no longer start.
      if (this.deliverPendingCancel({ cancellable: true })) {
        cstate.exclusiveRelease(this.#id);
        this.cancel();
        return false;
      }
      
      this.#entered = true;
      return this.#entered;
    }
    
    isRunningState() { return this.#state !== AsyncTask.State.RESOLVED; }
    isResolvedState() { return this.#state === AsyncTask.State.RESOLVED; }
    isResolved() { return this.#state === AsyncTask.State.RESOLVED; }
    isExited() { return this.#exited; }
    
    async waitUntil(opts) {
      const { readyFn, cancellable } = opts;
      _debugLog('[AsyncTask#waitUntil()] args', { taskID: this.#id, args: { cancellable } });
      
      // TODO(fix): check for cancel
      // TODO(fix): determinism
      
      const keepGoing = await this.suspendUntil({
        readyFn,
        cancellable,
      });
      
      return keepGoing;
    }
    
    async yieldUntil(opts) {
      const { readyFn, cancellable } = opts;
      _debugLog('[AsyncTask#yieldUntil()]', {
        taskID: this.#id,
        args: {
          cancellable,
        },
        componentIdx: this.#componentIdx,
      });
      
      // A callback YIELD is an explicit request to let other work run.
      // Unlike waits whose condition is already ready, it must always
      // suspend for at least one scheduler turn.
      const keepGoing = await this.immediateSuspend({ readyFn, cancellable });
      if (keepGoing) {
        return {
          code: ASYNC_EVENT_CODE.NONE,
          payload0: 0,
          payload1: 0,
        };
      }
      
      return {
        code: ASYNC_EVENT_CODE.TASK_CANCELLED,
        payload0: 0,
        payload1: 0,
      };
    }
    
    async suspendUntil(opts) {
      const { cancellable, readyFn } = opts;
      _debugLog('[AsyncTask#suspendUntil()] args', {
        taskID: this.#id,
        args: {
          cancellable,
        },
        componentIdx: this.#componentIdx,
      });
      
      const pendingCancelled = this.deliverPendingCancel({ cancellable });
      if (pendingCancelled) { return false; }
      
      const completed = await this.immediateSuspendUntil({ readyFn, cancellable });
      return completed;
    }
    
    suspendUntilCallback(opts, onResume) {
      const { cancellable, readyFn } = opts;
      if (this.deliverPendingCancel({ cancellable })) {
        onResume(false);
        return;
      }
      
      const cstate = getOrCreateAsyncState(this.#componentIdx);
      cstate.suspendTask({
        task: this,
        cancellable,
        readyFn: () => {
          if (cancellable && this.#state === AsyncTask.State.CANCEL_PENDING) {
            return true;
          }
          return readyFn();
        },
        onResume: (keepGoing) => {
          if (keepGoing && this.deliverPendingCancel({ cancellable })) {
            keepGoing = false;
          }
          onResume(keepGoing);
        },
      });
    }
    
    // TODO(threads): equivalent to thread.suspend_until()
    async immediateSuspendUntil(opts) {
      const { cancellable, readyFn } = opts;
      _debugLog('[AsyncTask#immediateSuspendUntil()] args', {
        args: {
          cancellable,
          readyFn,
        },
        taskID: this.#id,
        componentIdx: this.#componentIdx,
      });
      
      const ready = readyFn();
      if (ready && ASYNC_DETERMINISM === 'random') {
        const coinFlip = _coinFlip();
        if (coinFlip) { return true }
      }
      
      const keepGoing = await this.immediateSuspend({ cancellable, readyFn });
      return keepGoing;
    }
    
    async immediateSuspend(opts) {
      // Entry can wait on backpressure before a guest register window exists.
      const cstate = getOrCreateAsyncState(this.#componentIdx);
      const thread = this.#entered
      ? cstate.threads.get(_getGlobalCurrentThreadMeta(this.#componentIdx).threadID)
      : this.defaultThread();
      if (!thread || thread.task() !== this) { throw new Error('invalid current thread for wait'); }
      // Suspending a core Wasm stack retains the task's exclusive
      // lock. Only a callback return releases it between slices.
      return thread.suspend({ ...opts, releaseLock: false });
    }
    
    deliverPendingCancel(opts) {
      const { cancellable } = opts;
      _debugLog('[AsyncTask#deliverPendingCancel()]', {
        args: { cancellable },
        taskID: this.#id,
        componentIdx: this.#componentIdx,
      });
      
      if (cancellable && this.#state === AsyncTask.State.CANCEL_PENDING) {
        this.#state = AsyncTask.State.CANCEL_DELIVERED;
        return true;
      }
      
      return false;
    }
    
    isCancelled() { return this.cancelled }
    cancellationRequested() { return this.cancelRequested; }
    cancellationPending() { return this.#state === AsyncTask.State.CANCEL_PENDING; }
    
    // Request cooperative cancellation of this task, called on behalf of a
    // supertask performing `subtask.cancel` on the subtask this task backs.
    //
    // The request is delivered at this task's next cancellable wait
    // (see suspendUntil/immediateSuspend), at which point the task is
    // expected to acknowledge via `task.cancel` or still resolve via
    // `task.return`.
    requestCancellation() {
      _debugLog('[AsyncTask#requestCancellation()] args', {
        taskID: this.#id,
        componentIdx: this.#componentIdx,
        state: this.#state,
      });
      if (this.isResolvedState() || this.cancelRequested) { return; }
      this.cancelRequested = true;
      if (this.#state === AsyncTask.State.INITIAL) {
        this.#state = AsyncTask.State.CANCEL_PENDING;
        // A task still waiting for backpressure or its
        // initial instance lock has no guest thread to
        // resume. Cancellation is therefore delivered
        // and acknowledged eagerly as
        // CANCELLED_BEFORE_STARTED.
        if (!this.#entered) {
          this.deliverPendingCancel({ cancellable: true });
          this.cancel();
          
          const cstate = getOrCreateAsyncState(this.#componentIdx);
          // `enter()` may already be parked either in the scheduler's
          // explicit-backpressure wait or in the exclusive-lock FIFO.
          // Wake/remove that entry work now so it cannot retain a timer,
          // waiter count, or lock-queue slot after cancellation.
          cstate.resumeTaskByID(this.#id);
          cstate.cancelExclusiveLockWaiter(this.#id);
          
          // No guest thread was registered, so no driver loop will call
          // `exit()`. Retire the task's JS bookkeeping explicitly.
          this.exit({ skipExclusiveLockCheck: true });
          return;
        }
      }
      // Nudge the component's tick loop so that any suspended cancellable
      // wait observes the pending cancellation promptly
      getOrCreateAsyncState(this.#componentIdx).runTickLoop();
    }
    
    cancel(args) {
      _debugLog('[AsyncTask#cancel()] args', { });
      if (this.taskState() !== AsyncTask.State.CANCEL_DELIVERED) {
        throw new Error('`task.cancel` called by task which has not been cancelled');
      }
      this.validateResourceBorrowScope();
      this.cancelled = true;
      // Cancelled tasks resolve with no value (spec: `Task.cancel` calls
      // `on_resolve(None)`); an explicit error is only present on the
      // host-driven rejection path (see `reject()`).
      this.onResolve(args?.error ?? null);
      this.#state = AsyncTask.State.RESOLVED;
      // A task cancelled before entry has no driver loop that can
      // report its exit. Entered tasks notify after releasing their
      // component slice in `exit()`.
      if (!this.#entered) { this.notifyProgress(); }
    }
    
    onResolve(taskValue) {
      const handlers = this.#onResolveHandlers;
      this.#onResolveHandlers = [];
      for (const f of handlers) {
        try {
          f(taskValue);
        } catch (err) {
          _debugLog("[AsyncTask#onResolve] error during task resolve handler", err);
          throw err;
        }
      }
      
      // Rejections are control-flow failures, not canonical ABI results.
      // Propagate them through the subtask chain without running return
      // lowering or post-return hooks for a successful result.
      if (this.#rejected) {
        this.#parentSubtask?.reject(taskValue);
        return;
      }
      
      // NOTE: if the parent subtask has already been resolved (e.g. it was
      // cancelled via `subtask.cancel` while this task was still pending),
      // this task's resolution must be discarded rather than delivered.
      const parentSubtaskPending = this.#parentSubtask && !this.#parentSubtask.isResolved();
      const taskReturned = !this.isCancelled();
      
      if (parentSubtaskPending && taskReturned) {
        const meta = this.#parentSubtask.getCallMetadata();
        // Run the rturn fn if it has not already been called -- this *should* have happened in
        // `task.return`, but some paths do not go through task.return (e.g. async lower of sync fn
        // which goes through prepare + async-start-call)
        if (meta.returnFn && !meta.returnFnCalled) {
          _debugLog('[AsyncTask#onResolve()] running returnFn', {
            componentIdx: this.#componentIdx,
            taskID: this.#id,
            subtaskID: this.#parentSubtask.id(),
          });
          const callerTask = this.#parentSubtask.getParentTask();
          _withGlobalCurrentThreadMeta({
            taskID: callerTask.id(),
            threadID: meta.callerThreadID,
            componentIdx: callerTask.componentIdx(),
            fn: () => meta.returnFn.apply(null, [taskValue, meta.resultPtr]),
          });
          meta.returnFnCalled = true;
        }
      }
      
      if (this.#postReturnFn && taskReturned) {
        _debugLog('[AsyncTask#onResolve()] running post return ', {
          componentIdx: this.#componentIdx,
          taskID: this.#id,
        });
        try {
          _withGlobalCurrentThreadMeta({
            taskID: this.#id,
            threadID: this.defaultThread().id(),
            componentIdx: this.#componentIdx,
            fn: () => this.#postReturnFn(taskValue),
          });
        } catch (err) {
          err = _postReturnTrap(this.#componentIdx, err);
          this.reject(err);
          throw err;
        }
      }
      
      if (parentSubtaskPending) {
        this.#parentSubtask.onResolve(taskValue);
      }
    }
    
    registerOnResolveHandler(f) {
      this.#onResolveHandlers.push(f);
    }
    
    isRejected() { return this.#rejected; }
    
    isErrored() { return this.#errored; }
    setErrored(err) {
      // Preserve the originating trap when unwinding through
      // additional guest frames produces secondary traps (often
      // an `unreachable` after a call which was expected to trap).
      if (this.#errored === null) { this.#errored = err; }
    }
    
    reject(taskErr) {
      _debugLog('[AsyncTask#reject()] args', {
        componentIdx: this.#componentIdx,
        taskID: this.#id,
        parentSubtask: this.#parentSubtask,
        parentSubtaskID: this.#parentSubtask?.id(),
        entryFnName: this.entryFnName(),
        callbackFnName: this.#callbackFnName,
        errMsg: taskErr.message,
      });
      
      this.setErrored(taskErr);
      if (this.#rejected) { return; }
      
      // A task may call `task.return` before its callback exits.
      // A trap in cleanup or spawned work must still reject the
      // host call and poison the enclosing task chain.
      if (this.isResolvedState()) {
        this.#rejected = true;
        this.#errored = taskErr;
        const parentTask = this.#parentSubtask?.getParentTask();
        if (parentTask) { parentTask.reject(taskErr); }
        return;
      }
      
      this.#rejected = true;
      this.cancelRequested = true;
      this.#state = AsyncTask.State.CANCEL_PENDING;
      const cancelled = this.deliverPendingCancel({ cancellable: true });
      
      // TODO: do cleanup here to reset the machinery so we can run again?
      
      this.cancel({ error: taskErr });
    }
    
    resolve(results) {
      _debugLog('[AsyncTask#resolve()] args', {
        componentIdx: this.#componentIdx,
        taskID: this.#id,
        entryFnName: this.entryFnName(),
        callbackFnName: this.#callbackFnName,
      });
      
      if (this.#state === AsyncTask.State.RESOLVED) {
        if (this.#errored !== null) { throw this.#errored; }
        throw new Error(`(component [${this.#componentIdx}]) task [${this.#id}]  is already resolved (did you forget to wait for an import?)`);
      }
      
      this.validateResourceBorrowScope();
      
      this.#state = AsyncTask.State.RESOLVED;
      
      switch (results.length) {
        case 0:
        this.onResolve(undefined);
        break;
        case 1:
        this.onResolve(results[0]);
        break;
        default:
        _debugLog('[AsyncTask#resolve()] unexpected number of results', {
          componentIdx: this.#componentIdx,
          results,
          taskID: this.#id,
          subtaskID: this.#parentSubtask?.id(),
          entryFnName: this.#entryFnName,
          callbackFnName: this.#callbackFnName,
        });
        throw new Error('unexpected number of results');
      }
    }
    
    exit(args) {
      _debugLog('[AsyncTask#exit()]', {
        componentIdx: this.#componentIdx,
        taskID: this.#id,
      });
      
      this.endFirstSlice();
      if (this.#exited)  { throw new Error("task has already exited"); }
      
      // Validate before publishing exit or running its handlers.
      // Otherwise a failed exit leaves #exited true, so the driver
      // cannot finish rejection cleanup and settle the host call.
      this.#checkThreadExit();
      this.validateResourceBorrowScope();
      
      // Host (-1) tasks have no component instance (and so no async
      // state or exclusive lock); do not materialise one for them.
      const state = this.#componentIdx === -1
      ? null
      : getOrCreateAsyncState(this.#componentIdx);
      if (!state && this.#componentIdx !== -1) {
        throw new Error('missing async state for component [' + this.#componentIdx + ']');
      }
      
      if (state && !args?.skipExclusiveLockCheck && !this.#lockFreeEntry) {
        if (this.needsExclusiveLock() && !state.exclusivelyLockedBy(this.#id)) {
          throw new Error(`task [${this.#id}] exit: component [${this.#componentIdx}] should have been exclusively locked by it`);
        }
      }
      
      // Ownership-checked: releases only this task's own hold (a
      // task exiting while another task's slice holds the lock no
      // longer clears the foreign hold).
      if (state) { state.exclusiveRelease(this.#id); }
      this.notifyProgress();
      
      for (const f of this.#onExitHandlers) {
        try {
          f();
        } catch (err) {
          console.error("error during task exit handler", err);
          throw err;
        }
      }
      
      this.#exited = true;
      this.removeThread(this.defaultThread());
      if (this.#threads.length > 0) {
        clearCurrentThread(this.#componentIdx, this.#id, { preserveTask: true });
      }
    }
    
    needsExclusiveLock() {
      // Host (-1) tasks model host-side import handling: there is no
      // guest linear memory or executor state to protect, and host
      // calls from unrelated guest components would contend spuriously.
      if (this.#componentIdx === -1) { return false; }
      // Import-handler tasks (CallInterface) run host code nested
      // inside the calling guest slice, which already holds the
      // lock; only tasks that execute guest slices need it.
      if (!this.#callingWasmExport) { return false; }
      // Automatic backpressure only applies to async function
      // types. A sync-typed call may run ready callback threads
      // before returning, so holding this lock would deadlock it.
      if (!this.#funcTypeIsAsync) { return false; }
      // A sync-lifted callee cannot be reentered until the whole
      // stackful call returns. This is the Component Model's
      // automatic-backpressure rule for async-lowered calls.
      return !this.#isAsync || this.hasCallback() || this.#calleeIsAsync === false;
    }
    
    createSubtask(args) {
      _debugLog('[AsyncTask#createSubtask()] args', args);
      const { componentIdx, childTask, callMetadata, fnName, isAsync, isManualAsync } = args;
      
      const cstate = getOrCreateAsyncState(this.#componentIdx);
      if (!cstate) {
        throw new Error(`invalid/missing async state for component idx [${componentIdx}]`);
      }
      
      const waitable = new Waitable({
        componentIdx: this.#componentIdx,
        target: `subtask (internal ID [${this.#id}])`,
      });
      
      const newSubtask = new AsyncSubtask({
        componentIdx,
        childTask,
        parentTask: this,
        callMetadata,
        isAsync,
        isManualAsync,
        fnName,
        waitable,
      });
      this.#subtasks.push(newSubtask);
      newSubtask.setTarget(`subtask (internal ID [${newSubtask.id()}], waitable [${waitable.idx()}], component [${componentIdx}])`);
      // The subtask gets a handle index in this task's table only once the
      // guest is given it (see `AsyncSubtask#ensureHandle`).
      return newSubtask;
    }
    
    getLatestSubtask() {
      return this.#subtasks.at(-1);
    }
    
    getSubtaskByWaitableRep(rep) {
      if (rep === undefined) { throw new TypeError('missing rep'); }
      return this.#subtasks.find(s => s.waitableRep() === rep);
    }
    
    currentSubtask() {
      _debugLog('[AsyncTask#currentSubtask()]');
      if (this.#subtasks.length === 0) { return undefined; }
      return this.#subtasks.at(-1);
    }
    
    removeSubtask(subtask) {
      if (this.#subtasks.length === 0) {
        throw new Error('cannot end current subtask: no current subtask');
      }
      this.#subtasks = this.#subtasks.filter(t => t !== subtask);
      return subtask;
    }
  }
  
  class AsyncThread {
    static State = {
      // Explicitly spawned (`thread.new-indirect`) threads start
      // here; the implicit thread of a task skips straight to
      // RUNNING since it is the task's own entry.
      NOT_STARTED: 'not-started',
      RUNNING: 'running',
      COMPLETED: 'completed',
    }
    
    // Component-instance-scoped id (the canonical ABI's
    // `Thread.index`) -- globally unique within the instance,
    // *not* per task, since a thread of one task may be
    // targeted (e.g. by `thread.resume-later`) by code
    // running in a different task of the same instance.
    // Assigned post-construction by `Task#addThread()`, which
    // is the only code allowed to insert into the instance's
    // thread table.
    #id = null;
    #componentIdx;
    #task;
    #state;
    #startFn = null;
    storage = [0, 0];
    
    constructor (opts) {
      if (opts?.componentIdx === undefined) {
        throw new TypeError('missing component id during thread creation');
      }
      this.#componentIdx = opts.componentIdx;
      
      if (!opts?.task) {
        throw new TypeError('missing task during thread creation');
      }
      this.#task = opts.task;
      
      // Explicitly spawned threads provide a `startFn` and begin
      // NOT_STARTED; the task's own implicit thread has none and
      // is already running by the time it's constructed.
      if (opts.startFn) {
        this.#startFn = opts.startFn;
        this.#state = AsyncThread.State.NOT_STARTED;
      } else {
        this.#state = AsyncThread.State.RUNNING;
      }
    }
    
    id() { return this.#id; }
    setId(id) {
      if (this.#id !== null) { throw new Error('thread id can only be assigned once'); }
      this.#id = id;
    }
    componentIdx() { return this.#componentIdx; }
    task() { return this.#task; }
    taskID() { return this.#task.id(); }
    state() { return this.#state; }
    
    // Delegate task-scoped bookkeeping that doesn't yet have a
    // thread-specific notion (cancellation, progress waiters,
    // the subtask this task is answering) to the owning task.
    isCancelled() { return this.#task.isCancelled(); }
    isRejected() { return this.#task.isRejected(); }
    notifyProgress() { return this.#task.notifyProgress(); }
    entryFnName() { return this.#task.entryFnName(); }
    getParentSubtask() { return this.#task.getParentSubtask(); }
    
    async suspend(opts) {
      const cstate = getOrCreateAsyncState(this.#componentIdx);
      const task = this.#task;
      task.endFirstSlice();
      task.settleCompletion();
      const { cancellable, readyFn, releaseLock = false } = opts;
      if (task.deliverPendingCancel({ cancellable })) { return false; }
      const reacquireLock = releaseLock && task.needsExclusiveLock() && cstate.exclusivelyLockedBy(task.id());
      if (reacquireLock) { cstate.exclusiveRelease(task.id()); }
      try {
        const keepGoing = await cstate.suspendThread({
          ...opts,
          thread: this,
          readyFn: () => (cancellable && task.cancellationPending()) || readyFn(),
        });
        cstate.throwIfInterrupted();
        if (keepGoing && task.deliverPendingCancel({ cancellable })) { return false; }
        return keepGoing;
      } finally {
        if (reacquireLock && !cstate.exclusivelyLockedBy(task.id())) {
          await cstate.acquireExclusiveLock(task.id());
        }
      }
    }
    
    // Begins running a NOT_STARTED (explicitly spawned) thread's
    // start function. Runs detached -- the caller (`thread.resume-later`)
    // does not wait for the thread to finish or even reach its first
    // suspension point.
    start() {
      if (this.#state !== AsyncThread.State.NOT_STARTED) {
        throw new Error(`cannot start thread [${this.#id}] in state [${this.#state}]`);
      }
      this.#state = AsyncThread.State.RUNNING;
      const startFn = this.#startFn;
      this.#startFn = null;
      
      _withGlobalCurrentThreadMetaAsync({
        threadID: this.#id,
        taskID: this.#task.id(),
        componentIdx: this.#componentIdx,
        fn: () => {
          getOrCreateAsyncState(this.#componentIdx).throwIfInterrupted();
          return startFn();
        },
      }).then(() => {
        this.#state = AsyncThread.State.COMPLETED;
        this.#task.removeThread(this);
      }).catch((err) => {
        err = _recordTrap(this.#componentIdx, err);
        this.#state = AsyncThread.State.COMPLETED;
        _debugLog('[AsyncThread#start()] spawned thread errored', {
          threadID: this.#id,
          taskID: this.#task.id(),
          componentIdx: this.#componentIdx,
          err,
        });
        this.#task.setErrored(err);
        this.#task.reject(err);
        this.#task.removeThread(this);
      });
    }
  }
  
  function createNewCurrentImplicitThread(args) {
    _debugLog('[createNewCurrentImplicitThread()] args', args);
    const {
      componentIdx,
      isAsync,
      isManualAsync,
      preserveFutureResult,
      entryFnName,
      parentSubtaskID,
      callbackFnName,
      getCallbackFn,
      getParamsFn,
      stringEncoding,
      errHandling,
      getCalleeParamsFn,
      resultPtr,
      callingWasmExport,
    } = args;
    if (componentIdx === undefined || componentIdx === null) {
      throw new Error('missing/invalid component instance index while starting task');
    }
    let taskMetas = ASYNC_TASKS_BY_COMPONENT_IDX.get(componentIdx);
    const callbackFn = getCallbackFn ? getCallbackFn() : null;
    
    const newTask = new AsyncTask({
      componentIdx,
      isAsync,
      isManualAsync,
      preserveFutureResult,
      entryFnName,
      callbackFn,
      callbackFnName,
      stringEncoding,
      getCalleeParamsFn,
      resultPtr,
      errHandling,
      callingWasmExport,
    });
    
    const newTaskID = newTask.id();
    const newTaskMeta = { id: newTaskID, componentIdx, task: newTask };
    
    // Every task starts with a single implicit thread, registered
    // the same way as explicitly spawned ones (`thread.new-indirect`):
    // `addThread` assigns its real, instance-scoped id.
    const newThread = newTask.addThread(new AsyncThread({
      componentIdx,
      task: newTask,
    }));
    const newThreadID = newThread.id();
    
    // NOTE: do not track host tasks
    ASYNC_CURRENT_TASK_IDS.push(newTaskID);
    ASYNC_CURRENT_COMPONENT_IDXS.push(componentIdx);
    
    if (!taskMetas) {
      taskMetas = [newTaskMeta];
      ASYNC_TASKS_BY_COMPONENT_IDX.set(componentIdx, [newTaskMeta]);
    } else {
      taskMetas.push(newTaskMeta);
    }
    
    return { task: newTask, thread: newThread, taskID: newTaskID, threadID: newThreadID };
  }
  
  function _checkMayLeave(componentIdx) {
    if (INSTANCE_FLAGS.get(componentIdx)?.value !== 1) {
      throw new WebAssemblyRuntimeError('cannot leave component instance');
    }
  }
  
  function _setGlobalCurrentThreadMeta(args) {
    if (!args) { throw new TypeError('args missing'); }
    if (args.taskID === undefined) { throw new TypeError('missing task ID'); }
    if (args.componentIdx === undefined) { throw new TypeError('missing component idx'); }
    if (args.threadID === undefined) { throw new TypeError('missing thread ID'); }
    const { taskID, threadID, componentIdx } = args;
    return CURRENT_TASK_META.current =
    CURRENT_TASK_META[componentIdx] = { threadID, taskID, componentIdx };
  }
  
  
  async function _clearCurrentTask(args) {
    _debugLog('[_clearCurrentTask()] args', args);
    if (!args) { throw new TypeError('args missing'); }
    if (args.taskID === undefined) { throw new TypeError('missing task ID'); }
    if (args.threadID === undefined) { throw new TypeError('missing thread ID'); }
    if (args.componentIdx === undefined) { throw new TypeError('missing component idx'); }
    const { taskID, threadID, componentIdx } = args;
    
    const meta = CURRENT_TASK_META[componentIdx];
    if (!meta) { throw new Error(`missing current thread meta for component idx [${componentIdx}]`); }
    
    if (meta.threadID !== threadID) {
      throw new Error(`thread ID [${meta.threadID}] != requested ID [${threadID}]`);
    }
    if (meta.taskID !== taskID) {
      throw new Error(`task ID [${meta.taskID}] != requested ID [${taskID}]`);
    }
    if (meta.componentIdx !== componentIdx) {
      throw new Error(`component idx [${meta.componentIdx}] != requested idx [${componentIdx}]`);
    }
    
    CURRENT_TASK_META[componentIdx] = null;
    if (CURRENT_TASK_META.current?.threadID === threadID &&
    CURRENT_TASK_META.current?.taskID === taskID) {
      CURRENT_TASK_META.current = null;
    }
  }
  
  function _lowerImportBackwardsCompat(args) {
    const params = [...arguments].slice(1);
    _debugLog('[_lowerImportBackwardsCompat()] args', { args, params });
    const {
      functionIdx,
      componentIdx,
      isAsync,
      isManualAsync,
      paramLiftFns,
      resultLowerFns,
      hasResultPointer,
      funcTypeIsAsync,
      metadata,
      memoryIdx,
      getMemoryFn,
      getReallocFn,
      importFn,
      stringEncoding,
    } = args;
    
    _checkMayLeave(componentIdx);
    
    const meta = _getGlobalCurrentThreadMeta(componentIdx);
    let taskMeta = meta && getCurrentTask(componentIdx, meta.taskID);
    let createdTask;
    
    // Some components depend on initialization logic (i.e. `_initialize` or some such
    // core wasm export) that is embedded in the component, but is not executed or wizer'd
    // away before the transpiled component is attempted to be used.
    //
    // These components execut their initialization logic *when they are imported* in the
    // transpiled context -- so we may get a call to an export that is lowered without going
    // through `CallWasm` or `CallInterface`.
    //
    // A nested synchronous call can leave global metadata for a task
    // that has already exited. Treat it like a call with no current task.
    if (!taskMeta) {
      if (funcTypeIsAsync || (isAsync && !isManualAsync)) {
        throw new Error('p3 async wasm exports cannot use backwards compat auto-task init');
      }
      
      const { task: newTask, taskID: newTaskID, threadID: newThreadID } = createNewCurrentImplicitThread({
        componentIdx,
        isAsync,
        isManualAsync,
        callingWasmExport: false,
      });
      createdTask = newTask;
      
      // Since we're managing the task creation ourselves we must clear ourselves
      createdTask.registerOnResolveHandler(() => {
        _clearCurrentTask({
          taskID: task.id(),
          threadID: newThreadID,
          componentIdx: task.componentIdx(),
        });
      });
      
      _setGlobalCurrentThreadMeta({
        componentIdx,
        taskID: newTaskID,
        threadID: newThreadID,
      });
      
      taskMeta = getCurrentTask(componentIdx, newTaskID);
    }
    if (!taskMeta) {
      throw new Error('invalid/missing async task meta');
    }
    
    const task = taskMeta.task;
    if (!task) { throw new Error('invalid/missing async task'); }
    
    const cstate = getOrCreateAsyncState(componentIdx);
    
    if (!task.mayBlock() && funcTypeIsAsync && !isAsync) {
      throw new Error("non async exports cannot synchronously call async functions");
    }
    
    // If there is an existing task, this should be part of a subtask
    const memory = getMemoryFn();
    // Canonical ABI lower appends result storage as a trailing
    // param when async lower has any flat result, or sync lower
    // has more than one flat result.
    const resultPtr = hasResultPointer ? params[params.length - 1] >>> 0 : undefined;
    const subtask = task.createSubtask({
      componentIdx,
      parentTask: task,
      fnName: importFn.fnName,
      isAsync,
      isManualAsync,
      callMetadata: {
        memoryIdx,
        memory,
        realloc: getReallocFn?.(),
        getReallocFn,
        resultPtr,
        lowers: resultLowerFns,
        funcTypeIsAsync,
        stringEncoding,
      }
    });
    task.setReturnMemoryIdx(memoryIdx);
    task.setReturnMemory(getMemoryFn());
    
    subtask.onStart();
    
    // If dealing with a sync lowered sync function, we can directly return results
    //
    // TODO(breaking): remove once we get rid of manual async import specification,
    // as func types cannot be detected in that case only (and we don't need that w/ p3)
    if (!isManualAsync && !isAsync && !funcTypeIsAsync) {
      if (createdTask) { createdTask.enterSync(); }
      
      const res = importFn(...params);
      
      // TODO(breaking): remove once we get rid of manual async import specification,
      // as func types cannot be detected in that case only (and we don't need that w/ p3)
      if (!funcTypeIsAsync && !subtask.isReturned()) {
        throw new Error('post-execution subtasks must either be async or returned');
      }
      
      const syncRes = subtask.getResult();
      if (createdTask) { createdTask.resolve([syncRes]); }
      
      return syncRes;
    }
    
    // Sync-lowered async functions requires async behavior because the callee *can* block,
    // but this call must *act* synchronously and return immediately with the result
    // (i.e. not returning until the work is done)
    //
    // TODO(breaking): remove checking for manual async specification here, once we can go p3-only
    //
    if (!isManualAsync && !isAsync && funcTypeIsAsync) {
      const { promise, resolve, reject } = promiseWithResolvers();
      queueMicrotask(async () => {
        try {
          await importFn(...params);
          if (!subtask.isResolved()) {
            await task.suspendUntil({ readyFn: () => subtask.isResolved() });
          }
          resolve(subtask.getResult());
        } catch (err) {
          reject(err);
        }
      });
      return promise;
    }
    
    // NOTE: at this point we know that we are working with an async lowered import
    
    const subtaskState = subtask.getStateNumber();
    if (subtaskState < 0 || subtaskState >= 2**4) {
      throw new Error('invalid subtask state, out of valid range');
    }
    
    subtask.setOnProgressFn(() => {
      subtask.setPendingEvent(() => {
        if (subtask.isResolved()) { subtask.deliverResolve(); }
        const event = {
          code: ASYNC_EVENT_CODE.SUBTASK,
          payload0: subtask.waitableRep(),
          payload1: subtask.getStateNumber(),
        }
        return event;
      });
    });
    
    // This is a hack to maintain backwards compatibility with
    // manually-specified async imports, used in wasm exports that are
    // not actually async (but are specified as so).
    //
    // This is not normal p3 sync behavior but instead anticipating that
    // the caller that is doing manual async will be given the *actual* result,
    // or a promise that resolves to it.
    //
    // TODO(breaking): remove once manually specified async is removed
    //
    // There are a few cases:
    // 1. sync function with async types (e.g. `f: func() -> stream<u32>`)
    // 2. async function with async types (e.g. `f: async func() -> stream<u32>`)
    // 3. async function with sync types (e.g. `f: async func() -> list<u32>`)
    // 4. sync function with non-async types (e.g. `f: func() -> list<u32>`)
    //
    // This hack *only* applies to 4 -- the case where an async JS host function
    // is supplied to a Wasm export which does *not* need to do any async abi
    // lifting/lowering (async ABI did not exist when JSPI integratiton was
    // initially merged to enable asynchronously returning values from the host)
    //
    // The host is called now rather than in a microtask, so that a host which
    // answers synchronously is answered synchronously: a task that cannot block
    // (e.g. a resource destructor) traps on a promise.
    if (!isAsync && !funcTypeIsAsync && isManualAsync) {
      _debugLog('[_lowerImportBackwardsCompat()] calling lowered import', { importFn, params });
      if (createdTask) { createdTask.enterSync(); }
      try {
        const res = importFn(...params);
        const isPromise = res !== null &&
        (typeof res === 'object' || typeof res === 'function') &&
        typeof res.then === 'function';
        // Without a current task the call cannot block, and its caller
        // traps on a promise, so a task created for it ends now.
        if (createdTask) { createdTask.resolve([isPromise ? undefined : res]); }
        if (!isPromise) { return subtask.getResult(); }
        return Promise.resolve(res).then(() => subtask.getResult());
      } catch (err) {
        if (createdTask) { createdTask.reject(err); }
        throw err;
      } finally {
        // Resolution delivers the result; exit retires the implicit
        // thread and removes the task from the component's registry.
        if (createdTask) { createdTask.exit(); }
      }
    }
    
    queueMicrotask(async () => {
      try {
        _debugLog('[_lowerImportBackwardsCompat()] calling lowered import', { importFn, params });
        if (createdTask) { await createdTask.enter(); }
        
        const asyncRes = await importFn(...params);
        if (createdTask) { createdTask.resolve([asyncRes]); }
      } catch (err) {
        _debugLog("[_lowerImportBackwardsCompat()] import fn error:", err);
        task.setErrored(err);
        task.reject(err);
      }
    });
    
    _debugLog('[_lowerImportBackwardsCompat()] async-lowered import return', {
      fnName: importFn.fnName,
      componentIdx,
      subtaskID: subtask.id(),
      waitableRep: subtask.waitableRep(),
      subtaskState,
      packedResult: Number(subtask.waitableRep()) << 4 | subtaskState,
    });
    
    return Number(subtask.ensureHandle()) << 4 | subtaskState;
  }
  
  const CURRENT_TASK_MAY_BLOCK= typeof WebAssembly !== 'undefined' ? new WebAssembly.Global({ value: 'i32', mutable: true }, 0) : false;
  
  
  function _liftFlatBool(ctx) {
    _debugLog('[_liftFlatBool()] args', { ctx });
    let val;
    
    if (ctx.useDirectParams) {
      if (ctx.params.length === 0) { throw new Error('expected at least a single i32 argument'); }
      val = ctx.params[0] === 1;
      ctx.params = ctx.params.slice(1);
      return [val, ctx];
    }
    
    if (ctx.storageLen !== undefined && ctx.storageLen < 1) {
      throw new Error(`insufficient storage ([${ctx.storageLen}] bytes) for lift (bool requires 1 byte)`);
    }
    
    val = new DataView(ctx.memory.buffer).getUint8(ctx.storagePtr, true) === 1;
    
    ctx.storagePtr += 1;
    if (ctx.storageLen !== undefined) { ctx.storageLen -= 1; }
    
    return [val, ctx];
  }
  
  
  function _liftFlatU8(ctx) {
    _debugLog('[_liftFlatU8()] args', { ctx });
    let val;
    
    if (ctx.useDirectParams) {
      if (ctx.params.length === 0) { throw new Error('expected at least a single i32 argument'); }
      val = ctx.params[0];
      ctx.params = ctx.params.slice(1);
      return [val, ctx];
    }
    
    if (ctx.storageLen !== undefined && ctx.storageLen < 1) {
      throw new Error(`insufficient storage ([${ctx.storageLen}] bytes) for lift (u8 requires 1 byte)`);
    }
    
    val = new DataView(ctx.memory.buffer).getUint8(ctx.storagePtr, true);
    
    ctx.storagePtr += 1;
    if (ctx.storageLen !== undefined) { ctx.storageLen -= 1; }
    
    return [val, ctx];
  }
  
  
  function _liftFlatU16(ctx) {
    _debugLog('[_liftFlatU16()] args', { ctx });
    let val;
    
    if (ctx.useDirectParams) {
      if (ctx.params.length === 0) { throw new Error('expected at least a single i32 argument'); }
      val = ctx.params[0];
      ctx.params = ctx.params.slice(1);
      return [val, ctx];
    }
    
    if (ctx.storageLen !== undefined && ctx.storageLen < 2) {
      throw new Error(`insufficient storage ([${ctx.storageLen}] bytes) for lift (u16 requires 2 bytes)`);
    }
    
    val = new DataView(ctx.memory.buffer).getUint16(ctx.storagePtr, true);
    
    ctx.storagePtr += 2;
    if (ctx.storageLen !== undefined) { ctx.storageLen -= 2; }
    
    const rem = ctx.storagePtr % 2;
    if (rem !== 0) { ctx.storagePtr += (2 - rem); }
    
    return [val, ctx];
  }
  
  
  function _liftFlatS32(ctx) {
    _debugLog('[_liftFlatS32()] args', { ctx });
    let val;
    
    if (ctx.useDirectParams) {
      if (ctx.params.length === 0) { throw new Error('expected at least a single i32 argument'); }
      val = ctx.params[0];
      ctx.params = ctx.params.slice(1);
      return [val, ctx];
    }
    
    if (ctx.storageLen !== undefined && ctx.storageLen < 4) {
      throw new Error(`insufficient storage ([${ctx.storageLen}] bytes) for lift (s32 requires 4 bytes)`);
    }
    
    val = new DataView(ctx.memory.buffer).getInt32(ctx.storagePtr, true);
    ctx.storagePtr += 4;
    if (ctx.storageLen !== undefined) { ctx.storageLen -= 4; }
    
    return [val, ctx];
  }
  
  
  function _liftFlatU32(ctx) {
    _debugLog('[_liftFlatU32()] args', { ctx });
    let val;
    
    if (ctx.useDirectParams) {
      if (ctx.params.length === 0) { throw new Error('expected at least a single i34 argument'); }
      // core i32 values arrive as signed numbers
      val = ctx.params[0] >>> 0;
      ctx.params = ctx.params.slice(1);
      return [val, ctx];
    }
    
    if (ctx.storageLen !== undefined && ctx.storageLen < 4) {
      throw new Error(`insufficient storage ([${ctx.storageLen}] bytes) for lift (u32 requires 4 bytes)`);
    }
    val = new DataView(ctx.memory.buffer).getUint32(ctx.storagePtr, true);
    ctx.storagePtr += 4;
    if (ctx.storageLen !== undefined) { ctx.storageLen -= 4; }
    
    return [val, ctx];
  }
  
  
  function _liftFlatU64(ctx) {
    _debugLog('[_liftFlatU64()] args', { ctx });
    let val;
    
    if (ctx.useDirectParams) {
      if (ctx.params.length === 0) { throw new Error('expected at least one single i64 argument'); }
      if (typeof ctx.params[0] !== 'bigint') { throw new Error('expected bigint'); }
      // core i64 values arrive as signed BigInts
      val = BigInt.asUintN(64, ctx.params[0]);
      ctx.params = ctx.params.slice(1);
      return [val, ctx];
    }
    
    if (ctx.storageLen !== undefined && ctx.storageLen < 8) {
      throw new Error(`insufficient storage ([${ctx.storageLen}] bytes) for lift (u64 requires 8 bytes)`);
    }
    
    val = new DataView(ctx.memory.buffer).getBigUint64(ctx.storagePtr, true);
    ctx.storagePtr += 8;
    if (ctx.storageLen !== undefined) { ctx.storageLen -= 8; }
    
    return [val, ctx];
  }
  
  
  function _liftFlatFloat64(ctx) {
    _debugLog('[_liftFlatFloat64()] args', { ctx });
    let val;
    
    if (ctx.useDirectParams) {
      if (ctx.params.length === 0) {
        throw new Error('expected at least one single f64 argument');
      }
      val = ctx.params[0];
      ctx.params = ctx.params.slice(1);
      
      return [val, ctx];
    }
    
    if (ctx.storageLen !== undefined && ctx.storageLen < 8) {
      throw new Error(`insufficient storage ([${ctx.storageLen}] bytes) for lift (f64 requires 8 bytes)`);
    }
    
    val = new DataView(ctx.memory.buffer).getFloat64(ctx.storagePtr, true);
    ctx.storagePtr += 8;
    if (ctx.storageLen !== undefined) { ctx.storageLen -= 8; }
    
    return [val, ctx];
  }
  
  
  function _liftFlatStringUTF8(ctx) {
    _debugLog('[_liftFlatStringUTF8()] args', { ctx });
    let val;
    
    if (ctx.useDirectParams) {
      if (ctx.params.length < 2) { throw new Error('expected at least two u32 arguments'); }
      let offset = ctx.params[0];
      if (typeof offset === 'bigint') { offset = Number(offset); } else { offset >>>= 0; }
      if (!Number.isSafeInteger(offset)) { throw new Error('invalid offset'); }
      const len = ctx.params[1];
      if (!Number.isSafeInteger(len)) {  throw new Error('invalid len'); }
      val = TEXT_DECODER_UTF8.decode(new DataView(ctx.memory.buffer, offset, len));
      ctx.params = ctx.params.slice(2);
      return [val, ctx];
    }
    
    const rem = ctx.storagePtr % 4;
    if (rem !== 0) { ctx.storagePtr += (4 - rem); }
    
    const dv = new DataView(ctx.memory.buffer);
    const start = dv.getUint32(ctx.storagePtr, true);
    const codeUnits = dv.getUint32(ctx.storagePtr + 4, true);
    
    val = TEXT_DECODER_UTF8.decode(new Uint8Array(ctx.memory.buffer, start, codeUnits));
    
    ctx.storagePtr += 8;
    if (ctx.storageLen !== undefined) { ctx.storageLen -= 8; }
    
    return [val, ctx];
  }
  
  function _liftFlatStringUTF16(ctx) {
    _debugLog('[_liftFlatStringUTF16()] args', { ctx });
    let val;
    
    if (ctx.useDirectParams) {
      if (ctx.params.length < 2) { throw new Error('expected at least two u32 arguments'); }
      let offset = ctx.params[0];
      if (typeof offset === 'bigint') { offset = Number(offset); } else { offset >>>= 0; }
      if (!Number.isSafeInteger(offset)) {  throw new Error('invalid offset'); }
      const len = ctx.params[1];
      if (!Number.isSafeInteger(len)) {  throw new Error('invalid len'); }
      val = utf16Decoder.decode(new DataView(ctx.memory.buffer, offset, len * 2));
      ctx.params = ctx.params.slice(2);
      return [val, ctx];
    }
    
    const rem = ctx.storagePtr % 4;
    if (rem !== 0) { ctx.storagePtr += (4 - rem); }
    
    const dv = new DataView(ctx.memory.buffer);
    const start = dv.getUint32(ctx.storagePtr, true);
    const codeUnits = dv.getUint32(ctx.storagePtr + 4, true);
    
    val = utf16Decoder.decode(new DataView(ctx.memory.buffer, start, codeUnits * 2));
    
    ctx.storagePtr += 8;
    if (ctx.storageLen !== undefined) { ctx.storageLen -= 8; }
    
    return [val, ctx];
  }
  
  function _liftFlatStringAny(ctx) {
    switch (ctx.stringEncoding) {
      case 'utf8':
      return _liftFlatStringUTF8(ctx);
      case 'utf16':
      return _liftFlatStringUTF16(ctx);
      case 'compact-utf16': {
        const [ptr, ptrCtx] = _liftFlatU32(ctx);
        const [len, newCtx] = _liftFlatU32(ptrCtx);
        return [_compactUtf16Decode(ctx.memory, ptr, len), newCtx];
      }
      default:
      throw new Error(`missing/unrecognized/unsupported string encoding [${ctx.stringEncoding}]`);
    }
  }
  
  const _liftFlatVariantScratch = new DataView(new ArrayBuffer(8));
  
  function _liftFlatVariant(meta) {
    const {
      caseMetas,
      variantSize32,
      variantAlign32,
      variantPayloadOffset32,
      variantFlatCount,
      variantPayloadFlatTypes,
      isEnum,
    } = meta;
    
    return function _liftFlatVariantInner(ctx) {
      _debugLog('[_liftFlatVariant()] args', { ctx });
      const origUseParams = ctx.useDirectParams;
      
      let caseIdx;
      let liftRes;
      const originalPtr = ctx.storagePtr;
      const numCases =  caseMetas.length;
      if (caseMetas.length < 256) {
        liftRes = _liftFlatU8(ctx);
      } else if (numCases >= 256 && numCases < 65536) {
        liftRes = _liftFlatU16(ctx);
      } else if (numCases >= 65536 && numCases < 4_294_967_296) {
        liftRes = _liftFlatU32(ctx);
      } else {
        throw new Error(`unsupported number of variant cases [${numCases}]`);
      }
      caseIdx = liftRes[0];
      ctx = liftRes[1];
      
      const [
      tag,
      liftFn,
      caseSize32,
      caseAlign32,
      caseFlatCount,
      caseFlatTypes,
      ] = caseMetas[caseIdx];
      
      if (variantPayloadOffset32 === undefined) {
        throw new Error('unexpectedly missing payload offset');
      }
      
      if (originalPtr !== undefined) {
        ctx.storagePtr = originalPtr + variantPayloadOffset32;
      }
      
      let val;
      if (liftFn === null) {
        val = { tag };
        // NOTE: here we need to move past the entire object in memory
        // despite moving to the payload which we now know is missing/unnecessary
        if (originalPtr !== undefined) {
          ctx.storagePtr = originalPtr + variantSize32;
        }
      } else {
        // When lifting from direct params, the payload arrives as the
        // *join* of all case flat representations: each slot whose
        // joined core type differs from the selected case's core type
        // must be reinterpreted before the payload lift
        // (see CanonicalABI `lift_flat_variant`)
        if (ctx.useDirectParams) {
          if (!variantPayloadFlatTypes || !caseFlatTypes) {
            throw new Error('missing variant flat type metadata during direct-param lift');
          }
          const scratch = _liftFlatVariantScratch;
          for (let i = 0; i < caseFlatTypes.length; i++) {
            const have = variantPayloadFlatTypes[i];
            const want = caseFlatTypes[i];
            if (have === want) { continue; }
            const val = ctx.params[i];
            if (have === 'i64' && want === 'i32') {
              ctx.params[i] = Number(BigInt.asIntN(32, val));
            } else if (have === 'i64' && want === 'f32') {
              scratch.setInt32(0, Number(BigInt.asIntN(32, val)), true);
              ctx.params[i] = scratch.getFloat32(0, true);
            } else if (have === 'i64' && want === 'f64') {
              scratch.setBigInt64(0, val, true);
              ctx.params[i] = scratch.getFloat64(0, true);
            } else if (have === 'i32' && want === 'f32') {
              scratch.setInt32(0, val, true);
              ctx.params[i] = scratch.getFloat32(0, true);
            } else {
              throw new Error(`invalid variant payload coercion [${have}] -> [${want}]`);
            }
          }
        }
        
        const [newVal, newCtx] = liftFn(ctx);
        val = { tag, val: newVal };
        ctx = newCtx;
      }
      
      if (origUseParams) {
        if (variantFlatCount === undefined || variantFlatCount === null) {
          _debugLog('[_liftFlatVariant()] variant with unknown flat count', { ctx, meta });
          throw new Error('cannot lift variant with unknown flat count');
        }
        if (caseFlatCount === undefined || caseFlatCount === null) {
          _debugLog('[_liftFlatVariant()] case with unknown flat count', { ctx, meta, case: meta.caseMetas[caseIdx] });
          throw new Error('cannot lift case with unknown flat count');
        }
        // NOTE: enums can be tightly packed and do not have a descriminant
        const remainingPayloadParams = variantFlatCount - caseFlatCount - (isEnum ? 0 : 1);
        if (remainingPayloadParams < 0) {
          throw new Error(`invalid variant flat count metadata`);
        }
        if (ctx.params.length < remainingPayloadParams) {
          throw new Error(`expected at least [${remainingPayloadParams}] remaining variant payload params, but got [${ctx.params.length}]`);
        }
        ctx.params = ctx.params.slice(remainingPayloadParams);
      }
      
      if (ctx.storagePtr !== undefined) {
        const rem = ctx.storagePtr % variantAlign32;
        if (rem !== 0) { ctx.storagePtr += variantAlign32 - rem; }
      }
      
      return [val, ctx];
    }
  }
  
  function _liftFlatList(meta) {
    const { elemLiftFn, elemSize32, elemAlign32, knownLen, typedArray } = meta;
    
    const listValue =
    typedArray === undefined
    ? values => values
    : values => new typedArray(values);
    
    const readValuesAndReset = (ctx, originalPtr, originalLen, dataPtr, len) => {
      if (
      dataPtr < 0 || len < 0 ||
      (elemSize32 !== 0 && len > Math.floor(((1 << 28) - 1) / elemSize32)) ||
      dataPtr > ctx.memory.buffer.byteLength ||
      (elemSize32 !== 0 && len > Math.floor((ctx.memory.buffer.byteLength - dataPtr) / elemSize32))
      ) {
        throw new WebAssemblyRuntimeError('wasm trap: out of bounds memory access');
      }
      if (dataPtr % elemAlign32 !== 0) {
        throw new TypeError(`list pointer [${dataPtr}] is not aligned to ${elemAlign32}`);
      }
      ctx.storagePtr = dataPtr;
      const val = [];
      for (var i = 0; i < len; i++) {
        const elemPtr = dataPtr + i * elemSize32;
        ctx.storagePtr = elemPtr;
        const [res, nextCtx] = elemLiftFn(ctx);
        val.push(res);
        ctx = nextCtx;
        
        ctx.storagePtr = Math.max(ctx.storagePtr, elemPtr + elemSize32);
      }
      if (originalPtr !== null) { ctx.storagePtr = originalPtr; }
      if (originalLen !== null) { ctx.storageLen = originalLen; }
      return [listValue(val), ctx];
    };
    
    return function _liftFlatListInner(ctx) {
      _debugLog('[_liftFlatList()] args', { ctx });
      
      let liftResults;
      if (knownLen !== undefined) { // list with known length
      if (ctx.useDirectParams) {
        _debugLog('memory unexpectedly missing while lifting unknown length list', { ctx });
        liftResults = [listValue(ctx.params.slice(0, knownLen)), ctx];
        ctx.params = ctx.params.slice(knownLen);
      } else { // indirect params
      if (ctx.memory === null) {
        _debugLog('memory unexpectedly missing while lifting known length list', { knownLen, ctx });
        throw new Error(`memory missing while lifting known length (${knownLen}) list`);
      }
      
      const originalLen = ctx.storageLen;
      const originalPtr = ctx.storagePtr;
      
      ctx.storageLen = knownLen * elemSize32;
      liftResults = readValuesAndReset(ctx, null, originalLen, ctx.storagePtr, knownLen);
    }
    
  } else { // unknown length list
  
  if (ctx.useDirectParams) {
    // unknown length list ptr w/ direct params
    const dataPtr = ctx.params[0] >>> 0;
    const len = ctx.params[1];
    ctx.params = ctx.params.slice(2);
    
    ctx.useDirectParams = false;
    const originalPtr = ctx.storagePtr;
    const originalLen = ctx.storageLen;
    ctx.storageLen = len * elemSize32;
    
    liftResults = readValuesAndReset(ctx, originalPtr, originalLen, dataPtr, len);
    
    ctx.useDirectParams = true;
  } else {
    // unknown length list ptr w/ in-memory params
    const originalLen = ctx.storageLen;
    ctx.storageLen = 8;
    
    const dataPtrLiftRes = _liftFlatU32(ctx);
    const dataPtr = dataPtrLiftRes[0];
    ctx = dataPtrLiftRes[1];
    
    const lenLiftRes = _liftFlatU32(ctx);
    const len = lenLiftRes[0];
    ctx = lenLiftRes[1];
    
    const originalPtr = ctx.storagePtr;
    ctx.storagePtr = dataPtr;
    
    ctx.storageLen = len * elemSize32;
    liftResults = readValuesAndReset(ctx, originalPtr, originalLen, dataPtr, len);
  }
}

return liftResults;
}
}

function _liftFlatOption(meta) {
  const { payloadMaybeNull } = meta;
  const f = _liftFlatVariant(meta);
  return function _liftFlatOptionInner(ctx) {
    _debugLog('[_liftFlatOption()] args', { ctx });
    const res = f(ctx);
    if (!payloadMaybeNull) {
      res[0] = res[0].tag === 'none' ? undefined : res[0].val;
    }
    return res;
  }
}

function _liftFlatResult(meta) {
  const f = _liftFlatVariant(meta);
  return function _liftFlatResultInner(ctx) {
    _debugLog('[_liftFlatResult()] args', { ctx });
    const res = f(ctx);
    if (!('val' in res[0])) { res[0].val = undefined; }
    return res;
  }
}

function _liftFlatBorrow(componentTableIdx, size, memory, vals, storagePtr, storageLen) {
  _debugLog('[_liftFlatBorrow()] args', { size, memory, vals, storagePtr, storageLen });
  throw new Error('flat lift for borrowed resources is not supported!');
}


function _lowerFlatBool(ctx) {
  _debugLog('[_lowerFlatBool()] args', { ctx });
  
  if (!ctx.memory) { throw new Error("missing memory for lower"); }
  if (ctx.vals.length !== 1) {
    throw new Error(`unexpected number [${ctx.vals.length}] of vals (expected 1)`);
  }
  
  
  new DataView(ctx.memory.buffer).setUint8(ctx.storagePtr, ctx.vals[0] ? 1 : 0);
  
  ctx.storagePtr += 1;
}

function _lowerFlatU8(ctx) {
  _debugLog('[_lowerFlatU8()] args', ctx);
  
  if (ctx.vals.length !== 1) {
    throw new Error(`unexpected number [${ctx.vals.length}] of vals (expected 1)`);
  }
  
  
  
  if (!ctx.memory) { throw new Error("missing memory for lower"); }
  new DataView(ctx.memory.buffer).setUint8(ctx.storagePtr, ctx.vals[0]);
  
  ctx.storagePtr += 1;
}

function _lowerFlatS16(ctx) {
  _debugLog('[_lowerFlatS16()] args', { ctx });
  
  if (!ctx.memory) { throw new Error("missing memory for lower"); }
  if (ctx.vals.length !== 1) {
    throw new Error(`unexpected number [${ctx.vals.length}] of vals (expected 1)`);
  }
  
  const rem = ctx.storagePtr % 2;
  if (rem !== 0) { ctx.storagePtr += (2 - rem); }
  
  
  new DataView(ctx.memory.buffer).setInt16(ctx.storagePtr, ctx.vals[0], true);
  
  ctx.storagePtr += 2;
}

function _lowerFlatU16(ctx) {
  _debugLog('[_lowerFlatU16()] args', { ctx });
  
  if (!ctx.memory) { throw new Error("missing memory for lower"); }
  if (ctx.vals.length !== 1) {
    throw new Error(`unexpected number [${ctx.vals.length}] of vals (expected 1)`);
  }
  
  const rem = ctx.storagePtr % 2;
  if (rem !== 0) { ctx.storagePtr += (2 - rem); }
  
  
  new DataView(ctx.memory.buffer).setUint16(ctx.storagePtr, ctx.vals[0], true);
  
  ctx.storagePtr += 2;
}

function _lowerFlatS32(ctx) {
  _debugLog('[_lowerFlatS32()] args', { ctx });
  
  if (ctx.vals.length !== 1) {
    throw new Error(`unexpected number [${ctx.vals.length}] of vals (expected 1)`);
  }
  
  const rem = ctx.storagePtr % 4;
  if (rem !== 0) { ctx.storagePtr += (4 - rem); }
  
  
  new DataView(ctx.memory.buffer).setInt32(ctx.storagePtr, ctx.vals[0], true);
  
  ctx.storagePtr += 4;
}

function _lowerFlatU32(ctx) {
  _debugLog('[_lowerFlatU32()] args', { ctx });
  
  if (ctx.vals.length !== 1) {
    throw new Error(`expected single value to lower, got [${ctx.vals.length}]`);
  }
  
  const rem = ctx.storagePtr % 4;
  if (rem !== 0) { ctx.storagePtr += (4 - rem); }
  
  
  new DataView(ctx.memory.buffer).setUint32(ctx.storagePtr, ctx.vals[0], true);
  
  ctx.storagePtr += 4;
}

function _lowerFlatU64(ctx) {
  _debugLog('[_lowerFlatU64()] args', { ctx });
  
  if (ctx.vals.length !== 1) { throw new Error('unexpected number of vals'); }
  
  const rem = ctx.storagePtr % 8;
  if (rem !== 0) { ctx.storagePtr += (8 - rem); }
  
  
  new DataView(ctx.memory.buffer).setBigUint64(ctx.storagePtr, ctx.vals[0], true);
  
  ctx.storagePtr += 8;
}

function _lowerFlatFloat32(ctx) {
  _debugLog('[_lowerFlatFloat32()] args', { ctx });
  
  if (ctx.vals.length !== 1) { throw new Error('unexpected number of vals'); }
  
  const rem = ctx.storagePtr % 4;
  if (rem !== 0) { ctx.storagePtr += (4 - rem); }
  
  
  new DataView(ctx.memory.buffer).setFloat32(ctx.storagePtr, ctx.vals[0], true);
  
  ctx.storagePtr += 4;
}

function _lowerFlatFloat64(ctx) {
  _debugLog('[_lowerFlatFloat64()] args', { ctx });
  
  if (ctx.vals.length !== 1) { throw new Error('unexpected number of vals'); }
  
  const rem = ctx.storagePtr % 8;
  if (rem !== 0) { ctx.storagePtr += (8 - rem); }
  
  
  new DataView(ctx.memory.buffer).setFloat64(ctx.storagePtr, ctx.vals[0], true);
  
  ctx.storagePtr += 8;
}

function _lowerFlatStringUTF8(ctx) {
  _debugLog('[_lowerFlatStringUTF8()] args', ctx);
  if (!ctx.realloc) { throw new Error('missing realloc during flat string lower'); }
  
  const { ptr, len } = _utf8AllocateAndEncode(ctx.vals[0], ctx.realloc, ctx.memory);
  
  const view = new DataView(ctx.memory.buffer);
  view.setUint32(ctx.storagePtr, ptr, true);
  view.setUint32(ctx.storagePtr + 4, len, true);
  
  ctx.storagePtr += 8;
}

function _lowerFlatStringUTF16(ctx) {
  _debugLog('[_lowerFlatStringUTF16()] args', { ctx });
  if (!ctx.realloc) { throw new Error('missing realloc during flat string lower'); }
  
  const { ptr, len } = _utf16AllocateAndEncode(ctx.vals[0], ctx.realloc, ctx.memory);
  
  const view = new DataView(ctx.memory.buffer);
  view.setUint32(ctx.storagePtr, ptr, true);
  view.setUint32(ctx.storagePtr + 4, len, true);
  
  ctx.storagePtr += 8;
}

function _lowerFlatStringAny(ctx) {
  switch (ctx.stringEncoding) {
    case 'utf8':
    return _lowerFlatStringUTF8(ctx);
    case 'utf16':
    return _lowerFlatStringUTF16(ctx);
    case 'compact-utf16': {
      if (!ctx.realloc) { throw new Error('missing realloc during flat string lower'); }
      const { ptr, len } = _compactUtf16AllocateAndEncode(ctx.vals[0], ctx.realloc, ctx.memory);
      const view = new DataView(ctx.memory.buffer);
      view.setUint32(ctx.storagePtr, ptr, true);
      view.setUint32(ctx.storagePtr + 4, len, true);
      ctx.storagePtr += 8;
      return;
    }
    default:
    throw new Error(`missing/unrecognized/unsupported string encoding [${ctx.stringEncoding}]`);
  }
}

function _lowerFlatRecord(meta) {
  const { fieldMetas, size32: recordSize32, align32: recordAlign32 } = meta;
  return function _lowerFlatRecordInner(ctx) {
    _debugLog('[_lowerFlatRecord()] args', { ctx });
    
    const originalPtr = ctx.storagePtr;
    const r = ctx.vals[0];
    for (const [tag, lowerFn, size32, align32 ] of fieldMetas) {
      const rem = ctx.storagePtr % align32;
      if (rem !== 0) { ctx.storagePtr += align32 - rem; }
      
      const fieldPtr = ctx.storagePtr;
      ctx.vals = [r[tag]];
      lowerFn(ctx);
      
      ctx.storagePtr = Math.max(ctx.storagePtr, fieldPtr + size32);
    }
    
    ctx.storagePtr = Math.max(ctx.storagePtr, originalPtr + recordSize32);
    
    const rem = ctx.storagePtr % recordAlign32;
    if (rem !== 0) {
      ctx.storagePtr += recordAlign32 - rem;
    }
  }
}

function _lowerFlatVariant(meta) {
  const { variantSize32, variantAlign32, variantPayloadOffset32, caseMetas } = meta;
  
  let caseLookup = {};
  for (const [idx, meta] of caseMetas.entries()) {
    let tag = meta[0];
    caseLookup[tag] = { discriminant: idx, meta };
  }
  
  return function _lowerFlatVariantInner(ctx) {
    _debugLog('[_lowerFlatVariant()] args', { ctx });
    
    const { tag, val } = ctx.vals[0];
    const variantCase = caseLookup[tag];
    if (!variantCase) {
      throw new Error(`missing tag [${tag}] (valid tags: ${Object.keys(caseLookup)})`);
    }
    
    const [ _tag, lowerFn, caseSize32, caseAlign32, caseFlatCount ] = variantCase.meta;
    
    const originalPtr = ctx.storagePtr;
    ctx.vals = [variantCase.discriminant];
    let discLowerRes;
    if (caseMetas.length < 256) {
      discLowerRes = _lowerFlatU8(ctx);
    } else if (caseMetas.length >= 256 && caseMetas.length < 65536) {
      discLowerRes = _lowerFlatU16(ctx);
    } else if (caseMetas.length >= 65536 && caseMetas.length < 4_294_967_296) {
      discLowerRes = _lowerFlatU32(ctx);
    } else {
      throw new Error(`unsupported number of cases [${caseMetas.length}]`);
    }
    
    const payloadOffsetPtr = originalPtr + variantPayloadOffset32;
    ctx.storagePtr = payloadOffsetPtr;
    ctx.vals = [val];
    if (lowerFn) { lowerFn(ctx); }
    
    ctx.storagePtr = Math.max(ctx.storagePtr, originalPtr + variantSize32);
    
    const rem = ctx.storagePtr % variantAlign32;
    if (rem !== 0) { ctx.storagePtr += variantAlign32 - rem; }
  }
}

function _lowerFlatList(meta) {
  const {
    elemLowerFn,
    knownLen,
    size32,
    align32,
    elemSize32,
    elemAlign32,
  } = meta;
  
  if (!elemLowerFn) { throw new TypeError("missing/invalid element lower fn for list"); }
  
  return function _lowerFlatListInner(ctx) {
    _debugLog('[_lowerFlatList()] args', { ctx });
    
    if (ctx.useDirectParams) {
      if (ctx.params.length < 2) { throw new Error('insufficient params left to lower list'); }
      const storagePtr = ctx.params[0] >>> 0;
      const elemCount = ctx.params[1];
      ctx.params = ctx.params.slice(2);
      
      const list = ctx.vals[0];
      if (!list) { throw new Error("missing direct param value"); }
      
      const lowerCtx = {
        storagePtr,
        memory: ctx.memory,
        stringEncoding: ctx.stringEncoding,
      };
      for (let idx = 0; idx < list.length; idx++) {
        const elemPtr = storagePtr + idx * elemSize32;
        lowerCtx.storagePtr = elemPtr;
        lowerCtx.vals = list.slice(idx, idx+1);
        elemLowerFn(lowerCtx);
        lowerCtx.storagePtr = Math.max(lowerCtx.storagePtr, elemPtr + elemSize32);
      }
      ctx.storagePtr = lowerCtx.storagePtr;
      
      // TODO: implement parma-only known-length processing
      
      return;
    }
    
    // TODO(fix): is it possible to get a vals that are a addr and length here from
    // a component lower?
    
    const elems = ctx.vals[0];
    if (knownLen === undefined) {
      // unknown length
      if (!ctx.realloc) { throw new Error('missing realloc during flat string lower'); }
      const dataPtr = ctx.realloc(0, 0, elemAlign32, elemSize32 * elems.length);
      
      ctx.vals[0] = dataPtr;
      _lowerFlatU32(ctx);
      
      ctx.vals[0] = elems.length;
      _lowerFlatU32(ctx);
      
      const origPtr = ctx.storagePtr;
      ctx.storagePtr = dataPtr;
      
      for (const [idx, elem] of elems.entries()) {
        const elemPtr = dataPtr + idx * elemSize32;
        ctx.storagePtr = elemPtr;
        ctx.vals = [elem];
        elemLowerFn(ctx);
        ctx.storagePtr = Math.max(ctx.storagePtr, elemPtr + elemSize32);
      }
      
      ctx.storagePtr = origPtr;
      
    } else {
      // known length
      
      if (elems.length !== knownLen) {
        throw new TypeError(`invalid list input of length [${elems.length}], must be length [${knownLen}]`);
      }
      
      const originalPtr = ctx.storagePtr;
      for (const [idx, elem] of elems.entries()) {
        const elemPtr = originalPtr + idx * elemSize32;
        ctx.storagePtr = elemPtr;
        ctx.vals = [elem];
        elemLowerFn(ctx);
        ctx.storagePtr = Math.max(ctx.storagePtr, elemPtr + elemSize32);
      }
    }
    
    // TODO(fix): special case for u8/u16/etc, we can do a direct copy
    
    const totalSizeBytes = elems.length * size32;
    if (ctx.storageLen !== undefined && totalSizeBytes > ctx.storageLen) {
      throw new Error('not enough storage remaining for list flat lower');
    }
  }
}

function _lowerFlatTuple(meta) {
  const { elemLowerMetas, size32: tupleSize32, align32: tupleAlign32 } = meta;
  return function _lowerFlatTupleInner(ctx) {
    _debugLog('[_lowerFlatTuple()] args', { ctx });
    const originalPtr = ctx.storagePtr;
    const tuple = ctx.vals[0];
    for (const [idx, [ lowerFn, size32, align32 ]]  of elemLowerMetas.entries()) {
      const rem = ctx.storagePtr % align32;
      if (rem !== 0) { ctx.storagePtr += align32 - rem; }
      
      const elemPtr = ctx.storagePtr;
      ctx.vals = [tuple[idx]];
      lowerFn(ctx);
      ctx.storagePtr = Math.max(ctx.storagePtr, elemPtr + size32);
    }
    
    ctx.storagePtr = Math.max(ctx.storagePtr, originalPtr + tupleSize32);
    
    const rem = ctx.storagePtr % tupleAlign32;
    if (rem !== 0) {
      ctx.storagePtr += tupleAlign32 - rem;
    }
  }
}

function _lowerFlatOption(meta) {
  const { payloadMaybeNull } = meta;
  const f = _lowerFlatVariant(meta);
  return function _lowerFlatOptionInner(ctx) {
    _debugLog('[_lowerFlatOption()] args', { ctx });
    
    const v = ctx.vals[0];
    if (v === null || v === undefined) {
      ctx.vals[0] = { tag: 'none' };
    } else if (payloadMaybeNull) {
      const isNotOptionObject = typeof v !== 'object'
      || Object.keys(v).length !== 2
      || !('tag' in v)
      || !(v.tag === 'some' || v.tag === 'none')
      || !('val' in v);
      if (isNotOptionObject) {
        ctx.vals[0] = { tag: 'some', val: v };
      }
    } else {
      ctx.vals[0] = { tag: 'some', val: v };
    }
    
    f(ctx);
  }
}

function _lowerFlatResult(meta) {
  const f = _lowerFlatVariant(meta);
  return function _lowerFlatResultInner(ctx) {
    _debugLog('[_lowerFlatResult()] args', { ctx });
    
    const v = ctx.vals[0];
    const isNotResultObject = typeof v !== 'object'
    || Object.keys(v).length !== 2
    || !('tag' in v)
    || !('ok' === v.tag || 'err' === v.tag)
    || !('val' in v);
    if (isNotResultObject) {
      ctx.vals[0] = { tag: 'ok', val: v };
    }
    
    f(ctx);
  };
}

function _lowerFlatOwn(meta) {
  const { lowerFn, componentIdx, tableIdx } = meta;
  
  return function _lowerFlatOwnInner(ctx) {
    _debugLog('[_lowerFlatOwn()] args', { ctx });
    const { createFn } = ctx;
    
    if (ctx.componentIdx !== componentIdx) {
      throw new Error(`component index mismatch (expected [${componentIdx}], lift called from [${ctx.componentIdx}])`);
    }
    
    const obj = ctx.vals[0];
    if (obj === undefined || obj === null) { throw new Error('missing resource'); }
    const handle = ctx.lowerResource
    ? ctx.lowerResource(obj, tableIdx)
    : lowerFn(obj);
    
    ctx.vals[0] = handle;
    _lowerFlatU32(ctx);
  };
}

function _trackHostOperation(operation) {
  const result = operation();
  if (result === null ||
  (typeof result !== 'object' && typeof result !== 'function') ||
  typeof result.then !== 'function') {
    return result;
  }
  
  STORE_ASYNC_STATE.pendingHostOperations++;
  const tracked = Promise.resolve(result).finally(() => {
    STORE_ASYNC_STATE.pendingHostOperations--;
    if (STORE_ASYNC_STATE.pendingHostOperations < 0) {
      throw new Error('negative pending host operation count');
    }
    for (const state of ASYNC_STATE.values()) { state.runTickLoop(); }
    _checkForDeadlock();
  });
  
  // Forward disposal to the host's value, so it is notified if a guest
  // discards the value (e.g. a future) produced from this operation
  for (const sym of [Symbol.asyncDispose, symbolDispose]) {
    if (typeof sym === 'symbol' && typeof result[sym] === 'function') {
      tracked[sym] = () => result[sym]();
    }
  }
  
  return tracked;
}

function _guardMayLeave(componentIdx, fn) {
  return function (...args) {
    _checkMayLeave(componentIdx);
    return fn.apply(this, args);
  };
}

function _guardTrap(componentIdx, fn, convertsRangeErrors = false) {
  return function (...args) {
    try {
      return fn.apply(this, args);
    } catch (err) {
      throw _recordTrap(componentIdx, err, convertsRangeErrors);
    }
  };
}

const base64Compile = str => WebAssembly.compile(
typeof Buffer !== 'undefined'
? Buffer.from(str, 'base64')
: Uint8Array.from(atob(str), b => b.charCodeAt(0))
);


const isNode = typeof process !== 'undefined' && process.versions && process.versions.node;
let _fs;
async function fetchCompile (url) {
  if (isNode) {
    _fs = _fs || await import('node:fs/promises');
    return WebAssembly.compile(await _fs.readFile(url));
  }
  return fetch(url).then(WebAssembly.compileStreaming);
}

const symbolCabiDispose = Symbol.for('cabiDispose');

const symbolRscHandle = Symbol('handle');

const symbolRscRep = Symbol.for('cabiRep');

const HANDLE_TABLES= [];


class ComponentError extends Error {
  constructor (value) {
    const enumerable = typeof value !== 'string';
    super(enumerable ? `${String(value)} (see error.payload)` : value);
    Object.defineProperty(this, 'payload', { value, enumerable });
  }
}

const hasOwnProperty = Object.prototype.hasOwnProperty;

function getErrorPayload(e) {
  if (e && hasOwnProperty.call(e, 'payload')) return e.payload;
  if (e instanceof Error) throw e;
  return e;
}

function throwInvalidBool() {
  throw new TypeError('invalid variant discriminant for bool');
}

const instantiateCore = WebAssembly.instantiate;

function _suspendingImport(componentIdx, fn, syncOnly = false, switchesTask = false, convertsRangeErrors = true, callsHost = false) {
  return function (...args) {
    _checkMayLeave(componentIdx);
    const saved = CURRENT_TASK_META[componentIdx] ?? null;
    
    const savedThread = saved
    ? getCurrentThread(saved.componentIdx, saved.threadID, saved.taskID)
    : null;
    const mayBlock = savedThread
    ? (savedThread?.task.mayBlock() ?? (CURRENT_TASK_MAY_BLOCK.value !== 0))
    : false;
    if (!saved && !mayBlock && !callsHost) {
      throw new WebAssemblyRuntimeError('cannot block a synchronous task before returning');
    }
    
    if (syncOnly || !mayBlock) {
      let result;
      try {
        result = fn.apply(null, args);
      } catch (err) {
        CURRENT_TASK_META[componentIdx] = saved;
        if (!switchesTask) { CURRENT_TASK_META.current = saved; }
        // A trap is recorded before it propagates: nothing else in
        // the store may run after it, not even a queued call.
        throw _recordTrap(componentIdx, err, convertsRangeErrors);
      }
      
      CURRENT_TASK_META[componentIdx] = saved;
      if (!switchesTask) { CURRENT_TASK_META.current = saved; }
      
      if (result !== null &&
      (typeof result === 'object' || typeof result === 'function') &&
      typeof result.then === 'function') {
        // The helper may already have returned a rejected promise.
        // Mark it handled before replacing it with the canonical
        // synchronous-task trap.
        Promise.resolve(result).catch(() => {});
        throw new WebAssemblyRuntimeError('cannot block a synchronous task before returning');
      }
      return result;
    }
    
    return (async () => {
      try {
        const result = await fn.apply(null, args);
        // The instance trapped while this stack was suspended: it
        // must not resume into guest code (Canonical ABI).
        getOrCreateAsyncState(componentIdx).throwIfInterrupted();
        return result;
      } catch (err) {
        throw _recordTrap(componentIdx, err, convertsRangeErrors);
      } finally {
        CURRENT_TASK_META[componentIdx] = saved;
        if (!switchesTask) { CURRENT_TASK_META.current = saved; }
      }
    })();
  };
}


let exports0;

const _trampoline0 = function(arg0, arg1) {
  _debugLog('[iface="tairitsu-browser:full/platform-helpers@0.2.0", function="set-timeout"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'setTimeout',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => setTimeout(BigInt.asUintN(64, BigInt(arg0)), arg1)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/platform-helpers@0.2.0", function="set-timeout"][Instruction::Return]', {
    funcName: 'set-timeout',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toInt32(ret)]);
  task.exit();
  return toInt32(ret);
}
_trampoline0.fnName = 'tairitsu-browser:full/platform-helpers@0.2.0#setTimeout';

const _trampoline1 = function(arg0, arg1) {
  _debugLog('[iface="tairitsu-browser:full/node@0.2.0", function="remove-child"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'removeChild',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => removeChild(BigInt.asUintN(64, BigInt(arg0)), BigInt.asUintN(64, BigInt(arg1)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/node@0.2.0", function="remove-child"][Instruction::Return]', {
    funcName: 'remove-child',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toUint64(ret)]);
  task.exit();
  return toUint64(ret);
}
_trampoline1.fnName = 'tairitsu-browser:full/node@0.2.0#removeChild';

const _trampoline2 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/event-target@0.2.0", function="prevent-default"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'preventDefault',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => preventDefault(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/event-target@0.2.0", function="prevent-default"][Instruction::Return]', {
    funcName: 'prevent-default',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline2.fnName = 'tairitsu-browser:full/event-target@0.2.0#preventDefault';

const _trampoline3 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/event-target@0.2.0", function="stop-propagation"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'stopPropagation',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => stopPropagation(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/event-target@0.2.0", function="stop-propagation"][Instruction::Return]', {
    funcName: 'stop-propagation',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline3.fnName = 'tairitsu-browser:full/event-target@0.2.0#stopPropagation';

const _trampoline4 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/element-css-inline-style@0.2.0", function="get-style"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getStyle',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getStyle(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/element-css-inline-style@0.2.0", function="get-style"][Instruction::Return]', {
    funcName: 'get-style',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toUint64(ret)]);
  task.exit();
  return toUint64(ret);
}
_trampoline4.fnName = 'tairitsu-browser:full/element-css-inline-style@0.2.0#getStyle';

const _trampoline5 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/platform-helpers@0.2.0", function="request-animation-frame"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'requestAnimationFrame',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => requestAnimationFrame(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/platform-helpers@0.2.0", function="request-animation-frame"][Instruction::Return]', {
    funcName: 'request-animation-frame',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toUint32(ret)]);
  task.exit();
  return toUint32(ret);
}
_trampoline5.fnName = 'tairitsu-browser:full/platform-helpers@0.2.0#requestAnimationFrame';

const _trampoline6 = function(arg0, arg1) {
  _debugLog('[iface="tairitsu-browser:full/platform-helpers@0.2.0", function="set-interval"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'setInterval',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => setInterval(BigInt.asUintN(64, BigInt(arg0)), arg1)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/platform-helpers@0.2.0", function="set-interval"][Instruction::Return]', {
    funcName: 'set-interval',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toInt32(ret)]);
  task.exit();
  return toInt32(ret);
}
_trampoline6.fnName = 'tairitsu-browser:full/platform-helpers@0.2.0#setInterval';

const _trampoline7 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/platform-helpers@0.2.0", function="clear-timeout"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'clearTimeout',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => clearTimeout(arg0)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/platform-helpers@0.2.0", function="clear-timeout"][Instruction::Return]', {
    funcName: 'clear-timeout',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline7.fnName = 'tairitsu-browser:full/platform-helpers@0.2.0#clearTimeout';

const _trampoline8 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/platform-helpers@0.2.0", function="clear-interval"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'clearInterval',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => clearInterval(arg0)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/platform-helpers@0.2.0", function="clear-interval"][Instruction::Return]', {
    funcName: 'clear-interval',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline8.fnName = 'tairitsu-browser:full/platform-helpers@0.2.0#clearInterval';

const _trampoline9 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/platform-helpers@0.2.0", function="cancel-animation-frame"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'cancelAnimationFrame',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => cancelAnimationFrame(arg0 >>> 0)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/platform-helpers@0.2.0", function="cancel-animation-frame"][Instruction::Return]', {
    funcName: 'cancel-animation-frame',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline9.fnName = 'tairitsu-browser:full/platform-helpers@0.2.0#cancelAnimationFrame';

const _trampoline10 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="get-class-list"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getClassList',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getClassList(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="get-class-list"][Instruction::Return]', {
    funcName: 'get-class-list',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toUint64(ret)]);
  task.exit();
  return toUint64(ret);
}
_trampoline10.fnName = 'tairitsu-browser:full/element@0.2.0#getClassList';

const _trampoline11 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="get-scroll-top"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getScrollTop',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getScrollTop(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="get-scroll-top"][Instruction::Return]', {
    funcName: 'get-scroll-top',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([+ret]);
  task.exit();
  return +ret;
}
_trampoline11.fnName = 'tairitsu-browser:full/element@0.2.0#getScrollTop';

const _trampoline12 = function(arg0, arg1) {
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="set-scroll-top"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'setScrollTop',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => setScrollTop(BigInt.asUintN(64, BigInt(arg0)), arg1)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="set-scroll-top"][Instruction::Return]', {
    funcName: 'set-scroll-top',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline12.fnName = 'tairitsu-browser:full/element@0.2.0#setScrollTop';

const _trampoline13 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="get-client-height"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getClientHeight',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getClientHeight(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="get-client-height"][Instruction::Return]', {
    funcName: 'get-client-height',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toInt32(ret)]);
  task.exit();
  return toInt32(ret);
}
_trampoline13.fnName = 'tairitsu-browser:full/element@0.2.0#getClientHeight';

const _trampoline14 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="get-scroll-height"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getScrollHeight',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getScrollHeight(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="get-scroll-height"][Instruction::Return]', {
    funcName: 'get-scroll-height',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toInt32(ret)]);
  task.exit();
  return toInt32(ret);
}
_trampoline14.fnName = 'tairitsu-browser:full/element@0.2.0#getScrollHeight';

const _trampoline15 = function() {
  _debugLog('[iface="tairitsu-browser:full/window@0.2.0", function="get-inner-width"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getInnerWidth',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getInnerWidth()),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/window@0.2.0", function="get-inner-width"][Instruction::Return]', {
    funcName: 'get-inner-width',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toInt32(ret)]);
  task.exit();
  return toInt32(ret);
}
_trampoline15.fnName = 'tairitsu-browser:full/window@0.2.0#getInnerWidth';

const _trampoline16 = function(arg0, arg1) {
  _debugLog('[iface="tairitsu-browser:full/node@0.2.0", function="append-child"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'appendChild',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => appendChild(BigInt.asUintN(64, BigInt(arg0)), BigInt.asUintN(64, BigInt(arg1)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/node@0.2.0", function="append-child"][Instruction::Return]', {
    funcName: 'append-child',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toUint64(ret)]);
  task.exit();
  return toUint64(ret);
}
_trampoline16.fnName = 'tairitsu-browser:full/node@0.2.0#appendChild';

const _trampoline17 = function(arg0, arg1, arg2, arg3) {
  let variant0;
  switch (arg2) {
    case 0: {
      variant0 = undefined;
      break;
    }
    case 1: {
      variant0 = BigInt.asUintN(64, BigInt(arg3));
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for option');
    }
  }
  _debugLog('[iface="tairitsu-browser:full/node@0.2.0", function="insert-before"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'insertBefore',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => insertBefore(BigInt.asUintN(64, BigInt(arg0)), BigInt.asUintN(64, BigInt(arg1)), variant0)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/node@0.2.0", function="insert-before"][Instruction::Return]', {
    funcName: 'insert-before',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toUint64(ret)]);
  task.exit();
  return toUint64(ret);
}
_trampoline17.fnName = 'tairitsu-browser:full/node@0.2.0#insertBefore';

const _trampoline18 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/node@0.2.0", function="get-child-nodes"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getChildNodes',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getChildNodes(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/node@0.2.0", function="get-child-nodes"][Instruction::Return]', {
    funcName: 'get-child-nodes',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toUint64(ret)]);
  task.exit();
  return toUint64(ret);
}
_trampoline18.fnName = 'tairitsu-browser:full/node@0.2.0#getChildNodes';

const _trampoline19 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/node-list@0.2.0", function="get-length"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getLength',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getLength(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/node-list@0.2.0", function="get-length"][Instruction::Return]', {
    funcName: 'get-length',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toUint32(ret)]);
  task.exit();
  return toUint32(ret);
}
_trampoline19.fnName = 'tairitsu-browser:full/node-list@0.2.0#getLength';

const _trampoline20 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-client-x"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getClientX',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getClientX(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-client-x"][Instruction::Return]', {
    funcName: 'get-client-x',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([+ret]);
  task.exit();
  return +ret;
}
_trampoline20.fnName = 'tairitsu-browser:full/mouse-event@0.2.0#getClientX';

const _trampoline21 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-client-y"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getClientY',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getClientY(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-client-y"][Instruction::Return]', {
    funcName: 'get-client-y',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([+ret]);
  task.exit();
  return +ret;
}
_trampoline21.fnName = 'tairitsu-browser:full/mouse-event@0.2.0#getClientY';

const _trampoline22 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-screen-x"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getScreenX',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getScreenX(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-screen-x"][Instruction::Return]', {
    funcName: 'get-screen-x',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([+ret]);
  task.exit();
  return +ret;
}
_trampoline22.fnName = 'tairitsu-browser:full/mouse-event@0.2.0#getScreenX';

const _trampoline23 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-screen-y"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getScreenY',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getScreenY(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-screen-y"][Instruction::Return]', {
    funcName: 'get-screen-y',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([+ret]);
  task.exit();
  return +ret;
}
_trampoline23.fnName = 'tairitsu-browser:full/mouse-event@0.2.0#getScreenY';

const _trampoline24 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-offset-x"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getOffsetX',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getOffsetX(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-offset-x"][Instruction::Return]', {
    funcName: 'get-offset-x',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([+ret]);
  task.exit();
  return +ret;
}
_trampoline24.fnName = 'tairitsu-browser:full/mouse-event@0.2.0#getOffsetX';

const _trampoline25 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-offset-y"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getOffsetY',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getOffsetY(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-offset-y"][Instruction::Return]', {
    funcName: 'get-offset-y',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([+ret]);
  task.exit();
  return +ret;
}
_trampoline25.fnName = 'tairitsu-browser:full/mouse-event@0.2.0#getOffsetY';

const _trampoline26 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-page-x"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getPageX',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getPageX(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-page-x"][Instruction::Return]', {
    funcName: 'get-page-x',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([+ret]);
  task.exit();
  return +ret;
}
_trampoline26.fnName = 'tairitsu-browser:full/mouse-event@0.2.0#getPageX';

const _trampoline27 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-page-y"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getPageY',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getPageY(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-page-y"][Instruction::Return]', {
    funcName: 'get-page-y',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([+ret]);
  task.exit();
  return +ret;
}
_trampoline27.fnName = 'tairitsu-browser:full/mouse-event@0.2.0#getPageY';

const _trampoline28 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-button"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getButton',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getButton(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-button"][Instruction::Return]', {
    funcName: 'get-button',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toInt16(ret)]);
  task.exit();
  return toInt16(ret);
}
_trampoline28.fnName = 'tairitsu-browser:full/mouse-event@0.2.0#getButton';

const _trampoline29 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-buttons"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getButtons',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getButtons(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-buttons"][Instruction::Return]', {
    funcName: 'get-buttons',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toUint16(ret)]);
  task.exit();
  return toUint16(ret);
}
_trampoline29.fnName = 'tairitsu-browser:full/mouse-event@0.2.0#getButtons';

const _trampoline30 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-ctrl-key"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getCtrlKey',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getCtrlKey(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-ctrl-key"][Instruction::Return]', {
    funcName: 'get-ctrl-key',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([ret ? 1 : 0]);
  task.exit();
  return ret ? 1 : 0;
}
_trampoline30.fnName = 'tairitsu-browser:full/mouse-event@0.2.0#getCtrlKey';

const _trampoline31 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-shift-key"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getShiftKey',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getShiftKey(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-shift-key"][Instruction::Return]', {
    funcName: 'get-shift-key',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([ret ? 1 : 0]);
  task.exit();
  return ret ? 1 : 0;
}
_trampoline31.fnName = 'tairitsu-browser:full/mouse-event@0.2.0#getShiftKey';

const _trampoline32 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-alt-key"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getAltKey',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getAltKey(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-alt-key"][Instruction::Return]', {
    funcName: 'get-alt-key',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([ret ? 1 : 0]);
  task.exit();
  return ret ? 1 : 0;
}
_trampoline32.fnName = 'tairitsu-browser:full/mouse-event@0.2.0#getAltKey';

const _trampoline33 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-meta-key"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getMetaKey',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getMetaKey(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/mouse-event@0.2.0", function="get-meta-key"][Instruction::Return]', {
    funcName: 'get-meta-key',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([ret ? 1 : 0]);
  task.exit();
  return ret ? 1 : 0;
}
_trampoline33.fnName = 'tairitsu-browser:full/mouse-event@0.2.0#getMetaKey';

const _trampoline34 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-pointer-id"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getPointerId',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getPointerId(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-pointer-id"][Instruction::Return]', {
    funcName: 'get-pointer-id',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toInt32(ret)]);
  task.exit();
  return toInt32(ret);
}
_trampoline34.fnName = 'tairitsu-browser:full/pointer-event@0.2.0#getPointerId';

const _trampoline35 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-is-primary"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getIsPrimary',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getIsPrimary(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-is-primary"][Instruction::Return]', {
    funcName: 'get-is-primary',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([ret ? 1 : 0]);
  task.exit();
  return ret ? 1 : 0;
}
_trampoline35.fnName = 'tairitsu-browser:full/pointer-event@0.2.0#getIsPrimary';

const _trampoline36 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-width"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getWidth',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getWidth(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-width"][Instruction::Return]', {
    funcName: 'get-width',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([+ret]);
  task.exit();
  return +ret;
}
_trampoline36.fnName = 'tairitsu-browser:full/pointer-event@0.2.0#getWidth';

const _trampoline37 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-height"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getHeight',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getHeight(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-height"][Instruction::Return]', {
    funcName: 'get-height',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([+ret]);
  task.exit();
  return +ret;
}
_trampoline37.fnName = 'tairitsu-browser:full/pointer-event@0.2.0#getHeight';

const _trampoline38 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-pressure"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getPressure',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getPressure(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-pressure"][Instruction::Return]', {
    funcName: 'get-pressure',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([+ret]);
  task.exit();
  return +ret;
}
_trampoline38.fnName = 'tairitsu-browser:full/pointer-event@0.2.0#getPressure';

const _trampoline39 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-tangential-pressure"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getTangentialPressure',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getTangentialPressure(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-tangential-pressure"][Instruction::Return]', {
    funcName: 'get-tangential-pressure',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([+ret]);
  task.exit();
  return +ret;
}
_trampoline39.fnName = 'tairitsu-browser:full/pointer-event@0.2.0#getTangentialPressure';

const _trampoline40 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-tilt-x"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getTiltX',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getTiltX(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-tilt-x"][Instruction::Return]', {
    funcName: 'get-tilt-x',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toInt32(ret)]);
  task.exit();
  return toInt32(ret);
}
_trampoline40.fnName = 'tairitsu-browser:full/pointer-event@0.2.0#getTiltX';

const _trampoline41 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-tilt-y"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getTiltY',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getTiltY(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-tilt-y"][Instruction::Return]', {
    funcName: 'get-tilt-y',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toInt32(ret)]);
  task.exit();
  return toInt32(ret);
}
_trampoline41.fnName = 'tairitsu-browser:full/pointer-event@0.2.0#getTiltY';

const _trampoline42 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-twist"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getTwist',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getTwist(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-twist"][Instruction::Return]', {
    funcName: 'get-twist',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toInt32(ret)]);
  task.exit();
  return toInt32(ret);
}
_trampoline42.fnName = 'tairitsu-browser:full/pointer-event@0.2.0#getTwist';

const _trampoline43 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/wheel-event@0.2.0", function="get-delta-x"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getDeltaX',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getDeltaX(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/wheel-event@0.2.0", function="get-delta-x"][Instruction::Return]', {
    funcName: 'get-delta-x',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([+ret]);
  task.exit();
  return +ret;
}
_trampoline43.fnName = 'tairitsu-browser:full/wheel-event@0.2.0#getDeltaX';

const _trampoline44 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/wheel-event@0.2.0", function="get-delta-y"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getDeltaY',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getDeltaY(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/wheel-event@0.2.0", function="get-delta-y"][Instruction::Return]', {
    funcName: 'get-delta-y',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([+ret]);
  task.exit();
  return +ret;
}
_trampoline44.fnName = 'tairitsu-browser:full/wheel-event@0.2.0#getDeltaY';

const _trampoline45 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/wheel-event@0.2.0", function="get-delta-z"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getDeltaZ',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getDeltaZ(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/wheel-event@0.2.0", function="get-delta-z"][Instruction::Return]', {
    funcName: 'get-delta-z',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([+ret]);
  task.exit();
  return +ret;
}
_trampoline45.fnName = 'tairitsu-browser:full/wheel-event@0.2.0#getDeltaZ';

const _trampoline46 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/wheel-event@0.2.0", function="get-delta-mode"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getDeltaMode',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getDeltaMode(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/wheel-event@0.2.0", function="get-delta-mode"][Instruction::Return]', {
    funcName: 'get-delta-mode',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toUint32(ret)]);
  task.exit();
  return toUint32(ret);
}
_trampoline46.fnName = 'tairitsu-browser:full/wheel-event@0.2.0#getDeltaMode';

const _trampoline53 = function(arg0) {
  let variant0;
  switch (arg0) {
    case 0: {
      variant0= {
        tag: 'ok',
        val: undefined
      };
      break;
    }
    case 1: {
      variant0= {
        tag: 'err',
        val: undefined
      };
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for expected');
    }
  }
  _debugLog('[iface="wasi:cli/exit@0.2.12", function="exit"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'exit',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => exit(variant0)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="wasi:cli/exit@0.2.12", function="exit"][Instruction::Return]', {
    funcName: 'exit',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline53.fnName = 'wasi:cli/exit@0.2.12#exit';

const handleTable0 = [T_FLAG, 0];
handleTable0._createdReps = new Set();
handleTable0._componentIdx = 0;


const captureTable0= new Map();
let captureCnt0= 0;

HANDLE_TABLES[0] = handleTable0;

const _trampoline54 = function(arg0) {
  var handle1 = arg0;
  
  var rep2 = handleTable0[(handle1 << 1) + 1] % T_OWN;
  var rsc0 = captureTable0.get(rep2);
  if (!rsc0) {
    rsc0 = Object.create(Pollable.prototype);
    Object.defineProperty(rsc0, symbolRscHandle, { writable: true, value: handle1});
    Object.defineProperty(rsc0, symbolRscRep, { writable: true, value: rep2});
  }
  
  curResourceBorrows.push(rsc0);
  _debugLog('[iface="wasi:io/poll@0.2.12", function="[method]pollable.block"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'block',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => rsc0.block()),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  for (const entry of curResourceBorrows) {
    const rsc = entry.rsc ?? entry;
    if (entry.drop) {
      if (rsc[symbolRscHandle]) {
        entry.drop(rsc[symbolRscHandle]);
      }
    }
    rsc[symbolRscHandle] = undefined;
  }
  curResourceBorrows = [];
  _debugLog('[iface="wasi:io/poll@0.2.12", function="[method]pollable.block"][Instruction::Return]', {
    funcName: '[method]pollable.block',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline54.fnName = 'wasi:io/poll@0.2.12#block';

const handleTable3 = [T_FLAG, 0];
handleTable3._createdReps = new Set();
handleTable3._componentIdx = 0;


const captureTable3= new Map();
let captureCnt3= 0;

HANDLE_TABLES[3] = handleTable3;

const _trampoline55 = function(arg0) {
  var handle1 = arg0;
  
  var rep2 = handleTable3[(handle1 << 1) + 1] % T_OWN;
  var rsc0 = captureTable3.get(rep2);
  if (!rsc0) {
    rsc0 = Object.create(OutputStream.prototype);
    Object.defineProperty(rsc0, symbolRscHandle, { writable: true, value: handle1});
    Object.defineProperty(rsc0, symbolRscRep, { writable: true, value: rep2});
  }
  
  curResourceBorrows.push(rsc0);
  _debugLog('[iface="wasi:io/streams@0.2.12", function="[method]output-stream.subscribe"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'subscribe',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => rsc0.subscribe()),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  for (const entry of curResourceBorrows) {
    const rsc = entry.rsc ?? entry;
    if (entry.drop) {
      if (rsc[symbolRscHandle]) {
        entry.drop(rsc[symbolRscHandle]);
      }
    }
    rsc[symbolRscHandle] = undefined;
  }
  curResourceBorrows = [];
  
  if (!(ret instanceof Pollable)) {
    throw new TypeError('Resource error: Not a valid \"Pollable\" resource.');
  }
  
  
  var handle3 = ret[symbolRscHandle];
  if (!handle3) {
    const rep = (ret[symbolRscRep] ?? ++captureCnt0) >>> 0;
    captureTable0.set(rep, ret);
    handle3 = rscTableCreateOwn(handleTable0, rep);
  }
  
  _debugLog('[iface="wasi:io/streams@0.2.12", function="[method]output-stream.subscribe"][Instruction::Return]', {
    funcName: '[method]output-stream.subscribe',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([handle3]);
  task.exit();
  return handle3;
}
_trampoline55.fnName = 'wasi:io/streams@0.2.12#subscribe';

const handleTable2 = [T_FLAG, 0];
handleTable2._createdReps = new Set();
handleTable2._componentIdx = 0;


const captureTable2= new Map();
let captureCnt2= 0;

HANDLE_TABLES[2] = handleTable2;

const _trampoline56 = function() {
  _debugLog('[iface="wasi:cli/stdin@0.2.12", function="get-stdin"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getStdin',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getStdin()),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  
  if (!(ret instanceof InputStream)) {
    throw new TypeError('Resource error: Not a valid \"InputStream\" resource.');
  }
  
  
  var handle0 = ret[symbolRscHandle];
  if (!handle0) {
    const rep = (ret[symbolRscRep] ?? ++captureCnt2) >>> 0;
    captureTable2.set(rep, ret);
    handle0 = rscTableCreateOwn(handleTable2, rep);
  }
  
  _debugLog('[iface="wasi:cli/stdin@0.2.12", function="get-stdin"][Instruction::Return]', {
    funcName: 'get-stdin',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([handle0]);
  task.exit();
  return handle0;
}
_trampoline56.fnName = 'wasi:cli/stdin@0.2.12#getStdin';

const _trampoline57 = function() {
  _debugLog('[iface="wasi:cli/stdout@0.2.12", function="get-stdout"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getStdout',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getStdout()),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  
  if (!(ret instanceof OutputStream)) {
    throw new TypeError('Resource error: Not a valid \"OutputStream\" resource.');
  }
  
  
  var handle0 = ret[symbolRscHandle];
  if (!handle0) {
    const rep = (ret[symbolRscRep] ?? ++captureCnt3) >>> 0;
    captureTable3.set(rep, ret);
    handle0 = rscTableCreateOwn(handleTable3, rep);
  }
  
  _debugLog('[iface="wasi:cli/stdout@0.2.12", function="get-stdout"][Instruction::Return]', {
    funcName: 'get-stdout',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([handle0]);
  task.exit();
  return handle0;
}
_trampoline57.fnName = 'wasi:cli/stdout@0.2.12#getStdout';

const _trampoline58 = function() {
  _debugLog('[iface="wasi:cli/stderr@0.2.12", function="get-stderr"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getStderr',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getStderr()),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  
  if (!(ret instanceof OutputStream)) {
    throw new TypeError('Resource error: Not a valid \"OutputStream\" resource.');
  }
  
  
  var handle0 = ret[symbolRscHandle];
  if (!handle0) {
    const rep = (ret[symbolRscRep] ?? ++captureCnt3) >>> 0;
    captureTable3.set(rep, ret);
    handle0 = rscTableCreateOwn(handleTable3, rep);
  }
  
  _debugLog('[iface="wasi:cli/stderr@0.2.12", function="get-stderr"][Instruction::Return]', {
    funcName: 'get-stderr',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([handle0]);
  task.exit();
  return handle0;
}
_trampoline58.fnName = 'wasi:cli/stderr@0.2.12#getStderr';

const _trampoline59 = function(arg0) {
  _debugLog('[iface="wasi:clocks/monotonic-clock@0.2.12", function="subscribe-duration"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'subscribeDuration',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => subscribeDuration(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  
  if (!(ret instanceof Pollable)) {
    throw new TypeError('Resource error: Not a valid \"Pollable\" resource.');
  }
  
  
  var handle0 = ret[symbolRscHandle];
  if (!handle0) {
    const rep = (ret[symbolRscRep] ?? ++captureCnt0) >>> 0;
    captureTable0.set(rep, ret);
    handle0 = rscTableCreateOwn(handleTable0, rep);
  }
  
  _debugLog('[iface="wasi:clocks/monotonic-clock@0.2.12", function="subscribe-duration"][Instruction::Return]', {
    funcName: 'subscribe-duration',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([handle0]);
  task.exit();
  return handle0;
}
_trampoline59.fnName = 'wasi:clocks/monotonic-clock@0.2.12#subscribeDuration';
let exports1;
let memory0;
let realloc0;
let realloc0Async;

const _trampoline60 = function(arg0, arg1, arg2, arg3, arg4) {
  var ptr0 = (arg0 >>> 0) >>> 0;
  var len0 = arg1;
  var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
  let variant2;
  switch (arg2) {
    case 0: {
      variant2 = undefined;
      break;
    }
    case 1: {
      var ptr1 = (arg3 >>> 0) >>> 0;
      var len1 = arg4;
      var result1 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr1, len1));
      variant2 = result1;
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for option');
    }
  }
  _debugLog('[iface="tairitsu-browser:full/platform-helpers@0.2.0", function="fetch-promise"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'fetchPromise',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => fetchPromise(result0, variant2)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/platform-helpers@0.2.0", function="fetch-promise"][Instruction::Return]', {
    funcName: 'fetch-promise',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toUint64(ret)]);
  task.exit();
  return toUint64(ret);
}
_trampoline60.fnName = 'tairitsu-browser:full/platform-helpers@0.2.0#fetchPromise';

const _trampoline61 = function(arg0, arg1, arg2, arg3, arg4) {
  var ptr0 = (arg1 >>> 0) >>> 0;
  var len0 = arg2;
  var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
  var ptr1 = (arg3 >>> 0) >>> 0;
  var len1 = arg4;
  var result1 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr1, len1));
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="set-attribute"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'setAttribute',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => setAttribute(BigInt.asUintN(64, BigInt(arg0)), result0, result1)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="set-attribute"][Instruction::Return]', {
    funcName: 'set-attribute',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline61.fnName = 'tairitsu-browser:full/element@0.2.0#setAttribute';

const _trampoline62 = function(arg0, arg1, arg2, arg3) {
  var ptr0 = (arg1 >>> 0) >>> 0;
  var len0 = arg2;
  var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="get-attribute"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getAttribute',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getAttribute(BigInt.asUintN(64, BigInt(arg0)), result0)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var variant2 = ret;
  if (variant2 === null || variant2=== undefined) {
    dataView(memory0).setInt8(((arg3 >>> 0)) + 0, 0, true);
  } else {
    const e = variant2;
    dataView(memory0).setInt8(((arg3 >>> 0)) + 0, 1, true);
    
    var encodeRes = _utf8AllocateAndEncode(e, ((...args) => {
      const state = getOrCreateAsyncState(0);
      const previousMayLeave = state.mayLeave;
      const meta = _getGlobalCurrentThreadMeta(0);
      const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
      const previousStorage = thread?.storage;
      if (thread) { thread.storage = [0, 0]; }
      state.mayLeave = false;
      const restore = () => {
        state.mayLeave = previousMayLeave;
        if (thread) { thread.storage = previousStorage; }
      };
      let result;
      try {
        result = realloc0(...args);
      } catch (err) {
        restore();
        throw err;
      }
      if (result && typeof result.then === 'function') {
        return Promise.resolve(result).finally(restore);
      }
      restore();
      return result;
    }), memory0);
    var ptr1= encodeRes.ptr;
    var len1 = encodeRes.len;
    
    dataView(memory0).setUint32(((arg3 >>> 0)) + 8, len1, true);
    dataView(memory0).setUint32(((arg3 >>> 0)) + 4, ptr1, true);
  }
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="get-attribute"][Instruction::Return]', {
    funcName: 'get-attribute',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline62.fnName = 'tairitsu-browser:full/element@0.2.0#getAttribute';

const _trampoline63 = function(arg0, arg1, arg2) {
  var ptr0 = (arg1 >>> 0) >>> 0;
  var len0 = arg2;
  var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="set-inner-html"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'setInnerHtml',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => setInnerHtml(BigInt.asUintN(64, BigInt(arg0)), result0)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="set-inner-html"][Instruction::Return]', {
    funcName: 'set-inner-html',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline63.fnName = 'tairitsu-browser:full/element@0.2.0#setInnerHtml';

const _trampoline64 = function(arg0, arg1, arg2) {
  var ptr0 = (arg1 >>> 0) >>> 0;
  var len0 = arg2;
  var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="remove-attribute"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'removeAttribute',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => removeAttribute(BigInt.asUintN(64, BigInt(arg0)), result0)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="remove-attribute"][Instruction::Return]', {
    funcName: 'remove-attribute',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline64.fnName = 'tairitsu-browser:full/element@0.2.0#removeAttribute';

const _trampoline65 = function(arg0, arg1) {
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="get-bounding-client-rect"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getBoundingClientRect',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getBoundingClientRect(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var {x: v0_0, y: v0_1, width: v0_2, height: v0_3 } = ret;
  dataView(memory0).setFloat64(((arg1 >>> 0)) + 0, +v0_0, true);
  dataView(memory0).setFloat64(((arg1 >>> 0)) + 8, +v0_1, true);
  dataView(memory0).setFloat64(((arg1 >>> 0)) + 16, +v0_2, true);
  dataView(memory0).setFloat64(((arg1 >>> 0)) + 24, +v0_3, true);
  _debugLog('[iface="tairitsu-browser:full/element@0.2.0", function="get-bounding-client-rect"][Instruction::Return]', {
    funcName: 'get-bounding-client-rect',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline65.fnName = 'tairitsu-browser:full/element@0.2.0#getBoundingClientRect';

const _trampoline66 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
  var ptr0 = (arg1 >>> 0) >>> 0;
  var len0 = arg2;
  var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
  var ptr1 = (arg3 >>> 0) >>> 0;
  var len1 = arg4;
  var result1 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr1, len1));
  let variant3;
  switch (arg5) {
    case 0: {
      variant3 = undefined;
      break;
    }
    case 1: {
      var ptr2 = (arg6 >>> 0) >>> 0;
      var len2 = arg7;
      var result2 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr2, len2));
      variant3 = result2;
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for option');
    }
  }
  _debugLog('[iface="tairitsu-browser:full/css-style-declaration@0.2.0", function="set-property"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'setProperty',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => setProperty(BigInt.asUintN(64, BigInt(arg0)), result0, result1, variant3)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/css-style-declaration@0.2.0", function="set-property"][Instruction::Return]', {
    funcName: 'set-property',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline66.fnName = 'tairitsu-browser:full/css-style-declaration@0.2.0#setProperty';

const _trampoline67 = function(arg0, arg1, arg2, arg3) {
  var ptr0 = (arg1 >>> 0) >>> 0;
  var len0 = arg2;
  var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
  _debugLog('[iface="tairitsu-browser:full/css-style-declaration@0.2.0", function="get-property-value"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getPropertyValue',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getPropertyValue(BigInt.asUintN(64, BigInt(arg0)), result0)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  
  var encodeRes = _utf8AllocateAndEncode(ret, ((...args) => {
    const state = getOrCreateAsyncState(0);
    const previousMayLeave = state.mayLeave;
    const meta = _getGlobalCurrentThreadMeta(0);
    const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
    const previousStorage = thread?.storage;
    if (thread) { thread.storage = [0, 0]; }
    state.mayLeave = false;
    const restore = () => {
      state.mayLeave = previousMayLeave;
      if (thread) { thread.storage = previousStorage; }
    };
    let result;
    try {
      result = realloc0(...args);
    } catch (err) {
      restore();
      throw err;
    }
    if (result && typeof result.then === 'function') {
      return Promise.resolve(result).finally(restore);
    }
    restore();
    return result;
  }), memory0);
  var ptr1= encodeRes.ptr;
  var len1 = encodeRes.len;
  
  dataView(memory0).setUint32(((arg3 >>> 0)) + 4, len1, true);
  dataView(memory0).setUint32(((arg3 >>> 0)) + 0, ptr1, true);
  _debugLog('[iface="tairitsu-browser:full/css-style-declaration@0.2.0", function="get-property-value"][Instruction::Return]', {
    funcName: 'get-property-value',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline67.fnName = 'tairitsu-browser:full/css-style-declaration@0.2.0#getPropertyValue';

const _trampoline68 = function(arg0, arg1) {
  _debugLog('[iface="tairitsu-browser:full/node@0.2.0", function="get-first-child"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getFirstChild',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getFirstChild(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var variant0 = ret;
  if (variant0 === null || variant0=== undefined) {
    dataView(memory0).setInt8(((arg1 >>> 0)) + 0, 0, true);
  } else {
    const e = variant0;
    dataView(memory0).setInt8(((arg1 >>> 0)) + 0, 1, true);
    dataView(memory0).setBigInt64(((arg1 >>> 0)) + 8, toUint64(e), true);
  }
  _debugLog('[iface="tairitsu-browser:full/node@0.2.0", function="get-first-child"][Instruction::Return]', {
    funcName: 'get-first-child',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline68.fnName = 'tairitsu-browser:full/node@0.2.0#getFirstChild';

const _trampoline69 = function(arg0, arg1, arg2, arg3) {
  let variant1;
  switch (arg1) {
    case 0: {
      variant1 = undefined;
      break;
    }
    case 1: {
      var ptr0 = (arg2 >>> 0) >>> 0;
      var len0 = arg3;
      var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
      variant1 = result0;
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for option');
    }
  }
  _debugLog('[iface="tairitsu-browser:full/node@0.2.0", function="set-text-content"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'setTextContent',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => setTextContent(BigInt.asUintN(64, BigInt(arg0)), variant1)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/node@0.2.0", function="set-text-content"][Instruction::Return]', {
    funcName: 'set-text-content',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline69.fnName = 'tairitsu-browser:full/node@0.2.0#setTextContent';

const _trampoline70 = function(arg0, arg1) {
  _debugLog('[iface="tairitsu-browser:full/node@0.2.0", function="get-parent-element"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getParentElement',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getParentElement(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var variant0 = ret;
  if (variant0 === null || variant0=== undefined) {
    dataView(memory0).setInt8(((arg1 >>> 0)) + 0, 0, true);
  } else {
    const e = variant0;
    dataView(memory0).setInt8(((arg1 >>> 0)) + 0, 1, true);
    dataView(memory0).setBigInt64(((arg1 >>> 0)) + 8, toUint64(e), true);
  }
  _debugLog('[iface="tairitsu-browser:full/node@0.2.0", function="get-parent-element"][Instruction::Return]', {
    funcName: 'get-parent-element',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline70.fnName = 'tairitsu-browser:full/node@0.2.0#getParentElement';

const _trampoline71 = function(arg0, arg1, arg2, arg3, arg4) {
  var ptr0 = (arg1 >>> 0) >>> 0;
  var len0 = arg2;
  var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
  var bool1 = arg3;
  _debugLog('[iface="tairitsu-browser:full/event-target@0.2.0", function="add-event-listener"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'addEventListener',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'result-catch-handler',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  try {
    const hostRet2 = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => addEventListener(BigInt.asUintN(64, BigInt(arg0)), result0, bool1 == 0 ? false : (bool1 == 1 ? true : throwInvalidBool()))),
    })
    ;
    ret = hostRet2 !== null && typeof hostRet2 === 'object' && (hostRet2.tag === 'ok' || hostRet2.tag === 'err')
    ? hostRet2
    : { tag: 'ok', val: hostRet2};
  } catch (e) {
    if (getOrCreateAsyncState(0).markTrapped(e)) { throw e; }
    ret = { tag: 'err', val: getErrorPayload(e) };
  }
  
  var variant4 = ret;
  switch (variant4.tag) {
    case 'ok': {
      const e = variant4.val;
      dataView(memory0).setInt8(((arg4 >>> 0)) + 0, 0, true);
      dataView(memory0).setBigInt64(((arg4 >>> 0)) + 8, toUint64(e), true);
      
      break;
    }
    case 'err': {
      const e = variant4.val;
      dataView(memory0).setInt8(((arg4 >>> 0)) + 0, 1, true);
      
      var encodeRes = _utf8AllocateAndEncode(e, ((...args) => {
        const state = getOrCreateAsyncState(0);
        const previousMayLeave = state.mayLeave;
        const meta = _getGlobalCurrentThreadMeta(0);
        const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
        const previousStorage = thread?.storage;
        if (thread) { thread.storage = [0, 0]; }
        state.mayLeave = false;
        const restore = () => {
          state.mayLeave = previousMayLeave;
          if (thread) { thread.storage = previousStorage; }
        };
        let result;
        try {
          result = realloc0(...args);
        } catch (err) {
          restore();
          throw err;
        }
        if (result && typeof result.then === 'function') {
          return Promise.resolve(result).finally(restore);
        }
        restore();
        return result;
      }), memory0);
      var ptr3= encodeRes.ptr;
      var len3 = encodeRes.len;
      
      dataView(memory0).setUint32(((arg4 >>> 0)) + 12, len3, true);
      dataView(memory0).setUint32(((arg4 >>> 0)) + 8, ptr3, true);
      
      break;
    }
    default: {
      _debugLog("ERROR: invalid value (expected result as object with 'tag' member)", { value: variant4, valueType: typeof variant4});
      throw new TypeError('invalid variant specified for result');
    }
  }
  _debugLog('[iface="tairitsu-browser:full/event-target@0.2.0", function="add-event-listener"][Instruction::Return]', {
    funcName: 'add-event-listener',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline71.fnName = 'tairitsu-browser:full/event-target@0.2.0#addEventListener';

const _trampoline72 = function(arg0, arg1, arg2) {
  _debugLog('[iface="tairitsu-browser:full/event-target@0.2.0", function="remove-event-listener"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'removeEventListener',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'result-catch-handler',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  try {
    const hostRet0 = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => removeEventListener(BigInt.asUintN(64, BigInt(arg0)), BigInt.asUintN(64, BigInt(arg1)))),
    })
    ;
    ret = hostRet0 !== null && typeof hostRet0 === 'object' && (hostRet0.tag === 'ok' || hostRet0.tag === 'err')
    ? hostRet0
    : { tag: 'ok', val: hostRet0};
  } catch (e) {
    if (getOrCreateAsyncState(0).markTrapped(e)) { throw e; }
    ret = { tag: 'err', val: getErrorPayload(e) };
  }
  
  var variant2 = ret;
  switch (variant2.tag) {
    case 'ok': {
      const e = variant2.val;
      dataView(memory0).setInt8(((arg2 >>> 0)) + 0, 0, true);
      
      break;
    }
    case 'err': {
      const e = variant2.val;
      dataView(memory0).setInt8(((arg2 >>> 0)) + 0, 1, true);
      
      var encodeRes = _utf8AllocateAndEncode(e, ((...args) => {
        const state = getOrCreateAsyncState(0);
        const previousMayLeave = state.mayLeave;
        const meta = _getGlobalCurrentThreadMeta(0);
        const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
        const previousStorage = thread?.storage;
        if (thread) { thread.storage = [0, 0]; }
        state.mayLeave = false;
        const restore = () => {
          state.mayLeave = previousMayLeave;
          if (thread) { thread.storage = previousStorage; }
        };
        let result;
        try {
          result = realloc0(...args);
        } catch (err) {
          restore();
          throw err;
        }
        if (result && typeof result.then === 'function') {
          return Promise.resolve(result).finally(restore);
        }
        restore();
        return result;
      }), memory0);
      var ptr1= encodeRes.ptr;
      var len1 = encodeRes.len;
      
      dataView(memory0).setUint32(((arg2 >>> 0)) + 8, len1, true);
      dataView(memory0).setUint32(((arg2 >>> 0)) + 4, ptr1, true);
      
      break;
    }
    default: {
      _debugLog("ERROR: invalid value (expected result as object with 'tag' member)", { value: variant2, valueType: typeof variant2});
      throw new TypeError('invalid variant specified for result');
    }
  }
  _debugLog('[iface="tairitsu-browser:full/event-target@0.2.0", function="remove-event-listener"][Instruction::Return]', {
    funcName: 'remove-event-listener',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline72.fnName = 'tairitsu-browser:full/event-target@0.2.0#removeEventListener';

const _trampoline73 = function(arg0, arg1) {
  _debugLog('[iface="tairitsu-browser:full/html-input-element@0.2.0", function="get-value"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getValue',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getValue(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  
  var encodeRes = _utf8AllocateAndEncode(ret, ((...args) => {
    const state = getOrCreateAsyncState(0);
    const previousMayLeave = state.mayLeave;
    const meta = _getGlobalCurrentThreadMeta(0);
    const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
    const previousStorage = thread?.storage;
    if (thread) { thread.storage = [0, 0]; }
    state.mayLeave = false;
    const restore = () => {
      state.mayLeave = previousMayLeave;
      if (thread) { thread.storage = previousStorage; }
    };
    let result;
    try {
      result = realloc0(...args);
    } catch (err) {
      restore();
      throw err;
    }
    if (result && typeof result.then === 'function') {
      return Promise.resolve(result).finally(restore);
    }
    restore();
    return result;
  }), memory0);
  var ptr0= encodeRes.ptr;
  var len0 = encodeRes.len;
  
  dataView(memory0).setUint32(((arg1 >>> 0)) + 4, len0, true);
  dataView(memory0).setUint32(((arg1 >>> 0)) + 0, ptr0, true);
  _debugLog('[iface="tairitsu-browser:full/html-input-element@0.2.0", function="get-value"][Instruction::Return]', {
    funcName: 'get-value',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline73.fnName = 'tairitsu-browser:full/html-input-element@0.2.0#getValue';

const _trampoline74 = function(arg0, arg1) {
  _debugLog('[iface="tairitsu-browser:full/event@0.2.0", function="get-target"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getTarget',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getTarget(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var variant0 = ret;
  if (variant0 === null || variant0=== undefined) {
    dataView(memory0).setInt8(((arg1 >>> 0)) + 0, 0, true);
  } else {
    const e = variant0;
    dataView(memory0).setInt8(((arg1 >>> 0)) + 0, 1, true);
    dataView(memory0).setBigInt64(((arg1 >>> 0)) + 8, toUint64(e), true);
  }
  _debugLog('[iface="tairitsu-browser:full/event@0.2.0", function="get-target"][Instruction::Return]', {
    funcName: 'get-target',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline74.fnName = 'tairitsu-browser:full/event@0.2.0#getTarget';

const _trampoline75 = function(arg0, arg1) {
  _debugLog('[iface="tairitsu-browser:full/event@0.2.0", function="get-current-target"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getCurrentTarget',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getCurrentTarget(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var variant0 = ret;
  if (variant0 === null || variant0=== undefined) {
    dataView(memory0).setInt8(((arg1 >>> 0)) + 0, 0, true);
  } else {
    const e = variant0;
    dataView(memory0).setInt8(((arg1 >>> 0)) + 0, 1, true);
    dataView(memory0).setBigInt64(((arg1 >>> 0)) + 8, toUint64(e), true);
  }
  _debugLog('[iface="tairitsu-browser:full/event@0.2.0", function="get-current-target"][Instruction::Return]', {
    funcName: 'get-current-target',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline75.fnName = 'tairitsu-browser:full/event@0.2.0#getCurrentTarget';

const _trampoline76 = function(arg0) {
  _debugLog('[iface="tairitsu-browser:full/document@0.2.0", function="get-body"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getBody',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getBody()),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var variant0 = ret;
  if (variant0 === null || variant0=== undefined) {
    dataView(memory0).setInt8(((arg0 >>> 0)) + 0, 0, true);
  } else {
    const e = variant0;
    dataView(memory0).setInt8(((arg0 >>> 0)) + 0, 1, true);
    dataView(memory0).setBigInt64(((arg0 >>> 0)) + 8, toUint64(e), true);
  }
  _debugLog('[iface="tairitsu-browser:full/document@0.2.0", function="get-body"][Instruction::Return]', {
    funcName: 'get-body',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline76.fnName = 'tairitsu-browser:full/document@0.2.0#getBody';

const _trampoline77 = function(arg0, arg1, arg2, arg3, arg4) {
  var ptr0 = (arg0 >>> 0) >>> 0;
  var len0 = arg1;
  var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
  let variant2;
  switch (arg2) {
    case 0: {
      variant2 = undefined;
      break;
    }
    case 1: {
      var ptr1 = (arg3 >>> 0) >>> 0;
      var len1 = arg4;
      var result1 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr1, len1));
      variant2 = result1;
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for option');
    }
  }
  _debugLog('[iface="tairitsu-browser:full/document@0.2.0", function="create-element"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'createElement',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => createElement(result0, variant2)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/document@0.2.0", function="create-element"][Instruction::Return]', {
    funcName: 'create-element',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toUint64(ret)]);
  task.exit();
  return toUint64(ret);
}
_trampoline77.fnName = 'tairitsu-browser:full/document@0.2.0#createElement';

const _trampoline78 = function(arg0, arg1) {
  var ptr0 = (arg0 >>> 0) >>> 0;
  var len0 = arg1;
  var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
  _debugLog('[iface="tairitsu-browser:full/document@0.2.0", function="create-text-node"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'createTextNode',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => createTextNode(result0)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/document@0.2.0", function="create-text-node"][Instruction::Return]', {
    funcName: 'create-text-node',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toUint64(ret)]);
  task.exit();
  return toUint64(ret);
}
_trampoline78.fnName = 'tairitsu-browser:full/document@0.2.0#createTextNode';

const _trampoline79 = function(arg0, arg1, arg2, arg3) {
  let variant1;
  switch (arg1) {
    case 0: {
      variant1 = undefined;
      break;
    }
    case 1: {
      var ptr0 = (arg2 >>> 0) >>> 0;
      var len0 = arg3;
      var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
      variant1 = result0;
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for option');
    }
  }
  _debugLog('[iface="tairitsu-browser:full/window@0.2.0", function="get-computed-style"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getComputedStyle',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getComputedStyle(BigInt.asUintN(64, BigInt(arg0)), variant1)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/window@0.2.0", function="get-computed-style"][Instruction::Return]', {
    funcName: 'get-computed-style',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toUint64(ret)]);
  task.exit();
  return toUint64(ret);
}
_trampoline79.fnName = 'tairitsu-browser:full/window@0.2.0#getComputedStyle';

const _trampoline80 = function(arg0, arg1, arg2, arg3) {
  var ptr0 = (arg1 >>> 0) >>> 0;
  var len0 = arg2;
  var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
  _debugLog('[iface="tairitsu-browser:full/parent-node@0.2.0", function="query-selector"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'querySelector',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => querySelector(BigInt.asUintN(64, BigInt(arg0)), result0)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var variant1 = ret;
  if (variant1 === null || variant1=== undefined) {
    dataView(memory0).setInt8(((arg3 >>> 0)) + 0, 0, true);
  } else {
    const e = variant1;
    dataView(memory0).setInt8(((arg3 >>> 0)) + 0, 1, true);
    dataView(memory0).setBigInt64(((arg3 >>> 0)) + 8, toUint64(e), true);
  }
  _debugLog('[iface="tairitsu-browser:full/parent-node@0.2.0", function="query-selector"][Instruction::Return]', {
    funcName: 'query-selector',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline80.fnName = 'tairitsu-browser:full/parent-node@0.2.0#querySelector';

const _trampoline81 = function(arg0, arg1, arg2, arg3) {
  var ptr0 = (arg1 >>> 0) >>> 0;
  var len0 = arg2;
  var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
  _debugLog('[iface="tairitsu-browser:full/non-element-parent-node@0.2.0", function="get-element-by-id"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getElementById',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getElementById(BigInt.asUintN(64, BigInt(arg0)), result0)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var variant1 = ret;
  if (variant1 === null || variant1=== undefined) {
    dataView(memory0).setInt8(((arg3 >>> 0)) + 0, 0, true);
  } else {
    const e = variant1;
    dataView(memory0).setInt8(((arg3 >>> 0)) + 0, 1, true);
    dataView(memory0).setBigInt64(((arg3 >>> 0)) + 8, toUint64(e), true);
  }
  _debugLog('[iface="tairitsu-browser:full/non-element-parent-node@0.2.0", function="get-element-by-id"][Instruction::Return]', {
    funcName: 'get-element-by-id',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline81.fnName = 'tairitsu-browser:full/non-element-parent-node@0.2.0#getElementById';

const _trampoline82 = function(arg0, arg1, arg2) {
  _debugLog('[iface="tairitsu-browser:full/node-list@0.2.0", function="item"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'item',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => item(BigInt.asUintN(64, BigInt(arg0)), arg1 >>> 0)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var variant0 = ret;
  if (variant0 === null || variant0=== undefined) {
    dataView(memory0).setInt8(((arg2 >>> 0)) + 0, 0, true);
  } else {
    const e = variant0;
    dataView(memory0).setInt8(((arg2 >>> 0)) + 0, 1, true);
    dataView(memory0).setBigInt64(((arg2 >>> 0)) + 8, toUint64(e), true);
  }
  _debugLog('[iface="tairitsu-browser:full/node-list@0.2.0", function="item"][Instruction::Return]', {
    funcName: 'item',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline82.fnName = 'tairitsu-browser:full/node-list@0.2.0#item';

const _trampoline83 = function(arg0, arg1, arg2) {
  var len1 = arg2;
  var base1 = (arg1 >>> 0) >>> 0;
  if (base1 < 0 || len1 < 0 || len1 > 33554431 || base1 > memory0.buffer.byteLength || len1 > Math.floor((memory0.buffer.byteLength - base1) / 8)) throw new WebAssemblyRuntimeError('wasm trap: out of bounds memory access');
  if (base1 % 4 !== 0) throw new TypeError(`list pointer [${base1}] is not aligned to 4`);
  var result1 = [];
  for (let i = 0; i < len1; i++) {
    const base = base1 + i * 8;
    var ptr0 = dataView(memory0).getUint32((base) + 0, true) >>> 0;
    var len0 = dataView(memory0).getUint32((base) + 4, true);
    var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
    result1.push(result0);
  }
  _debugLog('[iface="tairitsu-browser:full/dom-token-list@0.2.0", function="add"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'add',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => add(BigInt.asUintN(64, BigInt(arg0)), result1)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/dom-token-list@0.2.0", function="add"][Instruction::Return]', {
    funcName: 'add',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline83.fnName = 'tairitsu-browser:full/dom-token-list@0.2.0#add';

const _trampoline84 = function(arg0, arg1, arg2) {
  var len1 = arg2;
  var base1 = (arg1 >>> 0) >>> 0;
  if (base1 < 0 || len1 < 0 || len1 > 33554431 || base1 > memory0.buffer.byteLength || len1 > Math.floor((memory0.buffer.byteLength - base1) / 8)) throw new WebAssemblyRuntimeError('wasm trap: out of bounds memory access');
  if (base1 % 4 !== 0) throw new TypeError(`list pointer [${base1}] is not aligned to 4`);
  var result1 = [];
  for (let i = 0; i < len1; i++) {
    const base = base1 + i * 8;
    var ptr0 = dataView(memory0).getUint32((base) + 0, true) >>> 0;
    var len0 = dataView(memory0).getUint32((base) + 4, true);
    var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
    result1.push(result0);
  }
  _debugLog('[iface="tairitsu-browser:full/dom-token-list@0.2.0", function="remove"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'remove',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => remove(BigInt.asUintN(64, BigInt(arg0)), result1)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/dom-token-list@0.2.0", function="remove"][Instruction::Return]', {
    funcName: 'remove',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline84.fnName = 'tairitsu-browser:full/dom-token-list@0.2.0#remove';

const _trampoline85 = function(arg0, arg1, arg2) {
  var ptr0 = (arg1 >>> 0) >>> 0;
  var len0 = arg2;
  var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
  _debugLog('[iface="tairitsu-browser:full/dom-token-list@0.2.0", function="contains"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'contains',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => contains(BigInt.asUintN(64, BigInt(arg0)), result0)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="tairitsu-browser:full/dom-token-list@0.2.0", function="contains"][Instruction::Return]', {
    funcName: 'contains',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([ret ? 1 : 0]);
  task.exit();
  return ret ? 1 : 0;
}
_trampoline85.fnName = 'tairitsu-browser:full/dom-token-list@0.2.0#contains';

const _trampoline86 = function(arg0, arg1) {
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-pointer-type"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getPointerType',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getPointerType(BigInt.asUintN(64, BigInt(arg0)))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  
  var encodeRes = _utf8AllocateAndEncode(ret, ((...args) => {
    const state = getOrCreateAsyncState(0);
    const previousMayLeave = state.mayLeave;
    const meta = _getGlobalCurrentThreadMeta(0);
    const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
    const previousStorage = thread?.storage;
    if (thread) { thread.storage = [0, 0]; }
    state.mayLeave = false;
    const restore = () => {
      state.mayLeave = previousMayLeave;
      if (thread) { thread.storage = previousStorage; }
    };
    let result;
    try {
      result = realloc0(...args);
    } catch (err) {
      restore();
      throw err;
    }
    if (result && typeof result.then === 'function') {
      return Promise.resolve(result).finally(restore);
    }
    restore();
    return result;
  }), memory0);
  var ptr0= encodeRes.ptr;
  var len0 = encodeRes.len;
  
  dataView(memory0).setUint32(((arg1 >>> 0)) + 4, len0, true);
  dataView(memory0).setUint32(((arg1 >>> 0)) + 0, ptr0, true);
  _debugLog('[iface="tairitsu-browser:full/pointer-event@0.2.0", function="get-pointer-type"][Instruction::Return]', {
    funcName: 'get-pointer-type',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline86.fnName = 'tairitsu-browser:full/pointer-event@0.2.0#getPointerType';

const _trampoline87 = function(arg0) {
  _debugLog('[iface="wasi:random/insecure-seed@0.2.12", function="insecure-seed"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'insecureSeed',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => insecureSeed()),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var [tuple0_0, tuple0_1] = ret;
  dataView(memory0).setBigInt64(((arg0 >>> 0)) + 0, toUint64(tuple0_0), true);
  dataView(memory0).setBigInt64(((arg0 >>> 0)) + 8, toUint64(tuple0_1), true);
  _debugLog('[iface="wasi:random/insecure-seed@0.2.12", function="insecure-seed"][Instruction::Return]', {
    funcName: 'insecure-seed',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline87.fnName = 'wasi:random/insecure-seed@0.2.12#insecureSeed';

const _trampoline88 = function(arg0, arg1, arg2) {
  var len3 = arg1;
  var base3 = (arg0 >>> 0) >>> 0;
  if (base3 < 0 || len3 < 0 || len3 > 67108863 || base3 > memory0.buffer.byteLength || len3 > Math.floor((memory0.buffer.byteLength - base3) / 4)) throw new WebAssemblyRuntimeError('wasm trap: out of bounds memory access');
  if (base3 % 4 !== 0) throw new TypeError(`list pointer [${base3}] is not aligned to 4`);
  var result3 = [];
  for (let i = 0; i < len3; i++) {
    const base = base3 + i * 4;
    var handle1 = dataView(memory0).getInt32((base) + 0, true);
    
    var rep2 = handleTable0[(handle1 << 1) + 1] % T_OWN;
    var rsc0 = captureTable0.get(rep2);
    if (!rsc0) {
      rsc0 = Object.create(Pollable.prototype);
      Object.defineProperty(rsc0, symbolRscHandle, { writable: true, value: handle1});
      Object.defineProperty(rsc0, symbolRscRep, { writable: true, value: rep2});
    }
    
    curResourceBorrows.push(rsc0);
    result3.push(rsc0);
  }
  _debugLog('[iface="wasi:io/poll@0.2.12", function="poll"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'poll',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => poll(result3)),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  for (const entry of curResourceBorrows) {
    const rsc = entry.rsc ?? entry;
    if (entry.drop) {
      if (rsc[symbolRscHandle]) {
        entry.drop(rsc[symbolRscHandle]);
      }
    }
    rsc[symbolRscHandle] = undefined;
  }
  curResourceBorrows = [];
  var val4 = ret;
  var len4 = val4.length;
  var ptr4 = ((...args) => {
    const state = getOrCreateAsyncState(0);
    const previousMayLeave = state.mayLeave;
    const meta = _getGlobalCurrentThreadMeta(0);
    const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
    const previousStorage = thread?.storage;
    if (thread) { thread.storage = [0, 0]; }
    state.mayLeave = false;
    const restore = () => {
      state.mayLeave = previousMayLeave;
      if (thread) { thread.storage = previousStorage; }
    };
    let result;
    try {
      result = realloc0(...args);
    } catch (err) {
      restore();
      throw err;
    }
    if (result && typeof result.then === 'function') {
      return Promise.resolve(result).finally(restore);
    }
    restore();
    return result;
  })(0, 0, 4, len4 * 4);
  
  let valData4;
  const valLenBytes4 = len4 * 4;
  if (Array.isArray(val4)) {
    // Regular array likely containing numbers, write values to memory
    let offset = 0;
    const dv4 = new DataView(memory0.buffer);
    for (const v of val4) {
      _requireValidNumericPrimitive.bind(null, 'u32')(v);
      dv4.setUint32(ptr4+ offset, v, true);
      offset += 4;
    }
  } else {
    // TypedArray / ArrayBuffer-like, direct copy
    valData4 = new Uint8Array(val4.buffer || val4, val4.byteOffset, valLenBytes4);
    const out4 = new Uint8Array(memory0.buffer, ptr4, valLenBytes4);
    out4.set(valData4);
  }
  
  dataView(memory0).setUint32(((arg2 >>> 0)) + 4, len4, true);
  dataView(memory0).setUint32(((arg2 >>> 0)) + 0, ptr4, true);
  _debugLog('[iface="wasi:io/poll@0.2.12", function="poll"][Instruction::Return]', {
    funcName: 'poll',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline88.fnName = 'wasi:io/poll@0.2.12#poll';

const handleTable1 = [T_FLAG, 0];
handleTable1._createdReps = new Set();
handleTable1._componentIdx = 0;


const captureTable1= new Map();
let captureCnt1= 0;

HANDLE_TABLES[1] = handleTable1;

const _trampoline89 = function(arg0, arg1) {
  var handle1 = arg0;
  
  var rep2 = handleTable3[(handle1 << 1) + 1] % T_OWN;
  var rsc0 = captureTable3.get(rep2);
  if (!rsc0) {
    rsc0 = Object.create(OutputStream.prototype);
    Object.defineProperty(rsc0, symbolRscHandle, { writable: true, value: handle1});
    Object.defineProperty(rsc0, symbolRscRep, { writable: true, value: rep2});
  }
  
  curResourceBorrows.push(rsc0);
  _debugLog('[iface="wasi:io/streams@0.2.12", function="[method]output-stream.check-write"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'checkWrite',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'result-catch-handler',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  try {
    const hostRet3 = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => rsc0.checkWrite()),
    })
    ;
    ret = hostRet3 !== null && typeof hostRet3 === 'object' && (hostRet3.tag === 'ok' || hostRet3.tag === 'err')
    ? hostRet3
    : { tag: 'ok', val: hostRet3};
  } catch (e) {
    if (getOrCreateAsyncState(0).markTrapped(e)) { throw e; }
    ret = { tag: 'err', val: getErrorPayload(e) };
  }
  
  for (const entry of curResourceBorrows) {
    const rsc = entry.rsc ?? entry;
    if (entry.drop) {
      if (rsc[symbolRscHandle]) {
        entry.drop(rsc[symbolRscHandle]);
      }
    }
    rsc[symbolRscHandle] = undefined;
  }
  curResourceBorrows = [];
  var variant6 = ret;
  switch (variant6.tag) {
    case 'ok': {
      const e = variant6.val;
      dataView(memory0).setInt8(((arg1 >>> 0)) + 0, 0, true);
      dataView(memory0).setBigInt64(((arg1 >>> 0)) + 8, toUint64(e), true);
      
      break;
    }
    case 'err': {
      const e = variant6.val;
      dataView(memory0).setInt8(((arg1 >>> 0)) + 0, 1, true);
      var variant5 = e;
      switch (variant5.tag) {
        case 'last-operation-failed': {
          const e = variant5.val;
          dataView(memory0).setInt8(((arg1 >>> 0)) + 8, 0, true);
          
          if (!(e instanceof Error$1)) {
            throw new TypeError('Resource error: Not a valid \"Error\" resource.');
          }
          
          
          var handle4 = e[symbolRscHandle];
          if (!handle4) {
            const rep = (e[symbolRscRep] ?? ++captureCnt1) >>> 0;
            captureTable1.set(rep, e);
            handle4 = rscTableCreateOwn(handleTable1, rep);
          }
          
          dataView(memory0).setInt32(((arg1 >>> 0)) + 12, handle4, true);
          break;
        }
        case 'closed': {
          dataView(memory0).setInt8(((arg1 >>> 0)) + 8, 1, true);
          break;
        }
        default: {
          throw new TypeError(`invalid variant tag value \`${JSON.stringify(variant5.tag)}\` (received \`${variant5}\`) specified for \`StreamError\``);
        }
      }
      
      break;
    }
    default: {
      _debugLog("ERROR: invalid value (expected result as object with 'tag' member)", { value: variant6, valueType: typeof variant6});
      throw new TypeError('invalid variant specified for result');
    }
  }
  _debugLog('[iface="wasi:io/streams@0.2.12", function="[method]output-stream.check-write"][Instruction::Return]', {
    funcName: '[method]output-stream.check-write',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline89.fnName = 'wasi:io/streams@0.2.12#checkWrite';

const _trampoline90 = function(arg0, arg1, arg2, arg3) {
  var handle1 = arg0;
  
  var rep2 = handleTable3[(handle1 << 1) + 1] % T_OWN;
  var rsc0 = captureTable3.get(rep2);
  if (!rsc0) {
    rsc0 = Object.create(OutputStream.prototype);
    Object.defineProperty(rsc0, symbolRscHandle, { writable: true, value: handle1});
    Object.defineProperty(rsc0, symbolRscRep, { writable: true, value: rep2});
  }
  
  curResourceBorrows.push(rsc0);
  var ptr3 = (arg1 >>> 0) >>> 0;
  var len3 = arg2;
  if (ptr3 < 0 || len3 < 0 || len3 > 268435455 || ptr3 > memory0.buffer.byteLength || len3 > Math.floor((memory0.buffer.byteLength - ptr3) / 1)) throw new WebAssemblyRuntimeError('wasm trap: out of bounds memory access');
  if (ptr3 % 1 !== 0) throw new TypeError(`list pointer [${ptr3}] is not aligned to 1`);
  var result3 = new Uint8Array(memory0.buffer.slice(ptr3, ptr3 + len3 * 1));
  _debugLog('[iface="wasi:io/streams@0.2.12", function="[method]output-stream.write"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'write',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'result-catch-handler',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  try {
    const hostRet4 = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => rsc0.write(result3)),
    })
    ;
    ret = hostRet4 !== null && typeof hostRet4 === 'object' && (hostRet4.tag === 'ok' || hostRet4.tag === 'err')
    ? hostRet4
    : { tag: 'ok', val: hostRet4};
  } catch (e) {
    if (getOrCreateAsyncState(0).markTrapped(e)) { throw e; }
    ret = { tag: 'err', val: getErrorPayload(e) };
  }
  
  for (const entry of curResourceBorrows) {
    const rsc = entry.rsc ?? entry;
    if (entry.drop) {
      if (rsc[symbolRscHandle]) {
        entry.drop(rsc[symbolRscHandle]);
      }
    }
    rsc[symbolRscHandle] = undefined;
  }
  curResourceBorrows = [];
  var variant7 = ret;
  switch (variant7.tag) {
    case 'ok': {
      const e = variant7.val;
      dataView(memory0).setInt8(((arg3 >>> 0)) + 0, 0, true);
      
      break;
    }
    case 'err': {
      const e = variant7.val;
      dataView(memory0).setInt8(((arg3 >>> 0)) + 0, 1, true);
      var variant6 = e;
      switch (variant6.tag) {
        case 'last-operation-failed': {
          const e = variant6.val;
          dataView(memory0).setInt8(((arg3 >>> 0)) + 4, 0, true);
          
          if (!(e instanceof Error$1)) {
            throw new TypeError('Resource error: Not a valid \"Error\" resource.');
          }
          
          
          var handle5 = e[symbolRscHandle];
          if (!handle5) {
            const rep = (e[symbolRscRep] ?? ++captureCnt1) >>> 0;
            captureTable1.set(rep, e);
            handle5 = rscTableCreateOwn(handleTable1, rep);
          }
          
          dataView(memory0).setInt32(((arg3 >>> 0)) + 8, handle5, true);
          break;
        }
        case 'closed': {
          dataView(memory0).setInt8(((arg3 >>> 0)) + 4, 1, true);
          break;
        }
        default: {
          throw new TypeError(`invalid variant tag value \`${JSON.stringify(variant6.tag)}\` (received \`${variant6}\`) specified for \`StreamError\``);
        }
      }
      
      break;
    }
    default: {
      _debugLog("ERROR: invalid value (expected result as object with 'tag' member)", { value: variant7, valueType: typeof variant7});
      throw new TypeError('invalid variant specified for result');
    }
  }
  _debugLog('[iface="wasi:io/streams@0.2.12", function="[method]output-stream.write"][Instruction::Return]', {
    funcName: '[method]output-stream.write',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline90.fnName = 'wasi:io/streams@0.2.12#write';

const _trampoline91 = function(arg0, arg1) {
  var handle1 = arg0;
  
  var rep2 = handleTable3[(handle1 << 1) + 1] % T_OWN;
  var rsc0 = captureTable3.get(rep2);
  if (!rsc0) {
    rsc0 = Object.create(OutputStream.prototype);
    Object.defineProperty(rsc0, symbolRscHandle, { writable: true, value: handle1});
    Object.defineProperty(rsc0, symbolRscRep, { writable: true, value: rep2});
  }
  
  curResourceBorrows.push(rsc0);
  _debugLog('[iface="wasi:io/streams@0.2.12", function="[method]output-stream.blocking-flush"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'blockingFlush',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'result-catch-handler',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  try {
    const hostRet3 = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => rsc0.blockingFlush()),
    })
    ;
    ret = hostRet3 !== null && typeof hostRet3 === 'object' && (hostRet3.tag === 'ok' || hostRet3.tag === 'err')
    ? hostRet3
    : { tag: 'ok', val: hostRet3};
  } catch (e) {
    if (getOrCreateAsyncState(0).markTrapped(e)) { throw e; }
    ret = { tag: 'err', val: getErrorPayload(e) };
  }
  
  for (const entry of curResourceBorrows) {
    const rsc = entry.rsc ?? entry;
    if (entry.drop) {
      if (rsc[symbolRscHandle]) {
        entry.drop(rsc[symbolRscHandle]);
      }
    }
    rsc[symbolRscHandle] = undefined;
  }
  curResourceBorrows = [];
  var variant6 = ret;
  switch (variant6.tag) {
    case 'ok': {
      const e = variant6.val;
      dataView(memory0).setInt8(((arg1 >>> 0)) + 0, 0, true);
      
      break;
    }
    case 'err': {
      const e = variant6.val;
      dataView(memory0).setInt8(((arg1 >>> 0)) + 0, 1, true);
      var variant5 = e;
      switch (variant5.tag) {
        case 'last-operation-failed': {
          const e = variant5.val;
          dataView(memory0).setInt8(((arg1 >>> 0)) + 4, 0, true);
          
          if (!(e instanceof Error$1)) {
            throw new TypeError('Resource error: Not a valid \"Error\" resource.');
          }
          
          
          var handle4 = e[symbolRscHandle];
          if (!handle4) {
            const rep = (e[symbolRscRep] ?? ++captureCnt1) >>> 0;
            captureTable1.set(rep, e);
            handle4 = rscTableCreateOwn(handleTable1, rep);
          }
          
          dataView(memory0).setInt32(((arg1 >>> 0)) + 8, handle4, true);
          break;
        }
        case 'closed': {
          dataView(memory0).setInt8(((arg1 >>> 0)) + 4, 1, true);
          break;
        }
        default: {
          throw new TypeError(`invalid variant tag value \`${JSON.stringify(variant5.tag)}\` (received \`${variant5}\`) specified for \`StreamError\``);
        }
      }
      
      break;
    }
    default: {
      _debugLog("ERROR: invalid value (expected result as object with 'tag' member)", { value: variant6, valueType: typeof variant6});
      throw new TypeError('invalid variant specified for result');
    }
  }
  _debugLog('[iface="wasi:io/streams@0.2.12", function="[method]output-stream.blocking-flush"][Instruction::Return]', {
    funcName: '[method]output-stream.blocking-flush',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline91.fnName = 'wasi:io/streams@0.2.12#blockingFlush';

const _trampoline92 = function(arg0) {
  _debugLog('[iface="wasi:cli/environment@0.2.12", function="get-environment"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getEnvironment',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getEnvironment()),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var vec3 = ret;
  var len3 = vec3.length;
  var result3 = ((...args) => {
    const state = getOrCreateAsyncState(0);
    const previousMayLeave = state.mayLeave;
    const meta = _getGlobalCurrentThreadMeta(0);
    const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
    const previousStorage = thread?.storage;
    if (thread) { thread.storage = [0, 0]; }
    state.mayLeave = false;
    const restore = () => {
      state.mayLeave = previousMayLeave;
      if (thread) { thread.storage = previousStorage; }
    };
    let result;
    try {
      result = realloc0(...args);
    } catch (err) {
      restore();
      throw err;
    }
    if (result && typeof result.then === 'function') {
      return Promise.resolve(result).finally(restore);
    }
    restore();
    return result;
  })(0, 0, 4, len3 * 16);
  if (result3 < 0 || len3 < 0 || len3 > 16777215 || result3 > memory0.buffer.byteLength || len3 > Math.floor((memory0.buffer.byteLength - result3) / 16)) throw new WebAssemblyRuntimeError('wasm trap: out of bounds memory access');
  for (let i = 0; i < vec3.length; i++) {
    const e = vec3[i];
    const base = result3 + i * 16;var [tuple0_0, tuple0_1] = e;
    
    var encodeRes = _utf8AllocateAndEncode(tuple0_0, ((...args) => {
      const state = getOrCreateAsyncState(0);
      const previousMayLeave = state.mayLeave;
      const meta = _getGlobalCurrentThreadMeta(0);
      const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
      const previousStorage = thread?.storage;
      if (thread) { thread.storage = [0, 0]; }
      state.mayLeave = false;
      const restore = () => {
        state.mayLeave = previousMayLeave;
        if (thread) { thread.storage = previousStorage; }
      };
      let result;
      try {
        result = realloc0(...args);
      } catch (err) {
        restore();
        throw err;
      }
      if (result && typeof result.then === 'function') {
        return Promise.resolve(result).finally(restore);
      }
      restore();
      return result;
    }), memory0);
    var ptr1= encodeRes.ptr;
    var len1 = encodeRes.len;
    
    dataView(memory0).setUint32((base) + 4, len1, true);
    dataView(memory0).setUint32((base) + 0, ptr1, true);
    
    var encodeRes = _utf8AllocateAndEncode(tuple0_1, ((...args) => {
      const state = getOrCreateAsyncState(0);
      const previousMayLeave = state.mayLeave;
      const meta = _getGlobalCurrentThreadMeta(0);
      const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
      const previousStorage = thread?.storage;
      if (thread) { thread.storage = [0, 0]; }
      state.mayLeave = false;
      const restore = () => {
        state.mayLeave = previousMayLeave;
        if (thread) { thread.storage = previousStorage; }
      };
      let result;
      try {
        result = realloc0(...args);
      } catch (err) {
        restore();
        throw err;
      }
      if (result && typeof result.then === 'function') {
        return Promise.resolve(result).finally(restore);
      }
      restore();
      return result;
    }), memory0);
    var ptr2= encodeRes.ptr;
    var len2 = encodeRes.len;
    
    dataView(memory0).setUint32((base) + 12, len2, true);
    dataView(memory0).setUint32((base) + 8, ptr2, true);
  }
  dataView(memory0).setUint32(((arg0 >>> 0)) + 4, len3, true);
  dataView(memory0).setUint32(((arg0 >>> 0)) + 0, result3, true);
  _debugLog('[iface="wasi:cli/environment@0.2.12", function="get-environment"][Instruction::Return]', {
    funcName: 'get-environment',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline92.fnName = 'wasi:cli/environment@0.2.12#getEnvironment';

const handleTable4 = [T_FLAG, 0];
handleTable4._createdReps = new Set();
handleTable4._componentIdx = 0;


const captureTable4= new Map();
let captureCnt4= 0;

HANDLE_TABLES[4] = handleTable4;

const _trampoline93 = function(arg0) {
  _debugLog('[iface="wasi:cli/terminal-stdin@0.2.12", function="get-terminal-stdin"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getTerminalStdin',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getTerminalStdin()),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var variant1 = ret;
  if (variant1 === null || variant1=== undefined) {
    dataView(memory0).setInt8(((arg0 >>> 0)) + 0, 0, true);
  } else {
    const e = variant1;
    dataView(memory0).setInt8(((arg0 >>> 0)) + 0, 1, true);
    
    if (!(e instanceof TerminalInput)) {
      throw new TypeError('Resource error: Not a valid \"TerminalInput\" resource.');
    }
    
    
    var handle0 = e[symbolRscHandle];
    if (!handle0) {
      const rep = (e[symbolRscRep] ?? ++captureCnt4) >>> 0;
      captureTable4.set(rep, e);
      handle0 = rscTableCreateOwn(handleTable4, rep);
    }
    
    dataView(memory0).setInt32(((arg0 >>> 0)) + 4, handle0, true);
  }
  _debugLog('[iface="wasi:cli/terminal-stdin@0.2.12", function="get-terminal-stdin"][Instruction::Return]', {
    funcName: 'get-terminal-stdin',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline93.fnName = 'wasi:cli/terminal-stdin@0.2.12#getTerminalStdin';

const handleTable5 = [T_FLAG, 0];
handleTable5._createdReps = new Set();
handleTable5._componentIdx = 0;


const captureTable5= new Map();
let captureCnt5= 0;

HANDLE_TABLES[5] = handleTable5;

const _trampoline94 = function(arg0) {
  _debugLog('[iface="wasi:cli/terminal-stdout@0.2.12", function="get-terminal-stdout"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getTerminalStdout',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getTerminalStdout()),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var variant1 = ret;
  if (variant1 === null || variant1=== undefined) {
    dataView(memory0).setInt8(((arg0 >>> 0)) + 0, 0, true);
  } else {
    const e = variant1;
    dataView(memory0).setInt8(((arg0 >>> 0)) + 0, 1, true);
    
    if (!(e instanceof TerminalOutput)) {
      throw new TypeError('Resource error: Not a valid \"TerminalOutput\" resource.');
    }
    
    
    var handle0 = e[symbolRscHandle];
    if (!handle0) {
      const rep = (e[symbolRscRep] ?? ++captureCnt5) >>> 0;
      captureTable5.set(rep, e);
      handle0 = rscTableCreateOwn(handleTable5, rep);
    }
    
    dataView(memory0).setInt32(((arg0 >>> 0)) + 4, handle0, true);
  }
  _debugLog('[iface="wasi:cli/terminal-stdout@0.2.12", function="get-terminal-stdout"][Instruction::Return]', {
    funcName: 'get-terminal-stdout',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline94.fnName = 'wasi:cli/terminal-stdout@0.2.12#getTerminalStdout';

const _trampoline95 = function(arg0) {
  _debugLog('[iface="wasi:cli/terminal-stderr@0.2.12", function="get-terminal-stderr"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let threadID;
  let subtask;
  
  const createTask = () => {
    ({ task, threadID } = createNewCurrentImplicitThread({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getTerminalStderr',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    }));
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentThreadMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  let ret;
  
  
  try {
    ret = _withGlobalCurrentThreadMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      threadID,
      fn: () => _trackHostOperation(() => getTerminalStderr()),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    getOrCreateAsyncState(0).markTrapped(err);
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var variant1 = ret;
  if (variant1 === null || variant1=== undefined) {
    dataView(memory0).setInt8(((arg0 >>> 0)) + 0, 0, true);
  } else {
    const e = variant1;
    dataView(memory0).setInt8(((arg0 >>> 0)) + 0, 1, true);
    
    if (!(e instanceof TerminalOutput)) {
      throw new TypeError('Resource error: Not a valid \"TerminalOutput\" resource.');
    }
    
    
    var handle0 = e[symbolRscHandle];
    if (!handle0) {
      const rep = (e[symbolRscRep] ?? ++captureCnt5) >>> 0;
      captureTable5.set(rep, e);
      handle0 = rscTableCreateOwn(handleTable5, rep);
    }
    
    dataView(memory0).setInt32(((arg0 >>> 0)) + 4, handle0, true);
  }
  _debugLog('[iface="wasi:cli/terminal-stderr@0.2.12", function="get-terminal-stderr"][Instruction::Return]', {
    funcName: 'get-terminal-stderr',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline95.fnName = 'wasi:cli/terminal-stderr@0.2.12#getTerminalStderr';
let exports2;
let postReturn0;
let postReturn0Async;
let postReturn1;
let postReturn1Async;
let postReturn2;
let postReturn2Async;
let postReturn3;
let postReturn3Async;
let postReturn4;
let postReturn4Async;
let eventCallbacks020OnMouseEvent;

function onMouseEvent(arg0, arg1, arg2) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'eventCallbacks020OnMouseEvent',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (null!== null) {
    task.setReturnMemoryIdx(null);
    task.setReturnMemory((() => null)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        var {clientX: v0_0, clientY: v0_1, offsetX: v0_2, offsetY: v0_3, button: v0_4, buttons: v0_5, ctrlKey: v0_6, shiftKey: v0_7, altKey: v0_8, metaKey: v0_9 } = arg2;
        _debugLog('[iface="tairitsu-browser:full/event-callbacks@0.2.0", function="on-mouse-event"][Instruction::CallWasm] enter', {
          funcName: 'on-mouse-event',
          paramCount: 12,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => eventCallbacks020OnMouseEvent(toUint64(arg0), toUint64(arg1), +v0_0, +v0_1, +v0_2, +v0_3, toUint8(v0_4), toUint8(v0_5), v0_6 ? 1 : 0, v0_7 ? 1 : 0, v0_8 ? 1 : 0, v0_9 ? 1 : 0),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/event-callbacks@0.2.0", function="on-mouse-event"][Instruction::Return]', {
          funcName: 'on-mouse-event',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let eventCallbacks020OnKeyboardEvent;

function onKeyboardEvent(arg0, arg1, arg2) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'eventCallbacks020OnKeyboardEvent',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        var {key: v0_0, code: v0_1, keyCode: v0_2, ctrlKey: v0_3, shiftKey: v0_4, altKey: v0_5, metaKey: v0_6, repeat: v0_7 } = arg2;
        
        var encodeRes = _utf8AllocateAndEncode(v0_0, ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        }), memory0);
        var ptr1= encodeRes.ptr;
        var len1 = encodeRes.len;
        
        
        var encodeRes = _utf8AllocateAndEncode(v0_1, ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        }), memory0);
        var ptr2= encodeRes.ptr;
        var len2 = encodeRes.len;
        
        _debugLog('[iface="tairitsu-browser:full/event-callbacks@0.2.0", function="on-keyboard-event"][Instruction::CallWasm] enter', {
          funcName: 'on-keyboard-event',
          paramCount: 12,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => eventCallbacks020OnKeyboardEvent(toUint64(arg0), toUint64(arg1), ptr1, len1, ptr2, len2, toUint32(v0_2), v0_3 ? 1 : 0, v0_4 ? 1 : 0, v0_5 ? 1 : 0, v0_6 ? 1 : 0, v0_7 ? 1 : 0),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/event-callbacks@0.2.0", function="on-keyboard-event"][Instruction::Return]', {
          funcName: 'on-keyboard-event',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let eventCallbacks020OnFocusEvent;

function onFocusEvent(arg0, arg1, arg2) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'eventCallbacks020OnFocusEvent',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (null!== null) {
    task.setReturnMemoryIdx(null);
    task.setReturnMemory((() => null)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        var {relatedTarget: v0_0 } = arg2;
        var variant1 = v0_0;
        let variant1_0;
        let variant1_1;
        if (variant1 === null || variant1=== undefined) {
          variant1_0 = 0;
          variant1_1 = 0n;
        } else {
          const e = variant1;
          variant1_0 = 1;
          variant1_1 = toUint64(e);
        }
        _debugLog('[iface="tairitsu-browser:full/event-callbacks@0.2.0", function="on-focus-event"][Instruction::CallWasm] enter', {
          funcName: 'on-focus-event',
          paramCount: 4,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => eventCallbacks020OnFocusEvent(toUint64(arg0), toUint64(arg1), variant1_0, variant1_1),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/event-callbacks@0.2.0", function="on-focus-event"][Instruction::Return]', {
          funcName: 'on-focus-event',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let eventCallbacks020OnInputEvent;

function onInputEvent(arg0, arg1, arg2) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'eventCallbacks020OnInputEvent',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        var {data: v0_0, inputType: v0_1 } = arg2;
        var variant2 = v0_0;
        let variant2_0;
        let variant2_1;
        let variant2_2;
        if (variant2 === null || variant2=== undefined) {
          variant2_0 = 0;
          variant2_1 = 0;
          variant2_2 = 0;
        } else {
          const e = variant2;
          
          var encodeRes = _utf8AllocateAndEncode(e, ((...args) => {
            const state = getOrCreateAsyncState(0);
            const previousMayLeave = state.mayLeave;
            const meta = _getGlobalCurrentThreadMeta(0);
            const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
            const previousStorage = thread?.storage;
            if (thread) { thread.storage = [0, 0]; }
            state.mayLeave = false;
            const restore = () => {
              state.mayLeave = previousMayLeave;
              if (thread) { thread.storage = previousStorage; }
            };
            let result;
            try {
              result = realloc0(...args);
            } catch (err) {
              restore();
              throw err;
            }
            if (result && typeof result.then === 'function') {
              return Promise.resolve(result).finally(restore);
            }
            restore();
            return result;
          }), memory0);
          var ptr1= encodeRes.ptr;
          var len1 = encodeRes.len;
          
          variant2_0 = 1;
          variant2_1 = ptr1;
          variant2_2 = len1;
        }
        
        var encodeRes = _utf8AllocateAndEncode(v0_1, ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        }), memory0);
        var ptr3= encodeRes.ptr;
        var len3 = encodeRes.len;
        
        _debugLog('[iface="tairitsu-browser:full/event-callbacks@0.2.0", function="on-input-event"][Instruction::CallWasm] enter', {
          funcName: 'on-input-event',
          paramCount: 7,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => eventCallbacks020OnInputEvent(toUint64(arg0), toUint64(arg1), variant2_0, variant2_1, variant2_2, ptr3, len3),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/event-callbacks@0.2.0", function="on-input-event"][Instruction::Return]', {
          funcName: 'on-input-event',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let eventCallbacks020OnGenericEvent;

function onGenericEvent(arg0, arg1, arg2) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'eventCallbacks020OnGenericEvent',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        
        var encodeRes = _utf8AllocateAndEncode(arg2, ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        }), memory0);
        var ptr0= encodeRes.ptr;
        var len0 = encodeRes.len;
        
        _debugLog('[iface="tairitsu-browser:full/event-callbacks@0.2.0", function="on-generic-event"][Instruction::CallWasm] enter', {
          funcName: 'on-generic-event',
          paramCount: 4,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => eventCallbacks020OnGenericEvent(toUint64(arg0), toUint64(arg1), ptr0, len0),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/event-callbacks@0.2.0", function="on-generic-event"][Instruction::Return]', {
          funcName: 'on-generic-event',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let lifecycle020Start;

function start() {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'lifecycle020Start',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'throw-result-err',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(3);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        _debugLog('[iface="tairitsu-browser:full/lifecycle@0.2.0", function="start"][Instruction::CallWasm] enter', {
          funcName: 'start',
          paramCount: 0,
          async: false,
          postReturn: true,
        });
        
        let ret;
        
        try {
          ret =  _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => lifecycle020Start(),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        let variant1;
        switch (dataView(memory0).getUint8(((ret >>> 0)) + 0, true)) {
          case 0: {
            variant1= {
              tag: 'ok',
              val: undefined
            };
            break;
          }
          case 1: {
            var ptr0 = dataView(memory0).getUint32(((ret >>> 0)) + 4, true) >>> 0;
            var len0 = dataView(memory0).getUint32(((ret >>> 0)) + 8, true);
            var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
            variant1= {
              tag: 'err',
              val: result0
            };
            break;
          }
          default: {
            throw new TypeError('invalid variant discriminant for expected');
          }
        }
        _debugLog('[iface="tairitsu-browser:full/lifecycle@0.2.0", function="start"][Instruction::Return]', {
          funcName: 'start',
          paramCount: 1,
          async: false,
          postReturn: true
        });
        const retCopy = variant1;
        task.resolve([retCopy]);
        
        let cstate = getOrCreateAsyncState(0);
        cstate.mayLeave = false;
        try {
          postReturn0(ret);
        } catch (err) {
          err = _postReturnTrap(0, err);
          // Cleanup can fail after the result was recorded.
          task.reject(err);
          throw err;
        }
        cstate.mayLeave = true;
        task.exit();
        
        
        
        if (typeof retCopy === 'object' && retCopy.tag === 'err') {
          throw new ComponentError(retCopy.val);
        }
        return retCopy.val;
        
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let timerCallbacks020OnTimeout;

function onTimeout(arg0) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'timerCallbacks020OnTimeout',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (null!== null) {
    task.setReturnMemoryIdx(null);
    task.setReturnMemory((() => null)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        _debugLog('[iface="tairitsu-browser:full/timer-callbacks@0.2.0", function="on-timeout"][Instruction::CallWasm] enter', {
          funcName: 'on-timeout',
          paramCount: 1,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => timerCallbacks020OnTimeout(toUint64(arg0)),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/timer-callbacks@0.2.0", function="on-timeout"][Instruction::Return]', {
          funcName: 'on-timeout',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let timerCallbacks020OnInterval;

function onInterval(arg0) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'timerCallbacks020OnInterval',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (null!== null) {
    task.setReturnMemoryIdx(null);
    task.setReturnMemory((() => null)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        _debugLog('[iface="tairitsu-browser:full/timer-callbacks@0.2.0", function="on-interval"][Instruction::CallWasm] enter', {
          funcName: 'on-interval',
          paramCount: 1,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => timerCallbacks020OnInterval(toUint64(arg0)),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/timer-callbacks@0.2.0", function="on-interval"][Instruction::Return]', {
          funcName: 'on-interval',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let animationCallbacks020OnFrame;

function onFrame(arg0, arg1) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'animationCallbacks020OnFrame',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (null!== null) {
    task.setReturnMemoryIdx(null);
    task.setReturnMemory((() => null)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        _debugLog('[iface="tairitsu-browser:full/animation-callbacks@0.2.0", function="on-frame"][Instruction::CallWasm] enter', {
          funcName: 'on-frame',
          paramCount: 2,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => animationCallbacks020OnFrame(toUint64(arg0), +arg1),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/animation-callbacks@0.2.0", function="on-frame"][Instruction::Return]', {
          funcName: 'on-frame',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let resizeObserverCallbacks020OnResize;

function onResize(arg0, arg1) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'resizeObserverCallbacks020OnResize',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        var vec2 = arg1;
        var len2 = vec2.length;
        var result2 = ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        })(0, 0, 8, len2 * 40);
        if (result2 < 0 || len2 < 0 || len2 > 6710886 || result2 > memory0.buffer.byteLength || len2 > Math.floor((memory0.buffer.byteLength - result2) / 40)) throw new WebAssemblyRuntimeError('wasm trap: out of bounds memory access');
        for (let i = 0; i < vec2.length; i++) {
          const e = vec2[i];
          const base = result2 + i * 40;var {target: v0_0, contentRect: v0_1 } = e;
          dataView(memory0).setBigInt64((base) + 0, toUint64(v0_0), true);
          var {x: v1_0, y: v1_1, width: v1_2, height: v1_3 } = v0_1;
          dataView(memory0).setFloat64((base) + 8, +v1_0, true);
          dataView(memory0).setFloat64((base) + 16, +v1_1, true);
          dataView(memory0).setFloat64((base) + 24, +v1_2, true);
          dataView(memory0).setFloat64((base) + 32, +v1_3, true);
        }
        _debugLog('[iface="tairitsu-browser:full/resize-observer-callbacks@0.2.0", function="on-resize"][Instruction::CallWasm] enter', {
          funcName: 'on-resize',
          paramCount: 3,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => resizeObserverCallbacks020OnResize(toUint64(arg0), result2, len2),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/resize-observer-callbacks@0.2.0", function="on-resize"][Instruction::Return]', {
          funcName: 'on-resize',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let mutationObserverCallbacks020OnMutate;

function onMutate(arg0, arg1) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'mutationObserverCallbacks020OnMutate',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        var vec12 = arg1;
        var len12 = vec12.length;
        var result12 = ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        })(0, 0, 8, len12 * 120);
        if (result12 < 0 || len12 < 0 || len12 > 2236962 || result12 > memory0.buffer.byteLength || len12 > Math.floor((memory0.buffer.byteLength - result12) / 120)) throw new WebAssemblyRuntimeError('wasm trap: out of bounds memory access');
        for (let i = 0; i < vec12.length; i++) {
          const e = vec12[i];
          const base = result12 + i * 120;var {mutationType: v0_0, target: v0_1, addedNodes: v0_2, removedNodes: v0_3, previousSibling: v0_4, nextSibling: v0_5, attributeName: v0_6, attributeNamespace: v0_7, oldValue: v0_8 } = e;
          
          var encodeRes = _utf8AllocateAndEncode(v0_0, ((...args) => {
            const state = getOrCreateAsyncState(0);
            const previousMayLeave = state.mayLeave;
            const meta = _getGlobalCurrentThreadMeta(0);
            const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
            const previousStorage = thread?.storage;
            if (thread) { thread.storage = [0, 0]; }
            state.mayLeave = false;
            const restore = () => {
              state.mayLeave = previousMayLeave;
              if (thread) { thread.storage = previousStorage; }
            };
            let result;
            try {
              result = realloc0(...args);
            } catch (err) {
              restore();
              throw err;
            }
            if (result && typeof result.then === 'function') {
              return Promise.resolve(result).finally(restore);
            }
            restore();
            return result;
          }), memory0);
          var ptr1= encodeRes.ptr;
          var len1 = encodeRes.len;
          
          dataView(memory0).setUint32((base) + 4, len1, true);
          dataView(memory0).setUint32((base) + 0, ptr1, true);
          dataView(memory0).setBigInt64((base) + 8, toUint64(v0_1), true);
          var variant2 = v0_2;
          if (variant2 === null || variant2=== undefined) {
            dataView(memory0).setInt8((base) + 16, 0, true);
          } else {
            const e = variant2;
            dataView(memory0).setInt8((base) + 16, 1, true);
            dataView(memory0).setBigInt64((base) + 24, toUint64(e), true);
          }
          var variant3 = v0_3;
          if (variant3 === null || variant3=== undefined) {
            dataView(memory0).setInt8((base) + 32, 0, true);
          } else {
            const e = variant3;
            dataView(memory0).setInt8((base) + 32, 1, true);
            dataView(memory0).setBigInt64((base) + 40, toUint64(e), true);
          }
          var variant4 = v0_4;
          if (variant4 === null || variant4=== undefined) {
            dataView(memory0).setInt8((base) + 48, 0, true);
          } else {
            const e = variant4;
            dataView(memory0).setInt8((base) + 48, 1, true);
            dataView(memory0).setBigInt64((base) + 56, toUint64(e), true);
          }
          var variant5 = v0_5;
          if (variant5 === null || variant5=== undefined) {
            dataView(memory0).setInt8((base) + 64, 0, true);
          } else {
            const e = variant5;
            dataView(memory0).setInt8((base) + 64, 1, true);
            dataView(memory0).setBigInt64((base) + 72, toUint64(e), true);
          }
          var variant7 = v0_6;
          if (variant7 === null || variant7=== undefined) {
            dataView(memory0).setInt8((base) + 80, 0, true);
          } else {
            const e = variant7;
            dataView(memory0).setInt8((base) + 80, 1, true);
            
            var encodeRes = _utf8AllocateAndEncode(e, ((...args) => {
              const state = getOrCreateAsyncState(0);
              const previousMayLeave = state.mayLeave;
              const meta = _getGlobalCurrentThreadMeta(0);
              const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
              const previousStorage = thread?.storage;
              if (thread) { thread.storage = [0, 0]; }
              state.mayLeave = false;
              const restore = () => {
                state.mayLeave = previousMayLeave;
                if (thread) { thread.storage = previousStorage; }
              };
              let result;
              try {
                result = realloc0(...args);
              } catch (err) {
                restore();
                throw err;
              }
              if (result && typeof result.then === 'function') {
                return Promise.resolve(result).finally(restore);
              }
              restore();
              return result;
            }), memory0);
            var ptr6= encodeRes.ptr;
            var len6 = encodeRes.len;
            
            dataView(memory0).setUint32((base) + 88, len6, true);
            dataView(memory0).setUint32((base) + 84, ptr6, true);
          }
          var variant9 = v0_7;
          if (variant9 === null || variant9=== undefined) {
            dataView(memory0).setInt8((base) + 92, 0, true);
          } else {
            const e = variant9;
            dataView(memory0).setInt8((base) + 92, 1, true);
            
            var encodeRes = _utf8AllocateAndEncode(e, ((...args) => {
              const state = getOrCreateAsyncState(0);
              const previousMayLeave = state.mayLeave;
              const meta = _getGlobalCurrentThreadMeta(0);
              const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
              const previousStorage = thread?.storage;
              if (thread) { thread.storage = [0, 0]; }
              state.mayLeave = false;
              const restore = () => {
                state.mayLeave = previousMayLeave;
                if (thread) { thread.storage = previousStorage; }
              };
              let result;
              try {
                result = realloc0(...args);
              } catch (err) {
                restore();
                throw err;
              }
              if (result && typeof result.then === 'function') {
                return Promise.resolve(result).finally(restore);
              }
              restore();
              return result;
            }), memory0);
            var ptr8= encodeRes.ptr;
            var len8 = encodeRes.len;
            
            dataView(memory0).setUint32((base) + 100, len8, true);
            dataView(memory0).setUint32((base) + 96, ptr8, true);
          }
          var variant11 = v0_8;
          if (variant11 === null || variant11=== undefined) {
            dataView(memory0).setInt8((base) + 104, 0, true);
          } else {
            const e = variant11;
            dataView(memory0).setInt8((base) + 104, 1, true);
            
            var encodeRes = _utf8AllocateAndEncode(e, ((...args) => {
              const state = getOrCreateAsyncState(0);
              const previousMayLeave = state.mayLeave;
              const meta = _getGlobalCurrentThreadMeta(0);
              const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
              const previousStorage = thread?.storage;
              if (thread) { thread.storage = [0, 0]; }
              state.mayLeave = false;
              const restore = () => {
                state.mayLeave = previousMayLeave;
                if (thread) { thread.storage = previousStorage; }
              };
              let result;
              try {
                result = realloc0(...args);
              } catch (err) {
                restore();
                throw err;
              }
              if (result && typeof result.then === 'function') {
                return Promise.resolve(result).finally(restore);
              }
              restore();
              return result;
            }), memory0);
            var ptr10= encodeRes.ptr;
            var len10 = encodeRes.len;
            
            dataView(memory0).setUint32((base) + 112, len10, true);
            dataView(memory0).setUint32((base) + 108, ptr10, true);
          }
        }
        _debugLog('[iface="tairitsu-browser:full/mutation-observer-callbacks@0.2.0", function="on-mutate"][Instruction::CallWasm] enter', {
          funcName: 'on-mutate',
          paramCount: 3,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => mutationObserverCallbacks020OnMutate(toUint64(arg0), result12, len12),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/mutation-observer-callbacks@0.2.0", function="on-mutate"][Instruction::Return]', {
          funcName: 'on-mutate',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let mediaQueryListCallbacks020OnChange;

function onChange(arg0, arg1) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'mediaQueryListCallbacks020OnChange',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (null!== null) {
    task.setReturnMemoryIdx(null);
    task.setReturnMemory((() => null)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        _debugLog('[iface="tairitsu-browser:full/media-query-list-callbacks@0.2.0", function="on-change"][Instruction::CallWasm] enter', {
          funcName: 'on-change',
          paramCount: 2,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => mediaQueryListCallbacks020OnChange(toUint64(arg0), arg1 ? 1 : 0),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/media-query-list-callbacks@0.2.0", function="on-change"][Instruction::Return]', {
          funcName: 'on-change',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let scrollCallbacks020OnScrollEvent;

function onScrollEvent(arg0, arg1, arg2) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'scrollCallbacks020OnScrollEvent',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (null!== null) {
    task.setReturnMemoryIdx(null);
    task.setReturnMemory((() => null)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        _debugLog('[iface="tairitsu-browser:full/scroll-callbacks@0.2.0", function="on-scroll-event"][Instruction::CallWasm] enter', {
          funcName: 'on-scroll-event',
          paramCount: 3,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => scrollCallbacks020OnScrollEvent(toUint64(arg0), +arg1, +arg2),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/scroll-callbacks@0.2.0", function="on-scroll-event"][Instruction::Return]', {
          funcName: 'on-scroll-event',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let windowResizeCallbacks020OnWindowResize;

function onWindowResize(arg0, arg1, arg2) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'windowResizeCallbacks020OnWindowResize',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (null!== null) {
    task.setReturnMemoryIdx(null);
    task.setReturnMemory((() => null)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        _debugLog('[iface="tairitsu-browser:full/window-resize-callbacks@0.2.0", function="on-window-resize"][Instruction::CallWasm] enter', {
          funcName: 'on-window-resize',
          paramCount: 3,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => windowResizeCallbacks020OnWindowResize(toUint64(arg0), toInt32(arg1), toInt32(arg2)),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/window-resize-callbacks@0.2.0", function="on-window-resize"][Instruction::Return]', {
          funcName: 'on-window-resize',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let videoFrameCallbacks020OnVideoFrame;

function onVideoFrame(arg0, arg1) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'videoFrameCallbacks020OnVideoFrame',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        
        var encodeRes = _utf8AllocateAndEncode(arg1, ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        }), memory0);
        var ptr0= encodeRes.ptr;
        var len0 = encodeRes.len;
        
        _debugLog('[iface="tairitsu-browser:full/video-frame-callbacks@0.2.0", function="on-video-frame"][Instruction::CallWasm] enter', {
          funcName: 'on-video-frame',
          paramCount: 3,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => videoFrameCallbacks020OnVideoFrame(toUint64(arg0), ptr0, len0),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/video-frame-callbacks@0.2.0", function="on-video-frame"][Instruction::Return]', {
          funcName: 'on-video-frame',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let promiseCallbacks020OnPromiseResolved;

function onPromiseResolved(arg0, arg1) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'promiseCallbacks020OnPromiseResolved',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        
        var encodeRes = _utf8AllocateAndEncode(arg1, ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        }), memory0);
        var ptr0= encodeRes.ptr;
        var len0 = encodeRes.len;
        
        _debugLog('[iface="tairitsu-browser:full/promise-callbacks@0.2.0", function="on-promise-resolved"][Instruction::CallWasm] enter', {
          funcName: 'on-promise-resolved',
          paramCount: 3,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => promiseCallbacks020OnPromiseResolved(toUint64(arg0), ptr0, len0),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/promise-callbacks@0.2.0", function="on-promise-resolved"][Instruction::Return]', {
          funcName: 'on-promise-resolved',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let promiseCallbacks020OnPromiseRejected;

function onPromiseRejected(arg0, arg1) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'promiseCallbacks020OnPromiseRejected',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        
        var encodeRes = _utf8AllocateAndEncode(arg1, ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        }), memory0);
        var ptr0= encodeRes.ptr;
        var len0 = encodeRes.len;
        
        _debugLog('[iface="tairitsu-browser:full/promise-callbacks@0.2.0", function="on-promise-rejected"][Instruction::CallWasm] enter', {
          funcName: 'on-promise-rejected',
          paramCount: 3,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => promiseCallbacks020OnPromiseRejected(toUint64(arg0), ptr0, len0),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/promise-callbacks@0.2.0", function="on-promise-rejected"][Instruction::Return]', {
          funcName: 'on-promise-rejected',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let geolocationCallbacks020OnPositionSuccess;

function onPositionSuccess(arg0, arg1) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'geolocationCallbacks020OnPositionSuccess',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (null!== null) {
    task.setReturnMemoryIdx(null);
    task.setReturnMemory((() => null)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        var {latitude: v0_0, longitude: v0_1, altitude: v0_2, accuracy: v0_3, altitudeAccuracy: v0_4, heading: v0_5, speed: v0_6 } = arg1;
        var variant1 = v0_2;
        let variant1_0;
        let variant1_1;
        if (variant1 === null || variant1=== undefined) {
          variant1_0 = 0;
          variant1_1 = 0;
        } else {
          const e = variant1;
          variant1_0 = 1;
          variant1_1 = +e;
        }
        var variant2 = v0_4;
        let variant2_0;
        let variant2_1;
        if (variant2 === null || variant2=== undefined) {
          variant2_0 = 0;
          variant2_1 = 0;
        } else {
          const e = variant2;
          variant2_0 = 1;
          variant2_1 = +e;
        }
        var variant3 = v0_5;
        let variant3_0;
        let variant3_1;
        if (variant3 === null || variant3=== undefined) {
          variant3_0 = 0;
          variant3_1 = 0;
        } else {
          const e = variant3;
          variant3_0 = 1;
          variant3_1 = +e;
        }
        var variant4 = v0_6;
        let variant4_0;
        let variant4_1;
        if (variant4 === null || variant4=== undefined) {
          variant4_0 = 0;
          variant4_1 = 0;
        } else {
          const e = variant4;
          variant4_0 = 1;
          variant4_1 = +e;
        }
        _debugLog('[iface="tairitsu-browser:full/geolocation-callbacks@0.2.0", function="on-position-success"][Instruction::CallWasm] enter', {
          funcName: 'on-position-success',
          paramCount: 12,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => geolocationCallbacks020OnPositionSuccess(toUint64(arg0), +v0_0, +v0_1, variant1_0, variant1_1, +v0_3, variant2_0, variant2_1, variant3_0, variant3_1, variant4_0, variant4_1),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/geolocation-callbacks@0.2.0", function="on-position-success"][Instruction::Return]', {
          funcName: 'on-position-success',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let geolocationCallbacks020OnPositionError;

function onPositionError(arg0, arg1) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'geolocationCallbacks020OnPositionError',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        var {code: v0_0, message: v0_1 } = arg1;
        
        var encodeRes = _utf8AllocateAndEncode(v0_1, ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        }), memory0);
        var ptr1= encodeRes.ptr;
        var len1 = encodeRes.len;
        
        _debugLog('[iface="tairitsu-browser:full/geolocation-callbacks@0.2.0", function="on-position-error"][Instruction::CallWasm] enter', {
          funcName: 'on-position-error',
          paramCount: 4,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => geolocationCallbacks020OnPositionError(toUint64(arg0), toUint8(v0_0), ptr1, len1),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/geolocation-callbacks@0.2.0", function="on-position-error"][Instruction::Return]', {
          funcName: 'on-position-error',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let idbCallbacks020OnIdbRequestSuccess;

function onIdbRequestSuccess(arg0, arg1) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'idbCallbacks020OnIdbRequestSuccess',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        var variant1 = arg1;
        let variant1_0;
        let variant1_1;
        let variant1_2;
        if (variant1 === null || variant1=== undefined) {
          variant1_0 = 0;
          variant1_1 = 0;
          variant1_2 = 0;
        } else {
          const e = variant1;
          
          var encodeRes = _utf8AllocateAndEncode(e, ((...args) => {
            const state = getOrCreateAsyncState(0);
            const previousMayLeave = state.mayLeave;
            const meta = _getGlobalCurrentThreadMeta(0);
            const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
            const previousStorage = thread?.storage;
            if (thread) { thread.storage = [0, 0]; }
            state.mayLeave = false;
            const restore = () => {
              state.mayLeave = previousMayLeave;
              if (thread) { thread.storage = previousStorage; }
            };
            let result;
            try {
              result = realloc0(...args);
            } catch (err) {
              restore();
              throw err;
            }
            if (result && typeof result.then === 'function') {
              return Promise.resolve(result).finally(restore);
            }
            restore();
            return result;
          }), memory0);
          var ptr0= encodeRes.ptr;
          var len0 = encodeRes.len;
          
          variant1_0 = 1;
          variant1_1 = ptr0;
          variant1_2 = len0;
        }
        _debugLog('[iface="tairitsu-browser:full/idb-callbacks@0.2.0", function="on-idb-request-success"][Instruction::CallWasm] enter', {
          funcName: 'on-idb-request-success',
          paramCount: 4,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => idbCallbacks020OnIdbRequestSuccess(toUint64(arg0), variant1_0, variant1_1, variant1_2),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/idb-callbacks@0.2.0", function="on-idb-request-success"][Instruction::Return]', {
          funcName: 'on-idb-request-success',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let idbCallbacks020OnIdbRequestError;

function onIdbRequestError(arg0, arg1) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'idbCallbacks020OnIdbRequestError',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        
        var encodeRes = _utf8AllocateAndEncode(arg1, ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        }), memory0);
        var ptr0= encodeRes.ptr;
        var len0 = encodeRes.len;
        
        _debugLog('[iface="tairitsu-browser:full/idb-callbacks@0.2.0", function="on-idb-request-error"][Instruction::CallWasm] enter', {
          funcName: 'on-idb-request-error',
          paramCount: 3,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => idbCallbacks020OnIdbRequestError(toUint64(arg0), ptr0, len0),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/idb-callbacks@0.2.0", function="on-idb-request-error"][Instruction::Return]', {
          funcName: 'on-idb-request-error',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let fileReaderCallbacks020OnFileReaderLoad;

function onFileReaderLoad(arg0, arg1) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'fileReaderCallbacks020OnFileReaderLoad',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        
        var encodeRes = _utf8AllocateAndEncode(arg1, ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        }), memory0);
        var ptr0= encodeRes.ptr;
        var len0 = encodeRes.len;
        
        _debugLog('[iface="tairitsu-browser:full/file-reader-callbacks@0.2.0", function="on-file-reader-load"][Instruction::CallWasm] enter', {
          funcName: 'on-file-reader-load',
          paramCount: 3,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => fileReaderCallbacks020OnFileReaderLoad(toUint64(arg0), ptr0, len0),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/file-reader-callbacks@0.2.0", function="on-file-reader-load"][Instruction::Return]', {
          funcName: 'on-file-reader-load',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let fileReaderCallbacks020OnFileReaderError;

function onFileReaderError(arg0, arg1) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'fileReaderCallbacks020OnFileReaderError',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        
        var encodeRes = _utf8AllocateAndEncode(arg1, ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        }), memory0);
        var ptr0= encodeRes.ptr;
        var len0 = encodeRes.len;
        
        _debugLog('[iface="tairitsu-browser:full/file-reader-callbacks@0.2.0", function="on-file-reader-error"][Instruction::CallWasm] enter', {
          funcName: 'on-file-reader-error',
          paramCount: 3,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => fileReaderCallbacks020OnFileReaderError(toUint64(arg0), ptr0, len0),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/file-reader-callbacks@0.2.0", function="on-file-reader-error"][Instruction::Return]', {
          funcName: 'on-file-reader-error',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let webSocketCallbacks020OnWebSocketOpen;

function onWebSocketOpen(arg0) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'webSocketCallbacks020OnWebSocketOpen',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (null!== null) {
    task.setReturnMemoryIdx(null);
    task.setReturnMemory((() => null)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        _debugLog('[iface="tairitsu-browser:full/web-socket-callbacks@0.2.0", function="on-web-socket-open"][Instruction::CallWasm] enter', {
          funcName: 'on-web-socket-open',
          paramCount: 1,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => webSocketCallbacks020OnWebSocketOpen(toUint64(arg0)),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/web-socket-callbacks@0.2.0", function="on-web-socket-open"][Instruction::Return]', {
          funcName: 'on-web-socket-open',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let webSocketCallbacks020OnWebSocketMessage;

function onWebSocketMessage(arg0, arg1) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'webSocketCallbacks020OnWebSocketMessage',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        
        var encodeRes = _utf8AllocateAndEncode(arg1, ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        }), memory0);
        var ptr0= encodeRes.ptr;
        var len0 = encodeRes.len;
        
        _debugLog('[iface="tairitsu-browser:full/web-socket-callbacks@0.2.0", function="on-web-socket-message"][Instruction::CallWasm] enter', {
          funcName: 'on-web-socket-message',
          paramCount: 3,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => webSocketCallbacks020OnWebSocketMessage(toUint64(arg0), ptr0, len0),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/web-socket-callbacks@0.2.0", function="on-web-socket-message"][Instruction::Return]', {
          funcName: 'on-web-socket-message',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let webSocketCallbacks020OnWebSocketClose;

function onWebSocketClose(arg0, arg1, arg2) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'webSocketCallbacks020OnWebSocketClose',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        
        var encodeRes = _utf8AllocateAndEncode(arg2, ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        }), memory0);
        var ptr0= encodeRes.ptr;
        var len0 = encodeRes.len;
        
        _debugLog('[iface="tairitsu-browser:full/web-socket-callbacks@0.2.0", function="on-web-socket-close"][Instruction::CallWasm] enter', {
          funcName: 'on-web-socket-close',
          paramCount: 4,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => webSocketCallbacks020OnWebSocketClose(toUint64(arg0), toUint16(arg1), ptr0, len0),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/web-socket-callbacks@0.2.0", function="on-web-socket-close"][Instruction::Return]', {
          funcName: 'on-web-socket-close',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let webSocketCallbacks020OnWebSocketError;

function onWebSocketError(arg0) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'webSocketCallbacks020OnWebSocketError',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(5);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (null!== null) {
    task.setReturnMemoryIdx(null);
    task.setReturnMemory((() => null)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        _debugLog('[iface="tairitsu-browser:full/web-socket-callbacks@0.2.0", function="on-web-socket-error"][Instruction::CallWasm] enter', {
          funcName: 'on-web-socket-error',
          paramCount: 1,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => webSocketCallbacks020OnWebSocketError(toUint64(arg0)),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu-browser:full/web-socket-callbacks@0.2.0", function="on-web-socket-error"][Instruction::Return]', {
          funcName: 'on-web-socket-error',
          paramCount: 0,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let versionGetVersion;

function getVersion() {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'versionGetVersion',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(20);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        _debugLog('[iface="tairitsu:style/version", function="get-version"][Instruction::CallWasm] enter', {
          funcName: 'get-version',
          paramCount: 0,
          async: false,
          postReturn: true,
        });
        
        let ret;
        
        try {
          ret =  _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => versionGetVersion(),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        var ptr0 = dataView(memory0).getUint32(((ret >>> 0)) + 0, true) >>> 0;
        var len0 = dataView(memory0).getUint32(((ret >>> 0)) + 4, true);
        var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
        _debugLog('[iface="tairitsu:style/version", function="get-version"][Instruction::Return]', {
          funcName: 'get-version',
          paramCount: 1,
          async: false,
          postReturn: true
        });
        task.resolve([result0]);
        const retCopy = result0;
        
        let cstate = getOrCreateAsyncState(0);
        cstate.mayLeave = false;
        try {
          postReturn1(ret);
        } catch (err) {
          err = _postReturnTrap(0, err);
          // Cleanup can fail after the result was recorded.
          task.reject(err);
          throw err;
        }
        cstate.mayLeave = true;
        task.exit();
        return retCopy;
        
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let builderBuildStyleString;

function buildStyleString(arg0) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'builderBuildStyleString',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(20);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        var vec3 = arg0;
        var len3 = vec3.length;
        var result3 = ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        })(0, 0, 4, len3 * 16);
        if (result3 < 0 || len3 < 0 || len3 > 16777215 || result3 > memory0.buffer.byteLength || len3 > Math.floor((memory0.buffer.byteLength - result3) / 16)) throw new WebAssemblyRuntimeError('wasm trap: out of bounds memory access');
        for (let i = 0; i < vec3.length; i++) {
          const e = vec3[i];
          const base = result3 + i * 16;var [tuple0_0, tuple0_1] = e;
          
          var encodeRes = _utf8AllocateAndEncode(tuple0_0, ((...args) => {
            const state = getOrCreateAsyncState(0);
            const previousMayLeave = state.mayLeave;
            const meta = _getGlobalCurrentThreadMeta(0);
            const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
            const previousStorage = thread?.storage;
            if (thread) { thread.storage = [0, 0]; }
            state.mayLeave = false;
            const restore = () => {
              state.mayLeave = previousMayLeave;
              if (thread) { thread.storage = previousStorage; }
            };
            let result;
            try {
              result = realloc0(...args);
            } catch (err) {
              restore();
              throw err;
            }
            if (result && typeof result.then === 'function') {
              return Promise.resolve(result).finally(restore);
            }
            restore();
            return result;
          }), memory0);
          var ptr1= encodeRes.ptr;
          var len1 = encodeRes.len;
          
          dataView(memory0).setUint32((base) + 4, len1, true);
          dataView(memory0).setUint32((base) + 0, ptr1, true);
          
          var encodeRes = _utf8AllocateAndEncode(tuple0_1, ((...args) => {
            const state = getOrCreateAsyncState(0);
            const previousMayLeave = state.mayLeave;
            const meta = _getGlobalCurrentThreadMeta(0);
            const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
            const previousStorage = thread?.storage;
            if (thread) { thread.storage = [0, 0]; }
            state.mayLeave = false;
            const restore = () => {
              state.mayLeave = previousMayLeave;
              if (thread) { thread.storage = previousStorage; }
            };
            let result;
            try {
              result = realloc0(...args);
            } catch (err) {
              restore();
              throw err;
            }
            if (result && typeof result.then === 'function') {
              return Promise.resolve(result).finally(restore);
            }
            restore();
            return result;
          }), memory0);
          var ptr2= encodeRes.ptr;
          var len2 = encodeRes.len;
          
          dataView(memory0).setUint32((base) + 12, len2, true);
          dataView(memory0).setUint32((base) + 8, ptr2, true);
        }
        _debugLog('[iface="tairitsu:style/builder", function="build-style-string"][Instruction::CallWasm] enter', {
          funcName: 'build-style-string',
          paramCount: 2,
          async: false,
          postReturn: true,
        });
        
        let ret;
        
        try {
          ret =  _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => builderBuildStyleString(result3, len3),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        var ptr4 = dataView(memory0).getUint32(((ret >>> 0)) + 0, true) >>> 0;
        var len4 = dataView(memory0).getUint32(((ret >>> 0)) + 4, true);
        var result4 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr4, len4));
        _debugLog('[iface="tairitsu:style/builder", function="build-style-string"][Instruction::Return]', {
          funcName: 'build-style-string',
          paramCount: 1,
          async: false,
          postReturn: true
        });
        task.resolve([result4]);
        const retCopy = result4;
        
        let cstate = getOrCreateAsyncState(0);
        cstate.mayLeave = false;
        try {
          postReturn1(ret);
        } catch (err) {
          err = _postReturnTrap(0, err);
          // Cleanup can fail after the result was recorded.
          task.reject(err);
          throw err;
        }
        cstate.mayLeave = true;
        task.exit();
        return retCopy;
        
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let builderCssPropertyToString;

function cssPropertyToString(arg0) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'builderCssPropertyToString',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(20);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        
        var encodeRes = _utf8AllocateAndEncode(arg0, ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        }), memory0);
        var ptr0= encodeRes.ptr;
        var len0 = encodeRes.len;
        
        _debugLog('[iface="tairitsu:style/builder", function="css-property-to-string"][Instruction::CallWasm] enter', {
          funcName: 'css-property-to-string',
          paramCount: 2,
          async: false,
          postReturn: true,
        });
        
        let ret;
        
        try {
          ret =  _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => builderCssPropertyToString(ptr0, len0),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        var ptr1 = dataView(memory0).getUint32(((ret >>> 0)) + 0, true) >>> 0;
        var len1 = dataView(memory0).getUint32(((ret >>> 0)) + 4, true);
        var result1 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr1, len1));
        _debugLog('[iface="tairitsu:style/builder", function="css-property-to-string"][Instruction::Return]', {
          funcName: 'css-property-to-string',
          paramCount: 1,
          async: false,
          postReturn: true
        });
        task.resolve([result1]);
        const retCopy = result1;
        
        let cstate = getOrCreateAsyncState(0);
        cstate.mayLeave = false;
        try {
          postReturn1(ret);
        } catch (err) {
          err = _postReturnTrap(0, err);
          // Cleanup can fail after the result was recorded.
          task.reject(err);
          throw err;
        }
        cstate.mayLeave = true;
        task.exit();
        return retCopy;
        
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let builderCssLengthToString;

function cssLengthToString(arg0, arg1) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'builderCssLengthToString',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(20);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        
        var encodeRes = _utf8AllocateAndEncode(arg1, ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        }), memory0);
        var ptr0= encodeRes.ptr;
        var len0 = encodeRes.len;
        
        _debugLog('[iface="tairitsu:style/builder", function="css-length-to-string"][Instruction::CallWasm] enter', {
          funcName: 'css-length-to-string',
          paramCount: 3,
          async: false,
          postReturn: true,
        });
        
        let ret;
        
        try {
          ret =  _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => builderCssLengthToString(+arg0, ptr0, len0),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        var ptr1 = dataView(memory0).getUint32(((ret >>> 0)) + 0, true) >>> 0;
        var len1 = dataView(memory0).getUint32(((ret >>> 0)) + 4, true);
        var result1 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr1, len1));
        _debugLog('[iface="tairitsu:style/builder", function="css-length-to-string"][Instruction::Return]', {
          funcName: 'css-length-to-string',
          paramCount: 1,
          async: false,
          postReturn: true
        });
        task.resolve([result1]);
        const retCopy = result1;
        
        let cstate = getOrCreateAsyncState(0);
        cstate.mayLeave = false;
        try {
          postReturn1(ret);
        } catch (err) {
          err = _postReturnTrap(0, err);
          // Cleanup can fail after the result was recorded.
          task.reject(err);
          throw err;
        }
        cstate.mayLeave = true;
        task.exit();
        return retCopy;
        
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let utilityGenerateUtilityCss;

function generateUtilityCss(arg0) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'utilityGenerateUtilityCss',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(15);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        
        var encodeRes = _utf8AllocateAndEncode(arg0, ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        }), memory0);
        var ptr0= encodeRes.ptr;
        var len0 = encodeRes.len;
        
        _debugLog('[iface="tairitsu:style/utility", function="generate-utility-css"][Instruction::CallWasm] enter', {
          funcName: 'generate-utility-css',
          paramCount: 2,
          async: false,
          postReturn: true,
        });
        
        let ret;
        
        try {
          ret =  _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => utilityGenerateUtilityCss(ptr0, len0),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        let variant2;
        switch (dataView(memory0).getUint8(((ret >>> 0)) + 0, true)) {
          case 0: {
            variant2 = undefined;
            break;
          }
          case 1: {
            var ptr1 = dataView(memory0).getUint32(((ret >>> 0)) + 4, true) >>> 0;
            var len1 = dataView(memory0).getUint32(((ret >>> 0)) + 8, true);
            var result1 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr1, len1));
            variant2 = result1;
            break;
          }
          default: {
            throw new TypeError('invalid variant discriminant for option');
          }
        }
        _debugLog('[iface="tairitsu:style/utility", function="generate-utility-css"][Instruction::Return]', {
          funcName: 'generate-utility-css',
          paramCount: 1,
          async: false,
          postReturn: true
        });
        task.resolve([variant2]);
        const retCopy = variant2;
        
        let cstate = getOrCreateAsyncState(0);
        cstate.mayLeave = false;
        try {
          postReturn2(ret);
        } catch (err) {
          err = _postReturnTrap(0, err);
          // Cleanup can fail after the result was recorded.
          task.reject(err);
          throw err;
        }
        cstate.mayLeave = true;
        task.exit();
        return retCopy;
        
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let utilityGenerateStylesheet;

function generateStylesheet(arg0) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'utilityGenerateStylesheet',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(20);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        var vec1 = arg0;
        var len1 = vec1.length;
        var result1 = ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        })(0, 0, 4, len1 * 8);
        if (result1 < 0 || len1 < 0 || len1 > 33554431 || result1 > memory0.buffer.byteLength || len1 > Math.floor((memory0.buffer.byteLength - result1) / 8)) throw new WebAssemblyRuntimeError('wasm trap: out of bounds memory access');
        for (let i = 0; i < vec1.length; i++) {
          const e = vec1[i];
          const base = result1 + i * 8;
          var encodeRes = _utf8AllocateAndEncode(e, ((...args) => {
            const state = getOrCreateAsyncState(0);
            const previousMayLeave = state.mayLeave;
            const meta = _getGlobalCurrentThreadMeta(0);
            const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
            const previousStorage = thread?.storage;
            if (thread) { thread.storage = [0, 0]; }
            state.mayLeave = false;
            const restore = () => {
              state.mayLeave = previousMayLeave;
              if (thread) { thread.storage = previousStorage; }
            };
            let result;
            try {
              result = realloc0(...args);
            } catch (err) {
              restore();
              throw err;
            }
            if (result && typeof result.then === 'function') {
              return Promise.resolve(result).finally(restore);
            }
            restore();
            return result;
          }), memory0);
          var ptr0= encodeRes.ptr;
          var len0 = encodeRes.len;
          
          dataView(memory0).setUint32((base) + 4, len0, true);
          dataView(memory0).setUint32((base) + 0, ptr0, true);
        }
        _debugLog('[iface="tairitsu:style/utility", function="generate-stylesheet"][Instruction::CallWasm] enter', {
          funcName: 'generate-stylesheet',
          paramCount: 2,
          async: false,
          postReturn: true,
        });
        
        let ret;
        
        try {
          ret =  _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => utilityGenerateStylesheet(result1, len1),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        var ptr2 = dataView(memory0).getUint32(((ret >>> 0)) + 0, true) >>> 0;
        var len2 = dataView(memory0).getUint32(((ret >>> 0)) + 4, true);
        var result2 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr2, len2));
        _debugLog('[iface="tairitsu:style/utility", function="generate-stylesheet"][Instruction::Return]', {
          funcName: 'generate-stylesheet',
          paramCount: 1,
          async: false,
          postReturn: true
        });
        task.resolve([result2]);
        const retCopy = result2;
        
        let cstate = getOrCreateAsyncState(0);
        cstate.mayLeave = false;
        try {
          postReturn1(ret);
        } catch (err) {
          err = _postReturnTrap(0, err);
          // Cleanup can fail after the result was recorded.
          task.reject(err);
          throw err;
        }
        cstate.mayLeave = true;
        task.exit();
        return retCopy;
        
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let utilityParseUtilityBase;

function parseUtilityBase(arg0) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'utilityParseUtilityBase',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(20);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        
        var encodeRes = _utf8AllocateAndEncode(arg0, ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        }), memory0);
        var ptr0= encodeRes.ptr;
        var len0 = encodeRes.len;
        
        _debugLog('[iface="tairitsu:style/utility", function="parse-utility-base"][Instruction::CallWasm] enter', {
          funcName: 'parse-utility-base',
          paramCount: 2,
          async: false,
          postReturn: true,
        });
        
        let ret;
        
        try {
          ret =  _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => utilityParseUtilityBase(ptr0, len0),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        var ptr1 = dataView(memory0).getUint32(((ret >>> 0)) + 0, true) >>> 0;
        var len1 = dataView(memory0).getUint32(((ret >>> 0)) + 4, true);
        var result1 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr1, len1));
        _debugLog('[iface="tairitsu:style/utility", function="parse-utility-base"][Instruction::Return]', {
          funcName: 'parse-utility-base',
          paramCount: 1,
          async: false,
          postReturn: true
        });
        task.resolve([result1]);
        const retCopy = result1;
        
        let cstate = getOrCreateAsyncState(0);
        cstate.mayLeave = false;
        try {
          postReturn1(ret);
        } catch (err) {
          err = _postReturnTrap(0, err);
          // Cleanup can fail after the result was recorded.
          task.reject(err);
          throw err;
        }
        cstate.mayLeave = true;
        task.exit();
        return retCopy;
        
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let classesBuildClasses;

function buildClasses(arg0) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'classesBuildClasses',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(20);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        var vec1 = arg0;
        var len1 = vec1.length;
        var result1 = ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        })(0, 0, 4, len1 * 8);
        if (result1 < 0 || len1 < 0 || len1 > 33554431 || result1 > memory0.buffer.byteLength || len1 > Math.floor((memory0.buffer.byteLength - result1) / 8)) throw new WebAssemblyRuntimeError('wasm trap: out of bounds memory access');
        for (let i = 0; i < vec1.length; i++) {
          const e = vec1[i];
          const base = result1 + i * 8;
          var encodeRes = _utf8AllocateAndEncode(e, ((...args) => {
            const state = getOrCreateAsyncState(0);
            const previousMayLeave = state.mayLeave;
            const meta = _getGlobalCurrentThreadMeta(0);
            const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
            const previousStorage = thread?.storage;
            if (thread) { thread.storage = [0, 0]; }
            state.mayLeave = false;
            const restore = () => {
              state.mayLeave = previousMayLeave;
              if (thread) { thread.storage = previousStorage; }
            };
            let result;
            try {
              result = realloc0(...args);
            } catch (err) {
              restore();
              throw err;
            }
            if (result && typeof result.then === 'function') {
              return Promise.resolve(result).finally(restore);
            }
            restore();
            return result;
          }), memory0);
          var ptr0= encodeRes.ptr;
          var len0 = encodeRes.len;
          
          dataView(memory0).setUint32((base) + 4, len0, true);
          dataView(memory0).setUint32((base) + 0, ptr0, true);
        }
        _debugLog('[iface="tairitsu:style/classes", function="build-classes"][Instruction::CallWasm] enter', {
          funcName: 'build-classes',
          paramCount: 2,
          async: false,
          postReturn: true,
        });
        
        let ret;
        
        try {
          ret =  _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => classesBuildClasses(result1, len1),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        var ptr2 = dataView(memory0).getUint32(((ret >>> 0)) + 0, true) >>> 0;
        var len2 = dataView(memory0).getUint32(((ret >>> 0)) + 4, true);
        var result2 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr2, len2));
        _debugLog('[iface="tairitsu:style/classes", function="build-classes"][Instruction::Return]', {
          funcName: 'build-classes',
          paramCount: 1,
          async: false,
          postReturn: true
        });
        task.resolve([result2]);
        const retCopy = result2;
        
        let cstate = getOrCreateAsyncState(0);
        cstate.mayLeave = false;
        try {
          postReturn1(ret);
        } catch (err) {
          err = _postReturnTrap(0, err);
          // Cleanup can fail after the result was recorded.
          task.reject(err);
          throw err;
        }
        cstate.mayLeave = true;
        task.exit();
        return retCopy;
        
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let classesBuildClassesConditional;

function buildClassesConditional(arg0) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'classesBuildClassesConditional',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(20);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        var vec2 = arg0;
        var len2 = vec2.length;
        var result2 = ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        })(0, 0, 4, len2 * 12);
        if (result2 < 0 || len2 < 0 || len2 > 22369621 || result2 > memory0.buffer.byteLength || len2 > Math.floor((memory0.buffer.byteLength - result2) / 12)) throw new WebAssemblyRuntimeError('wasm trap: out of bounds memory access');
        for (let i = 0; i < vec2.length; i++) {
          const e = vec2[i];
          const base = result2 + i * 12;var [tuple0_0, tuple0_1] = e;
          
          var encodeRes = _utf8AllocateAndEncode(tuple0_0, ((...args) => {
            const state = getOrCreateAsyncState(0);
            const previousMayLeave = state.mayLeave;
            const meta = _getGlobalCurrentThreadMeta(0);
            const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
            const previousStorage = thread?.storage;
            if (thread) { thread.storage = [0, 0]; }
            state.mayLeave = false;
            const restore = () => {
              state.mayLeave = previousMayLeave;
              if (thread) { thread.storage = previousStorage; }
            };
            let result;
            try {
              result = realloc0(...args);
            } catch (err) {
              restore();
              throw err;
            }
            if (result && typeof result.then === 'function') {
              return Promise.resolve(result).finally(restore);
            }
            restore();
            return result;
          }), memory0);
          var ptr1= encodeRes.ptr;
          var len1 = encodeRes.len;
          
          dataView(memory0).setUint32((base) + 4, len1, true);
          dataView(memory0).setUint32((base) + 0, ptr1, true);
          dataView(memory0).setInt8((base) + 8, tuple0_1 ? 1 : 0, true);
        }
        _debugLog('[iface="tairitsu:style/classes", function="build-classes-conditional"][Instruction::CallWasm] enter', {
          funcName: 'build-classes-conditional',
          paramCount: 2,
          async: false,
          postReturn: true,
        });
        
        let ret;
        
        try {
          ret =  _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => classesBuildClassesConditional(result2, len2),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        var ptr3 = dataView(memory0).getUint32(((ret >>> 0)) + 0, true) >>> 0;
        var len3 = dataView(memory0).getUint32(((ret >>> 0)) + 4, true);
        var result3 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr3, len3));
        _debugLog('[iface="tairitsu:style/classes", function="build-classes-conditional"][Instruction::Return]', {
          funcName: 'build-classes-conditional',
          paramCount: 1,
          async: false,
          postReturn: true
        });
        task.resolve([result3]);
        const retCopy = result3;
        
        let cstate = getOrCreateAsyncState(0);
        cstate.mayLeave = false;
        try {
          postReturn1(ret);
        } catch (err) {
          err = _postReturnTrap(0, err);
          // Cleanup can fail after the result was recorded.
          task.reject(err);
          throw err;
        }
        cstate.mayLeave = true;
        task.exit();
        return retCopy;
        
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let versionGetVersion$1;

function getVersion$1() {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'versionGetVersion$1',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(20);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        _debugLog('[iface="tairitsu:hooks/version", function="get-version"][Instruction::CallWasm] enter', {
          funcName: 'get-version',
          paramCount: 0,
          async: false,
          postReturn: true,
        });
        
        let ret;
        
        try {
          ret =  _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => versionGetVersion$1(),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        var ptr0 = dataView(memory0).getUint32(((ret >>> 0)) + 0, true) >>> 0;
        var len0 = dataView(memory0).getUint32(((ret >>> 0)) + 4, true);
        var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
        _debugLog('[iface="tairitsu:hooks/version", function="get-version"][Instruction::Return]', {
          funcName: 'get-version',
          paramCount: 1,
          async: false,
          postReturn: true
        });
        task.resolve([result0]);
        const retCopy = result0;
        
        let cstate = getOrCreateAsyncState(0);
        cstate.mayLeave = false;
        try {
          postReturn3(ret);
        } catch (err) {
          err = _postReturnTrap(0, err);
          // Cleanup can fail after the result was recorded.
          task.reject(err);
          throw err;
        }
        cstate.mayLeave = true;
        task.exit();
        return retCopy;
        
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let stateMachineTransition;

function transition(arg0, arg1) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'stateMachineTransition',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(61);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        var val0 = arg0;
        let enum0;
        switch (val0) {
          case "idle": {
            enum0 = 0;
            break;
          }
          case "hover": {
            enum0 = 1;
            break;
          }
          case "active": {
            enum0 = 2;
            break;
          }
          case "focused": {
            enum0 = 3;
            break;
          }
          case "disabled": {
            enum0 = 4;
            break;
          }
          default: {
            if ((arg0) instanceof Error) {
              console.error(arg0);
            }
            
            throw new TypeError(`"${val0}" is not one of the cases of interaction-state`);
          }
        }
        var val1 = arg1;
        let enum1;
        switch (val1) {
          case "mouse-enter": {
            enum1 = 0;
            break;
          }
          case "mouse-leave": {
            enum1 = 1;
            break;
          }
          case "mouse-down": {
            enum1 = 2;
            break;
          }
          case "mouse-up": {
            enum1 = 3;
            break;
          }
          case "focus": {
            enum1 = 4;
            break;
          }
          case "blur": {
            enum1 = 5;
            break;
          }
          case "disable": {
            enum1 = 6;
            break;
          }
          case "enable": {
            enum1 = 7;
            break;
          }
          default: {
            if ((arg1) instanceof Error) {
              console.error(arg1);
            }
            
            throw new TypeError(`"${val1}" is not one of the cases of interaction-event`);
          }
        }
        _debugLog('[iface="tairitsu:hooks/state-machine", function="transition"][Instruction::CallWasm] enter', {
          funcName: 'transition',
          paramCount: 2,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          ret =  _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => stateMachineTransition(enum0, enum1),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        let variant3;
        switch (dataView(memory0).getUint8(((ret >>> 0)) + 0, true)) {
          case 0: {
            variant3 = undefined;
            break;
          }
          case 1: {
            let enum2;
            switch (dataView(memory0).getUint8(((ret >>> 0)) + 1, true)) {
              case 0: {
                enum2 = 'idle';
                break;
              }
              case 1: {
                enum2 = 'hover';
                break;
              }
              case 2: {
                enum2 = 'active';
                break;
              }
              case 3: {
                enum2 = 'focused';
                break;
              }
              case 4: {
                enum2 = 'disabled';
                break;
              }
              default: {
                throw new TypeError('invalid discriminant specified for InteractionState');
              }
            }
            variant3 = enum2;
            break;
          }
          default: {
            throw new TypeError('invalid variant discriminant for option');
          }
        }
        _debugLog('[iface="tairitsu:hooks/state-machine", function="transition"][Instruction::Return]', {
          funcName: 'transition',
          paramCount: 1,
          async: false,
          postReturn: false
        });
        task.resolve([variant3]);
        task.exit();
        return variant3;
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let stateMachineIsInteractive;

function isInteractive(arg0) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'stateMachineIsInteractive',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(17);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (null!== null) {
    task.setReturnMemoryIdx(null);
    task.setReturnMemory((() => null)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        var val0 = arg0;
        let enum0;
        switch (val0) {
          case "idle": {
            enum0 = 0;
            break;
          }
          case "hover": {
            enum0 = 1;
            break;
          }
          case "active": {
            enum0 = 2;
            break;
          }
          case "focused": {
            enum0 = 3;
            break;
          }
          case "disabled": {
            enum0 = 4;
            break;
          }
          default: {
            if ((arg0) instanceof Error) {
              console.error(arg0);
            }
            
            throw new TypeError(`"${val0}" is not one of the cases of interaction-state`);
          }
        }
        _debugLog('[iface="tairitsu:hooks/state-machine", function="is-interactive"][Instruction::CallWasm] enter', {
          funcName: 'is-interactive',
          paramCount: 1,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          ret =  _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => stateMachineIsInteractive(enum0),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        var bool1 = ret;
        _debugLog('[iface="tairitsu:hooks/state-machine", function="is-interactive"][Instruction::Return]', {
          funcName: 'is-interactive',
          paramCount: 1,
          async: false,
          postReturn: false
        });
        task.resolve([bool1 == 0 ? false : (bool1 == 1 ? true : throwInvalidBool())]);
        task.exit();
        return bool1 == 0 ? false : (bool1 == 1 ? true : throwInvalidBool());
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let animationTypesEvaluateEasing;

function evaluateEasing(arg0, arg1) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'animationTypesEvaluateEasing',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(26);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (null!== null) {
    task.setReturnMemoryIdx(null);
    task.setReturnMemory((() => null)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        var variant1 = arg0;
        let variant1_0;
        let variant1_1;
        let variant1_2;
        let variant1_3;
        let variant1_4;
        switch (variant1.tag) {
          case 'linear': {
            variant1_0 = 0;
            variant1_1 = 0;
            variant1_2 = 0;
            variant1_3 = 0;
            variant1_4 = 0;
            break;
          }
          case 'ease': {
            variant1_0 = 1;
            variant1_1 = 0;
            variant1_2 = 0;
            variant1_3 = 0;
            variant1_4 = 0;
            break;
          }
          case 'ease-in': {
            variant1_0 = 2;
            variant1_1 = 0;
            variant1_2 = 0;
            variant1_3 = 0;
            variant1_4 = 0;
            break;
          }
          case 'ease-out': {
            variant1_0 = 3;
            variant1_1 = 0;
            variant1_2 = 0;
            variant1_3 = 0;
            variant1_4 = 0;
            break;
          }
          case 'ease-in-out': {
            variant1_0 = 4;
            variant1_1 = 0;
            variant1_2 = 0;
            variant1_3 = 0;
            variant1_4 = 0;
            break;
          }
          case 'cubic-bezier': {
            const e = variant1.val;
            var [tuple0_0, tuple0_1, tuple0_2, tuple0_3] = e;
            variant1_0 = 5;
            variant1_1 = +tuple0_0;
            variant1_2 = +tuple0_1;
            variant1_3 = +tuple0_2;
            variant1_4 = +tuple0_3;
            break;
          }
          default: {
            throw new TypeError(`invalid variant tag value \`${JSON.stringify(variant1.tag)}\` (received \`${variant1}\`) specified for \`EasingFunction\``);
          }
        }
        _debugLog('[iface="tairitsu:hooks/animation-types", function="evaluate-easing"][Instruction::CallWasm] enter', {
          funcName: 'evaluate-easing',
          paramCount: 6,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          ret =  _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => animationTypesEvaluateEasing(variant1_0, variant1_1, variant1_2, variant1_3, variant1_4, +arg1),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu:hooks/animation-types", function="evaluate-easing"][Instruction::Return]', {
          funcName: 'evaluate-easing',
          paramCount: 1,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        return ret;
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let animationTypesLerpF32;

function lerpF32(arg0, arg1, arg2) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'animationTypesLerpF32',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(26);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (null!== null) {
    task.setReturnMemoryIdx(null);
    task.setReturnMemory((() => null)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        _debugLog('[iface="tairitsu:hooks/animation-types", function="lerp-f32"][Instruction::CallWasm] enter', {
          funcName: 'lerp-f32',
          paramCount: 3,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          ret =  _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => animationTypesLerpF32(+arg0, +arg1, +arg2),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        _debugLog('[iface="tairitsu:hooks/animation-types", function="lerp-f32"][Instruction::Return]', {
          funcName: 'lerp-f32',
          paramCount: 1,
          async: false,
          postReturn: false
        });
        task.resolve([ret]);
        task.exit();
        return ret;
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let animationBuilderDefaultAnimationConfig;

function defaultAnimationConfig() {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'animationBuilderDefaultAnimationConfig',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(66);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        _debugLog('[iface="tairitsu:hooks/animation-builder", function="default-animation-config"][Instruction::CallWasm] enter', {
          funcName: 'default-animation-config',
          paramCount: 0,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          ret =  _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => animationBuilderDefaultAnimationConfig(),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        let enum0;
        switch (dataView(memory0).getUint8(((ret >>> 0)) + 20, true)) {
          case 0: {
            enum0 = 'normal';
            break;
          }
          case 1: {
            enum0 = 'reverse';
            break;
          }
          case 2: {
            enum0 = 'alternate';
            break;
          }
          case 3: {
            enum0 = 'alternate-reverse';
            break;
          }
          default: {
            throw new TypeError('invalid discriminant specified for AnimationDirection');
          }
        }
        let variant1;
        switch (dataView(memory0).getUint8(((ret >>> 0)) + 24, true)) {
          case 0: {
            variant1= {
              tag: 'linear',
            };
            break;
          }
          case 1: {
            variant1= {
              tag: 'ease',
            };
            break;
          }
          case 2: {
            variant1= {
              tag: 'ease-in',
            };
            break;
          }
          case 3: {
            variant1= {
              tag: 'ease-out',
            };
            break;
          }
          case 4: {
            variant1= {
              tag: 'ease-in-out',
            };
            break;
          }
          case 5: {
            variant1= {
              tag: 'cubic-bezier',
              val: [dataView(memory0).getFloat32(((ret >>> 0)) + 28, true), dataView(memory0).getFloat32(((ret >>> 0)) + 32, true), dataView(memory0).getFloat32(((ret >>> 0)) + 36, true), dataView(memory0).getFloat32(((ret >>> 0)) + 40, true)]
            };
            break;
          }
          default: {
            throw new TypeError('invalid variant discriminant for EasingFunction');
          }
        }
        _debugLog('[iface="tairitsu:hooks/animation-builder", function="default-animation-config"][Instruction::Return]', {
          funcName: 'default-animation-config',
          paramCount: 1,
          async: false,
          postReturn: false
        });
        task.resolve([{
          durationMs: BigInt.asUintN(64, BigInt(dataView(memory0).getBigInt64(((ret >>> 0)) + 0, true))),
          delayMs: BigInt.asUintN(64, BigInt(dataView(memory0).getBigInt64(((ret >>> 0)) + 8, true))),
          iterations: dataView(memory0).getInt32(((ret >>> 0)) + 16, true) >>> 0,
          direction: enum0,
          easing: variant1,
        }]);
        task.exit();
        return {
          durationMs: BigInt.asUintN(64, BigInt(dataView(memory0).getBigInt64(((ret >>> 0)) + 0, true))),
          delayMs: BigInt.asUintN(64, BigInt(dataView(memory0).getBigInt64(((ret >>> 0)) + 8, true))),
          iterations: dataView(memory0).getInt32(((ret >>> 0)) + 16, true) >>> 0,
          direction: enum0,
          easing: variant1,
        };
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let asyncResourceResourceStatusQuery;

function resourceStatusQuery(arg0) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'asyncResourceResourceStatusQuery',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(67);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        _debugLog('[iface="tairitsu:hooks/async-resource", function="resource-status-query"][Instruction::CallWasm] enter', {
          funcName: 'resource-status-query',
          paramCount: 1,
          async: false,
          postReturn: false,
        });
        
        let ret;
        
        try {
          ret =  _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => asyncResourceResourceStatusQuery(toUint64(arg0)),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        let variant1;
        switch (dataView(memory0).getUint8(((ret >>> 0)) + 0, true)) {
          case 0: {
            variant1 = undefined;
            break;
          }
          case 1: {
            let enum0;
            switch (dataView(memory0).getUint8(((ret >>> 0)) + 1, true)) {
              case 0: {
                enum0 = 'loading';
                break;
              }
              case 1: {
                enum0 = 'ready';
                break;
              }
              case 2: {
                enum0 = 'error';
                break;
              }
              default: {
                throw new TypeError('invalid discriminant specified for ResourceStatus');
              }
            }
            variant1 = enum0;
            break;
          }
          default: {
            throw new TypeError('invalid variant discriminant for option');
          }
        }
        _debugLog('[iface="tairitsu:hooks/async-resource", function="resource-status-query"][Instruction::Return]', {
          funcName: 'resource-status-query',
          paramCount: 1,
          async: false,
          postReturn: false
        });
        task.resolve([variant1]);
        task.exit();
        return variant1;
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let versionGetVersion$2;

function getVersion$2() {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'versionGetVersion$2',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(20);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        _debugLog('[iface="tairitsu:vdom/version", function="get-version"][Instruction::CallWasm] enter', {
          funcName: 'get-version',
          paramCount: 0,
          async: false,
          postReturn: true,
        });
        
        let ret;
        
        try {
          ret =  _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => versionGetVersion$2(),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        var ptr0 = dataView(memory0).getUint32(((ret >>> 0)) + 0, true) >>> 0;
        var len0 = dataView(memory0).getUint32(((ret >>> 0)) + 4, true);
        var result0 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr0, len0));
        _debugLog('[iface="tairitsu:vdom/version", function="get-version"][Instruction::Return]', {
          funcName: 'get-version',
          paramCount: 1,
          async: false,
          postReturn: true
        });
        task.resolve([result0]);
        const retCopy = result0;
        
        let cstate = getOrCreateAsyncState(0);
        cstate.mayLeave = false;
        try {
          postReturn4(ret);
        } catch (err) {
          err = _postReturnTrap(0, err);
          // Cleanup can fail after the result was recorded.
          task.reject(err);
          throw err;
        }
        cstate.mayLeave = true;
        task.exit();
        return retCopy;
        
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let svgSanitizeSvg;

function sanitizeSvg(arg0) {
  
  const hostProvided = false;
  getOrCreateAsyncState(0).throwIfTrapped();
  
  const { task, threadID } = createNewCurrentImplicitThread({
    componentIdx: 0,
    isAsync: false,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'svgSanitizeSvg',
    getCallbackFn: () => null,
    callbackFnName: null,
    errHandling: 'none',
    callingWasmExport: true,
  });
  task.setCalleeIsAsync(false);
  task.setFuncTypeIsAsync(false);
  task.setReturnResultTypeIdx(20);
  
  const started = task.enterSync();
  CURRENT_TASK_MAY_BLOCK.value = task.mayBlock() ? 1 : 0;
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory((() => memory0)());
  }
  
  
  return _withGlobalCurrentThreadMeta({
    taskID: task.id(),
    threadID,
    componentIdx: task.componentIdx(),
    fn: () => {
      try {
        
        
        var encodeRes = _utf8AllocateAndEncode(arg0, ((...args) => {
          const state = getOrCreateAsyncState(0);
          const previousMayLeave = state.mayLeave;
          const meta = _getGlobalCurrentThreadMeta(0);
          const thread = meta && getCurrentThread(meta.componentIdx, meta.threadID, meta.taskID)?.thread;
          const previousStorage = thread?.storage;
          if (thread) { thread.storage = [0, 0]; }
          state.mayLeave = false;
          const restore = () => {
            state.mayLeave = previousMayLeave;
            if (thread) { thread.storage = previousStorage; }
          };
          let result;
          try {
            result = realloc0(...args);
          } catch (err) {
            restore();
            throw err;
          }
          if (result && typeof result.then === 'function') {
            return Promise.resolve(result).finally(restore);
          }
          restore();
          return result;
        }), memory0);
        var ptr0= encodeRes.ptr;
        var len0 = encodeRes.len;
        
        _debugLog('[iface="tairitsu:vdom/svg", function="sanitize-svg"][Instruction::CallWasm] enter', {
          funcName: 'sanitize-svg',
          paramCount: 2,
          async: false,
          postReturn: true,
        });
        
        let ret;
        
        try {
          ret =  _withGlobalCurrentThreadMeta({
            taskID: task.id(),
            threadID,
            componentIdx: task.componentIdx(),
            fn: () => svgSanitizeSvg(ptr0, len0),
          });
        } catch (err) {
          
          _debugLog('[Instruction::CallWasm] error during sync call', {
            taskID: task.id(),
            err,
          });
          getOrCreateAsyncState(0).markTrapped(err);
          task.setErrored(err);
          task.reject(err);
          task.exit();
          throw err;
          
        }
        task.endFirstSlice();
        
        var ptr1 = dataView(memory0).getUint32(((ret >>> 0)) + 0, true) >>> 0;
        var len1 = dataView(memory0).getUint32(((ret >>> 0)) + 4, true);
        var result1 = TEXT_DECODER_UTF8.decode(new Uint8Array(memory0.buffer, ptr1, len1));
        _debugLog('[iface="tairitsu:vdom/svg", function="sanitize-svg"][Instruction::Return]', {
          funcName: 'sanitize-svg',
          paramCount: 1,
          async: false,
          postReturn: true
        });
        task.resolve([result1]);
        const retCopy = result1;
        
        let cstate = getOrCreateAsyncState(0);
        cstate.mayLeave = false;
        try {
          postReturn4(ret);
        } catch (err) {
          err = _postReturnTrap(0, err);
          // Cleanup can fail after the result was recorded.
          task.reject(err);
          throw err;
        }
        cstate.mayLeave = true;
        task.exit();
        return retCopy;
        
        
      } catch (err) {
        if (!task.isResolvedState()) {
          task.setErrored(err);
          task.reject(err);
        }
        if (!task.isExited()) { task.exit({ skipExclusiveLockCheck: true }); }
        throw err;
      }
    },
  });
  
}
let trampoline0 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 0,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline0.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatS32],
  resultLowerFns: [_lowerFlatS32],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline0,
},
);
let trampoline1 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 1,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline1.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatU64],
  resultLowerFns: [_lowerFlatU64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline1,
},
);
let trampoline2 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 2,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline2.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline2,
},
);
let trampoline3 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 3,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline3.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline3,
},
);
let trampoline4 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 4,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline4.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatU64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline4,
},
);
let trampoline5 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 5,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline5.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatU32],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline5,
},
);
let trampoline6 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 6,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline6.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatS32],
  resultLowerFns: [_lowerFlatS32],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline6,
},
);
let trampoline7 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 7,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline7.manuallyAsync,
  paramLiftFns: [_liftFlatS32],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline7,
},
);
let trampoline8 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 8,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline8.manuallyAsync,
  paramLiftFns: [_liftFlatS32],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline8,
},
);
let trampoline9 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 9,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline9.manuallyAsync,
  paramLiftFns: [_liftFlatU32],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline9,
},
);
let trampoline10 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 10,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline10.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatU64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline10,
},
);
let trampoline11 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 11,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline11.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatFloat64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline11,
},
);
let trampoline12 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 12,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline12.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatFloat64],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline12,
},
);
let trampoline13 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 13,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline13.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatS32],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline13,
},
);
let trampoline14 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 14,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline14.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatS32],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline14,
},
);
let trampoline15 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 15,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline15.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatS32],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline15,
},
);
let trampoline16 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 16,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline16.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatU64],
  resultLowerFns: [_lowerFlatU64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline16,
},
);
let trampoline17 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 17,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline17.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatU64,
  _liftFlatOption({
    caseMetas: [
    ['none', null, 0, 0, 0, [] ],
    ['some', _liftFlatU64, 8, 8, 1, ['i64'] ],
    ],
    variantSize32: 16,
    variantAlign32: 8,
    variantPayloadOffset32: 8,
    variantFlatCount: 2,
    variantPayloadFlatTypes: ['i64'],
    payloadMaybeNull: false,
  })
  ],
  resultLowerFns: [_lowerFlatU64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline17,
},
);
let trampoline18 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 18,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline18.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatU64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline18,
},
);
let trampoline19 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 19,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline19.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatU32],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline19,
},
);
let trampoline20 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 20,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline20.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatFloat64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline20,
},
);
let trampoline21 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 21,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline21.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatFloat64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline21,
},
);
let trampoline22 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 22,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline22.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatFloat64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline22,
},
);
let trampoline23 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 23,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline23.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatFloat64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline23,
},
);
let trampoline24 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 24,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline24.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatFloat64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline24,
},
);
let trampoline25 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 25,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline25.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatFloat64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline25,
},
);
let trampoline26 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 26,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline26.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatFloat64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline26,
},
);
let trampoline27 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 27,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline27.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatFloat64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline27,
},
);
let trampoline28 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 28,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline28.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatS16],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline28,
},
);
let trampoline29 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 29,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline29.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatU16],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline29,
},
);
let trampoline30 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 30,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline30.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatBool],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline30,
},
);
let trampoline31 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 31,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline31.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatBool],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline31,
},
);
let trampoline32 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 32,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline32.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatBool],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline32,
},
);
let trampoline33 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 33,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline33.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatBool],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline33,
},
);
let trampoline34 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 34,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline34.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatS32],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline34,
},
);
let trampoline35 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 35,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline35.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatBool],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline35,
},
);
let trampoline36 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 36,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline36.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatFloat64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline36,
},
);
let trampoline37 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 37,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline37.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatFloat64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline37,
},
);
let trampoline38 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 38,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline38.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatFloat32],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline38,
},
);
let trampoline39 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 39,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline39.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatFloat32],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline39,
},
);
let trampoline40 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 40,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline40.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatS32],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline40,
},
);
let trampoline41 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 41,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline41.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatS32],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline41,
},
);
let trampoline42 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 42,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline42.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatS32],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline42,
},
);
let trampoline43 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 43,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline43.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatFloat64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline43,
},
);
let trampoline44 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 44,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline44.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatFloat64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline44,
},
);
let trampoline45 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 45,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline45.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatFloat64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline45,
},
);
let trampoline46 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 46,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline46.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatU32],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline46,
},
);
function trampoline47(handle) {
  const handleEntry = rscTableRemove(handleTable1, handle);
  if (handleEntry.own) {
    
    const rsc = captureTable1.get(handleEntry.rep);
    if (rsc) {
      if (rsc[symbolDispose]) rsc[symbolDispose]();
      captureTable1.delete(handleEntry.rep);
    } else if (Error$1[symbolCabiDispose]) {
      Error$1[symbolCabiDispose](handleEntry.rep);
    }
  }
}
function trampoline48(handle) {
  const handleEntry = rscTableRemove(handleTable0, handle);
  if (handleEntry.own) {
    
    const rsc = captureTable0.get(handleEntry.rep);
    if (rsc) {
      if (rsc[symbolDispose]) rsc[symbolDispose]();
      captureTable0.delete(handleEntry.rep);
    } else if (Pollable[symbolCabiDispose]) {
      Pollable[symbolCabiDispose](handleEntry.rep);
    }
  }
}
function trampoline49(handle) {
  const handleEntry = rscTableRemove(handleTable2, handle);
  if (handleEntry.own) {
    
    const rsc = captureTable2.get(handleEntry.rep);
    if (rsc) {
      if (rsc[symbolDispose]) rsc[symbolDispose]();
      captureTable2.delete(handleEntry.rep);
    } else if (InputStream[symbolCabiDispose]) {
      InputStream[symbolCabiDispose](handleEntry.rep);
    }
  }
}
function trampoline50(handle) {
  const handleEntry = rscTableRemove(handleTable3, handle);
  if (handleEntry.own) {
    
    const rsc = captureTable3.get(handleEntry.rep);
    if (rsc) {
      if (rsc[symbolDispose]) rsc[symbolDispose]();
      captureTable3.delete(handleEntry.rep);
    } else if (OutputStream[symbolCabiDispose]) {
      OutputStream[symbolCabiDispose](handleEntry.rep);
    }
  }
}
function trampoline51(handle) {
  const handleEntry = rscTableRemove(handleTable4, handle);
  if (handleEntry.own) {
    
    const rsc = captureTable4.get(handleEntry.rep);
    if (rsc) {
      if (rsc[symbolDispose]) rsc[symbolDispose]();
      captureTable4.delete(handleEntry.rep);
    } else if (TerminalInput[symbolCabiDispose]) {
      TerminalInput[symbolCabiDispose](handleEntry.rep);
    }
  }
}
function trampoline52(handle) {
  const handleEntry = rscTableRemove(handleTable5, handle);
  if (handleEntry.own) {
    
    const rsc = captureTable5.get(handleEntry.rep);
    if (rsc) {
      if (rsc[symbolDispose]) rsc[symbolDispose]();
      captureTable5.delete(handleEntry.rep);
    } else if (TerminalOutput[symbolCabiDispose]) {
      TerminalOutput[symbolCabiDispose](handleEntry.rep);
    }
  }
}
let trampoline53 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 53,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline53.manuallyAsync,
  paramLiftFns: [
  _liftFlatResult({
    caseMetas: [['ok', null, 0, 0, 0, []],['err', null, 0, 0, 0, []],],
    variantSize32: 1,
    variantAlign32: 1,
    variantPayloadOffset32: 1,
    variantFlatCount: 1,
    variantPayloadFlatTypes: [],
  })
  ],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline53,
},
);
let trampoline54 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 54,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline54.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 0)],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline54,
},
);
let trampoline55 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 55,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline55.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3)],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    tableIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_Pollable(obj) {
      if (!(obj instanceof Pollable)) {
        throw new TypeError('Resource error: Not a valid \"Pollable\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = (obj[symbolRscRep] ?? ++captureCnt0) >>> 0;
        captureTable0.set(rep, obj);
        handle = rscTableCreateOwn(handleTable0, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline55,
},
);
let trampoline56 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 56,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline56.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    tableIdx: 2,
    lowerFn: 
    function lowerImportedOwnedHost_InputStream(obj) {
      if (!(obj instanceof InputStream)) {
        throw new TypeError('Resource error: Not a valid \"InputStream\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = (obj[symbolRscRep] ?? ++captureCnt2) >>> 0;
        captureTable2.set(rep, obj);
        handle = rscTableCreateOwn(handleTable2, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline56,
},
);
let trampoline57 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 57,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline57.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    tableIdx: 3,
    lowerFn: 
    function lowerImportedOwnedHost_OutputStream(obj) {
      if (!(obj instanceof OutputStream)) {
        throw new TypeError('Resource error: Not a valid \"OutputStream\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = (obj[symbolRscRep] ?? ++captureCnt3) >>> 0;
        captureTable3.set(rep, obj);
        handle = rscTableCreateOwn(handleTable3, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline57,
},
);
let trampoline58 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 58,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline58.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    tableIdx: 3,
    lowerFn: 
    function lowerImportedOwnedHost_OutputStream(obj) {
      if (!(obj instanceof OutputStream)) {
        throw new TypeError('Resource error: Not a valid \"OutputStream\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = (obj[symbolRscRep] ?? ++captureCnt3) >>> 0;
        captureTable3.set(rep, obj);
        handle = rscTableCreateOwn(handleTable3, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline58,
},
);
let trampoline59 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 59,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline59.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    tableIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_Pollable(obj) {
      if (!(obj instanceof Pollable)) {
        throw new TypeError('Resource error: Not a valid \"Pollable\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = (obj[symbolRscRep] ?? ++captureCnt0) >>> 0;
        captureTable0.set(rep, obj);
        handle = rscTableCreateOwn(handleTable0, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline59,
},
);
let trampoline60 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 60,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline60.manuallyAsync,
  paramLiftFns: [_liftFlatStringAny,
  _liftFlatOption({
    caseMetas: [
    ['none', null, 0, 0, 0, [] ],
    ['some', _liftFlatStringAny, 8, 4, 2, ['i32','i32'] ],
    ],
    variantSize32: 12,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 3,
    variantPayloadFlatTypes: ['i32','i32'],
    payloadMaybeNull: false,
  })
  ],
  resultLowerFns: [_lowerFlatU64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline60,
},
);
let trampoline61 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 61,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline61.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatStringAny,_liftFlatStringAny],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline61,
},
);
let trampoline62 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 62,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline62.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatStringAny],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatStringAny, 8, 4, 2],
    ],
    variantSize32: 12,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 3,
    payloadMaybeNull: false,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: () => realloc0,
  importFn: _trampoline62,
},
);
let trampoline63 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 63,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline63.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatStringAny],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline63,
},
);
let trampoline64 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 64,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline64.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatStringAny],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline64,
},
);
let trampoline65 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 65,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline65.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatRecord({ fieldMetas: [['x', _lowerFlatFloat64, 8, 8 ],['y', _lowerFlatFloat64, 8, 8 ],['width', _lowerFlatFloat64, 8, 8 ],['height', _lowerFlatFloat64, 8, 8 ],], size32: 32, align32: 8 })],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline65,
},
);
let trampoline66 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 66,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline66.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatStringAny,_liftFlatStringAny,
  _liftFlatOption({
    caseMetas: [
    ['none', null, 0, 0, 0, [] ],
    ['some', _liftFlatStringAny, 8, 4, 2, ['i32','i32'] ],
    ],
    variantSize32: 12,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 3,
    variantPayloadFlatTypes: ['i32','i32'],
    payloadMaybeNull: false,
  })
  ],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline66,
},
);
let trampoline67 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 67,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline67.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatStringAny],
  resultLowerFns: [_lowerFlatStringAny],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: () => realloc0,
  importFn: _trampoline67,
},
);
let trampoline68 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 68,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline68.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatU64, 8, 8, 1],
    ],
    variantSize32: 16,
    variantAlign32: 8,
    variantPayloadOffset32: 8,
    variantFlatCount: 2,
    payloadMaybeNull: false,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline68,
},
);
let trampoline69 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 69,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline69.manuallyAsync,
  paramLiftFns: [_liftFlatU64,
  _liftFlatOption({
    caseMetas: [
    ['none', null, 0, 0, 0, [] ],
    ['some', _liftFlatStringAny, 8, 4, 2, ['i32','i32'] ],
    ],
    variantSize32: 12,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 3,
    variantPayloadFlatTypes: ['i32','i32'],
    payloadMaybeNull: false,
  })
  ],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline69,
},
);
let trampoline70 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 70,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline70.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatU64, 8, 8, 1],
    ],
    variantSize32: 16,
    variantAlign32: 8,
    variantPayloadOffset32: 8,
    variantFlatCount: 2,
    payloadMaybeNull: false,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline70,
},
);
let trampoline71 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 71,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline71.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatStringAny,_liftFlatBool],
  resultLowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', _lowerFlatU64, 16, 8, 8 ],
    [ 'err', _lowerFlatStringAny, 16, 8, 8 ],
    ],
    variantSize32: 16,
    variantAlign32: 8,
    variantPayloadOffset32: 8,
    variantFlatCount: 3,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: () => realloc0,
  importFn: _trampoline71,
},
);
let trampoline72 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 72,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline72.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatU64],
  resultLowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', null, 12, 4, 4 ],
    [ 'err', _lowerFlatStringAny, 12, 4, 4 ],
    ],
    variantSize32: 12,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 3,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: () => realloc0,
  importFn: _trampoline72,
},
);
let trampoline73 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 73,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline73.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatStringAny],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: () => realloc0,
  importFn: _trampoline73,
},
);
let trampoline74 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 74,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline74.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatU64, 8, 8, 1],
    ],
    variantSize32: 16,
    variantAlign32: 8,
    variantPayloadOffset32: 8,
    variantFlatCount: 2,
    payloadMaybeNull: false,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline74,
},
);
let trampoline75 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 75,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline75.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatU64, 8, 8, 1],
    ],
    variantSize32: 16,
    variantAlign32: 8,
    variantPayloadOffset32: 8,
    variantFlatCount: 2,
    payloadMaybeNull: false,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline75,
},
);
let trampoline76 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 76,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline76.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatU64, 8, 8, 1],
    ],
    variantSize32: 16,
    variantAlign32: 8,
    variantPayloadOffset32: 8,
    variantFlatCount: 2,
    payloadMaybeNull: false,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline76,
},
);
let trampoline77 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 77,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline77.manuallyAsync,
  paramLiftFns: [_liftFlatStringAny,
  _liftFlatOption({
    caseMetas: [
    ['none', null, 0, 0, 0, [] ],
    ['some', _liftFlatStringAny, 8, 4, 2, ['i32','i32'] ],
    ],
    variantSize32: 12,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 3,
    variantPayloadFlatTypes: ['i32','i32'],
    payloadMaybeNull: false,
  })
  ],
  resultLowerFns: [_lowerFlatU64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline77,
},
);
let trampoline78 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 78,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline78.manuallyAsync,
  paramLiftFns: [_liftFlatStringAny],
  resultLowerFns: [_lowerFlatU64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline78,
},
);
let trampoline79 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 79,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline79.manuallyAsync,
  paramLiftFns: [_liftFlatU64,
  _liftFlatOption({
    caseMetas: [
    ['none', null, 0, 0, 0, [] ],
    ['some', _liftFlatStringAny, 8, 4, 2, ['i32','i32'] ],
    ],
    variantSize32: 12,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 3,
    variantPayloadFlatTypes: ['i32','i32'],
    payloadMaybeNull: false,
  })
  ],
  resultLowerFns: [_lowerFlatU64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline79,
},
);
let trampoline80 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 80,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline80.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatStringAny],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatU64, 8, 8, 1],
    ],
    variantSize32: 16,
    variantAlign32: 8,
    variantPayloadOffset32: 8,
    variantFlatCount: 2,
    payloadMaybeNull: false,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline80,
},
);
let trampoline81 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 81,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline81.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatStringAny],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatU64, 8, 8, 1],
    ],
    variantSize32: 16,
    variantAlign32: 8,
    variantPayloadOffset32: 8,
    variantFlatCount: 2,
    payloadMaybeNull: false,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline81,
},
);
let trampoline82 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 82,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline82.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatU32],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatU64, 8, 8, 1],
    ],
    variantSize32: 16,
    variantAlign32: 8,
    variantPayloadOffset32: 8,
    variantFlatCount: 2,
    payloadMaybeNull: false,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline82,
},
);
let trampoline83 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 83,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline83.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatList({
    elemLiftFn: _liftFlatStringAny,
    elemAlign32: 4,
    elemSize32: 8,
    typedArray: undefined,
  })],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline83,
},
);
let trampoline84 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 84,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline84.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatList({
    elemLiftFn: _liftFlatStringAny,
    elemAlign32: 4,
    elemSize32: 8,
    typedArray: undefined,
  })],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline84,
},
);
let trampoline85 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 85,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline85.manuallyAsync,
  paramLiftFns: [_liftFlatU64,_liftFlatStringAny],
  resultLowerFns: [_lowerFlatBool],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline85,
},
);
let trampoline86 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 86,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline86.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatStringAny],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: () => realloc0,
  importFn: _trampoline86,
},
);
let trampoline87 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 87,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline87.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatTuple({ elemLowerMetas: [[_lowerFlatU64, 8, 8],[_lowerFlatU64, 8, 8],], size32: 16, align32: 8 })],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline87,
},
);
let trampoline88 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 88,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline88.manuallyAsync,
  paramLiftFns: [_liftFlatList({
    elemLiftFn: _liftFlatBorrow.bind(null, 0),
    elemAlign32: 4,
    elemSize32: 4,
    typedArray: undefined,
  })],
  resultLowerFns: [_lowerFlatList({
    elemLowerFn: _lowerFlatU32,
    elemSize32: 4,
    elemAlign32: 4,
  })],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: () => realloc0,
  importFn: _trampoline88,
},
);
let trampoline89 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 89,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline89.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3)],
  resultLowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', _lowerFlatU64, 16, 8, 8 ],
    [ 'err', _lowerFlatVariant({
      caseMetas: [[ "last-operation-failed", _lowerFlatOwn({
        componentIdx: 0,
        tableIdx: 1,
        lowerFn: 
        function lowerImportedOwnedHost_Error$1(obj) {
          if (!(obj instanceof Error$1)) {
            throw new TypeError('Resource error: Not a valid \"Error$1\" resource.');
          }
          let handle = obj[symbolRscHandle];
          if (!handle) {
            const rep = (obj[symbolRscRep] ?? ++captureCnt1) >>> 0;
            captureTable1.set(rep, obj);
            handle = rscTableCreateOwn(handleTable1, rep);
          }
          return handle;
        }
        ,
      }), 4, 4, 1 ],[ "closed", null, 0, 0, 0 ],],
      variantSize32: 8,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 2,
    } ), 16, 8, 8 ],
    ],
    variantSize32: 16,
    variantAlign32: 8,
    variantPayloadOffset32: 8,
    variantFlatCount: 3,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline89,
},
);
let trampoline90 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 90,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline90.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3),_liftFlatList({
    elemLiftFn: _liftFlatU8,
    elemAlign32: 1,
    elemSize32: 1,
    typedArray: Uint8Array,
  })],
  resultLowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', null, 12, 4, 4 ],
    [ 'err', _lowerFlatVariant({
      caseMetas: [[ "last-operation-failed", _lowerFlatOwn({
        componentIdx: 0,
        tableIdx: 1,
        lowerFn: 
        function lowerImportedOwnedHost_Error$1(obj) {
          if (!(obj instanceof Error$1)) {
            throw new TypeError('Resource error: Not a valid \"Error$1\" resource.');
          }
          let handle = obj[symbolRscHandle];
          if (!handle) {
            const rep = (obj[symbolRscRep] ?? ++captureCnt1) >>> 0;
            captureTable1.set(rep, obj);
            handle = rscTableCreateOwn(handleTable1, rep);
          }
          return handle;
        }
        ,
      }), 4, 4, 1 ],[ "closed", null, 0, 0, 0 ],],
      variantSize32: 8,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 2,
    } ), 12, 4, 4 ],
    ],
    variantSize32: 12,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 3,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline90,
},
);
let trampoline91 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 91,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline91.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3)],
  resultLowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', null, 12, 4, 4 ],
    [ 'err', _lowerFlatVariant({
      caseMetas: [[ "last-operation-failed", _lowerFlatOwn({
        componentIdx: 0,
        tableIdx: 1,
        lowerFn: 
        function lowerImportedOwnedHost_Error$1(obj) {
          if (!(obj instanceof Error$1)) {
            throw new TypeError('Resource error: Not a valid \"Error$1\" resource.');
          }
          let handle = obj[symbolRscHandle];
          if (!handle) {
            const rep = (obj[symbolRscRep] ?? ++captureCnt1) >>> 0;
            captureTable1.set(rep, obj);
            handle = rscTableCreateOwn(handleTable1, rep);
          }
          return handle;
        }
        ,
      }), 4, 4, 1 ],[ "closed", null, 0, 0, 0 ],],
      variantSize32: 8,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 2,
    } ), 12, 4, 4 ],
    ],
    variantSize32: 12,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 3,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline91,
},
);
let trampoline92 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 92,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline92.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatList({
    elemLowerFn: _lowerFlatTuple({ elemLowerMetas: [[_lowerFlatStringAny, 8, 4],[_lowerFlatStringAny, 8, 4],], size32: 16, align32: 4 }),
    elemSize32: 16,
    elemAlign32: 4,
  })],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: () => realloc0,
  importFn: _trampoline92,
},
);
let trampoline93 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 93,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline93.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatOwn({
      componentIdx: 0,
      tableIdx: 4,
      lowerFn: 
      function lowerImportedOwnedHost_TerminalInput(obj) {
        if (!(obj instanceof TerminalInput)) {
          throw new TypeError('Resource error: Not a valid \"TerminalInput\" resource.');
        }
        let handle = obj[symbolRscHandle];
        if (!handle) {
          const rep = (obj[symbolRscRep] ?? ++captureCnt4) >>> 0;
          captureTable4.set(rep, obj);
          handle = rscTableCreateOwn(handleTable4, rep);
        }
        return handle;
      }
      ,
    }), 4, 4, 1],
    ],
    variantSize32: 8,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 2,
    payloadMaybeNull: false,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline93,
},
);
let trampoline94 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 94,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline94.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatOwn({
      componentIdx: 0,
      tableIdx: 5,
      lowerFn: 
      function lowerImportedOwnedHost_TerminalOutput(obj) {
        if (!(obj instanceof TerminalOutput)) {
          throw new TypeError('Resource error: Not a valid \"TerminalOutput\" resource.');
        }
        let handle = obj[symbolRscHandle];
        if (!handle) {
          const rep = (obj[symbolRscRep] ?? ++captureCnt5) >>> 0;
          captureTable5.set(rep, obj);
          handle = rscTableCreateOwn(handleTable5, rep);
        }
        return handle;
      }
      ,
    }), 4, 4, 1],
    ],
    variantSize32: 8,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 2,
    payloadMaybeNull: false,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline94,
},
);
let trampoline95 = _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 95,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline95.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatOwn({
      componentIdx: 0,
      tableIdx: 5,
      lowerFn: 
      function lowerImportedOwnedHost_TerminalOutput(obj) {
        if (!(obj instanceof TerminalOutput)) {
          throw new TypeError('Resource error: Not a valid \"TerminalOutput\" resource.');
        }
        let handle = obj[symbolRscHandle];
        if (!handle) {
          const rep = (obj[symbolRscRep] ?? ++captureCnt5) >>> 0;
          captureTable5.set(rep, obj);
          handle = rscTableCreateOwn(handleTable5, rep);
        }
        return handle;
      }
      ,
    }), 4, 4, 1],
    ],
    variantSize32: 8,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 2,
    payloadMaybeNull: false,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline95,
},
);

const $init = (() => {
  let gen= (function* _initGenerator () {
    const module0 = fetchCompile(new URL('./_admin2.core.wasm?v=1794000983', import.meta.url));
    Promise.resolve(module0).catch(() => {});
    const module1 = base64Compile('AGFzbQEAAAABdRFgBX9/f39/AX5gBX5/f39/AGAEfn9/fwBgA35/fwBgAn5/AGAIfn9/f39/f38AYAR+f39/AGAFfn9/f38AYAN+fn8AYAF/AGACf38BfmAEfn9/fwF+YAN+f38AYAN+f38Bf2ADf39/AGACf38AYAR/f39/AAMlJAABAgMDBAUCBAYEBwgEBAQJAAoLAgIMAwMNBAkODxAPCQkJCQQFAXABJCQHtgElATAAAAExAAEBMgACATMAAwE0AAQBNQAFATYABgE3AAcBOAAIATkACQIxMAAKAjExAAsCMTIADAIxMwANAjE0AA4CMTUADwIxNgAQAjE3ABECMTgAEgIxOQATAjIwABQCMjEAFQIyMgAWAjIzABcCMjQAGAIyNQAZAjI2ABoCMjcAGwIyOAAcAjI5AB0CMzAAHgIzMQAfAjMyACACMzMAIQIzNAAiAjM1ACMIJGltcG9ydHMBAAr1AyQRACAAIAEgAiADIARBABEAAAsRACAAIAEgAiADIARBAREBAAsPACAAIAEgAiADQQIRAgALDQAgACABIAJBAxEDAAsNACAAIAEgAkEEEQMACwsAIAAgAUEFEQQACxcAIAAgASACIAMgBCAFIAYgB0EGEQUACw8AIAAgASACIANBBxECAAsLACAAIAFBCBEEAAsPACAAIAEgAiADQQkRBgALCwAgACABQQoRBAALEQAgACABIAIgAyAEQQsRBwALDQAgACABIAJBDBEIAAsLACAAIAFBDREEAAsLACAAIAFBDhEEAAsLACAAIAFBDxEEAAsJACAAQRARCQALEQAgACABIAIgAyAEQRERAAALCwAgACABQRIRCgALDwAgACABIAIgA0ETEQsACw8AIAAgASACIANBFBECAAsPACAAIAEgAiADQRURAgALDQAgACABIAJBFhEMAAsNACAAIAEgAkEXEQMACw0AIAAgASACQRgRAwALDQAgACABIAJBGRENAAsLACAAIAFBGhEEAAsJACAAQRsRCQALDQAgACABIAJBHBEOAAsLACAAIAFBHREPAAsPACAAIAEgAiADQR4REAALCwAgACABQR8RDwALCQAgAEEgEQkACwkAIABBIREJAAsJACAAQSIRCQALCQAgAEEjEQkACwAvCXByb2R1Y2VycwEMcHJvY2Vzc2VkLWJ5AQ13aXQtY29tcG9uZW50BzAuMjU0LjA');
    Promise.resolve(module1).catch(() => {});
    const module2 = base64Compile('AGFzbQEAAAABdRFgBX9/f39/AX5gBX5/f39/AGAEfn9/fwBgA35/fwBgAn5/AGAIfn9/f39/f38AYAR+f39/AGAFfn9/f38AYAN+fn8AYAF/AGACf38BfmAEfn9/fwF+YAN+f38AYAN+f38Bf2ADf39/AGACf38AYAR/f39/AALeASUAATAAAAABMQABAAEyAAIAATMAAwABNAADAAE1AAQAATYABQABNwACAAE4AAQAATkABgACMTAABAACMTEABwACMTIACAACMTMABAACMTQABAACMTUABAACMTYACQACMTcAAAACMTgACgACMTkACwACMjAAAgACMjEAAgACMjIADAACMjMAAwACMjQAAwACMjUADQACMjYABAACMjcACQACMjgADgACMjkADwACMzAAEAACMzEADwACMzIACQACMzMACQACMzQACQACMzUACQAIJGltcG9ydHMBcAEkJAkqAQBBAAskAAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8gISIjAC8JcHJvZHVjZXJzAQxwcm9jZXNzZWQtYnkBDXdpdC1jb21wb25lbnQHMC4yNTQuMA');
    Promise.resolve(module2).catch(() => {});
    const instanceFlags0 = new WebAssembly.Global({ value: "i32", mutable: true }, 1);
    INSTANCE_FLAGS.set(0, instanceFlags0);
    ({ exports: exports0 } = yield instantiateCore(yield module1));
    ({ exports: exports1 } = yield instantiateCore(yield module0, {
      "tairitsu-browser:full/css-style-declaration@0.2.0": {
        "get-property-value": Object.assign(exports0["7"], { _jcoMaySuspend: false }),
        "set-property": Object.assign(exports0["6"], { _jcoMaySuspend: false }),
      },
      "tairitsu-browser:full/document@0.2.0": {
        "create-element": Object.assign(exports0["17"], { _jcoMaySuspend: false }),
        "create-text-node": Object.assign(exports0["18"], { _jcoMaySuspend: false }),
        "get-body": Object.assign(exports0["16"], { _jcoMaySuspend: false }),
      },
      "tairitsu-browser:full/dom-token-list@0.2.0": {
        add: Object.assign(exports0["23"], { _jcoMaySuspend: false }),
        contains: Object.assign(exports0["25"], { _jcoMaySuspend: false }),
        remove: Object.assign(exports0["24"], { _jcoMaySuspend: false }),
      },
      "tairitsu-browser:full/element-css-inline-style@0.2.0": {
        "get-style": Object.assign(_guardTrap(0, trampoline4, false), { _jcoMaySuspend: false }),
      },
      "tairitsu-browser:full/element@0.2.0": {
        "get-attribute": Object.assign(exports0["2"], { _jcoMaySuspend: false }),
        "get-bounding-client-rect": Object.assign(exports0["5"], { _jcoMaySuspend: false }),
        "get-class-list": Object.assign(_guardTrap(0, trampoline10, false), { _jcoMaySuspend: false }),
        "get-client-height": Object.assign(_guardTrap(0, trampoline13, false), { _jcoMaySuspend: false }),
        "get-scroll-height": Object.assign(_guardTrap(0, trampoline14, false), { _jcoMaySuspend: false }),
        "get-scroll-top": Object.assign(_guardTrap(0, trampoline11, false), { _jcoMaySuspend: false }),
        "remove-attribute": Object.assign(exports0["4"], { _jcoMaySuspend: false }),
        "set-attribute": Object.assign(exports0["1"], { _jcoMaySuspend: false }),
        "set-inner-html": Object.assign(exports0["3"], { _jcoMaySuspend: false }),
        "set-scroll-top": Object.assign(_guardTrap(0, trampoline12, false), { _jcoMaySuspend: false }),
      },
      "tairitsu-browser:full/event-target@0.2.0": {
        "add-event-listener": Object.assign(exports0["11"], { _jcoMaySuspend: false }),
        "prevent-default": Object.assign(_guardTrap(0, trampoline2, false), { _jcoMaySuspend: false }),
        "remove-event-listener": Object.assign(exports0["12"], { _jcoMaySuspend: false }),
        "stop-propagation": Object.assign(_guardTrap(0, trampoline3, false), { _jcoMaySuspend: false }),
      },
      "tairitsu-browser:full/event@0.2.0": {
        "get-current-target": Object.assign(exports0["15"], { _jcoMaySuspend: false }),
        "get-target": Object.assign(exports0["14"], { _jcoMaySuspend: false }),
      },
      "tairitsu-browser:full/html-input-element@0.2.0": {
        "get-value": Object.assign(exports0["13"], { _jcoMaySuspend: false }),
      },
      "tairitsu-browser:full/mouse-event@0.2.0": {
        "get-alt-key": Object.assign(_guardTrap(0, trampoline32, false), { _jcoMaySuspend: false }),
        "get-button": Object.assign(_guardTrap(0, trampoline28, false), { _jcoMaySuspend: false }),
        "get-buttons": Object.assign(_guardTrap(0, trampoline29, false), { _jcoMaySuspend: false }),
        "get-client-x": Object.assign(_guardTrap(0, trampoline20, false), { _jcoMaySuspend: false }),
        "get-client-y": Object.assign(_guardTrap(0, trampoline21, false), { _jcoMaySuspend: false }),
        "get-ctrl-key": Object.assign(_guardTrap(0, trampoline30, false), { _jcoMaySuspend: false }),
        "get-meta-key": Object.assign(_guardTrap(0, trampoline33, false), { _jcoMaySuspend: false }),
        "get-offset-x": Object.assign(_guardTrap(0, trampoline24, false), { _jcoMaySuspend: false }),
        "get-offset-y": Object.assign(_guardTrap(0, trampoline25, false), { _jcoMaySuspend: false }),
        "get-page-x": Object.assign(_guardTrap(0, trampoline26, false), { _jcoMaySuspend: false }),
        "get-page-y": Object.assign(_guardTrap(0, trampoline27, false), { _jcoMaySuspend: false }),
        "get-screen-x": Object.assign(_guardTrap(0, trampoline22, false), { _jcoMaySuspend: false }),
        "get-screen-y": Object.assign(_guardTrap(0, trampoline23, false), { _jcoMaySuspend: false }),
        "get-shift-key": Object.assign(_guardTrap(0, trampoline31, false), { _jcoMaySuspend: false }),
      },
      "tairitsu-browser:full/node-list@0.2.0": {
        "get-length": Object.assign(_guardTrap(0, trampoline19, false), { _jcoMaySuspend: false }),
        item: Object.assign(exports0["22"], { _jcoMaySuspend: false }),
      },
      "tairitsu-browser:full/node@0.2.0": {
        "append-child": Object.assign(_guardTrap(0, trampoline16, false), { _jcoMaySuspend: false }),
        "get-child-nodes": Object.assign(_guardTrap(0, trampoline18, false), { _jcoMaySuspend: false }),
        "get-first-child": Object.assign(exports0["8"], { _jcoMaySuspend: false }),
        "get-parent-element": Object.assign(exports0["10"], { _jcoMaySuspend: false }),
        "insert-before": Object.assign(_guardTrap(0, trampoline17, false), { _jcoMaySuspend: false }),
        "remove-child": Object.assign(_guardTrap(0, trampoline1, false), { _jcoMaySuspend: false }),
        "set-text-content": Object.assign(exports0["9"], { _jcoMaySuspend: false }),
      },
      "tairitsu-browser:full/non-element-parent-node@0.2.0": {
        "get-element-by-id": Object.assign(exports0["21"], { _jcoMaySuspend: false }),
      },
      "tairitsu-browser:full/parent-node@0.2.0": {
        "query-selector": Object.assign(exports0["20"], { _jcoMaySuspend: false }),
      },
      "tairitsu-browser:full/platform-helpers@0.2.0": {
        "cancel-animation-frame": Object.assign(_guardTrap(0, trampoline9, false), { _jcoMaySuspend: false }),
        "clear-interval": Object.assign(_guardTrap(0, trampoline8, false), { _jcoMaySuspend: false }),
        "clear-timeout": Object.assign(_guardTrap(0, trampoline7, false), { _jcoMaySuspend: false }),
        "fetch-promise": Object.assign(exports0["0"], { _jcoMaySuspend: false }),
        "request-animation-frame": Object.assign(_guardTrap(0, trampoline5, false), { _jcoMaySuspend: false }),
        "set-interval": Object.assign(_guardTrap(0, trampoline6, false), { _jcoMaySuspend: false }),
        "set-timeout": Object.assign(_guardTrap(0, trampoline0, false), { _jcoMaySuspend: false }),
      },
      "tairitsu-browser:full/pointer-event@0.2.0": {
        "get-height": Object.assign(_guardTrap(0, trampoline37, false), { _jcoMaySuspend: false }),
        "get-is-primary": Object.assign(_guardTrap(0, trampoline35, false), { _jcoMaySuspend: false }),
        "get-pointer-id": Object.assign(_guardTrap(0, trampoline34, false), { _jcoMaySuspend: false }),
        "get-pointer-type": Object.assign(exports0["26"], { _jcoMaySuspend: false }),
        "get-pressure": Object.assign(_guardTrap(0, trampoline38, false), { _jcoMaySuspend: false }),
        "get-tangential-pressure": Object.assign(_guardTrap(0, trampoline39, false), { _jcoMaySuspend: false }),
        "get-tilt-x": Object.assign(_guardTrap(0, trampoline40, false), { _jcoMaySuspend: false }),
        "get-tilt-y": Object.assign(_guardTrap(0, trampoline41, false), { _jcoMaySuspend: false }),
        "get-twist": Object.assign(_guardTrap(0, trampoline42, false), { _jcoMaySuspend: false }),
        "get-width": Object.assign(_guardTrap(0, trampoline36, false), { _jcoMaySuspend: false }),
      },
      "tairitsu-browser:full/wheel-event@0.2.0": {
        "get-delta-mode": Object.assign(_guardTrap(0, trampoline46, false), { _jcoMaySuspend: false }),
        "get-delta-x": Object.assign(_guardTrap(0, trampoline43, false), { _jcoMaySuspend: false }),
        "get-delta-y": Object.assign(_guardTrap(0, trampoline44, false), { _jcoMaySuspend: false }),
        "get-delta-z": Object.assign(_guardTrap(0, trampoline45, false), { _jcoMaySuspend: false }),
      },
      "tairitsu-browser:full/window@0.2.0": {
        "get-computed-style": Object.assign(exports0["19"], { _jcoMaySuspend: false }),
        "get-inner-width": Object.assign(_guardTrap(0, trampoline15, false), { _jcoMaySuspend: false }),
      },
      "wasi:cli/environment@0.2.0": {
        "get-environment": Object.assign(exports0["32"], { _jcoMaySuspend: false }),
      },
      "wasi:cli/exit@0.2.0": {
        exit: Object.assign(_guardTrap(0, trampoline53, false), { _jcoMaySuspend: false }),
      },
      "wasi:cli/stderr@0.2.0": {
        "get-stderr": Object.assign(_guardTrap(0, trampoline58, false), { _jcoMaySuspend: false }),
      },
      "wasi:cli/stdin@0.2.0": {
        "get-stdin": Object.assign(_guardTrap(0, trampoline56, false), { _jcoMaySuspend: false }),
      },
      "wasi:cli/stdout@0.2.0": {
        "get-stdout": Object.assign(_guardTrap(0, trampoline57, false), { _jcoMaySuspend: false }),
      },
      "wasi:cli/terminal-input@0.2.0": {
        "[resource-drop]terminal-input": Object.assign(_guardTrap(0, _guardMayLeave(0, trampoline51), true), { _jcoMaySuspend: false }),
      },
      "wasi:cli/terminal-output@0.2.0": {
        "[resource-drop]terminal-output": Object.assign(_guardTrap(0, _guardMayLeave(0, trampoline52), true), { _jcoMaySuspend: false }),
      },
      "wasi:cli/terminal-stderr@0.2.0": {
        "get-terminal-stderr": Object.assign(exports0["35"], { _jcoMaySuspend: false }),
      },
      "wasi:cli/terminal-stdin@0.2.0": {
        "get-terminal-stdin": Object.assign(exports0["33"], { _jcoMaySuspend: false }),
      },
      "wasi:cli/terminal-stdout@0.2.0": {
        "get-terminal-stdout": Object.assign(exports0["34"], { _jcoMaySuspend: false }),
      },
      "wasi:clocks/monotonic-clock@0.2.0": {
        "subscribe-duration": Object.assign(_guardTrap(0, trampoline59, false), { _jcoMaySuspend: false }),
      },
      "wasi:io/error@0.2.0": {
        "[resource-drop]error": Object.assign(_guardTrap(0, _guardMayLeave(0, trampoline47), true), { _jcoMaySuspend: false }),
      },
      "wasi:io/poll@0.2.0": {
        "[method]pollable.block": Object.assign(_guardTrap(0, trampoline54, false), { _jcoMaySuspend: false }),
        "[resource-drop]pollable": Object.assign(_guardTrap(0, _guardMayLeave(0, trampoline48), true), { _jcoMaySuspend: false }),
        poll: Object.assign(exports0["28"], { _jcoMaySuspend: false }),
      },
      "wasi:io/streams@0.2.0": {
        "[method]output-stream.blocking-flush": Object.assign(exports0["31"], { _jcoMaySuspend: false }),
        "[method]output-stream.check-write": Object.assign(exports0["29"], { _jcoMaySuspend: false }),
        "[method]output-stream.subscribe": Object.assign(_guardTrap(0, trampoline55, false), { _jcoMaySuspend: false }),
        "[method]output-stream.write": Object.assign(exports0["30"], { _jcoMaySuspend: false }),
        "[resource-drop]input-stream": Object.assign(_guardTrap(0, _guardMayLeave(0, trampoline49), true), { _jcoMaySuspend: false }),
        "[resource-drop]output-stream": Object.assign(_guardTrap(0, _guardMayLeave(0, trampoline50), true), { _jcoMaySuspend: false }),
      },
      "wasi:random/insecure-seed@0.2.12": {
        "insecure-seed": Object.assign(exports0["27"], { _jcoMaySuspend: false }),
      },
    }));
    memory0 = exports1.memory;
    realloc0 = (oldPtr, oldSize, align, newSize) => exports1.cabi_realloc(oldPtr, oldSize, align, newSize) >>> 0;
    
    try {
      const realloc0Promising = WebAssembly.promising(exports1.cabi_realloc);
      realloc0Async = async (oldPtr, oldSize, align, newSize) => (await realloc0Promising(oldPtr, oldSize, align, newSize)) >>> 0;
    } catch(err) {
      realloc0Async = realloc0;
    }
    
    ({ exports: exports2 } = yield instantiateCore(yield module2, {
      "": {
        $imports: exports0.$imports,
        "0": Object.assign(_guardTrap(0, trampoline60, false), { _jcoMaySuspend: false }),
        "1": Object.assign(_guardTrap(0, trampoline61, false), { _jcoMaySuspend: false }),
        "10": Object.assign(_guardTrap(0, trampoline70, false), { _jcoMaySuspend: false }),
        "11": Object.assign(_guardTrap(0, trampoline71, false), { _jcoMaySuspend: false }),
        "12": Object.assign(_guardTrap(0, trampoline72, false), { _jcoMaySuspend: false }),
        "13": Object.assign(_guardTrap(0, trampoline73, false), { _jcoMaySuspend: false }),
        "14": Object.assign(_guardTrap(0, trampoline74, false), { _jcoMaySuspend: false }),
        "15": Object.assign(_guardTrap(0, trampoline75, false), { _jcoMaySuspend: false }),
        "16": Object.assign(_guardTrap(0, trampoline76, false), { _jcoMaySuspend: false }),
        "17": Object.assign(_guardTrap(0, trampoline77, false), { _jcoMaySuspend: false }),
        "18": Object.assign(_guardTrap(0, trampoline78, false), { _jcoMaySuspend: false }),
        "19": Object.assign(_guardTrap(0, trampoline79, false), { _jcoMaySuspend: false }),
        "2": Object.assign(_guardTrap(0, trampoline62, false), { _jcoMaySuspend: false }),
        "20": Object.assign(_guardTrap(0, trampoline80, false), { _jcoMaySuspend: false }),
        "21": Object.assign(_guardTrap(0, trampoline81, false), { _jcoMaySuspend: false }),
        "22": Object.assign(_guardTrap(0, trampoline82, false), { _jcoMaySuspend: false }),
        "23": Object.assign(_guardTrap(0, trampoline83, false), { _jcoMaySuspend: false }),
        "24": Object.assign(_guardTrap(0, trampoline84, false), { _jcoMaySuspend: false }),
        "25": Object.assign(_guardTrap(0, trampoline85, false), { _jcoMaySuspend: false }),
        "26": Object.assign(_guardTrap(0, trampoline86, false), { _jcoMaySuspend: false }),
        "27": Object.assign(_guardTrap(0, trampoline87, false), { _jcoMaySuspend: false }),
        "28": Object.assign(_guardTrap(0, trampoline88, false), { _jcoMaySuspend: false }),
        "29": Object.assign(_guardTrap(0, trampoline89, false), { _jcoMaySuspend: false }),
        "3": Object.assign(_guardTrap(0, trampoline63, false), { _jcoMaySuspend: false }),
        "30": Object.assign(_guardTrap(0, trampoline90, false), { _jcoMaySuspend: false }),
        "31": Object.assign(_guardTrap(0, trampoline91, false), { _jcoMaySuspend: false }),
        "32": Object.assign(_guardTrap(0, trampoline92, false), { _jcoMaySuspend: false }),
        "33": Object.assign(_guardTrap(0, trampoline93, false), { _jcoMaySuspend: false }),
        "34": Object.assign(_guardTrap(0, trampoline94, false), { _jcoMaySuspend: false }),
        "35": Object.assign(_guardTrap(0, trampoline95, false), { _jcoMaySuspend: false }),
        "4": Object.assign(_guardTrap(0, trampoline64, false), { _jcoMaySuspend: false }),
        "5": Object.assign(_guardTrap(0, trampoline65, false), { _jcoMaySuspend: false }),
        "6": Object.assign(_guardTrap(0, trampoline66, false), { _jcoMaySuspend: false }),
        "7": Object.assign(_guardTrap(0, trampoline67, false), { _jcoMaySuspend: false }),
        "8": Object.assign(_guardTrap(0, trampoline68, false), { _jcoMaySuspend: false }),
        "9": Object.assign(_guardTrap(0, trampoline69, false), { _jcoMaySuspend: false }),
      },
    }));
    postReturn0 = exports1["cabi_post_tairitsu-browser:full/lifecycle@0.2.0#start"];
    
    try {
      postReturn0Async = WebAssembly.promising(exports1["cabi_post_tairitsu-browser:full/lifecycle@0.2.0#start"]);
    } catch(err) {
      postReturn0Async = exports1["cabi_post_tairitsu-browser:full/lifecycle@0.2.0#start"];
    }
    
    postReturn1 = exports1["cabi_post_tairitsu:style/builder#build-style-string"];
    
    try {
      postReturn1Async = WebAssembly.promising(exports1["cabi_post_tairitsu:style/builder#build-style-string"]);
    } catch(err) {
      postReturn1Async = exports1["cabi_post_tairitsu:style/builder#build-style-string"];
    }
    
    postReturn2 = exports1["cabi_post_tairitsu:style/utility#generate-utility-css"];
    
    try {
      postReturn2Async = WebAssembly.promising(exports1["cabi_post_tairitsu:style/utility#generate-utility-css"]);
    } catch(err) {
      postReturn2Async = exports1["cabi_post_tairitsu:style/utility#generate-utility-css"];
    }
    
    postReturn3 = exports1["cabi_post_tairitsu:hooks/version#get-version"];
    
    try {
      postReturn3Async = WebAssembly.promising(exports1["cabi_post_tairitsu:hooks/version#get-version"]);
    } catch(err) {
      postReturn3Async = exports1["cabi_post_tairitsu:hooks/version#get-version"];
    }
    
    postReturn4 = exports1["cabi_post_tairitsu:vdom/svg#sanitize-svg"];
    
    try {
      postReturn4Async = WebAssembly.promising(exports1["cabi_post_tairitsu:vdom/svg#sanitize-svg"]);
    } catch(err) {
      postReturn4Async = exports1["cabi_post_tairitsu:vdom/svg#sanitize-svg"];
    }
    
    eventCallbacks020OnMouseEvent = exports1["tairitsu-browser:full/event-callbacks@0.2.0#on-mouse-event"];
    eventCallbacks020OnKeyboardEvent = exports1["tairitsu-browser:full/event-callbacks@0.2.0#on-keyboard-event"];
    eventCallbacks020OnFocusEvent = exports1["tairitsu-browser:full/event-callbacks@0.2.0#on-focus-event"];
    eventCallbacks020OnInputEvent = exports1["tairitsu-browser:full/event-callbacks@0.2.0#on-input-event"];
    eventCallbacks020OnGenericEvent = exports1["tairitsu-browser:full/event-callbacks@0.2.0#on-generic-event"];
    lifecycle020Start = exports1["tairitsu-browser:full/lifecycle@0.2.0#start"];
    timerCallbacks020OnTimeout = exports1["tairitsu-browser:full/timer-callbacks@0.2.0#on-timeout"];
    timerCallbacks020OnInterval = exports1["tairitsu-browser:full/timer-callbacks@0.2.0#on-interval"];
    animationCallbacks020OnFrame = exports1["tairitsu-browser:full/animation-callbacks@0.2.0#on-frame"];
    resizeObserverCallbacks020OnResize = exports1["tairitsu-browser:full/resize-observer-callbacks@0.2.0#on-resize"];
    mutationObserverCallbacks020OnMutate = exports1["tairitsu-browser:full/mutation-observer-callbacks@0.2.0#on-mutate"];
    mediaQueryListCallbacks020OnChange = exports1["tairitsu-browser:full/media-query-list-callbacks@0.2.0#on-change"];
    scrollCallbacks020OnScrollEvent = exports1["tairitsu-browser:full/scroll-callbacks@0.2.0#on-scroll-event"];
    windowResizeCallbacks020OnWindowResize = exports1["tairitsu-browser:full/window-resize-callbacks@0.2.0#on-window-resize"];
    videoFrameCallbacks020OnVideoFrame = exports1["tairitsu-browser:full/video-frame-callbacks@0.2.0#on-video-frame"];
    promiseCallbacks020OnPromiseResolved = exports1["tairitsu-browser:full/promise-callbacks@0.2.0#on-promise-resolved"];
    promiseCallbacks020OnPromiseRejected = exports1["tairitsu-browser:full/promise-callbacks@0.2.0#on-promise-rejected"];
    geolocationCallbacks020OnPositionSuccess = exports1["tairitsu-browser:full/geolocation-callbacks@0.2.0#on-position-success"];
    geolocationCallbacks020OnPositionError = exports1["tairitsu-browser:full/geolocation-callbacks@0.2.0#on-position-error"];
    idbCallbacks020OnIdbRequestSuccess = exports1["tairitsu-browser:full/idb-callbacks@0.2.0#on-idb-request-success"];
    idbCallbacks020OnIdbRequestError = exports1["tairitsu-browser:full/idb-callbacks@0.2.0#on-idb-request-error"];
    fileReaderCallbacks020OnFileReaderLoad = exports1["tairitsu-browser:full/file-reader-callbacks@0.2.0#on-file-reader-load"];
    fileReaderCallbacks020OnFileReaderError = exports1["tairitsu-browser:full/file-reader-callbacks@0.2.0#on-file-reader-error"];
    webSocketCallbacks020OnWebSocketOpen = exports1["tairitsu-browser:full/web-socket-callbacks@0.2.0#on-web-socket-open"];
    webSocketCallbacks020OnWebSocketMessage = exports1["tairitsu-browser:full/web-socket-callbacks@0.2.0#on-web-socket-message"];
    webSocketCallbacks020OnWebSocketClose = exports1["tairitsu-browser:full/web-socket-callbacks@0.2.0#on-web-socket-close"];
    webSocketCallbacks020OnWebSocketError = exports1["tairitsu-browser:full/web-socket-callbacks@0.2.0#on-web-socket-error"];
    versionGetVersion = exports1["tairitsu:style/version#get-version"];
    builderBuildStyleString = exports1["tairitsu:style/builder#build-style-string"];
    builderCssPropertyToString = exports1["tairitsu:style/builder#css-property-to-string"];
    builderCssLengthToString = exports1["tairitsu:style/builder#css-length-to-string"];
    utilityGenerateUtilityCss = exports1["tairitsu:style/utility#generate-utility-css"];
    utilityGenerateStylesheet = exports1["tairitsu:style/utility#generate-stylesheet"];
    utilityParseUtilityBase = exports1["tairitsu:style/utility#parse-utility-base"];
    classesBuildClasses = exports1["tairitsu:style/classes#build-classes"];
    classesBuildClassesConditional = exports1["tairitsu:style/classes#build-classes-conditional"];
    versionGetVersion$1 = exports1["tairitsu:hooks/version#get-version"];
    stateMachineTransition = exports1["tairitsu:hooks/state-machine#transition"];
    stateMachineIsInteractive = exports1["tairitsu:hooks/state-machine#is-interactive"];
    animationTypesEvaluateEasing = exports1["tairitsu:hooks/animation-types#evaluate-easing"];
    animationTypesLerpF32 = exports1["tairitsu:hooks/animation-types#lerp-f32"];
    animationBuilderDefaultAnimationConfig = exports1["tairitsu:hooks/animation-builder#default-animation-config"];
    asyncResourceResourceStatusQuery = exports1["tairitsu:hooks/async-resource#resource-status-query"];
    versionGetVersion$2 = exports1["tairitsu:vdom/version#get-version"];
    svgSanitizeSvg = exports1["tairitsu:vdom/svg#sanitize-svg"];
  })();
  let promise, resolve, reject;
  function normalizeInstantiationError(e) {
    // Native JSPI rejects a suspending import called from a
    // core start function before entering its JS wrapper.
    // At component instantiation time that always means the
    // implicit synchronous task attempted to block.
    if (typeof WebAssembly !== 'undefined' && typeof WebAssembly.SuspendError === 'function' && e instanceof WebAssembly.SuspendError) {
      return new WebAssembly.RuntimeError('cannot block a synchronous task before returning');
    }
    return e;
  }
  function runNext(value) {
    try {
      let done;
      do {
        ({ value, done } = gen.next(value));
      } while (!(value instanceof Promise) && !done);
      if (done) {
        if (resolve) resolve(value);
        else return value;
      }
      if (!promise) promise = new Promise((_resolve, _reject) => (resolve = _resolve, reject= _reject));
      value.then(runNext, e => reject(normalizeInstantiationError(e)));
    }
    catch (e) {
      e = normalizeInstantiationError(e);
      if (reject) reject(e);
      else throw e;
    }
  }
  const maybeSyncReturn = runNext(null);
  return promise || maybeSyncReturn;
})();

await $init;
const animationCallbacks020 = {
  onFrame: onFrame,
  
};
const eventCallbacks020 = {
  onFocusEvent: onFocusEvent,
  onGenericEvent: onGenericEvent,
  onInputEvent: onInputEvent,
  onKeyboardEvent: onKeyboardEvent,
  onMouseEvent: onMouseEvent,
  
};
const fileReaderCallbacks020 = {
  onFileReaderError: onFileReaderError,
  onFileReaderLoad: onFileReaderLoad,
  
};
const geolocationCallbacks020 = {
  onPositionError: onPositionError,
  onPositionSuccess: onPositionSuccess,
  
};
const idbCallbacks020 = {
  onIdbRequestError: onIdbRequestError,
  onIdbRequestSuccess: onIdbRequestSuccess,
  
};
const lifecycle020 = {
  start: start,
  
};
const mediaQueryListCallbacks020 = {
  onChange: onChange,
  
};
const mutationObserverCallbacks020 = {
  onMutate: onMutate,
  
};
const promiseCallbacks020 = {
  onPromiseRejected: onPromiseRejected,
  onPromiseResolved: onPromiseResolved,
  
};
const resizeObserverCallbacks020 = {
  onResize: onResize,
  
};
const scrollCallbacks020 = {
  onScrollEvent: onScrollEvent,
  
};
const timerCallbacks020 = {
  onInterval: onInterval,
  onTimeout: onTimeout,
  
};
const videoFrameCallbacks020 = {
  onVideoFrame: onVideoFrame,
  
};
const webSocketCallbacks020 = {
  onWebSocketClose: onWebSocketClose,
  onWebSocketError: onWebSocketError,
  onWebSocketMessage: onWebSocketMessage,
  onWebSocketOpen: onWebSocketOpen,
  
};
const windowResizeCallbacks020 = {
  onWindowResize: onWindowResize,
  
};
const animationBuilder = {
  defaultAnimationConfig: defaultAnimationConfig,
  
};
const animationTypes = {
  evaluateEasing: evaluateEasing,
  lerpF32: lerpF32,
  
};
const asyncResource = {
  resourceStatusQuery: resourceStatusQuery,
  
};
const stateMachine = {
  isInteractive: isInteractive,
  transition: transition,
  
};
const version = {
  getVersion: getVersion$1,
  
};
const builder = {
  buildStyleString: buildStyleString,
  cssLengthToString: cssLengthToString,
  cssPropertyToString: cssPropertyToString,
  
};
const classes = {
  buildClasses: buildClasses,
  buildClassesConditional: buildClassesConditional,
  
};
const utility = {
  generateStylesheet: generateStylesheet,
  generateUtilityCss: generateUtilityCss,
  parseUtilityBase: parseUtilityBase,
  
};
const version$1 = {
  getVersion: getVersion,
  
};
const svg = {
  sanitizeSvg: sanitizeSvg,
  
};
const version$2 = {
  getVersion: getVersion$2,
  
};

export { animationBuilder, animationCallbacks020 as animationCallbacks, animationTypes, asyncResource, builder, classes, eventCallbacks020 as eventCallbacks, fileReaderCallbacks020 as fileReaderCallbacks, geolocationCallbacks020 as geolocationCallbacks, idbCallbacks020 as idbCallbacks, lifecycle020 as lifecycle, mediaQueryListCallbacks020 as mediaQueryListCallbacks, mutationObserverCallbacks020 as mutationObserverCallbacks, promiseCallbacks020 as promiseCallbacks, resizeObserverCallbacks020 as resizeObserverCallbacks, scrollCallbacks020 as scrollCallbacks, stateMachine, svg, timerCallbacks020 as timerCallbacks, utility, version, videoFrameCallbacks020 as videoFrameCallbacks, webSocketCallbacks020 as webSocketCallbacks, windowResizeCallbacks020 as windowResizeCallbacks, animationCallbacks020 as "tairitsu-browser:full/animation-callbacks@0.2.0", eventCallbacks020 as "tairitsu-browser:full/event-callbacks@0.2.0", fileReaderCallbacks020 as "tairitsu-browser:full/file-reader-callbacks@0.2.0", geolocationCallbacks020 as "tairitsu-browser:full/geolocation-callbacks@0.2.0", idbCallbacks020 as "tairitsu-browser:full/idb-callbacks@0.2.0", lifecycle020 as "tairitsu-browser:full/lifecycle@0.2.0", mediaQueryListCallbacks020 as "tairitsu-browser:full/media-query-list-callbacks@0.2.0", mutationObserverCallbacks020 as "tairitsu-browser:full/mutation-observer-callbacks@0.2.0", promiseCallbacks020 as "tairitsu-browser:full/promise-callbacks@0.2.0", resizeObserverCallbacks020 as "tairitsu-browser:full/resize-observer-callbacks@0.2.0", scrollCallbacks020 as "tairitsu-browser:full/scroll-callbacks@0.2.0", timerCallbacks020 as "tairitsu-browser:full/timer-callbacks@0.2.0", videoFrameCallbacks020 as "tairitsu-browser:full/video-frame-callbacks@0.2.0", webSocketCallbacks020 as "tairitsu-browser:full/web-socket-callbacks@0.2.0", windowResizeCallbacks020 as "tairitsu-browser:full/window-resize-callbacks@0.2.0", animationBuilder as "tairitsu:hooks/animation-builder", animationTypes as "tairitsu:hooks/animation-types", asyncResource as "tairitsu:hooks/async-resource", stateMachine as "tairitsu:hooks/state-machine", version as "tairitsu:hooks/version", builder as "tairitsu:style/builder", classes as "tairitsu:style/classes", utility as "tairitsu:style/utility", version$1 as "tairitsu:style/version", svg as "tairitsu:vdom/svg", version$2 as "tairitsu:vdom/version",  }
export const _util = {
  
}

