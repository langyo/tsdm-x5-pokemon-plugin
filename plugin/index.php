<?php
// Legacy asset proxy is retired. Assets are served directly from images/ and
// wasm/; request-controlled paths must never be passed to readfile().
http_response_code(404);
exit;
