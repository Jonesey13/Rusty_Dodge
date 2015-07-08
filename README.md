# Rusty_Dodge
Porting the Circular Dodge Game to Rust with Glium. This project is primarily to help me learn how to code in Rust and OpenGL.

![](https://github.com/Jonesey13/Rusty_Dodge/blob/master/rusty_dodge.png)

IMPORTANT: Make sure the shaders folder is in the same directory that you build/run the project with (namely where you call Cargo build (or run) )

##Controls
Up/Down/Left/Right - Movement 

Escape - Exit 

Return/Enter - Reset

##Important Note for Windows Users

If you are getting gcc(or cc) linking errors of the form -lfreetype not found or -lbzip not found etc. then this means that the installation of rust is missing libraries it needs for the crate glium_text to work. To fix these issues I recommend downloading Cygwin (make sure to choose 32bit or 64bit correctly) and obtain the libraries from the package manager. After downloading the libraries, put them into your C:\{Rust_Dir}\bin\rustlib\{Windows_Ver}\lib folder.

### Thanks
Dylan Ede - For countless help and advice with coding in Rust/OpenGL

All the people at RCVGS

The Rust team / Tomaka 

