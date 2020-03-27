DIR=$(cd `dirname $0` && pwd)
bindgen -o $DIR/src/rush_ffi.rs \
--no-layout-tests \
--no-prepend-enum-name \
--with-derive-partialeq \
--with-derive-default \
--whitelist-function 'rush_.*' \
$DIR/vendor/Rush/RushC.h
