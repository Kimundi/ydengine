mkdir -p bin
rustc -L ../ogl/glfw3-rs/lib ydengine.rs -o ./bin/ydengine && ./bin/ydengine
