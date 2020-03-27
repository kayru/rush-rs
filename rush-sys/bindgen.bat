bindgen.exe -o src\rush_ffi.rs ^
--no-layout-tests ^
--no-prepend-enum-name ^
--with-derive-partialeq ^
--with-derive-default ^
--whitelist-function "rush_.*" ^
vendor\Rush\RushC.h
