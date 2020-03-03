# Calling C Code From Rust

- Unsafe: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code
- FFI: https://doc.rust-lang.org/nomicon/ffi.html
- C, C++: https://liufuyang.github.io/2020/02/02/call-c-in-rust.html
- https://blog.jfo.click/calling-a-c-function-from-rust/
- C++: http://gernotklingler.com/blog/creating-using-shared-libraries-different-compilers-different-operating-systems/
- static: https://stackoverflow.com/questions/43826572/where-should-i-place-a-static-library-so-i-can-link-it-with-a-rust-program
- https://www.reddit.com/r/rust/comments/2w0695/problem_linking_to_c_shared_libraries/
- makefiles: https://amirkoblog.wordpress.com/2018/07/05/calling-rust-code-from-c-c/

On Windows 10, scoop-installed `rust` and `clang`, getting a linking error:

```
$ rustc -v -l doubler.lib main.rs
error: linking with `gcc` failed: exit code: 1
  |
  = note: "gcc" "-fno-use-linker-plugin" "-Wl,--nxcompat" "-nostdlib" "-m64" "C:\\Users\\Mihai\\scoop\\persist\\rustup\\.rustup\\toolchains\\stable-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\crt2.o" "C:\\Users\\Mihai\\scoop\\persist\\rustup\\.rustup\\toolchains\\stable-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\rsbegin.o" "-L" "C:\\Users\\Mihai\\scoop\\persist\\rustup\\.rustup\\toolchains\\stable-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "main.main.7rcbfp3g-cgu.0.rcgu.o" "main.main.7rcbfp3g-cgu.1.rcgu.o" "main.main.7rcbfp3g-cgu.2.rcgu.o" "main.main.7rcbfp3g-cgu.3.rcgu.o" "main.main.7rcbfp3g-cgu.4.rcgu.o" "main.main.7rcbfp3g-cgu.5.rcgu.o" "-o" "main.exe" "main.4s37gsrti678ik8u.rcgu.o" "-Wl,--gc-sections" "-nodefaultlibs" "-L" "C:\\Users\\Mihai\\scoop\\persist\\rustup\\.rustup\\toolchains\\stable-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "-ldoubler.lib" "-Wl,--start-group" "-Wl,-Bstatic" "C:\\Users\\Mihai\\scoop\\persist\\rustup\\.rustup\\toolchains\\stable-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libstd-b5bc0f9fc69c686a.rlib" "C:\\Users\\Mihai\\scoop\\persist\\rustup\\.rustup\\toolchains\\stable-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libpanic_unwind-4681759e0eff3e25.rlib" "C:\\Users\\Mihai\\scoop\\persist\\rustup\\.rustup\\toolchains\\stable-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libhashbrown-f713fea733c85eef.rlib" "C:\\Users\\Mihai\\scoop\\persist\\rustup\\.rustup\\toolchains\\stable-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\librustc_std_workspace_alloc-f1837b3f680b2f50.rlib" "C:\\Users\\Mihai\\scoop\\persist\\rustup\\.rustup\\toolchains\\stable-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libbacktrace-2bd76bc2623ee7b7.rlib" "C:\\Users\\Mihai\\scoop\\persist\\rustup\\.rustup\\toolchains\\stable-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libbacktrace_sys-af4540887bdb9059.rlib" "C:\\Users\\Mihai\\scoop\\persist\\rustup\\.rustup\\toolchains\\stable-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\librustc_demangle-a52515aca2c1c346.rlib" "C:\\Users\\Mihai\\scoop\\persist\\rustup\\.rustup\\toolchains\\stable-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libunwind-e5382999378a5223.rlib" "C:\\Users\\Mihai\\scoop\\persist\\rustup\\.rustup\\toolchains\\stable-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libcfg_if-e444b5c85663124e.rlib" "C:\\Users\\Mihai\\scoop\\persist\\rustup\\.rustup\\toolchains\\stable-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\liblibc-9c9deadf206e1eb8.rlib" "C:\\Users\\Mihai\\scoop\\persist\\rustup\\.rustup\\toolchains\\stable-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\liballoc-2a9bc12a9bb2b289.rlib" "C:\\Users\\Mihai\\scoop\\persist\\rustup\\.rustup\\toolchains\\stable-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\librustc_std_workspace_core-6b2cf061ea5ba9db.rlib" "C:\\Users\\Mihai\\scoop\\persist\\rustup\\.rustup\\toolchains\\stable-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libcore-11b0bb930e59b374.rlib" "-Wl,--end-group" "C:\\Users\\Mihai\\scoop\\persist\\rustup\\.rustup\\toolchains\\stable-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libcompiler_builtins-97cdc31cf0a3bed4.rlib" "-Wl,-Bdynamic" "-ladvapi32" "-lws2_32" "-luserenv" "-Wl,-Bstatic" "-lgcc_eh" "-lpthread" "-Wl,-Bdynamic" "-lmingwex" "-lmingw32" "-lgcc" "-lmsvcrt" "-lmsvcrt" "-luser32" "-lkernel32" "C:\\Users\\Mihai\\scoop\\persist\\rustup\\.rustup\\toolchains\\stable-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\rsend.o"
  = note: ld: cannot find -ldoubler.lib


error: aborting due to previous error

```

After quite a bit of fiddling, these commands work; 

```
$ clang --version
clang version 9.0.0 (tags/RELEASE_900/final)
Target: x86_64-pc-windows-msvc
Thread model: posix
InstalledDir: C:\Users\Mihai\scoop\apps\llvm\current\bin

$ pwd
/c/Users/Mihai/code/learning-rust/call-c-from-rust/src

$ clang doubler.c -c -o doubler.lib

$ rustc -v -l doubler -L. main.rs

$ ./main.exe
2
```

Note:
- works with or without a space after `L` in `-L.`
- name of library must end with `.lib`, but the extension must not be included in `rustc`

Related:
- `ld -ldoubler --verbose` - shows searched files (https://stackoverflow.com/a/21647591/447661)
- on Fedora: `dnf provides /usr/lib64/libinput.so`
- `gcc -print-search-dirs | sed '/^lib/b 1;d;:1;s,/[^/.][^/]*/\.\./,/,;t 1;s,:[^=]*=,:;,;s,;,;  ,g' | tr \; \\012` ([source](https://stackoverflow.com/a/21610523/447661))
- long (but allegedly good) read on linkers: https://lwn.net/Articles/276782/
