use crate::ffi::*;
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub revision: i32,
}
impl Default for Version {
    fn default() -> Self {
        Self::get_library_version()
    }
}
impl Version {
    /// The version of glfw header from which we generated the bindings
    pub fn get_header_version() -> Version {
        Self {
            major: GLFW_VERSION_MAJOR,
            minor: GLFW_VERSION_MINOR,
            revision: GLFW_VERSION_REVISION,
        }
    }
    /// Returns the version of the GLFW library by calling [glfwGetVersion]
    #[doc(alias = "glfwGetVersion")]
    pub fn get_library_version() -> Version {
        let mut major = 0;
        let mut minor = 0;
        let mut revision = 0;
        unsafe {
            glfwGetVersion(&mut major, &mut minor, &mut revision);
        }
        Version {
            major,
            minor,
            revision,
        }
    }
    /// gets glfw version as string using [glfwGetVersionString]
    /// WARNING: If you want glfw's version, prefer using [Self::get_library_version]
    #[doc(alias = "glfwGetVersionString")]
    pub fn get_version_str() -> &'static str {
        unsafe {
            let p = glfwGetVersionString();
            assert!(!p.is_null());
            std::ffi::CStr::from_ptr(p)
                .to_str()
                .expect("glfwGetVersionString is not UTF-8")
        }
    }
}
