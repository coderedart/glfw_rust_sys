//========================================================================
// Simple multi-window example
// Copyright (c) Camilla Löwy <elmindreda@glfw.org>
//
// This software is provided 'as-is', without any express or implied
// warranty. In no event will the authors be held liable for any damages
// arising from the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not
//    claim that you wrote the original software. If you use this software
//    in a product, an acknowledgment in the product documentation would
//    be appreciated but is not required.
//
// 2. Altered source versions must be plainly marked as such, and must not
//    be misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source
//    distribution.
//
//========================================================================
use std::ffi::{CStr, CString};

use glfw_rust_sys::*;

use gl11::*;
fn main() {
    unsafe {
        let mut xpos = 0i32;
        let mut ypos = 0i32;
        let mut height = 0i32;
        let mut description = std::ptr::null();
        let mut windows = [std::ptr::null_mut(); 4];

        if glfwInit() != GLFW_TRUE {
            glfwGetError(&mut description);

            panic!(
                "Error: {:?}\n",
                description
                    .is_null()
                    .then_some(c"")
                    .unwrap_or_else(|| CStr::from_ptr(description))
            );
        }

        glfwWindowHint(GLFW_DECORATED, GLFW_FALSE);

        glfwGetMonitorWorkarea(
            glfwGetPrimaryMonitor(),
            &mut xpos,
            &mut ypos,
            std::ptr::null_mut(),
            &mut height,
        );

        for i in 0..4usize {
            let size = height / 5;
            struct Color {
                r: f32,
                g: f32,
                b: f32,
            }
            let colors = [
                Color {
                    r: 0.95,
                    g: 0.32,
                    b: 0.11,
                },
                Color {
                    r: 0.50,
                    g: 0.80,
                    b: 0.16,
                },
                Color {
                    r: 0.,
                    g: 0.68,
                    b: 0.94,
                },
                Color {
                    r: 0.98,
                    g: 0.74,
                    b: 0.04,
                },
            ];

            if i > 0 {
                glfwWindowHint(GLFW_FOCUS_ON_SHOW, GLFW_FALSE);
            }

            glfwWindowHint(GLFW_POSITION_X, xpos + size * (1 + (i & 1) as i32));
            glfwWindowHint(GLFW_POSITION_Y, ypos + size * (1 + (i >> 1) as i32));

            let win = glfwCreateWindow(
                size,
                size,
                c"Multi-Window Example".as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            if win.is_null() {
                glfwGetError(&mut description);

                println!(
                    "Error: {:?}\n",
                    description
                        .is_null()
                        .then_some(c"")
                        .unwrap_or_else(|| CStr::from_ptr(description))
                );
                glfwTerminate();
                panic!();
            }

            glfwSetInputMode(win, GLFW_STICKY_KEYS, GLFW_TRUE);

            glfwMakeContextCurrent(win);

            gl11::load_with(|s| {
                let s = CString::new(s).unwrap();
                let result = glfwGetProcAddress(s.as_ptr())
                    .map(|p| p as _)
                    .unwrap_or(std::ptr::null());
                std::mem::drop(s);
                result
            });

            ClearColor(colors[i].r, colors[i].g, colors[i].b, 1.0);
            windows[i] = win;
        }
        'outer: loop {
            for win in windows.iter().copied() {
                glfwMakeContextCurrent(win);
                Clear(COLOR_BUFFER_BIT);
                glfwSwapBuffers(win);

                if glfwWindowShouldClose(win) == GLFW_TRUE
                    || glfwGetKey(win, GLFW_KEY_ESCAPE) == GLFW_TRUE
                {
                    break 'outer;
                }
            }
            glfwWaitEvents();
        }
        // shutdown
        glfwMakeContextCurrent(std::ptr::null_mut());
        for w in windows.into_iter() {
            glfwDestroyWindow(w);
        }
        glfwTerminate();
    }
}
