cargo install wasm-pack --version 0.9.1
wasm-pack build --target web
rmdir dist-web /s /q 2>nul
mkdir dist-web

xcopy index.html dist-web\
xcopy pkg\*.* dist-web\pkg\
xcopy assets\*.* dist-web\assets\