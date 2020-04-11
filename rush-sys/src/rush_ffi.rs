/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_vec2 {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_color_rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
pub type rush_platform_callback_startup =
    ::std::option::Option<unsafe extern "C" fn(user_data: *mut ::std::os::raw::c_void)>;
pub type rush_platform_callback_update =
    ::std::option::Option<unsafe extern "C" fn(user_data: *mut ::std::os::raw::c_void)>;
pub type rush_platform_callback_shutdown =
    ::std::option::Option<unsafe extern "C" fn(user_data: *mut ::std::os::raw::c_void)>;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rush_app_config {
    pub name: *const ::std::os::raw::c_char,
    pub vsync: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub max_width: ::std::os::raw::c_int,
    pub max_height: ::std::os::raw::c_int,
    pub fullscreen: bool,
    pub resizable: bool,
    pub debug: bool,
    pub warp: bool,
    pub minimize_latency: bool,
    pub argc: ::std::os::raw::c_int,
    pub argv: *mut *mut ::std::os::raw::c_char,
    pub user_data: *mut ::std::os::raw::c_void,
    pub on_startup: rush_platform_callback_startup,
    pub on_update: rush_platform_callback_update,
    pub on_shutdown: rush_platform_callback_shutdown,
}
impl Default for rush_app_config {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    pub fn rush_app_config_init(out_cfg: *mut rush_app_config);
}
extern "C" {
    pub fn rush_platform_main(cfg: *const rush_app_config) -> ::std::os::raw::c_int;
}
pub const RUSH_KEY_UNKNOWN: rush_key = 0;
pub const RUSH_KEY_SPACE: rush_key = 32;
pub const RUSH_KEY_APOSTROPHE: rush_key = 39;
pub const RUSH_KEY_COMMA: rush_key = 44;
pub const RUSH_KEY_MINUS: rush_key = 45;
pub const RUSH_KEY_PERIOD: rush_key = 46;
pub const RUSH_KEY_SLASH: rush_key = 47;
pub const RUSH_KEY_0: rush_key = 48;
pub const RUSH_KEY_1: rush_key = 49;
pub const RUSH_KEY_2: rush_key = 50;
pub const RUSH_KEY_3: rush_key = 51;
pub const RUSH_KEY_4: rush_key = 52;
pub const RUSH_KEY_5: rush_key = 53;
pub const RUSH_KEY_6: rush_key = 54;
pub const RUSH_KEY_7: rush_key = 55;
pub const RUSH_KEY_8: rush_key = 56;
pub const RUSH_KEY_9: rush_key = 57;
pub const RUSH_KEY_SEMICOLON: rush_key = 59;
pub const RUSH_KEY_EQUAL: rush_key = 61;
pub const RUSH_KEY_A: rush_key = 65;
pub const RUSH_KEY_B: rush_key = 66;
pub const RUSH_KEY_C: rush_key = 67;
pub const RUSH_KEY_D: rush_key = 68;
pub const RUSH_KEY_E: rush_key = 69;
pub const RUSH_KEY_F: rush_key = 70;
pub const RUSH_KEY_G: rush_key = 71;
pub const RUSH_KEY_H: rush_key = 72;
pub const RUSH_KEY_I: rush_key = 73;
pub const RUSH_KEY_J: rush_key = 74;
pub const RUSH_KEY_K: rush_key = 75;
pub const RUSH_KEY_L: rush_key = 76;
pub const RUSH_KEY_M: rush_key = 77;
pub const RUSH_KEY_N: rush_key = 78;
pub const RUSH_KEY_O: rush_key = 79;
pub const RUSH_KEY_P: rush_key = 80;
pub const RUSH_KEY_Q: rush_key = 81;
pub const RUSH_KEY_R: rush_key = 82;
pub const RUSH_KEY_S: rush_key = 83;
pub const RUSH_KEY_T: rush_key = 84;
pub const RUSH_KEY_U: rush_key = 85;
pub const RUSH_KEY_V: rush_key = 86;
pub const RUSH_KEY_W: rush_key = 87;
pub const RUSH_KEY_X: rush_key = 88;
pub const RUSH_KEY_Y: rush_key = 89;
pub const RUSH_KEY_Z: rush_key = 90;
pub const RUSH_KEY_LEFT_BRACKET: rush_key = 91;
pub const RUSH_KEY_BACKSLASH: rush_key = 92;
pub const RUSH_KEY_RIGHT_BRACKET: rush_key = 93;
pub const RUSH_KEY_BACKQUOTE: rush_key = 96;
pub const RUSH_KEY_ESCAPE: rush_key = 97;
pub const RUSH_KEY_ENTER: rush_key = 98;
pub const RUSH_KEY_TAB: rush_key = 99;
pub const RUSH_KEY_BACKSPACE: rush_key = 100;
pub const RUSH_KEY_INSERT: rush_key = 101;
pub const RUSH_KEY_DELETE: rush_key = 102;
pub const RUSH_KEY_RIGHT: rush_key = 103;
pub const RUSH_KEY_LEFT: rush_key = 104;
pub const RUSH_KEY_DOWN: rush_key = 105;
pub const RUSH_KEY_UP: rush_key = 106;
pub const RUSH_KEY_PAGE_UP: rush_key = 107;
pub const RUSH_KEY_PAGE_DOWN: rush_key = 108;
pub const RUSH_KEY_HOME: rush_key = 109;
pub const RUSH_KEY_END: rush_key = 110;
pub const RUSH_KEY_CAPS_LOCK: rush_key = 111;
pub const RUSH_KEY_SCROLL_LOCK: rush_key = 112;
pub const RUSH_KEY_NUM_LOCK: rush_key = 113;
pub const RUSH_KEY_PRINT_SCREEN: rush_key = 114;
pub const RUSH_KEY_PAUSE: rush_key = 115;
pub const RUSH_KEY_F1: rush_key = 116;
pub const RUSH_KEY_F2: rush_key = 117;
pub const RUSH_KEY_F3: rush_key = 118;
pub const RUSH_KEY_F4: rush_key = 119;
pub const RUSH_KEY_F5: rush_key = 120;
pub const RUSH_KEY_F6: rush_key = 121;
pub const RUSH_KEY_F7: rush_key = 122;
pub const RUSH_KEY_F8: rush_key = 123;
pub const RUSH_KEY_F9: rush_key = 124;
pub const RUSH_KEY_F10: rush_key = 125;
pub const RUSH_KEY_F11: rush_key = 126;
pub const RUSH_KEY_F12: rush_key = 127;
pub const RUSH_KEY_F13: rush_key = 128;
pub const RUSH_KEY_F14: rush_key = 129;
pub const RUSH_KEY_F15: rush_key = 130;
pub const RUSH_KEY_F16: rush_key = 131;
pub const RUSH_KEY_F17: rush_key = 132;
pub const RUSH_KEY_F18: rush_key = 133;
pub const RUSH_KEY_F19: rush_key = 134;
pub const RUSH_KEY_F20: rush_key = 135;
pub const RUSH_KEY_F21: rush_key = 136;
pub const RUSH_KEY_F22: rush_key = 137;
pub const RUSH_KEY_F23: rush_key = 138;
pub const RUSH_KEY_F24: rush_key = 139;
pub const RUSH_KEY_F25: rush_key = 140;
pub const RUSH_KEY_KP0: rush_key = 141;
pub const RUSH_KEY_KP1: rush_key = 142;
pub const RUSH_KEY_KP2: rush_key = 143;
pub const RUSH_KEY_KP3: rush_key = 144;
pub const RUSH_KEY_KP4: rush_key = 145;
pub const RUSH_KEY_KP5: rush_key = 146;
pub const RUSH_KEY_KP6: rush_key = 147;
pub const RUSH_KEY_KP7: rush_key = 148;
pub const RUSH_KEY_KP8: rush_key = 149;
pub const RUSH_KEY_KP9: rush_key = 150;
pub const RUSH_KEY_KP_DECIMAL: rush_key = 151;
pub const RUSH_KEY_KP_DIVIDE: rush_key = 152;
pub const RUSH_KEY_KP_MULTIPLY: rush_key = 153;
pub const RUSH_KEY_KP_SUBTRACT: rush_key = 154;
pub const RUSH_KEY_KP_ADD: rush_key = 155;
pub const RUSH_KEY_KP_ENTER: rush_key = 156;
pub const RUSH_KEY_KP_EQUAL: rush_key = 157;
pub const RUSH_KEY_LEFT_SHIFT: rush_key = 158;
pub const RUSH_KEY_LEFT_CONTROL: rush_key = 159;
pub const RUSH_KEY_LEFT_ALT: rush_key = 160;
pub const RUSH_KEY_RIGHT_SHIFT: rush_key = 161;
pub const RUSH_KEY_RIGHT_CONTROL: rush_key = 162;
pub const RUSH_KEY_RIGHT_ALT: rush_key = 163;
pub const RUSH_KEY_COUNT: rush_key = 164;
pub type rush_key = i32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rush_window_keyboard_state {
    pub keys: [bool; 164usize],
}
impl Default for rush_window_keyboard_state {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_window_mouse_state {
    pub buttons: [bool; 10usize],
    pub double_click: bool,
    pub pos: [f32; 2usize],
    pub wheel: [i32; 2usize],
}
pub const RUSH_WINDOW_EVENT_TYPE_UNKNOWN: rush_window_event_type = 0;
pub const RUSH_WINDOW_EVENT_TYPE_KEY_DOWN: rush_window_event_type = 1;
pub const RUSH_WINDOW_EVENT_TYPE_KEY_UP: rush_window_event_type = 2;
pub const RUSH_WINDOW_EVENT_TYPE_RESIZE: rush_window_event_type = 3;
pub const RUSH_WINDOW_EVENT_TYPE_CHAR: rush_window_event_type = 4;
pub const RUSH_WINDOW_EVENT_TYPE_MOUSE_DOWN: rush_window_event_type = 5;
pub const RUSH_WINDOW_EVENT_TYPE_MOUSE_UP: rush_window_event_type = 6;
pub const RUSH_WINDOW_EVENT_TYPE_MOUSE_MOVE: rush_window_event_type = 7;
pub const RUSH_WINDOW_EVENT_TYPE_SCROLL: rush_window_event_type = 8;
pub const RUSH_WINDOW_EVENT_TYPE_COUNT: rush_window_event_type = 9;
pub type rush_window_event_type = i32;
pub const RUSH_WINDOW_EVENT_MASK_KEY_DOWN: rush_window_event_mask = 2;
pub const RUSH_WINDOW_EVENT_MASK_KEY_UP: rush_window_event_mask = 4;
pub const RUSH_WINDOW_EVENT_MASK_RESIZE: rush_window_event_mask = 8;
pub const RUSH_WINDOW_EVENT_MASK_CHAR: rush_window_event_mask = 16;
pub const RUSH_WINDOW_EVENT_MASK_MOUSE_DOWN: rush_window_event_mask = 32;
pub const RUSH_WINDOW_EVENT_MASK_MOUSE_UP: rush_window_event_mask = 64;
pub const RUSH_WINDOW_EVENT_MASK_MOUSE_MOVE: rush_window_event_mask = 128;
pub const RUSH_WINDOW_EVENT_MASK_SCROLL: rush_window_event_mask = 256;
pub const RUSH_WINDOW_EVENT_MASK_KEY: rush_window_event_mask = 6;
pub const RUSH_WINDOW_EVENT_MASK_MOUSE: rush_window_event_mask = 224;
pub const RUSH_WINDOW_EVENT_MASK_ALL: rush_window_event_mask = 65535;
pub type rush_window_event_mask = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rush_window_event {
    pub event_type: rush_window_event_type,
    pub key: rush_key,
    pub character: u32,
    pub modifiers: u32,
    pub width: u32,
    pub height: u32,
    pub pos: [f32; 2usize],
    pub button: u32,
    pub double_click: bool,
    pub scroll: [f32; 2usize],
}
impl Default for rush_window_event {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rush_window {
    _unused: [u8; 0],
}
extern "C" {
    pub fn rush_platform_get_main_window() -> *mut rush_window;
}
extern "C" {
    pub fn rush_window_get_keyboard_state(
        window: *mut rush_window,
    ) -> *const rush_window_keyboard_state;
}
extern "C" {
    pub fn rush_window_get_mouse_state(window: *mut rush_window) -> *const rush_window_mouse_state;
}
extern "C" {
    pub fn rush_window_get_size(window: *mut rush_window) -> rush_vec2;
}
extern "C" {
    pub fn rush_window_get_framebuffer_size(window: *mut rush_window) -> rush_vec2;
}
extern "C" {
    pub fn rush_window_get_resolution_sclae(window: *mut rush_window) -> rush_vec2;
}
extern "C" {
    pub fn rush_window_get_aspect(window: *mut rush_window) -> f32;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rush_window_event_listener {
    _unused: [u8; 0],
}
extern "C" {
    pub fn rush_window_create_listener(
        window: *mut rush_window,
        event_mask: rush_window_event_mask,
    ) -> *mut rush_window_event_listener;
}
extern "C" {
    pub fn rush_window_destroy_listener(listener: *mut rush_window_event_listener);
}
extern "C" {
    pub fn rush_window_event_listener_count(listener: *mut rush_window_event_listener) -> u32;
}
extern "C" {
    pub fn rush_window_event_listener_receive(
        listener: *mut rush_window_event_listener,
        max_count: u32,
        out_events: *mut rush_window_event,
    ) -> u32;
}
extern "C" {
    pub fn rush_window_event_listener_clear(listener: *mut rush_window_event_listener);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rush_gfx_device {
    _unused: [u8; 0],
}
extern "C" {
    pub fn rush_platform_get_device() -> *mut rush_gfx_device;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rush_gfx_context {
    _unused: [u8; 0],
}
extern "C" {
    pub fn rush_platform_get_context() -> *mut rush_gfx_context;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_vertex_format {
    pub handle: u16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_vertex_shader {
    pub handle: u16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_pixel_shader {
    pub handle: u16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_geometry_shader {
    pub handle: u16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_compute_shader {
    pub handle: u16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_mesh_shader {
    pub handle: u16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_ray_tracing_pipeline {
    pub handle: u16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_acceleration_structure {
    pub handle: u16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_texture {
    pub handle: u16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_buffer {
    pub handle: u16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_sampler {
    pub handle: u16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_blend_state {
    pub handle: u16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_depth_stencil_state {
    pub handle: u16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_rasterizer_state {
    pub handle: u16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_technique {
    pub handle: u16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_descriptor_set {
    pub handle: u16,
}
pub const RUSH_GFX_BLEND_PARAM_ZERO: rush_gfx_blend_param = 0;
pub const RUSH_GFX_BLEND_PARAM_ONE: rush_gfx_blend_param = 1;
pub const RUSH_GFX_BLEND_PARAM_SRC_COLOR: rush_gfx_blend_param = 2;
pub const RUSH_GFX_BLEND_PARAM_INV_SRC_COLOR: rush_gfx_blend_param = 3;
pub const RUSH_GFX_BLEND_PARAM_SRC_ALPHA: rush_gfx_blend_param = 4;
pub const RUSH_GFX_BLEND_PARAM_INV_SRC_ALPHA: rush_gfx_blend_param = 5;
pub const RUSH_GFX_BLEND_PARAM_DEST_ALPHA: rush_gfx_blend_param = 6;
pub const RUSH_GFX_BLEND_PARAM_INV_DEST_ALPHA: rush_gfx_blend_param = 7;
pub const RUSH_GFX_BLEND_PARAM_DEST_COLOR: rush_gfx_blend_param = 8;
pub const RUSH_GFX_BLEND_PARAM_INV_DEST_COLOR: rush_gfx_blend_param = 9;
pub type rush_gfx_blend_param = i32;
pub const RUSH_GFX_BLEND_OP_ADD: rush_gfx_blend_op = 0;
pub const RUSH_GFX_BLEND_OP_SUBTRACT: rush_gfx_blend_op = 1;
pub const RUSH_GFX_BLEND_OP_REV_SUBTRACT: rush_gfx_blend_op = 2;
pub const RUSH_GFX_BLEND_OP_MIN: rush_gfx_blend_op = 3;
pub const RUSH_GFX_BLEND_OP_MAX: rush_gfx_blend_op = 4;
pub type rush_gfx_blend_op = i32;
pub const RUSH_GFX_TEXTURE_FILTER_POINT: rush_gfx_texture_filter = 0;
pub const RUSH_GFX_TEXTURE_FILTER_LINEAR: rush_gfx_texture_filter = 1;
pub const RUSH_GFX_TEXTURE_FILTER_ANISOTROPIC: rush_gfx_texture_filter = 2;
pub type rush_gfx_texture_filter = i32;
pub const RUSH_GFX_TEXTURE_WRAP_REPEAT: rush_gfx_texture_wrap = 0;
pub const RUSH_GFX_TEXTURE_WRAP_MIRROR: rush_gfx_texture_wrap = 1;
pub const RUSH_GFX_TEXTURE_WRAP_CLAMP: rush_gfx_texture_wrap = 2;
pub type rush_gfx_texture_wrap = i32;
pub const RUSH_GFX_COMPARE_FUNC_NEVER: rush_gfx_compare_func = 0;
pub const RUSH_GFX_COMPARE_FUNC_LESS: rush_gfx_compare_func = 1;
pub const RUSH_GFX_COMPARE_FUNC_EQUAL: rush_gfx_compare_func = 2;
pub const RUSH_GFX_COMPARE_FUNC_LESS_EQUAL: rush_gfx_compare_func = 3;
pub const RUSH_GFX_COMPARE_FUNC_GREATER: rush_gfx_compare_func = 4;
pub const RUSH_GFX_COMPARE_FUNC_NOT_EQUAL: rush_gfx_compare_func = 5;
pub const RUSH_GFX_COMPARE_FUNC_GREATER_EQUAL: rush_gfx_compare_func = 6;
pub const RUSH_GFX_COMPARE_FUNC_ALWAYS: rush_gfx_compare_func = 7;
pub type rush_gfx_compare_func = i32;
pub const RUSH_GFX_FILL_MODE_SOLID: rush_gfx_fill_mode = 0;
pub const RUSH_GFX_FILL_MODE_WIREFRAME: rush_gfx_fill_mode = 1;
pub type rush_gfx_fill_mode = i32;
pub const RUSH_GFX_CULL_MODE_NONE: rush_gfx_cull_mode = 0;
pub const RUSH_GFX_CULL_MODE_CW: rush_gfx_cull_mode = 1;
pub const RUSH_GFX_CULL_MODE_CCW: rush_gfx_cull_mode = 2;
pub type rush_gfx_cull_mode = i32;
pub const RUSH_GFX_EMBEDDED_SHADER_PRIMITIVE_PLAIN_PS: rush_gfx_embedded_shader_type = 0;
pub const RUSH_GFX_EMBEDDED_SHADER_PRIMITIVE_TEXTURED_PS: rush_gfx_embedded_shader_type = 1;
pub const RUSH_GFX_EMBEDDED_SHADER_PRIMITIVE_2D_VS: rush_gfx_embedded_shader_type = 2;
pub const RUSH_GFX_EMBEDDED_SHADER_PRIMITIVE_3D_VS: rush_gfx_embedded_shader_type = 3;
pub type rush_gfx_embedded_shader_type = i32;
pub const RUSH_GFX_PRIMITIVE_POINT_LIST: rush_gfx_primitive_type = 0;
pub const RUSH_GFX_PRIMITIVE_LINE_LIST: rush_gfx_primitive_type = 1;
pub const RUSH_GFX_PRIMITIVE_LINE_STRIP: rush_gfx_primitive_type = 2;
pub const RUSH_GFX_PRIMITIVE_TRIANGLE_LIST: rush_gfx_primitive_type = 3;
pub const RUSH_GFX_PRIMITIVE_TRIANGLE_STRIP: rush_gfx_primitive_type = 4;
pub type rush_gfx_primitive_type = i32;
pub const RUSH_GFX_PASS_NONE: rush_gfx_pass_flags = 0;
pub const RUSH_GFX_PASS_CLEAR_COLOR: rush_gfx_pass_flags = 1;
pub const RUSH_GFX_PASS_CLEAR_DEPTH_STENCIL: rush_gfx_pass_flags = 2;
pub const RUSH_GFX_PASS_DISCARD_COLOR: rush_gfx_pass_flags = 4;
pub type rush_gfx_pass_flags = i32;
pub const RUSH_GFX_SHADER_SOURCE_UNKNOWN: rush_gfx_shader_source_type = 0;
pub const RUSH_GFX_SHADER_SOURCE_SPV: rush_gfx_shader_source_type = 1;
pub const RUSH_GFX_SHADER_SOURCE_GLSL: rush_gfx_shader_source_type = 2;
pub const RUSH_GFX_SHADER_SOURCE_HLSL: rush_gfx_shader_source_type = 3;
pub const RUSH_GFX_SHADER_SOURCE_DXBC: rush_gfx_shader_source_type = 4;
pub const RUSH_GFX_SHADER_SOURCE_DXIL: rush_gfx_shader_source_type = 5;
pub const RUSH_GFX_SHADER_SOURCE_MSL: rush_gfx_shader_source_type = 6;
pub type rush_gfx_shader_source_type = i32;
pub const RUSH_GFX_FORMAT_UNKNOWN: rush_gfx_format = 0;
pub const RUSH_GFX_FORMAT_D24_UNORM_S8_UINT: rush_gfx_format = 1;
pub const RUSH_GFX_FORMAT_D24_UNORM_X8: rush_gfx_format = 2;
pub const RUSH_GFX_FORMAT_D32_FLOAT: rush_gfx_format = 3;
pub const RUSH_GFX_FORMAT_D32_FLOAT_S8_UINT: rush_gfx_format = 4;
pub const RUSH_GFX_FORMAT_R8_UNORM: rush_gfx_format = 5;
pub const RUSH_GFX_FORMAT_R16_FLOAT: rush_gfx_format = 6;
pub const RUSH_GFX_FORMAT_R16_UINT: rush_gfx_format = 7;
pub const RUSH_GFX_FORMAT_R32_FLOAT: rush_gfx_format = 8;
pub const RUSH_GFX_FORMAT_R32_UINT: rush_gfx_format = 9;
pub const RUSH_GFX_FORMAT_RG8_UNORM: rush_gfx_format = 10;
pub const RUSH_GFX_FORMAT_RG16_FLOAT: rush_gfx_format = 11;
pub const RUSH_GFX_FORMAT_RG32_FLOAT: rush_gfx_format = 12;
pub const RUSH_GFX_FORMAT_RGB32_FLOAT: rush_gfx_format = 13;
pub const RUSH_GFX_FORMAT_RGB8_UNORM: rush_gfx_format = 14;
pub const RUSH_GFX_FORMAT_RGBA16_FLOAT: rush_gfx_format = 15;
pub const RUSH_GFX_FORMAT_RGBA16_UNORM: rush_gfx_format = 16;
pub const RUSH_GFX_FORMAT_RGBA32_FLOAT: rush_gfx_format = 17;
pub const RUSH_GFX_FORMAT_RGBA8_UNORM: rush_gfx_format = 18;
pub const RUSH_GFX_FORMAT_RGBA8_SRGB: rush_gfx_format = 19;
pub const RUSH_GFX_FORMAT_BGRA8_UNORM: rush_gfx_format = 20;
pub const RUSH_GFX_FORMAT_BGRA8_SRGB: rush_gfx_format = 21;
pub const RUSH_GFX_FORMAT_BC1_UNORM: rush_gfx_format = 22;
pub const RUSH_GFX_FORMAT_BC1_UNORM_SRGB: rush_gfx_format = 23;
pub const RUSH_GFX_FORMAT_BC3_UNORM: rush_gfx_format = 24;
pub const RUSH_GFX_FORMAT_BC3_UNORM_SRGB: rush_gfx_format = 25;
pub const RUSH_GFX_FORMAT_BC4_UNORM: rush_gfx_format = 26;
pub const RUSH_GFX_FORMAT_BC5_UNORM: rush_gfx_format = 27;
pub const RUSH_GFX_FORMAT_BC6H_UFLOAT: rush_gfx_format = 28;
pub const RUSH_GFX_FORMAT_BC6H_SFLOAT: rush_gfx_format = 29;
pub const RUSH_GFX_FORMAT_BC7_UNORM: rush_gfx_format = 30;
pub const RUSH_GFX_FORMAT_BC7_UNORM_SRGB: rush_gfx_format = 31;
pub type rush_gfx_format = i32;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_color_target {
    pub target: rush_gfx_texture,
    pub clear_color: rush_color_rgba,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_depth_target {
    pub target: rush_gfx_texture,
    pub clear_depth: f32,
    pub clear_stencil: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rush_gfx_capability {
    pub api_name: *const ::std::os::raw::c_char,
}
impl Default for rush_gfx_capability {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_stats {
    pub draw_calls: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_viewport {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub depth_min: f32,
    pub depth_max: f32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_rect {
    pub left: ::std::os::raw::c_int,
    pub top: ::std::os::raw::c_int,
    pub right: ::std::os::raw::c_int,
    pub bottom: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rush_gfx_shader_source {
    pub type_: rush_gfx_shader_source_type,
    pub entry: *const ::std::os::raw::c_char,
    pub data: *const ::std::os::raw::c_void,
    pub size_bytes: u32,
}
impl Default for rush_gfx_shader_source {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub const RUSH_GFX_VERTEX_SEMANTIC_UNUSED: rush_gfx_vertex_semantic = 0;
pub const RUSH_GFX_VERTEX_SEMANTIC_POSITION: rush_gfx_vertex_semantic = 1;
pub const RUSH_GFX_VERTEX_SEMANTIC_TEXCOORD: rush_gfx_vertex_semantic = 2;
pub const RUSH_GFX_VERTEX_SEMANTIC_COLOR: rush_gfx_vertex_semantic = 3;
pub const RUSH_GFX_VERTEX_SEMANTIC_NORMAL: rush_gfx_vertex_semantic = 4;
pub const RUSH_GFX_VERTEX_SEMANTIC_TANGENTU: rush_gfx_vertex_semantic = 5;
pub const RUSH_GFX_VERTEX_SEMANTIC_TANGENTV: rush_gfx_vertex_semantic = 6;
pub const RUSH_GFX_VERTEX_SEMANTIC_INSTANCEDATA: rush_gfx_vertex_semantic = 7;
pub const RUSH_GFX_VERTEX_SEMANTIC_BONEINDEX: rush_gfx_vertex_semantic = 8;
pub const RUSH_GFX_VERTEX_SEMANTIC_BONEWEIGHT: rush_gfx_vertex_semantic = 9;
pub type rush_gfx_vertex_semantic = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rush_gfx_vertex_element {
    pub semantic: rush_gfx_vertex_semantic,
    pub index: u32,
    pub format: rush_gfx_format,
    pub stream: u32,
}
impl Default for rush_gfx_vertex_element {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub const RUSH_GFX_USAGE_SHADER_RESOURCE: rush_gfx_usage_flags = 1;
pub const RUSH_GFX_USAGE_RENDER_TARGET: rush_gfx_usage_flags = 2;
pub const RUSH_GFX_USAGE_DEPTH_STENCIL: rush_gfx_usage_flags = 4;
pub const RUSH_GFX_USAGE_STORAGE_IMAGE: rush_gfx_usage_flags = 8;
pub const RUSH_GFX_USAGE_TRANSFER_SRC: rush_gfx_usage_flags = 16;
pub const RUSH_GFX_USAGE_TRANSFER_DST: rush_gfx_usage_flags = 32;
pub type rush_gfx_usage_flags = i32;
pub const RUSH_GFX_TEXTURE_TYPE_1D: rush_gfx_texture_type = 0;
pub const RUSH_GFX_TEXTURE_TYPE_1D_ARRAY: rush_gfx_texture_type = 1;
pub const RUSH_GFX_TEXTURE_TYPE_2D: rush_gfx_texture_type = 2;
pub const RUSH_GFX_TEXTURE_TYPE_2D_ARRAY: rush_gfx_texture_type = 3;
pub const RUSH_GFX_TEXTURE_TYPE_3D: rush_gfx_texture_type = 4;
pub const RUSH_GFX_TEXTURE_TYPE_CUBE: rush_gfx_texture_type = 5;
pub const RUSH_GFX_TEXTURE_TYPE_CUBE_ARRAY: rush_gfx_texture_type = 6;
pub type rush_gfx_texture_type = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rush_gfx_texture_desc {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub mips: u32,
    pub samples: u32,
    pub format: rush_gfx_format,
    pub texture_type: rush_gfx_texture_type,
    pub usage: rush_gfx_usage_flags,
    pub debug_name: *const ::std::os::raw::c_char,
}
impl Default for rush_gfx_texture_desc {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rush_gfx_depth_stencil_desc {
    pub compare_func: rush_gfx_compare_func,
    pub enable: bool,
    pub write_enable: bool,
}
impl Default for rush_gfx_depth_stencil_desc {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rush_gfx_rasterizer_desc {
    pub fill_mode: rush_gfx_fill_mode,
    pub cull_mode: rush_gfx_cull_mode,
    pub depth_bias: f32,
    pub depth_bias_slope_scale: f32,
}
impl Default for rush_gfx_rasterizer_desc {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub const RUSH_GFX_BUFFER_FLAG_NONE: rush_gfx_buffer_flags = 0;
pub const RUSH_GFX_BUFFER_FLAG_VERTEX: rush_gfx_buffer_flags = 1;
pub const RUSH_GFX_BUFFER_FLAG_INDEX: rush_gfx_buffer_flags = 2;
pub const RUSH_GFX_BUFFER_FLAG_CONSTANT: rush_gfx_buffer_flags = 4;
pub const RUSH_GFX_BUFFER_FLAG_STORAGE: rush_gfx_buffer_flags = 8;
pub const RUSH_GFX_BUFFER_FLAG_TEXEL: rush_gfx_buffer_flags = 16;
pub const RUSH_GFX_BUFFER_FLAG_INDIRECT_ARGS: rush_gfx_buffer_flags = 32;
pub const RUSH_GFX_BUFFER_FLAG_RAYTRACING: rush_gfx_buffer_flags = 64;
pub const RUSH_GFX_BUFFER_FLAG_TRANSIENT: rush_gfx_buffer_flags = 1073741824;
pub type rush_gfx_buffer_flags = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rush_gfx_buffer_desc {
    pub flags: rush_gfx_buffer_flags,
    pub format: rush_gfx_format,
    pub stride: u32,
    pub count: u32,
    pub host_visible: bool,
}
impl Default for rush_gfx_buffer_desc {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rush_gfx_mapped_buffer {
    pub data: *mut ::std::os::raw::c_void,
    pub size: u32,
    pub handle: rush_gfx_buffer,
}
impl Default for rush_gfx_mapped_buffer {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct rush_gfx_spec_constant {
    pub id: u32,
    pub offset: u32,
    pub size: u32,
}
pub const RUSH_GFX_STAGE_FLAG_NONE: rush_gfx_stage_flags = 0;
pub const RUSH_GFX_STAGE_FLAG_VERTEX: rush_gfx_stage_flags = 1;
pub const RUSH_GFX_STAGE_FLAG_GEOMETRY: rush_gfx_stage_flags = 2;
pub const RUSH_GFX_STAGE_FLAG_PIXEL: rush_gfx_stage_flags = 4;
pub const RUSH_GFX_STAGE_FLAG_HULL: rush_gfx_stage_flags = 8;
pub const RUSH_GFX_STAGE_FLAG_DOMAIN: rush_gfx_stage_flags = 16;
pub const RUSH_GFX_STAGE_FLAG_COMPUTE: rush_gfx_stage_flags = 32;
pub const RUSH_GFX_STAGE_FLAG_MESH: rush_gfx_stage_flags = 64;
pub const RUSH_GFX_STAGE_FLAG_RAYTRACING: rush_gfx_stage_flags = 128;
pub type rush_gfx_stage_flags = i32;
pub const RUSH_GFX_DESCRIPTOR_SET_FLAG_NONE: rush_gfx_descriptor_set_flags = 0;
pub const RUSH_GFX_DESCRIPTOR_SET_FLAG_TEXTURE_ARRAY: rush_gfx_descriptor_set_flags = 1;
pub type rush_gfx_descriptor_set_flags = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rush_gfx_descriptor_set_desc {
    pub constant_buffers: u16,
    pub samplers: u16,
    pub textures: u16,
    pub rw_images: u16,
    pub rw_buffers: u16,
    pub rw_typed_buffers: u16,
    pub acceleration_structures: u16,
    pub stage_flags: rush_gfx_stage_flags,
    pub flags: rush_gfx_descriptor_set_flags,
}
impl Default for rush_gfx_descriptor_set_desc {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rush_gfx_shader_bindings_desc {
    pub descriptor_sets: *const rush_gfx_descriptor_set_desc,
    pub descriptor_set_count: u32,
    pub use_default_descriptor_set: bool,
}
impl Default for rush_gfx_shader_bindings_desc {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rush_gfx_technique_desc {
    pub cs: rush_gfx_compute_shader,
    pub ps: rush_gfx_pixel_shader,
    pub gs: rush_gfx_geometry_shader,
    pub vs: rush_gfx_vertex_shader,
    pub ms: rush_gfx_mesh_shader,
    pub vf: rush_gfx_vertex_format,
    pub bindings: rush_gfx_shader_bindings_desc,
    pub work_group_size: [u16; 3usize],
    pub spec_constant_count: u32,
    pub spec_constants: *const rush_gfx_spec_constant,
    pub spec_data: *const ::std::os::raw::c_void,
    pub spec_data_size: u32,
}
impl Default for rush_gfx_technique_desc {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rush_gfx_texture_data {
    pub offset: u64,
    pub pixels: *const ::std::os::raw::c_void,
    pub mip: u32,
    pub slice: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}
impl Default for rush_gfx_texture_data {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rush_gfx_blend_state_desc {
    pub src: rush_gfx_blend_param,
    pub dst: rush_gfx_blend_param,
    pub op: rush_gfx_blend_op,
    pub alpha_src: rush_gfx_blend_param,
    pub alpha_dst: rush_gfx_blend_param,
    pub alpha_op: rush_gfx_blend_op,
    pub alpha_separate: bool,
    pub enable: bool,
}
impl Default for rush_gfx_blend_state_desc {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rush_gfx_sampler_desc {
    pub filter_min: rush_gfx_texture_filter,
    pub filter_mag: rush_gfx_texture_filter,
    pub filter_mip: rush_gfx_texture_filter,
    pub wrap_u: rush_gfx_texture_wrap,
    pub wrap_v: rush_gfx_texture_wrap,
    pub wrap_w: rush_gfx_texture_wrap,
    pub compare_func: rush_gfx_compare_func,
    pub compare_enable: bool,
    pub anisotropy: f32,
    pub mip_lod_bias: f32,
}
impl Default for rush_gfx_sampler_desc {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    pub fn rush_gfx_set_present_interval(interval: u32);
}
extern "C" {
    pub fn rush_gfx_finish();
}
extern "C" {
    pub fn rush_gfx_get_capability() -> rush_gfx_capability;
}
extern "C" {
    pub fn rush_gfx_get_stats() -> rush_gfx_stats;
}
extern "C" {
    pub fn rush_gfx_reset_stats();
}
extern "C" {
    pub fn rush_gfx_create_vertex_format(
        elements: *const rush_gfx_vertex_element,
        count: u32,
    ) -> rush_gfx_vertex_format;
}
extern "C" {
    pub fn rush_gfx_create_vertex_shader(
        code: *const rush_gfx_shader_source,
    ) -> rush_gfx_vertex_shader;
}
extern "C" {
    pub fn rush_gfx_create_pixel_shader(
        code: *const rush_gfx_shader_source,
    ) -> rush_gfx_pixel_shader;
}
extern "C" {
    pub fn rush_gfx_create_geometry_shader(
        code: *const rush_gfx_shader_source,
    ) -> rush_gfx_geometry_shader;
}
extern "C" {
    pub fn rush_gfx_create_compute_shader(
        code: *const rush_gfx_shader_source,
    ) -> rush_gfx_compute_shader;
}
extern "C" {
    pub fn rush_gfx_create_technique(desc: *const rush_gfx_technique_desc) -> rush_gfx_technique;
}
extern "C" {
    pub fn rush_gfx_create_texture(
        tex: *const rush_gfx_texture_desc,
        data: *const rush_gfx_texture_data,
        count: u32,
        pixels: *const ::std::os::raw::c_void,
    ) -> rush_gfx_texture;
}
extern "C" {
    pub fn rush_gfx_create_blend_state(
        desc: *const rush_gfx_blend_state_desc,
    ) -> rush_gfx_blend_state;
}
extern "C" {
    pub fn rush_gfx_create_sampler_state(desc: *const rush_gfx_sampler_desc) -> rush_gfx_sampler;
}
extern "C" {
    pub fn rush_gfx_create_depth_stencil_state(
        desc: *const rush_gfx_depth_stencil_desc,
    ) -> rush_gfx_depth_stencil_state;
}
extern "C" {
    pub fn rush_gfx_create_rasterizer_state(
        desc: *const rush_gfx_rasterizer_desc,
    ) -> rush_gfx_rasterizer_state;
}
extern "C" {
    pub fn rush_gfx_create_buffer(
        desc: *const rush_gfx_buffer_desc,
        data: *const ::std::os::raw::c_void,
    ) -> rush_gfx_buffer;
}
extern "C" {
    pub fn rush_gfx_release_vertex_format(h: rush_gfx_vertex_format);
}
extern "C" {
    pub fn rush_gfx_release_vertex_shader(h: rush_gfx_vertex_shader);
}
extern "C" {
    pub fn rush_gfx_release_pixel_shader(h: rush_gfx_pixel_shader);
}
extern "C" {
    pub fn rush_gfx_release_geometry_shader(h: rush_gfx_geometry_shader);
}
extern "C" {
    pub fn rush_gfx_release_compute_shader(h: rush_gfx_compute_shader);
}
extern "C" {
    pub fn rush_gfx_release_mesh_shader(h: rush_gfx_mesh_shader);
}
extern "C" {
    pub fn rush_gfx_release_ray_tracing_pipeline(h: rush_gfx_ray_tracing_pipeline);
}
extern "C" {
    pub fn rush_gfx_release_acceleration_structure(h: rush_gfx_acceleration_structure);
}
extern "C" {
    pub fn rush_gfx_release_technique(h: rush_gfx_technique);
}
extern "C" {
    pub fn rush_gfx_release_texture(h: rush_gfx_texture);
}
extern "C" {
    pub fn rush_gfx_release_blend_state(h: rush_gfx_blend_state);
}
extern "C" {
    pub fn rush_gfx_release_sampler(h: rush_gfx_sampler);
}
extern "C" {
    pub fn rush_gfx_release_depth_stencil_state(h: rush_gfx_depth_stencil_state);
}
extern "C" {
    pub fn rush_gfx_release_rasterizer_state(h: rush_gfx_rasterizer_state);
}
extern "C" {
    pub fn rush_gfx_release_buffer(h: rush_gfx_buffer);
}
extern "C" {
    pub fn rush_gfx_release_descriptor_set(h: rush_gfx_descriptor_set);
}
extern "C" {
    pub fn rush_gfx_unmap_buffer(in_mapped_buffer: *const rush_gfx_mapped_buffer);
}
extern "C" {
    pub fn rush_gfx_update_buffer(
        ctx: *mut rush_gfx_context,
        h: rush_gfx_buffer,
        data: *const ::std::os::raw::c_void,
        size: u32,
    );
}
extern "C" {
    pub fn rush_gfx_begin_update_buffer(
        ctx: *mut rush_gfx_context,
        h: rush_gfx_buffer,
        size: u32,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn rush_gfx_end_update_buffer(ctx: *mut rush_gfx_context, h: rush_gfx_buffer);
}
extern "C" {
    pub fn rush_gfx_begin_pass(
        ctx: *mut rush_gfx_context,
        color_count: u32,
        color: *const rush_gfx_color_target,
        depth: *const rush_gfx_depth_target,
        flags: rush_gfx_pass_flags,
    );
}
extern "C" {
    pub fn rush_gfx_end_pass(ctx: *mut rush_gfx_context);
}
extern "C" {
    pub fn rush_gfx_set_viewport(ctx: *mut rush_gfx_context, _viewport: *const rush_gfx_viewport);
}
extern "C" {
    pub fn rush_gfx_set_scissor_rect(ctx: *mut rush_gfx_context, rect: *const rush_gfx_rect);
}
extern "C" {
    pub fn rush_gfx_set_technique(ctx: *mut rush_gfx_context, h: rush_gfx_technique);
}
extern "C" {
    pub fn rush_gfx_set_primitive(ctx: *mut rush_gfx_context, type_: rush_gfx_primitive_type);
}
extern "C" {
    pub fn rush_gfx_set_index_stream(ctx: *mut rush_gfx_context, h: rush_gfx_buffer);
}
extern "C" {
    pub fn rush_gfx_set_vertex_stream(ctx: *mut rush_gfx_context, idx: u32, h: rush_gfx_buffer);
}
extern "C" {
    pub fn rush_gfx_set_texture(ctx: *mut rush_gfx_context, idx: u32, h: rush_gfx_texture);
}
extern "C" {
    pub fn rush_gfx_set_sampler(ctx: *mut rush_gfx_context, idx: u32, h: rush_gfx_sampler);
}
extern "C" {
    pub fn rush_gfx_set_storage_image(ctx: *mut rush_gfx_context, idx: u32, h: rush_gfx_texture);
}
extern "C" {
    pub fn rush_gfx_set_storage_buffer(ctx: *mut rush_gfx_context, idx: u32, h: rush_gfx_buffer);
}
extern "C" {
    pub fn rush_gfx_set_blend_state(ctx: *mut rush_gfx_context, h: rush_gfx_blend_state);
}
extern "C" {
    pub fn rush_gfx_set_depth_stencil_state(
        ctx: *mut rush_gfx_context,
        h: rush_gfx_depth_stencil_state,
    );
}
extern "C" {
    pub fn rush_gfx_set_rasterizer_state(ctx: *mut rush_gfx_context, h: rush_gfx_rasterizer_state);
}
extern "C" {
    pub fn rush_gfx_set_constant_buffer(
        ctx: *mut rush_gfx_context,
        idx: u32,
        h: rush_gfx_buffer,
        offset: u32,
    );
}
extern "C" {
    pub fn rush_gfx_resolve_image(
        ctx: *mut rush_gfx_context,
        src: rush_gfx_texture,
        dst: rush_gfx_texture,
    );
}
extern "C" {
    pub fn rush_gfx_dispatch(ctx: *mut rush_gfx_context, size_x: u32, size_y: u32, size_z: u32);
}
extern "C" {
    pub fn rush_gfx_dispatch2(
        ctx: *mut rush_gfx_context,
        size_x: u32,
        size_y: u32,
        size_z: u32,
        push_constants: *const ::std::os::raw::c_void,
        push_constants_size: u32,
    );
}
extern "C" {
    pub fn rush_gfx_draw(ctx: *mut rush_gfx_context, first_vertex: u32, vertex_count: u32);
}
extern "C" {
    pub fn rush_gfx_draw_indexed(
        ctx: *mut rush_gfx_context,
        index_count: u32,
        first_index: u32,
        base_vertex: u32,
        vertex_count: u32,
    );
}
extern "C" {
    pub fn rush_gfx_draw_indexed2(
        ctx: *mut rush_gfx_context,
        index_count: u32,
        first_index: u32,
        base_vertex: u32,
        vertex_count: u32,
        push_constants: *const ::std::os::raw::c_void,
        push_constants_size: u32,
    );
}
extern "C" {
    pub fn rush_gfx_draw_indexed_instanced(
        ctx: *mut rush_gfx_context,
        index_count: u32,
        first_index: u32,
        base_vertex: u32,
        vertex_count: u32,
        instance_count: u32,
        instance_offset: u32,
    );
}
extern "C" {
    pub fn rush_gfx_draw_indexed_indirect(
        ctx: *mut rush_gfx_context,
        args_buffer: rush_gfx_buffer,
        args_buffer_offset: u32,
        draw_count: u32,
    );
}
extern "C" {
    pub fn rush_gfx_dispatch_indirect(
        ctx: *mut rush_gfx_context,
        args_buffer: rush_gfx_buffer,
        args_buffer_offset: u32,
        push_constants: *const ::std::os::raw::c_void,
        push_constants_size: u32,
    );
}
extern "C" {
    pub fn rush_gfx_push_marker(ctx: *mut rush_gfx_context, marker: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn rush_gfx_pop_marker(ctx: *mut rush_gfx_context);
}
extern "C" {
    pub fn rush_gfx_begin_timer(ctx: *mut rush_gfx_context, timestamp_id: u32);
}
extern "C" {
    pub fn rush_gfx_end_timer(ctx: *mut rush_gfx_context, timestamp_id: u32);
}
extern "C" {
    pub fn rush_gfx_get_embedded_shader(
        type_: rush_gfx_embedded_shader_type,
    ) -> rush_gfx_shader_source;
}
extern "C" {
    pub fn rush_embedded_font_blit_6x8(
        output: *mut u32,
        output_offset_pixels: u32,
        width: u32,
        color: u32,
        text: *const ::std::os::raw::c_char,
    );
}
