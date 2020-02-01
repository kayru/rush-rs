DIR=$(cd `dirname $0` && pwd)
OUT_FILE=$DIR/src/rush_ffi.rs
IN_FILE=$DIR/vendor/Rush/RushC.h
bindgen -o $OUT_FILE --whitelist-function 'rush_.*' $IN_FILE
