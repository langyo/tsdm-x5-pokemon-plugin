import { _getPreopens } from "../filesystem.js";
import { writeFileAt as writeFileAtImpl, readFileAt as readFileAtImpl, } from "../../common/helpers/filesystem.js";
/** Write bytes into a preopened directory's tree from host JS, without a guest. */
export function writeFileAt(virtualPath, path, data, options = {}) {
    writeFileAtImpl(_getPreopens(), virtualPath, path, data, options);
}
/** Read the full contents of a file under a preopened directory from host JS, without a guest. */
export function readFileAt(virtualPath, path) {
    return readFileAtImpl(_getPreopens(), virtualPath, path);
}
