// modules

// pub exports
pub type VideoDriverType = OpenGL21;

// priv uses
use engine::VideoDriver;

// priv data structures
struct OpenGL21;

impl VideoDriver for OpenGL21 {
    #[inline(always)]
    static fn init() -> OpenGL21 {
        OpenGL21
    }

}
