bindgen.exe -o src\rush_ffi.rs ^
--no-layout-tests ^
--no-prepend-enum-name ^
--with-derive-partialeq ^
--whitelist-function "rush_.*" ^
vendor\Rush\RushC.h
