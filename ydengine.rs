mod engine;

// Glfw3 - OpenGl21 target
mod glfw3platform;
mod opengl21videodriver;
use PlatformType    = glfw3platform::PlatformType;
use VideoDriverType = opengl21videodriver::VideoDriverType;

fn main() {
    engine::run::<PlatformType, VideoDriverType>();
}




