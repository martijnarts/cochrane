#!/bin/bash
set -xe
: "${DIOXUS_ENV?Need a Dioxus environment}"

sed -i "s|<\/head>|<script>const env = $DIOXUS_ENV;<\/script>\n&|" /usr/share/web/dist/index.html

exec "$@"
