![](icon/rotational.png)

# Rotational

- a simple windows tray system tool to rotate your screen writen in Rust
- you can use Alt + "arrow keys" to rotate
- you can use the tray icon menu to show the virtual keyboard or rotate the screen

# Build and run

```
git clone https://github.com/NotsOverflow/rotational
cd rotational
cargo build --release
```

copy `target\release\rotational.exe` into `%userprofile%\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup`

logout and login again and a new tray icon should be showing up.
