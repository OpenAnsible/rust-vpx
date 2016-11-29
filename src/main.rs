#![allow(dead_code, unused_imports, unused_variables, unused_mut, unused_must_use, unreachable_code, non_snake_case, non_camel_case_types)]

use std::os::raw::c_int;
use std::ffi::CString;

use std::{ env, ptr, mem, slice };
use std::fs::{ File, OpenOptions };

use std::convert::AsMut;
use std::default::Default;
use std::io::{Write, Read};

pub mod ffi;

use ffi::vp8::{
    vpx_color_space, vpx_color_space_t, vpx_color_range, vpx_color_range_t,
    vpx_img_fmt, vpx_img_fmt_t,
    vpx_image, vpx_image_t, vpx_image_rect, vpx_image_rect_t, 
    vpx_codec_ctx, vpx_codec_ctx_t, 

    vpx_ref_frame, vpx_ref_frame_t,

    vpx_img_alloc, vpx_img_wrap, vpx_img_set_rect, vpx_img_flip, vpx_img_free,
    vpx_codec_destroy, 
};

fn main (){
    unsafe {
        let width: c_int  = 1440;
        let height: c_int = 900;
    }
}