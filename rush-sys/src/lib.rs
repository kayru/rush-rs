#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate const_cstr;

include!("rush_ffi.rs");

const_cstr! {
    DEFAULT_APP_NAME = "RushApp";
}

impl rush_app_config {
    pub fn new() -> rush_app_config {
        rush_app_config {
            name: DEFAULT_APP_NAME.as_ptr(),
            vsync: 1,
            width: 640,
            height: 480,

            max_width: 0,
            max_height: 0,

            fullscreen: false,
            resizable: false,
            debug: false,
            warp: false,
            minimize_latency: false,

            argc: 0,
            argv: std::ptr::null_mut(),

            user_data: std::ptr::null_mut(),

            on_startup: None,
            on_update: None,
            on_shutdown: None,
        }
    }
}

impl Default for rush_gfx_descriptor_set_desc {
    fn default() -> Self {
        rush_gfx_descriptor_set_desc{
            constant_buffers: 0,
            samplers: 0,
            textures: 0,
            rw_images: 0,
            rw_buffers: 0,
            rw_typed_buffers: 0,
            acceleration_structures: 0,
            stage_flags: 0 as rush_gfx_stage_flags,
            flags: 0 as rush_gfx_descriptor_set_flags,
        }
    }
}

impl Default for rush_gfx_technique_desc {
    fn default() -> Self {
        rush_gfx_technique_desc{
            cs: rush_gfx_compute_shader{handle: 0},
            ps: rush_gfx_pixel_shader{handle: 0},
            gs: rush_gfx_geometry_shader{handle: 0},
            vs: rush_gfx_vertex_shader{handle: 0},
            ms: rush_gfx_mesh_shader{handle: 0},
            vf: rush_gfx_vertex_format{handle: 0},
            bindings: rush_gfx_shader_bindings_desc{
                descriptor_sets: std::ptr::null() as *const rush_gfx_descriptor_set_desc,
                descriptor_set_count: 0,
                use_default_descriptor_set: true,
            },
            work_group_size: [0,0,0],
            spec_constant_count: 0,
            spec_constants: std::ptr::null() as *const rush_gfx_spec_constant,
            spec_data: std::ptr::null() as *const ::std::os::raw::c_void,
            spec_data_size: 0,
        }
    }
}
