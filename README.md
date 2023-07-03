# Rust Desktop App Code Template (following AwesomeApp Blueprint)

Base desktop application code with Tauri, Native Web Components, and SurrealDB (follow the VMES app architecture)

**See [awesomeapp.dev](https://awesomeapp.dev) for more info**

**[Troubleshooting](#troubleshooting)** | **[Changelog](https://awesomeapp.dev/changelog)** | **[Discord Awesome App](https://discord.gg/XuKWrNGKpC)**

> Note: To enable persitent storage, edit the `src-tauri/Cargo.toml` to enable all `surrealdb` features. 

# Hot Reload dev

For hot-reload UI and Tauri development, run the following in your VSCode from this root folder: 

```sh
awesome-app dev
```

> This assumes `awesome-app` was installed locally (e.g., `cargo install awesome-app`)

> **IMPORTANT** - Requires **node.js v8 and above**. 


# How it works

`awesome-app dev` will create an `Awesome.toml` which will be the list of commands it will run (format is self-explanatory). 

You can run the commands manually if you want, or see below for list of commands. 

We recommend using `awesome-app dev` but running each command manually might help troubleshoot.

# Build manually

IMPORTANT: Make sure to have **node.js latest of 16** or above. 

- `npm run tauri icon src-tauri/icons/app-icon.png` - This will build the application icons. 

- `npm run pcss` - This will build the postcss files (`src-ui/pcss/**/*.pcss`).

- `npm run rollup` - This will build and package the typescript files (`src-ui/src/**/*.ts`).

- `npm run localhost` - This will run a localhost server with the `dist/` folder as root (frontend hot reload)

- In another terminal, `npm run tauri dev` - Will start the Tauri build and start the process.

<br />

# Troubleshooting

- Make sure to have **node.js 18** or above.

- If some cryptic errors, run the command above one by one. 

- If `npm tauri dev` commands fail, try to do:
  - `cd src-tauri`
  - `cargo build` 
  - This might be an important first step when using full surrealdb (i.e., with default features and not only kv-mem) 
  
- It failed to compile and came up with the error `failed to download replaced source registry crates-io`. **Deleting** the **cargo.lock** file and **package-lock.json** file fixed it.  

- Installing Tauri in case some issues: 
```sh
# install latest tauri in case there is none
npm i -g @tauri-apps/cli @tauri-apps/api
```



## Requirements on fedora 36:

On Fedora, and probably linux, the following needs to be present on the system. 

```sh
dnf install gtk3-devel
dnf install webkit2gtk3-jsc-devel 
dnf install libsoup-devel
dnf install webkit2gtk3-devel.x86_64
```

## Requirements on Ubuntu 20

```sh
npm i
npm i -g tauri
sudo aptitude install -y \
  build-essential \
  libpango1.0-dev \
  libsoup2.4-dev \
  libjavascriptcoregtk-4.0-dev \
  libgdk-pixbuf2.0-dev \
  libgtk-3-dev \
  libwebkit2gtk-4.0-dev
npm run tauri dev
```

<br /><br />

[This repo on GitHub](https://github.com/awesomeapp-dev/rust-desktop-app)

