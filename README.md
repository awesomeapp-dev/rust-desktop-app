# Rust Base App Template for AWESOME-APP


Base desktop application code with Tauri, Native Web Components, and SurrealDB (follow the VMES app architecture)

**See [awesomeapp.org](https://awesomeapp.org) for more info**

**[Discord Awesome App](https://discord.gg/XuKWrNGKpC) for any questions, issues, or anything else**

# Hot Reload dev

For hot-reload UI and Tauri development, run the following in your VSCode from this root folder: 

```sh
awesome-app dev
```

> **IMPORTANT** - Requires **node.js v8 and above**. 


> This assumes `awesome-app` was installed locally (e.g., `cargo install awesome-app`)

# How it works

`awesome-app dev` will create an `Awesome.toml` which will be the list of commands it will run (format is self-explanatory). 

You can run the commands manually if you want, or see below for list of commands. 

We recommend using `awesome-app dev` but running each command manually might help troubleshoot.

# Build manually

IMPORTANT: Make sure to have **node.js 18** and above. 

- `npm run tauri icon src-tauri/icons/app-icon.png` - This will build the application icons. 

- `npm run pcss` - This will build the postcss files (`src-ui/pcss/**/*.pcss`).

- `npm run rollup` - This will build and package the typescript files (`src-ui/src/**/*.ts`).

- `npm run localhost` - This will run a localhost server with the `dist/` folder as root (frontend hot reload)

- In another terminal, `npm tauri dev` - Will start the Tauri build and start the process. 

<br />

# Troubleshooting

- Make sure to have **node.js 18** or above.
- If some cryptic errors, run the command above one by one. 
- If `npm tauri dev` commands fail, try to do:
  - `cd src-tauri`
  - `cargo build` 
  - This might be an important first step when using full surrealdb (i.e., with default features and not only kv-mem) 

## Requirements on fedora 36:

On Fedora, and probably linux, the following needs to be present on the system. 

```sh
dnf install gtk3-devel
dnf install webkit2gtk3-jsc-devel 
dnf install libsoup-devel
dnf install webkit2gtk3-devel.x86_64
```


<br /><br />

## Happy Coding!