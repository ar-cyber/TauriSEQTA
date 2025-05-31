> [!CAUTION]
> ## THIS IS IN EARLY DEVELOPMENT. DO NOT USE PRIMARILY - NOT FULLY FEATURE COMPLETE

![DesQTA](https://socialify.git.ci/BetterSEQTA/DesQTA/image?description=1&font=Raleway&forks=1&issues=1&language=1&logo=data%3Aimage%2Fsvg%2Bxml%2C%253Csvg%20height%3D%27656pt%27%20fill%3D%27white%27%20preserveAspectRatio%3D%27xMidYMid%20meet%27%20viewBox%3D%270%200%20658%20656%27%20width%3D%27658pt%27%20xmlns%3D%27http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%27%253E%253Cg%20transform%3D%27matrix(.1%200%200%20-.1%200%20656)%27%253E%253Cpath%20d%3D%27m2960%206499c-918-100-1726-561-2278-1299-196-262-374-609-475-925-171-533-203-1109-91-1655%20228-1115%201030-2032%202104-2408%20356-124%20680-177%201080-176%20269%201%20403%2014%20650%2064%20790%20159%201503%20624%201980%201290%20714%20998%20799%202342%20217%203420-488%20902-1361%201515-2382%201671-113%2017-196%2022-430%2024-159%202-328-1-375-6zm566-1443c476-99%20885-385%201134-791%20190-309%20282-696%20250-1045-22-240-73-420-180-635-78-156-159-275-274-401l-77-84h445%20446v-235-236l-1162%204-1163%203-100%2023c-449%20101-812%20337-1071%20697-77%20107-193%20335-233%20459-115%20358-116%20726-1%201078%20209%20644%20766%201101%201446%201187%20128%2016%20405%204%20540-24z%27%2F%253E%253Cpath%20d%3D%27m3065%204604c-250-36-396-89-576-209-280-187-470-478-535-821-25-135-16-395%2019-525%2095-351%20331-644%20651-806%2098-49%20225-93%20331-114%2092-18%20368-18%20460%200%20481%2095%20853%20444%20982%20921%2035%20129%2044%20389%2019%20524-36%20191-121%20387-228%20531-186%20249-476%20428-783%20485-65%2012-291%2021-340%2014z%27%2F%253E%253C%2Fg%253E%253C%2Fsvg%253E&name=1&owner=1&pattern=Signal&stargazers=1&theme=Auto)



<p align="center">
  <a target="_blank" href="https://discord.gg/YzmbnCDkat"><img src="https://github.com/SethBurkart123/EvenBetterSEQTA/assets/108050083/23055730-b16e-44c0-9bef-221d8545af92" width="240" style="border-radius:10%;" />
  </a>
</p>

- [Features](#features)
- [Getting Started](#getting-started)
  - [Running Development](#running-development)
  - [Folder Structure](#folder-structure)
- [Contributors](#contributors)

  
## Features

- **Custom Homepage**: A personalized homepage that displays upcoming assessments, lessons, and notices with subject-specific colors.

- **All SEQTA functionality kept**: Most SEQTA actions can be preformed here.

- **Timetable Colors**: Integration of timetable colors for subjects to enhance visual identification throuhout the entire app.

- **Sidebar Navigation**: A sidebar with quick navigation links to various sections like Home, News, Assessments, Courses, Dashboard, etc.

- **Session Management**: 
  - Handles user sessions with login and logout functionality.
  - Stores session data securely.

- **API Integration**: 
  - Fetches data from SEQTA APIs for assessments, subjects, and user preferences.
  - Utilizes Rust for backend operations with Tauri.

- **Responsive Design**: 
  - Utilizes Tailwind CSS for styling.
  - Ensures a responsive and modern UI across different devices.

## Getting started

Clone the repo:
```bash
git clone --branch develop https://github.com/betterseqta/desqta 
```
Install modules
```bash
npm i # or your preferred like pnpm or yarn
```

### Running Development
Run the script
```
npm run tauri dev
```
> [!WARNING]
> Do NOT run `npm run dev` as it starts the backend webui only; the pages loaded by DesQTA are built to be run by a framework not present in regular browsers. This is the same with `npm run build`; it will only build the webpage files, not the app.

> [!IMPORTANT]  
> To run the dev script you **<ins>MUST</ins>** have [rust, cargo (included with rust)](https://www.rust-lang.org/tools/install) and pnpm (`npm install -g pnpm`) on your system or it will not build.
>

> [!CAUTION]
> You should not build this - it is not stable and certain features (such as logging in) are buggy. <br><br> If you understand the risks, you can build using ```npm run tauri build```



## Folder Structure

<details>

<summary>Folder Structure:</summary>

├───.github
│   └───workflows
├───.svelte-kit
│   ├───generated
│   │   ├───client
│   │   │   └───nodes
│   │   └───server
│   └───types
│       └───src
│           └───routes
│               ├───assessments
│               │   └───[id]
│               │       └───[metaclass]
│               ├───courses
│               ├───dashboard
│               ├───direqt-messages
│               ├───news
│               ├───notices
│               ├───reports
│               ├───settings
│               │   └───plugins
│               ├───timetable
│               └───welcome
├───.vscode
├───node_modules
│   ├───.bin
│   ├───.vite
│   │   └───deps
│   ├───.vite-temp
│   ├───@alloc
│   │   └───quick-lru
│   ├───@ampproject
│   │   └───remapping
│   │       └───dist
│   │           └───types
│   ├───@babel
│   │   └───runtime
│   │       ├───helpers
│   │       │   └───esm
│   │       └───regenerator
│   ├───@esbuild
│   │   └───win32-x64
│   ├───@isaacs
│   │   └───cliui
│   │       └───build
│   │           └───lib
│   ├───@jridgewell
│   │   ├───gen-mapping
│   │   │   └───dist
│   │   │       └───types
│   │   ├───resolve-uri
│   │   │   └───dist
│   │   │       └───types
│   │   ├───set-array
│   │   │   └───dist
│   │   │       └───types
│   │   ├───sourcemap-codec
│   │   │   └───dist
│   │   │       └───types
│   │   └───trace-mapping
│   │       └───dist
│   │           └───types
│   ├───@nodelib
│   │   ├───fs.scandir
│   │   │   └───out
│   │   │       ├───adapters
│   │   │       ├───providers
│   │   │       ├───types
│   │   │       └───utils
│   │   ├───fs.stat
│   │   │   └───out
│   │   │       ├───adapters
│   │   │       ├───providers
│   │   │       └───types
│   │   └───fs.walk
│   │       └───out
│   │           ├───providers
│   │           ├───readers
│   │           └───types
│   ├───@pkgjs
│   │   └───parseargs
│   │       ├───examples
│   │       └───internal
│   ├───@polka
│   │   └───url
│   ├───@popperjs
│   │   └───core
│   │       ├───dist
│   │       │   ├───cjs
│   │       │   ├───esm
│   │       │   │   ├───dom-utils
│   │       │   │   ├───modifiers
│   │       │   │   └───utils
│   │       │   └───umd
│   │       └───lib
│   │           ├───dom-utils
│   │           ├───modifiers
│   │           └───utils
│   ├───@remirror
│   │   └───core-constants
│   │       ├───dist
│   │       └───dist-types
│   ├───@rollup
│   │   └───rollup-win32-x64-msvc
│   ├───@steeze-ui
│   │   └───heroicons
│   │       └───dist
│   ├───@sveltejs
│   │   ├───acorn-typescript
│   │   ├───adapter-static
│   │   ├───kit
│   │   │   ├───src
│   │   │   │   ├───core
│   │   │   │   │   ├───adapt
│   │   │   │   │   ├───config
│   │   │   │   │   ├───generate_manifest
│   │   │   │   │   ├───postbuild
│   │   │   │   │   └───sync
│   │   │   │   │       ├───create_manifest_data
│   │   │   │   │       └───write_types
│   │   │   │   ├───exports
│   │   │   │   │   ├───hooks
│   │   │   │   │   ├───node
│   │   │   │   │   └───vite
│   │   │   │   │       ├───build
│   │   │   │   │       ├───dev
│   │   │   │   │       ├───graph_analysis
│   │   │   │   │       ├───preview
│   │   │   │   │       └───static_analysis
│   │   │   │   ├───runtime
│   │   │   │   │   ├───app
│   │   │   │   │   │   ├───environment
│   │   │   │   │   │   ├───paths
│   │   │   │   │   │   ├───server
│   │   │   │   │   │   └───state
│   │   │   │   │   ├───client
│   │   │   │   │   ├───components
│   │   │   │   │   │   ├───svelte-4
│   │   │   │   │   │   └───svelte-5
│   │   │   │   │   ├───env
│   │   │   │   │   │   └───dynamic
│   │   │   │   │   └───server
│   │   │   │   │       ├───data
│   │   │   │   │       └───page
│   │   │   │   ├───types
│   │   │   │   │   └───synthetic
│   │   │   │   └───utils
│   │   │   └───types
│   │   ├───vite-plugin-svelte
│   │   │   ├───src
│   │   │   │   ├───types
│   │   │   │   └───utils
│   │   │   └───types
│   │   └───vite-plugin-svelte-inspector
│   │       ├───src
│   │       │   └───runtime
│   │       └───types
│   ├───@tailwindcss
│   │   └───typography
│   │       ├───node_modules
│   │       │   └───postcss-selector-parser
│   │       │       └───dist
│   │       │           ├───selectors
│   │       │           └───util
│   │       └───src
│   ├───@tauri-apps
│   │   ├───api
│   │   │   ├───external
│   │   │   │   └───tslib
│   │   │   └───menu
│   │   ├───cli
│   │   │   ├───src
│   │   │   └───__tests__
│   │   │       └───fixtures
│   │   │           └───empty
│   │   │               └───dist
│   │   ├───cli-win32-x64-msvc
│   │   ├───plugin-notification
│   │   │   └───dist-js
│   │   └───plugin-opener
│   │       └───dist-js
│   ├───@tiptap
│   │   ├───core
│   │   │   ├───dist
│   │   │   │   ├───commands
│   │   │   │   ├───extensions
│   │   │   │   ├───helpers
│   │   │   │   ├───inputRules
│   │   │   │   ├───pasteRules
│   │   │   │   └───utilities
│   │   │   └───src
│   │   │       ├───commands
│   │   │       ├───extensions
│   │   │       ├───helpers
│   │   │       ├───inputRules
│   │   │       ├───pasteRules
│   │   │       └───utilities
│   │   ├───extension-blockquote
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-bold
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-bubble-menu
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-bullet-list
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-code
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-code-block
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-document
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-dropcursor
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-gapcursor
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-hard-break
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-heading
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-history
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-horizontal-rule
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-italic
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-link
│   │   │   ├───dist
│   │   │   │   └───helpers
│   │   │   └───src
│   │   │       └───helpers
│   │   ├───extension-list-item
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-ordered-list
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-paragraph
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-placeholder
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-strike
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-task-item
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-task-list
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-text
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-text-style
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───extension-typography
│   │   │   ├───dist
│   │   │   └───src
│   │   ├───pm
│   │   │   ├───changeset
│   │   │   │   └───dist
│   │   │   ├───collab
│   │   │   │   └───dist
│   │   │   ├───commands
│   │   │   │   └───dist
│   │   │   ├───dropcursor
│   │   │   │   └───dist
│   │   │   ├───gapcursor
│   │   │   │   └───dist
│   │   │   ├───history
│   │   │   │   └───dist
│   │   │   ├───inputrules
│   │   │   │   └───dist
│   │   │   ├───keymap
│   │   │   │   └───dist
│   │   │   ├───markdown
│   │   │   │   └───dist
│   │   │   ├───menu
│   │   │   │   └───dist
│   │   │   ├───model
│   │   │   │   └───dist
│   │   │   ├───schema-basic
│   │   │   │   └───dist
│   │   │   ├───schema-list
│   │   │   │   └───dist
│   │   │   ├───state
│   │   │   │   └───dist
│   │   │   ├───tables
│   │   │   │   └───dist
│   │   │   ├───trailing-node
│   │   │   │   └───dist
│   │   │   ├───transform
│   │   │   │   └───dist
│   │   │   └───view
│   │   │       └───dist
│   │   ├───starter-kit
│   │   │   ├───dist
│   │   │   └───src
│   │   └───suggestion
│   │       ├───dist
│   │       └───src
│   ├───@types
│   │   ├───cookie
│   │   ├───estree
│   │   ├───file-saver
│   │   ├───linkify-it
│   │   │   └───build
│   │   ├───markdown-it
│   │   │   ├───dist
│   │   │   └───lib
│   │   │       ├───common
│   │   │       ├───helpers
│   │   │       ├───rules_block
│   │   │       ├───rules_core
│   │   │       └───rules_inline
│   │   ├───mdurl
│   │   │   ├───build
│   │   │   └───lib
│   │   ├───raf
│   │   └───trusted-types
│   │       └───lib
│   ├───acorn
│   │   ├───bin
│   │   └───dist
│   ├───ansi-regex
│   ├───ansi-styles
│   ├───any-promise
│   │   └───register
│   ├───anymatch
│   │   └───node_modules
│   │       └───picomatch
│   │           └───lib
│   ├───arg
│   ├───argparse
│   │   └───lib
│   ├───aria-query
│   │   └───lib
│   │       ├───etc
│   │       │   └───roles
│   │       │       ├───abstract
│   │       │       ├───dpub
│   │       │       ├───graphics
│   │       │       └───literal
│   │       └───util
│   ├───atob
│   │   └───bin
│   ├───autoprefixer
│   │   ├───bin
│   │   ├───data
│   │   └───lib
│   │       └───hacks
│   ├───axobject-query
│   │   └───lib
│   │       ├───etc
│   │       │   └───objects
│   │       └───util
│   ├───balanced-match
│   │   └───.github
│   ├───base64-arraybuffer
│   │   └───dist
│   │       ├───lib
│   │       └───types
│   ├───binary-extensions
│   ├───brace-expansion
│   │   └───.github
│   ├───braces
│   │   └───lib
│   ├───browserslist
│   ├───btoa
│   │   └───bin
│   ├───camelcase-css
│   ├───caniuse-lite
│   │   ├───data
│   │   │   ├───features
│   │   │   └───regions
│   │   └───dist
│   │       ├───lib
│   │       └───unpacker
│   ├───canvg
│   │   └───lib
│   │       ├───Document
│   │       ├───presets
│   │       ├───Transform
│   │       └───util
│   ├───chokidar
│   │   └───esm
│   ├───clsx
│   │   └───dist
│   ├───color-convert
│   ├───color-name
│   ├───commander
│   │   └───typings
│   ├───cookie
│   ├───core-js
│   │   ├───actual
│   │   │   ├───array
│   │   │   │   └───virtual
│   │   │   ├───array-buffer
│   │   │   ├───async-disposable-stack
│   │   │   ├───async-iterator
│   │   │   ├───data-view
│   │   │   ├───date
│   │   │   ├───disposable-stack
│   │   │   ├───dom-collections
│   │   │   ├───dom-exception
│   │   │   ├───error
│   │   │   ├───function
│   │   │   │   └───virtual
│   │   │   ├───instance
│   │   │   ├───iterator
│   │   │   ├───json
│   │   │   ├───map
│   │   │   ├───math
│   │   │   ├───number
│   │   │   │   └───virtual
│   │   │   ├───object
│   │   │   ├───promise
│   │   │   ├───reflect
│   │   │   ├───regexp
│   │   │   ├───set
│   │   │   ├───string
│   │   │   │   └───virtual
│   │   │   ├───symbol
│   │   │   ├───typed-array
│   │   │   ├───url
│   │   │   ├───url-search-params
│   │   │   ├───weak-map
│   │   │   └───weak-set
│   │   ├───es
│   │   │   ├───array
│   │   │   │   └───virtual
│   │   │   ├───array-buffer
│   │   │   ├───data-view
│   │   │   ├───date
│   │   │   ├───error
│   │   │   ├───function
│   │   │   │   └───virtual
│   │   │   ├───instance
│   │   │   ├───iterator
│   │   │   ├───json
│   │   │   ├───map
│   │   │   ├───math
│   │   │   ├───number
│   │   │   │   └───virtual
│   │   │   ├───object
│   │   │   ├───promise
│   │   │   ├───reflect
│   │   │   ├───regexp
│   │   │   ├───set
│   │   │   ├───string
│   │   │   │   └───virtual
│   │   │   ├───symbol
│   │   │   ├───typed-array
│   │   │   ├───weak-map
│   │   │   └───weak-set
│   │   ├───features
│   │   │   ├───array
│   │   │   │   └───virtual
│   │   │   ├───array-buffer
│   │   │   ├───async-disposable-stack
│   │   │   ├───async-iterator
│   │   │   ├───bigint
│   │   │   ├───data-view
│   │   │   ├───date
│   │   │   ├───disposable-stack
│   │   │   ├───dom-collections
│   │   │   ├───dom-exception
│   │   │   ├───error
│   │   │   ├───function
│   │   │   │   └───virtual
│   │   │   ├───instance
│   │   │   ├───iterator
│   │   │   ├───json
│   │   │   ├───map
│   │   │   ├───math
│   │   │   ├───number
│   │   │   │   └───virtual
│   │   │   ├───object
│   │   │   ├───observable
│   │   │   ├───promise
│   │   │   ├───reflect
│   │   │   ├───regexp
│   │   │   ├───set
│   │   │   ├───string
│   │   │   │   └───virtual
│   │   │   ├───symbol
│   │   │   ├───typed-array
│   │   │   ├───url
│   │   │   ├───url-search-params
│   │   │   ├───weak-map
│   │   │   └───weak-set
│   │   ├───full
│   │   │   ├───array
│   │   │   │   └───virtual
│   │   │   ├───array-buffer
│   │   │   ├───async-disposable-stack
│   │   │   ├───async-iterator
│   │   │   ├───bigint
│   │   │   ├───data-view
│   │   │   ├───date
│   │   │   ├───disposable-stack
│   │   │   ├───dom-collections
│   │   │   ├───dom-exception
│   │   │   ├───error
│   │   │   ├───function
│   │   │   │   └───virtual
│   │   │   ├───instance
│   │   │   ├───iterator
│   │   │   ├───json
│   │   │   ├───map
│   │   │   ├───math
│   │   │   ├───number
│   │   │   │   └───virtual
│   │   │   ├───object
│   │   │   ├───observable
│   │   │   ├───promise
│   │   │   ├───reflect
│   │   │   ├───regexp
│   │   │   ├───set
│   │   │   ├───string
│   │   │   │   └───virtual
│   │   │   ├───symbol
│   │   │   ├───typed-array
│   │   │   ├───url
│   │   │   ├───url-search-params
│   │   │   ├───weak-map
│   │   │   └───weak-set
│   │   ├───internals
│   │   ├───modules
│   │   ├───proposals
│   │   ├───stable
│   │   │   ├───array
│   │   │   │   └───virtual
│   │   │   ├───array-buffer
│   │   │   ├───data-view
│   │   │   ├───date
│   │   │   ├───dom-collections
│   │   │   ├───dom-exception
│   │   │   ├───error
│   │   │   ├───function
│   │   │   │   └───virtual
│   │   │   ├───instance
│   │   │   ├───iterator
│   │   │   ├───json
│   │   │   ├───map
│   │   │   ├───math
│   │   │   ├───number
│   │   │   │   └───virtual
│   │   │   ├───object
│   │   │   ├───promise
│   │   │   ├───reflect
│   │   │   ├───regexp
│   │   │   ├───set
│   │   │   ├───string
│   │   │   │   └───virtual
│   │   │   ├───symbol
│   │   │   ├───typed-array
│   │   │   ├───url
│   │   │   ├───url-search-params
│   │   │   ├───weak-map
│   │   │   └───weak-set
│   │   ├───stage
│   │   └───web
│   ├───crelt
│   │   └───dist
│   ├───cross-spawn
│   │   └───lib
│   │       └───util
│   ├───css-line-break
│   │   └───dist
│   │       ├───lib
│   │       └───types
│   ├───cssesc
│   │   ├───bin
│   │   └───man
│   ├───debug
│   │   └───src
│   ├───deepmerge
│   │   └───dist
│   ├───devalue
│   │   ├───src
│   │   └───types
│   ├───didyoumean
│   ├───dlv
│   │   └───dist
│   ├───dompurify
│   │   └───dist
│   ├───eastasianwidth
│   ├───electron-to-chromium
│   ├───emoji-regex
│   │   └───es2015
│   ├───entities
│   │   └───lib
│   │       ├───esm
│   │       │   └───generated
│   │       └───generated
│   ├───esbuild
│   │   ├───bin
│   │   └───lib
│   ├───escalade
│   │   ├───dist
│   │   └───sync
│   ├───escape-string-regexp
│   ├───esm-env
│   ├───esrap
│   │   ├───src
│   │   └───types
│   ├───fast-glob
│   │   ├───node_modules
│   │   │   └───glob-parent
│   │   └───out
│   │       ├───managers
│   │       ├───providers
│   │       │   ├───filters
│   │       │   ├───matchers
│   │       │   └───transformers
│   │       ├───readers
│   │       ├───types
│   │       └───utils
│   ├───fastq
│   │   ├───.github
│   │   │   └───workflows
│   │   └───test
│   ├───fdir
│   │   └───dist
│   │       ├───api
│   │       │   └───functions
│   │       └───builder
│   ├───fflate
│   │   ├───esm
│   │   ├───lib
│   │   └───umd
│   ├───file-saver
│   │   ├───dist
│   │   └───src
│   ├───fill-range
│   ├───foreground-child
│   │   └───dist
│   │       ├───commonjs
│   │       └───esm
│   ├───fraction.js
│   ├───function-bind
│   │   ├───.github
│   │   └───test
│   ├───glob
│   │   └───dist
│   │       ├───commonjs
│   │       └───esm
│   ├───glob-parent
│   ├───hasown
│   │   └───.github
│   ├───html2canvas
│   │   └───dist
│   │       ├───lib
│   │       │   ├───core
│   │       │   │   ├───__mocks__
│   │       │   │   └───__tests__
│   │       │   ├───css
│   │       │   │   ├───layout
│   │       │   │   │   └───__mocks__
│   │       │   │   ├───property-descriptors
│   │       │   │   │   └───__tests__
│   │       │   │   ├───syntax
│   │       │   │   │   └───__tests__
│   │       │   │   └───types
│   │       │   │       ├───functions
│   │       │   │       │   └───__tests__
│   │       │   │       └───__tests__
│   │       │   ├───dom
│   │       │   │   ├───elements
│   │       │   │   ├───replaced-elements
│   │       │   │   └───__mocks__
│   │       │   ├───render
│   │       │   │   └───canvas
│   │       │   └───__tests__
│   │       └───types
│   │           ├───core
│   │           │   ├───__mocks__
│   │           │   └───__tests__
│   │           ├───css
│   │           │   ├───layout
│   │           │   │   └───__mocks__
│   │           │   ├───property-descriptors
│   │           │   │   └───__tests__
│   │           │   ├───syntax
│   │           │   │   └───__tests__
│   │           │   └───types
│   │           │       ├───functions
│   │           │       │   └───__tests__
│   │           │       └───__tests__
│   │           ├───dom
│   │           │   ├───elements
│   │           │   ├───replaced-elements
│   │           │   └───__mocks__
│   │           ├───render
│   │           │   └───canvas
│   │           └───__tests__
│   ├───is-binary-path
│   ├───is-core-module
│   │   └───test
│   ├───is-extglob
│   ├───is-fullwidth-code-point
│   ├───is-glob
│   ├───is-number
│   ├───is-reference
│   │   ├───src
│   │   └───types
│   ├───isexe
│   │   └───test
│   ├───jackspeak
│   │   └───dist
│   │       ├───commonjs
│   │       └───esm
│   ├───jiti
│   │   ├───bin
│   │   ├───dist
│   │   │   └───plugins
│   │   └───lib
│   ├───jspdf
│   │   ├───dist
│   │   └───types
│   ├───jspdf-autotable
│   │   └───dist
│   ├───kleur
│   ├───lilconfig
│   │   └───src
│   ├───lines-and-columns
│   │   └───build
│   ├───linkify-it
│   │   ├───build
│   │   └───lib
│   ├───linkifyjs
│   │   └───dist
│   ├───locate-character
│   │   ├───src
│   │   └───types
│   ├───lodash.castarray
│   ├───lodash.isplainobject
│   ├───lodash.merge
│   ├───lru-cache
│   │   └───dist
│   │       ├───commonjs
│   │       └───esm
│   ├───magic-string
│   │   └───dist
│   ├───markdown-it
│   │   ├───bin
│   │   ├───dist
│   │   └───lib
│   │       ├───common
│   │       ├───helpers
│   │       ├───presets
│   │       ├───rules_block
│   │       ├───rules_core
│   │       └───rules_inline
│   ├───marked
│   │   ├───bin
│   │   ├───lib
│   │   └───man
│   ├───mdurl
│   │   ├───build
│   │   └───lib
│   ├───merge2
│   ├───micromatch
│   │   └───node_modules
│   │       └───picomatch
│   │           └───lib
│   ├───minimatch
│   │   └───dist
│   │       ├───commonjs
│   │       └───esm
│   ├───minipass
│   │   └───dist
│   │       ├───commonjs
│   │       └───esm
│   ├───mri
│   │   └───lib
│   ├───mrmime
│   ├───ms
│   ├───mz
│   ├───nanoid
│   │   ├───async
│   │   ├───bin
│   │   ├───non-secure
│   │   └───url-alphabet
│   ├───node-releases
│   │   └───data
│   │       ├───processed
│   │       └───release-schedule
│   ├───normalize-path
│   ├───normalize-range
│   ├───object-assign
│   ├───object-hash
│   │   └───dist
│   ├───orderedmap
│   │   └───dist
│   ├───package-json-from-dist
│   │   └───dist
│   │       ├───commonjs
│   │       └───esm
│   ├───path-key
│   ├───path-parse
│   ├───path-scurry
│   │   └───dist
│   │       ├───commonjs
│   │       └───esm
│   ├───performance-now
│   │   ├───lib
│   │   ├───src
│   │   └───test
│   │       └───scripts
│   ├───picocolors
│   ├───picomatch
│   │   └───lib
│   ├───pify
│   ├───pirates
│   │   └───lib
│   ├───postcss
│   │   └───lib
│   ├───postcss-import
│   │   └───lib
│   ├───postcss-js
│   ├───postcss-load-config
│   │   └───src
│   ├───postcss-nested
│   ├───postcss-selector-parser
│   │   └───dist
│   │       ├───selectors
│   │       └───util
│   ├───postcss-value-parser
│   │   └───lib
│   ├───prosemirror-changeset
│   │   ├───dist
│   │   ├───src
│   │   └───test
│   ├───prosemirror-collab
│   │   ├───dist
│   │   └───src
│   ├───prosemirror-commands
│   │   ├───dist
│   │   └───src
│   ├───prosemirror-dropcursor
│   │   ├───dist
│   │   └───src
│   ├───prosemirror-gapcursor
│   │   ├───dist
│   │   ├───src
│   │   └───style
│   ├───prosemirror-history
│   │   ├───dist
│   │   └───src
│   ├───prosemirror-inputrules
│   │   ├───dist
│   │   └───src
│   ├───prosemirror-keymap
│   │   ├───dist
│   │   └───src
│   ├───prosemirror-markdown
│   │   ├───dist
│   │   └───src
│   ├───prosemirror-menu
│   │   ├───dist
│   │   ├───src
│   │   └───style
│   ├───prosemirror-model
│   │   ├───dist
│   │   └───src
│   ├───prosemirror-schema-basic
│   │   ├───dist
│   │   └───src
│   ├───prosemirror-schema-list
│   │   ├───dist
│   │   └───src
│   ├───prosemirror-state
│   │   ├───dist
│   │   └───src
│   ├───prosemirror-tables
│   │   ├───dist
│   │   └───style
│   ├───prosemirror-trailing-node
│   │   ├───dist
│   │   └───dist-types
│   ├───prosemirror-transform
│   │   ├───dist
│   │   └───src
│   ├───prosemirror-view
│   │   ├───dist
│   │   ├───src
│   │   └───style
│   ├───punycode.js
│   ├───queue-microtask
│   ├───raf
│   ├───read-cache
│   ├───readdirp
│   │   └───esm
│   ├───regenerator-runtime
│   ├───resolve
│   │   ├───.github
│   │   ├───bin
│   │   ├───example
│   │   ├───lib
│   │   └───test
│   │       ├───dotdot
│   │       │   └───abc
│   │       ├───module_dir
│   │       │   ├───xmodules
│   │       │   │   └───aaa
│   │       │   ├───ymodules
│   │       │   │   └───aaa
│   │       │   └───zmodules
│   │       │       └───bbb
│   │       ├───node_path
│   │       │   ├───x
│   │       │   │   ├───aaa
│   │       │   │   └───ccc
│   │       │   └───y
│   │       │       ├───bbb
│   │       │       └───ccc
│   │       ├───pathfilter
│   │       │   └───deep_ref
│   │       ├───precedence
│   │       │   ├───aaa
│   │       │   └───bbb
│   │       ├───resolver
│   │       │   ├───baz
│   │       │   ├───browser_field
│   │       │   ├───dot_main
│   │       │   ├───dot_slash_main
│   │       │   ├───false_main
│   │       │   ├───incorrect_main
│   │       │   ├───invalid_main
│   │       │   ├───multirepo
│   │       │   │   └───packages
│   │       │   │       ├───package-a
│   │       │   │       └───package-b
│   │       │   ├───nested_symlinks
│   │       │   │   └───mylib
│   │       │   ├───other_path
│   │       │   │   └───lib
│   │       │   ├───quux
│   │       │   │   └───foo
│   │       │   ├───same_names
│   │       │   │   └───foo
│   │       │   ├───symlinked
│   │       │   │   ├───package
│   │       │   │   └───_
│   │       │   │       ├───node_modules
│   │       │   │       └───symlink_target
│   │       │   └───without_basedir
│   │       └───shadowed_core
│   │           └───node_modules
│   │               └───util
│   ├───reusify
│   │   ├───.github
│   │   │   └───workflows
│   │   └───benchmarks
│   ├───rgbcolor
│   ├───rollup
│   │   └───dist
│   │       ├───bin
│   │       ├───es
│   │       │   └───shared
│   │       └───shared
│   ├───rope-sequence
│   │   └───dist
│   ├───rss-parser
│   │   ├───.github
│   │   │   └───workflows
│   │   ├───dist
│   │   ├───lib
│   │   ├───node_modules
│   │   │   └───entities
│   │   │       └───lib
│   │   │           └───maps
│   │   └───scripts
│   ├───run-parallel
│   ├───sade
│   │   └───lib
│   ├───sax
│   │   └───lib
│   ├───set-cookie-parser
│   │   └───lib
│   ├───shebang-command
│   ├───shebang-regex
│   ├───signal-exit
│   │   └───dist
│   │       ├───cjs
│   │       └───mjs
│   ├───sirv
│   ├───source-map-js
│   │   └───lib
│   ├───stackblur-canvas
│   │   ├───dist
│   │   └───src
│   ├───string-width
│   ├───string-width-cjs
│   │   └───node_modules
│   │       ├───ansi-regex
│   │       ├───emoji-regex
│   │       │   └───es2015
│   │       └───strip-ansi
│   ├───strip-ansi
│   ├───strip-ansi-cjs
│   │   └───node_modules
│   │       └───ansi-regex
│   ├───sucrase
│   │   ├───bin
│   │   ├───dist
│   │   │   ├───esm
│   │   │   │   ├───parser
│   │   │   │   │   ├───plugins
│   │   │   │   │   │   └───jsx
│   │   │   │   │   ├───tokenizer
│   │   │   │   │   ├───traverser
│   │   │   │   │   └───util
│   │   │   │   ├───transformers
│   │   │   │   └───util
│   │   │   ├───parser
│   │   │   │   ├───plugins
│   │   │   │   │   └───jsx
│   │   │   │   ├───tokenizer
│   │   │   │   ├───traverser
│   │   │   │   └───util
│   │   │   ├───transformers
│   │   │   ├───types
│   │   │   │   ├───parser
│   │   │   │   │   ├───plugins
│   │   │   │   │   │   └───jsx
│   │   │   │   │   ├───tokenizer
│   │   │   │   │   ├───traverser
│   │   │   │   │   └───util
│   │   │   │   ├───transformers
│   │   │   │   └───util
│   │   │   └───util
│   │   ├───register
│   │   └───ts-node-plugin
│   ├───supports-preserve-symlinks-flag
│   │   ├───.github
│   │   └───test
│   ├───svelte
│   │   ├───compiler
│   │   ├───src
│   │   │   ├───animate
│   │   │   ├───compiler
│   │   │   │   ├───migrate
│   │   │   │   ├───phases
│   │   │   │   │   ├───1-parse
│   │   │   │   │   │   ├───read
│   │   │   │   │   │   ├───state
│   │   │   │   │   │   └───utils
│   │   │   │   │   ├───2-analyze
│   │   │   │   │   │   ├───css
│   │   │   │   │   │   ├───utils
│   │   │   │   │   │   └───visitors
│   │   │   │   │   │       └───shared
│   │   │   │   │   └───3-transform
│   │   │   │   │       ├───client
│   │   │   │   │       │   └───visitors
│   │   │   │   │       │       └───shared
│   │   │   │   │       ├───css
│   │   │   │   │       ├───server
│   │   │   │   │       │   └───visitors
│   │   │   │   │       │       └───shared
│   │   │   │   │       └───shared
│   │   │   │   ├───preprocess
│   │   │   │   └───utils
│   │   │   ├───easing
│   │   │   ├───events
│   │   │   ├───internal
│   │   │   │   ├───client
│   │   │   │   │   ├───dev
│   │   │   │   │   ├───dom
│   │   │   │   │   │   ├───blocks
│   │   │   │   │   │   ├───elements
│   │   │   │   │   │   │   └───bindings
│   │   │   │   │   │   └───legacy
│   │   │   │   │   └───reactivity
│   │   │   │   ├───flags
│   │   │   │   ├───server
│   │   │   │   │   └───blocks
│   │   │   │   └───shared
│   │   │   ├───legacy
│   │   │   ├───motion
│   │   │   ├───reactivity
│   │   │   │   └───window
│   │   │   ├───server
│   │   │   ├───store
│   │   │   │   └───shared
│   │   │   └───transition
│   │   └───types
│   │       └───compiler
│   ├───svelte-check
│   │   ├───bin
│   │   └───dist
│   │       └───src
│   ├───svelte-hero-icons
│   │   └───dist
│   ├───svg-pathdata
│   │   ├───lib
│   │   └───src
│   ├───tailwindcss
│   │   ├───lib
│   │   │   ├───cli
│   │   │   │   ├───build
│   │   │   │   ├───help
│   │   │   │   └───init
│   │   │   ├───css
│   │   │   ├───lib
│   │   │   ├───postcss-plugins
│   │   │   │   └───nesting
│   │   │   ├───public
│   │   │   ├───util
│   │   │   └───value-parser
│   │   ├───nesting
│   │   ├───node_modules
│   │   │   ├───chokidar
│   │   │   │   ├───lib
│   │   │   │   ├───node_modules
│   │   │   │   │   └───glob-parent
│   │   │   │   └───types
│   │   │   ├───picomatch
│   │   │   │   └───lib
│   │   │   └───readdirp
│   │   ├───peers
│   │   ├───scripts
│   │   ├───src
│   │   │   ├───cli
│   │   │   │   ├───build
│   │   │   │   ├───help
│   │   │   │   └───init
│   │   │   ├───css
│   │   │   ├───lib
│   │   │   ├───postcss-plugins
│   │   │   │   └───nesting
│   │   │   ├───public
│   │   │   ├───util
│   │   │   └───value-parser
│   │   ├───stubs
│   │   └───types
│   │       └───generated
│   ├───text-segmentation
│   │   └───dist
│   │       ├───lib
│   │       └───types
│   ├───thenify
│   ├───thenify-all
│   ├───tinyglobby
│   │   └───dist
│   ├───tippy.js
│   │   ├───animations
│   │   ├───dist
│   │   ├───headless
│   │   │   └───dist
│   │   └───themes
│   ├───to-regex-range
│   ├───totalist
│   │   ├───dist
│   │   └───sync
│   ├───ts-interface-checker
│   │   └───dist
│   ├───typescript
│   │   ├───bin
│   │   └───lib
│   │       ├───cs
│   │       ├───de
│   │       ├───es
│   │       ├───fr
│   │       ├───it
│   │       ├───ja
│   │       ├───ko
│   │       ├───pl
│   │       ├───pt-br
│   │       ├───ru
│   │       ├───tr
│   │       ├───zh-cn
│   │       └───zh-tw
│   ├───uc.micro
│   │   ├───build
│   │   ├───categories
│   │   │   ├───Cc
│   │   │   ├───Cf
│   │   │   ├───P
│   │   │   ├───S
│   │   │   └───Z
│   │   └───properties
│   │       └───Any
│   ├───update-browserslist-db
│   ├───util-deprecate
│   ├───utrie
│   │   └───dist
│   │       ├───lib
│   │       └───types
│   ├───vite
│   │   ├───bin
│   │   ├───dist
│   │   │   ├───client
│   │   │   ├───node
│   │   │   │   └───chunks
│   │   │   └───node-cjs
│   │   ├───misc
│   │   └───types
│   │       └───internal
│   ├───vitefu
│   │   └───src
│   ├───w3c-keyname
│   ├───which
│   │   └───bin
│   ├───wrap-ansi
│   ├───wrap-ansi-cjs
│   │   └───node_modules
│   │       ├───ansi-regex
│   │       ├───ansi-styles
│   │       ├───emoji-regex
│   │       │   └───es2015
│   │       ├───string-width
│   │       └───strip-ansi
│   ├───xml2js
│   │   └───lib
│   ├───xmlbuilder
│   │   ├───lib
│   │   └───typings
│   ├───yaml
│   │   ├───browser
│   │   │   └───dist
│   │   │       ├───compose
│   │   │       ├───doc
│   │   │       ├───nodes
│   │   │       ├───parse
│   │   │       ├───schema
│   │   │       │   ├───common
│   │   │       │   ├───core
│   │   │       │   ├───json
│   │   │       │   └───yaml-1.1
│   │   │       └───stringify
│   │   └───dist
│   │       ├───compose
│   │       ├───doc
│   │       ├───nodes
│   │       ├───parse
│   │       ├───schema
│   │       │   ├───common
│   │       │   ├───core
│   │       │   ├───json
│   │       │   └───yaml-1.1
│   │       └───stringify
│   └───zimmerframe
│       ├───src
│       └───types
├───src
│   ├───components
│   │   └───Editor
│   │       └───Plugins
│   │           └───Commands
│   ├───routes
│   │   ├───assessments
│   │   │   └───[id]
│   │   │       └───[metaclass]
│   │   ├───courses
│   │   │   └───components
│   │   ├───dashboard
│   │   ├───direqt-messages
│   │   │   └───components
│   │   ├───news
│   │   ├───notices
│   │   ├───reports
│   │   ├───settings
│   │   │   └───plugins
│   │   ├───timetable
│   │   └───welcome
│   └───utils
├───src-tauri
│   ├───capabilities
│   ├───gen
│   │   ├───apple
│   │   │   ├───Assets.xcassets
│   │   │   │   └───AppIcon.appiconset
│   │   │   ├───desqta.xcodeproj
│   │   │   │   ├───project.xcworkspace
│   │   │   │   │   └───xcshareddata
│   │   │   │   └───xcshareddata
│   │   │   │       └───xcschemes
│   │   │   ├───desqta_iOS
│   │   │   └───Sources
│   │   │       └───desqta
│   │   │           └───bindings
│   │   └───schemas
│   ├───icons
│   │   ├───android
│   │   │   ├───mipmap-hdpi
│   │   │   ├───mipmap-mdpi
│   │   │   ├───mipmap-xhdpi
│   │   │   ├───mipmap-xxhdpi
│   │   │   └───mipmap-xxxhdpi
│   │   └───ios
│   ├───src
│   │   ├───auth
│   │   ├───mobilechanges
│   │   └───utils
│   └───target
│       └───debug
│           ├───.fingerprint
│           │   ├───adler2-7449e868672588b4
│           │   ├───aho-corasick-b66cdf5c505485c4
│           │   ├───alloc-no-stdlib-3b02cb65baf29aa9
│           │   ├───alloc-stdlib-c73d438c55d7873a
│           │   ├───anyhow-1637c556491076f6
│           │   ├───anyhow-7d7dd0ce4993f3cf
│           │   ├───anyhow-cf3c20f70e89be43
│           │   ├───atomic-waker-e95759f92b68da78
│           │   ├───autocfg-ba5ee7d8b85f634f
│           │   ├───base64-89ad825825fe0ba3
│           │   ├───bitflags-86086ead68e92337
│           │   ├───bitflags-d15393d08e2f31fc
│           │   ├───block-buffer-e28c37018a32e06a
│           │   ├───brotli-ce3a917201e99a30
│           │   ├───brotli-decompressor-e3563f4a6e6cf9f5
│           │   ├───byteorder-6964839b8b19be1a
│           │   ├───bytes-d18dcb52e317328a
│           │   ├───camino-1f42e42fb414156d
│           │   ├───camino-8873ad4b31ddf564
│           │   ├───camino-b51df652bf17afdd
│           │   ├───cargo-platform-3c1deda8b78341fa
│           │   ├───cargo_metadata-68ede86c722cf98a
│           │   ├───cargo_toml-a477164668136af6
│           │   ├───cc-6a428b30622fd2bc
│           │   ├───cfb-0ba84e6767d4c496
│           │   ├───cfb-1d0e564e3e18ba16
│           │   ├───cfg-if-885ced7a34e6dac7
│           │   ├───cfg_aliases-85ec491f704a2d6b
│           │   ├───convert_case-ffa9445cd589c980
│           │   ├───cookie-84aad316634926e2
│           │   ├───cookie-991d259c3b26bddd
│           │   ├───cookie-bc683a131b4ddbd3
│           │   ├───cpufeatures-ecadcb609a73bfed
│           │   ├───crc32fast-38522adc696c8c22
│           │   ├───crossbeam-channel-11212871ae39e9ae
│           │   ├───crossbeam-utils-2915f5b5ef2a676d
│           │   ├───crossbeam-utils-52f34570ef26a1e2
│           │   ├───crossbeam-utils-836d87636bd098b4
│           │   ├───crypto-common-160b0070b8ef8b0e
│           │   ├───cssparser-245648d4f6282168
│           │   ├───cssparser-9860304fb6748866
│           │   ├───cssparser-a6be3a5f052b9cff
│           │   ├───cssparser-c2f3790229370681
│           │   ├───cssparser-macros-c76580689221abc1
│           │   ├───ctor-0afdbaf077195dd4
│           │   ├───darling-6ea4d80e47033f5f
│           │   ├───darling_core-f2c600c2fd0ceba3
│           │   ├───darling_macro-f221cb42c541b5dc
│           │   ├───deranged-a1859c4e0a116b3a
│           │   ├───derive_more-9503d6a6233130c8
│           │   ├───desqta-013e4646e2c70dfd
│           │   ├───desqta-06724d4cc8d7b5a1
│           │   ├───desqta-5a3137496ff09f6e
│           │   ├───digest-400feabaaaf06ca8
│           │   ├───dirs-9c87a6faf17d92d8
│           │   ├───dirs-dbb6a49c9537b700
│           │   ├───dirs-next-39a5894818edb716
│           │   ├───dirs-sys-1d36716bbbc9bb2b
│           │   ├───dirs-sys-863257ff307e57f8
│           │   ├───dirs-sys-next-a73d2807956ba79e
│           │   ├───displaydoc-5183406b2ae2b5cc
│           │   ├───dpi-e7405e9849be3fad
│           │   ├───dtoa-4843f453037243c1
│           │   ├───dtoa-short-357e87365f190ba4
│           │   ├───dunce-cde01fce8a9be1d0
│           │   ├───dyn-clone-cba93cd0e465feaa
│           │   ├───embed-resource-7d45dbee4833bf35
│           │   ├───encoding_rs-44070206e3fd8922
│           │   ├───equivalent-c413381703bf56a9
│           │   ├───erased-serde-325130d9a618b1bf
│           │   ├───erased-serde-e22f4e03847f458d
│           │   ├───fastrand-1d2c31c3dfe5dd91
│           │   ├───fdeflate-6c0b5ad5e8acb4d4
│           │   ├───flate2-7c76cac28e90e5a1
│           │   ├───fnv-d6becae5f753067f
│           │   ├───form_urlencoded-130c8eefe8412f78
│           │   ├───form_urlencoded-8a549036c9d51861
│           │   ├───futf-e8dae120705755ed
│           │   ├───futures-channel-181363e58a669c05
│           │   ├───futures-core-a0aadc232b8ba14e
│           │   ├───futures-io-5df57e6657e7aed5
│           │   ├───futures-lite-0287034705ab5581
│           │   ├───futures-macro-9025c6c156e684a8
│           │   ├───futures-sink-a3addf02a6f815fc
│           │   ├───futures-task-ba72005275e252c2
│           │   ├───futures-util-e36e526951e8ed46
│           │   ├───fxhash-48dd0c9a82e79a1b
│           │   ├───generic-array-5d37241524d72078
│           │   ├───generic-array-da0fa3a3c1d67262
│           │   ├───generic-array-f7c00ba5dc107a3f
│           │   ├───getrandom-22d05919b60adc02
│           │   ├───getrandom-39570cbb15d1c017
│           │   ├───getrandom-67b38228a8923159
│           │   ├───getrandom-7227783ca4181770
│           │   ├───getrandom-91e5009a59f1493b
│           │   ├───getrandom-d2035d6f9d82cd3f
│           │   ├───getrandom-f95aca9517f41846
│           │   ├───glob-603e7c1e77736211
│           │   ├───h2-4d430bee73a1ccfe
│           │   ├───hashbrown-38a3128d33becaf9
│           │   ├───hashbrown-69fd523d2cce3f60
│           │   ├───heck-ad6cb38f0d35f428
│           │   ├───html5ever-3c5cee7a927a0998
│           │   ├───html5ever-8540f8bb0e9f2ffa
│           │   ├───html5ever-a955fdac1b54a0f7
│           │   ├───html5ever-bf899283ce2a3ede
│           │   ├───http-78c62bdf4dfbc253
│           │   ├───http-body-643b7022a7f802e5
│           │   ├───http-body-util-daeae2f792c3c841
│           │   ├───httparse-2b5156c72c5d0c9c
│           │   ├───httparse-df7a6a039bff498a
│           │   ├───httparse-f14563759deb6f04
│           │   ├───hyper-24bae0da4100f8b9
│           │   ├───hyper-tls-f3d259405719df96
│           │   ├───hyper-util-004d1b1afc03c9f4
│           │   ├───ico-ab182942a249f01a
│           │   ├───icu_collections-cd5b24168a531e63
│           │   ├───icu_locale_core-90a57360225190e5
│           │   ├───icu_normalizer-191818c97e0e11cb
│           │   ├───icu_normalizer-8f5bd3a93662f203
│           │   ├───icu_normalizer_data-332f9891ac4e9e61
│           │   ├───icu_normalizer_data-75fbb0a487078202
│           │   ├───icu_normalizer_data-94cf1647bdfcde83
│           │   ├───icu_properties-9eb5371044ac60fa
│           │   ├───icu_properties_data-0af33b8292226d76
│           │   ├───icu_properties_data-666614c411bd8fff
│           │   ├───icu_properties_data-78e46d2e34107432
│           │   ├───icu_provider-52f1d07d0418d8c9
│           │   ├───ident_case-e4376db720288c8a
│           │   ├───idna-c41539deee0e02de
│           │   ├───idna-cead8e0f876899c2
│           │   ├───idna_adapter-528097ff6d1c1018
│           │   ├───idna_adapter-cc3b37f89f85e9bd
│           │   ├───indexmap-2769c497f6fc107d
│           │   ├───indexmap-4fde09b1a4e32048
│           │   ├───indexmap-55bdcc8d2468b5e5
│           │   ├───indexmap-5ddbcaad8ee24f20
│           │   ├───indexmap-b58531a49d120311
│           │   ├───indexmap-be68ec549dc8249a
│           │   ├───indexmap-df3e5eaed74a3bba
│           │   ├───infer-639079de2361ac2b
│           │   ├───infer-8f5c1d954f82fa10
│           │   ├───ipnet-fda9f50117118336
│           │   ├───itoa-06b1d72093d037ce
│           │   ├───itoa-0707f7a92355706b
│           │   ├───json-patch-6a4ecfd278bcc8a6
│           │   ├───json-patch-db8c2945f69dd8a3
│           │   ├───jsonptr-86942bab6994259f
│           │   ├───jsonptr-f0acf3b7e74db137
│           │   ├───keyboard-types-9df9e35f250f4603
│           │   ├───kuchikiki-5691da7bf738c5ae
│           │   ├───kuchikiki-7ef94c5304a8c4bd
│           │   ├───lazy_static-c28c0e13093a4168
│           │   ├───libc-0c82a85a0aaaa622
│           │   ├───libc-480ccc4107e8a74f
│           │   ├───libc-8e49c3ac649c7b9b
│           │   ├───litemap-a77567b62c2f8214
│           │   ├───lock_api-119ef4e2af9ec58b
│           │   ├───lock_api-9a26a4de427051ad
│           │   ├───lock_api-d2fd67df1cf0bc8a
│           │   ├───log-17408f5d9644b296
│           │   ├───mac-572bba28f7ca7ae4
│           │   ├───markup5ever-2610d7e644976a8d
│           │   ├───markup5ever-2fe7aad104fb1976
│           │   ├───markup5ever-60f3cd8f735a97ef
│           │   ├───markup5ever-ead3671b755587cd
│           │   ├───matches-1debe877e101b799
│           │   ├───memchr-d0055077279ca6ea
│           │   ├───mime-136eeb42bb314931
│           │   ├───miniz_oxide-2b8bde7438b543f8
│           │   ├───mio-8dbfc7c0b5edfa26
│           │   ├───muda-de85b87f5d306644
│           │   ├───native-tls-64f0e1e73c33b28e
│           │   ├───native-tls-8eeb18d790d7e531
│           │   ├───native-tls-f21272ed536a6d04
│           │   ├───new_debug_unreachable-cb906420d9772d9b
│           │   ├───nodrop-3baa0fa8aa9651ff
│           │   ├───notify-rust-32d4efba92f9e75a
│           │   ├───num-conv-aef5fdf2015cd21e
│           │   ├───once_cell-749b83ebf9ed78a6
│           │   ├───open-1da84c03dd410ca7
│           │   ├───option-ext-47f695b2156552fe
│           │   ├───parking-f1928c95bdcc5aba
│           │   ├───parking_lot-b09cb017d0571e9e
│           │   ├───parking_lot-dd10ba9e61a14976
│           │   ├───parking_lot_core-208d0614abf25f2d
│           │   ├───parking_lot_core-83220260dc85e1c4
│           │   ├───parking_lot_core-c409a69e19a952f8
│           │   ├───parking_lot_core-c4b4bf6626dc9db3
│           │   ├───percent-encoding-4c28601bc51ff47e
│           │   ├───percent-encoding-637e59c038477292
│           │   ├───phf-0b4ef414a59c2bd5
│           │   ├───phf-4abe6bd460fd1bbf
│           │   ├───phf-b339726d3fe513b9
│           │   ├───phf-df1383b23748ecbd
│           │   ├───phf_codegen-4d70b154099de2e0
│           │   ├───phf_codegen-fc0cc624f7d584fb
│           │   ├───phf_generator-718144699bd1df98
│           │   ├───phf_generator-757fc4b0458bc1de
│           │   ├───phf_generator-b29b19fa98007c4e
│           │   ├───phf_macros-01aa7e88e34e8129
│           │   ├───phf_macros-5f480f18cf42b9da
│           │   ├───phf_shared-06087baa44a20ec8
│           │   ├───phf_shared-753fcce526f8b9e5
│           │   ├───phf_shared-b25af8ec64c4dd9a
│           │   ├───phf_shared-c3e14428d7d14012
│           │   ├───pin-project-lite-7484e2466bb0ce7a
│           │   ├───pin-utils-b6e2238d34ce41c4
│           │   ├───png-3389c509d11301d5
│           │   ├───potential_utf-275ddff4a8ed6860
│           │   ├───powerfmt-d88dac87313a90a3
│           │   ├───ppv-lite86-0dff4c0751d26dcf
│           │   ├───precomputed-hash-bf2dd2f28690ebfb
│           │   ├───proc-macro-hack-96697eda46054250
│           │   ├───proc-macro-hack-fa269454effc8944
│           │   ├───proc-macro-hack-fac53b41d2378f6b
│           │   ├───proc-macro2-3a3ce87370a3e0ce
│           │   ├───proc-macro2-9c2600b01018d993
│           │   ├───proc-macro2-ae2daf3fd4822f3e
│           │   ├───quick-xml-83033a5a0f6427ed
│           │   ├───quote-1090f4776d77f528
│           │   ├───rand-7858bd45ae92deee
│           │   ├───rand-b75971639f9c73ba
│           │   ├───rand-c10d4f3d28466d40
│           │   ├───rand_chacha-1e4d6ef25fcea7b5
│           │   ├───rand_chacha-960e04254e5834a0
│           │   ├───rand_core-2ea2358024a7bc5e
│           │   ├───rand_core-fe900bb42eba261f
│           │   ├───rand_pcg-c0664030c30389b8
│           │   ├───raw-window-handle-e8c638c76537fe26
│           │   ├───regex-ad4182f1ee2bb2c8
│           │   ├───regex-automata-c85b1e74a40167d9
│           │   ├───regex-syntax-3804368148aaec6a
│           │   ├───reqwest-567386dbe328ec0c
│           │   ├───rustc_version-d96b2da9ebb1af19
│           │   ├───rustls-pemfile-b1b0e8ab9baf1fdb
│           │   ├───rustls-pki-types-512d69200aa9b15e
│           │   ├───ryu-cd11dc58f3c65c1f
│           │   ├───same-file-ce9e698a39e38e54
│           │   ├───same-file-d22bfd18656ec329
│           │   ├───schannel-208523d9d5639790
│           │   ├───schemars-5dbb942307511abe
│           │   ├───schemars-dd2da27c3d0b8cd2
│           │   ├───schemars-ee00b9fbc424a2bf
│           │   ├───schemars_derive-ae5751b141c568b9
│           │   ├───scopeguard-7e452dbf10560bbb
│           │   ├───selectors-733d4769d77407ca
│           │   ├───selectors-ca99bf712183894a
│           │   ├───selectors-cbd00c1735783093
│           │   ├───selectors-ff5e7d5ff3252990
│           │   ├───semver-0077db6ee1c9d2aa
│           │   ├───semver-0286b1041ee67cc9
│           │   ├───semver-47925e7c43d485ad
│           │   ├───semver-62f2e01527362871
│           │   ├───semver-68ebe31b77fadc59
│           │   ├───semver-83e4adb7e11c22c5
│           │   ├───serde-01d22dffb2f0f420
│           │   ├───serde-a4b52885f22edf1a
│           │   ├───serde-a6d946ec49c3a842
│           │   ├───serde-aa7a417492e75c30
│           │   ├───serde-ca58033216cc5f76
│           │   ├───serde-d4ff44a625c2f4c4
│           │   ├───serde-untagged-1c45a863a27b9cbf
│           │   ├───serde-untagged-51692d2eab2bae34
│           │   ├───serde_derive-27d0197ace73b8df
│           │   ├───serde_derive_internals-0b8e0f7d0ed51fd9
│           │   ├───serde_json-0ddcfd2f3b722ff1
│           │   ├───serde_json-4a10cb2d7b6223f5
│           │   ├───serde_json-59f450e854f1edae
│           │   ├───serde_json-60ef2ddd9e4a6b47
│           │   ├───serde_json-ccba958c39febe43
│           │   ├───serde_json-f775e973005b51be
│           │   ├───serde_repr-9c265e6f2c1e6d83
│           │   ├───serde_spanned-0eb62903dc7dc52c
│           │   ├───serde_spanned-43bb920da38d2f50
│           │   ├───serde_urlencoded-444aa1ad24eab1d0
│           │   ├───serde_with-7144eb174e9cc71b
│           │   ├───serde_with-ba49ea99a1ff8fa4
│           │   ├───serde_with_macros-37ffba8226bf0e2f
│           │   ├───serialize-to-javascript-44e107024436f098
│           │   ├───serialize-to-javascript-impl-4b02dca8f91aa568
│           │   ├───servo_arc-5c6eb8b36e96a7c5
│           │   ├───sha2-9c84ad3647a988d1
│           │   ├───shlex-f7812d5de40ee760
│           │   ├───simd-adler32-4ccfcfd652c022d1
│           │   ├───siphasher-8932516ccd747256
│           │   ├───siphasher-c22abea0fbce0dbc
│           │   ├───slab-79e9f1326b07f9da
│           │   ├───slab-af68c12364def23d
│           │   ├───slab-daf9e2e7c27fcc81
│           │   ├───smallvec-68fb1f410a81fc20
│           │   ├───smallvec-abef0c1c3dc0f172
│           │   ├───socket2-ef7d730fe9461d2c
│           │   ├───softbuffer-3c15d9c9488d6c52
│           │   ├───softbuffer-66cb3a694bb5c1f4
│           │   ├───softbuffer-8376d3944747bb96
│           │   ├───stable_deref_trait-37262a002198052c
│           │   ├───string_cache-136f944b35db11bc
│           │   ├───string_cache-2910597214e7fd96
│           │   ├───string_cache_codegen-f7a644abaac6fdb6
│           │   ├───strsim-f621e9dd16146f27
│           │   ├───syn-4c18b15f9938b276
│           │   ├───syn-67bc42f24052e48f
│           │   ├───syn-8dac8d106a163f4a
│           │   ├───syn-aa09cab961faeab9
│           │   ├───sync_wrapper-6f2a6492281c32ab
│           │   ├───synstructure-86449b83d5168d58
│           │   ├───tao-f7469b07291d71d4
│           │   ├───tauri-1605535ac03b4def
│           │   ├───tauri-559a98665d759ad1
│           │   ├───tauri-build-41f7b489a8d3c60c
│           │   ├───tauri-codegen-95f81f0bf604e758
│           │   ├───tauri-d1382184e1beef99
│           │   ├───tauri-macros-7c269807fb5073f0
│           │   ├───tauri-plugin-deep-link-68b3802563e859bd
│           │   ├───tauri-plugin-deep-link-a7c29d3ecef09352
│           │   ├───tauri-plugin-deep-link-c36936649f222d7a
│           │   ├───tauri-plugin-e96afa6f95d05da3
│           │   ├───tauri-plugin-notification-484d2209ad394160
│           │   ├───tauri-plugin-notification-8ca3a356fbf75f00
│           │   ├───tauri-plugin-notification-98954df206ce3f92
│           │   ├───tauri-plugin-opener-39b439acd83c4a71
│           │   ├───tauri-plugin-opener-947ba2d54e38b713
│           │   ├───tauri-plugin-opener-9c25f9fcfdf9e38e
│           │   ├───tauri-plugin-single-instance-f3dd86f771d40599
│           │   ├───tauri-runtime-125914ebfc6cdb0e
│           │   ├───tauri-runtime-75943df4f0b344f2
│           │   ├───tauri-runtime-b94a141332a6221d
│           │   ├───tauri-runtime-wry-76e08c4e5372928a
│           │   ├───tauri-runtime-wry-d8206410fcee719a
│           │   ├───tauri-runtime-wry-ef7dc3e3853c67ba
│           │   ├───tauri-utils-1fbefc5cf72e8688
│           │   ├───tauri-utils-d1258b957f11ad9b
│           │   ├───tauri-winres-5fdd6b40bfa9f2b4
│           │   ├───tauri-winrt-notification-f6dc637f51863a65
│           │   ├───tendril-0329000da5b5106e
│           │   ├───thin-slice-53efdea08ada6157
│           │   ├───thiserror-1654b9df548507c7
│           │   ├───thiserror-172bfa115b24c576
│           │   ├───thiserror-565f8cd4fb08e474
│           │   ├───thiserror-a4adc56dd3d8d03f
│           │   ├───thiserror-d5434fa7970f8e4d
│           │   ├───thiserror-e41546a061920656
│           │   ├───thiserror-impl-26dab6d449afa63a
│           │   ├───thiserror-impl-613f29ada679864f
│           │   ├───time-1d6d7c2ea1250751
│           │   ├───time-core-2ddc42bc423ca99e
│           │   ├───time-macros-24d7471a093f57a1
│           │   ├───tinystr-f0583de89e94ffa6
│           │   ├───tokio-a8f65cc48bef6593
│           │   ├───tokio-macros-689a7f0eb0a8555e
│           │   ├───tokio-native-tls-8dc7a8cdf21f18d9
│           │   ├───tokio-util-a056fa328d2f6936
│           │   ├───toml-22d6e48bae063b24
│           │   ├───toml-5c9f8d63d9286ea5
│           │   ├───toml_datetime-778d60907d48addc
│           │   ├───toml_datetime-8710ee96f40c4cf2
│           │   ├───toml_edit-c2febb532b4d53dd
│           │   ├───toml_edit-d1b4929bcf5b4488
│           │   ├───toml_write-3dc4abd1ef6a3376
│           │   ├───tower-8bcf2afc37cfc7b8
│           │   ├───tower-layer-933dde192a8fd827
│           │   ├───tower-service-f8512bed5f782afc
│           │   ├───tracing-9ca553641afd62e4
│           │   ├───tracing-attributes-f213f3c3901db0a7
│           │   ├───tracing-core-2e070b7cd536035b
│           │   ├───try-lock-cdfd0a4f8e6e7602
│           │   ├───typeid-4501528fe4c5fbe5
│           │   ├───typeid-816437f92dcf5abc
│           │   ├───typeid-d6fbee1d8432e325
│           │   ├───typenum-21539e70f5800262
│           │   ├───typenum-8a6e949b1837fb90
│           │   ├───typenum-9b4415a7dfd746b4
│           │   ├───unic-char-property-3f92e97cf017f90b
│           │   ├───unic-char-range-4e4efb7b66753361
│           │   ├───unic-common-e0e091273baa013d
│           │   ├───unic-ucd-ident-919342c58d2a0a45
│           │   ├───unic-ucd-version-78f21346618f02ae
│           │   ├───unicode-ident-65bfde4205dfbe5f
│           │   ├───unicode-segmentation-f315a29559cc3526
│           │   ├───url-08e5735a6af42241
│           │   ├───url-71901f36d4fb9b97
│           │   ├───urlencoding-3e05ff644c27a32d
│           │   ├───urlpattern-2a9c354d50ce1eaa
│           │   ├───urlpattern-3cd12ccc02719bc3
│           │   ├───utf-8-af98a19d2b6e4978
│           │   ├───utf8_iter-f251a36b2b336c04
│           │   ├───uuid-2dd52616be0d511a
│           │   ├───uuid-d3614515d7d5c4ce
│           │   ├───version_check-1214210d485f79ca
│           │   ├───vswhom-00e3c3720a8f68a2
│           │   ├───vswhom-sys-6a09da14d1e7622f
│           │   ├───vswhom-sys-8a387bfd3692209c
│           │   ├───vswhom-sys-f6186a9fcead940b
│           │   ├───walkdir-b70d48e69a33f864
│           │   ├───walkdir-bcfbe125aad739aa
│           │   ├───want-62aab1eac342ac11
│           │   ├───webview2-com-31d122e889787aa8
│           │   ├───webview2-com-macros-0e357e876f4f24ed
│           │   ├───webview2-com-sys-373af34a7375c901
│           │   ├───webview2-com-sys-9453e3fc819c0343
│           │   ├───webview2-com-sys-edbf22649da77a46
│           │   ├───winapi-0122aa7164306e8e
│           │   ├───winapi-271376a9e24b2939
│           │   ├───winapi-b46664574246d2cd
│           │   ├───winapi-util-68d17161287483f7
│           │   ├───winapi-util-e62dbf041efbfef6
│           │   ├───window-vibrancy-3bd475f0d72c6e4f
│           │   ├───windows-52c5ec91c5e8d913
│           │   ├───windows-920ab2bab3e1cd8c
│           │   ├───windows-collections-0de24c992b7ef329
│           │   ├───windows-collections-29b91f7183285158
│           │   ├───windows-core-65a2ca9ec19470f4
│           │   ├───windows-core-d4847c56634c4fe8
│           │   ├───windows-future-86d623890fa65b8a
│           │   ├───windows-future-8f1f2cc66c93e298
│           │   ├───windows-implement-14a70400894522e8
│           │   ├───windows-implement-adec792be6357edc
│           │   ├───windows-interface-f16e940b628e706c
│           │   ├───windows-link-8bb1a36473e33ba2
│           │   ├───windows-numerics-c277148a5232a56f
│           │   ├───windows-numerics-c3b94283c50ab19c
│           │   ├───windows-registry-45397260015dac4e
│           │   ├───windows-registry-e3fd31f4bb9297bf
│           │   ├───windows-result-21df02a86d6b3236
│           │   ├───windows-strings-7a6e7cc1522028f8
│           │   ├───windows-strings-7d6e18feb5f6d58d
│           │   ├───windows-sys-0c7dd696c5826187
│           │   ├───windows-sys-593086cc27309c95
│           │   ├───windows-sys-ae524478933a490a
│           │   ├───windows-sys-e384aa4023aaecda
│           │   ├───windows-targets-27818c136e6a585e
│           │   ├───windows-targets-433f1ae85898c523
│           │   ├───windows-targets-4aa2adf6f6e70891
│           │   ├───windows-version-24bb7dca8a29156b
│           │   ├───windows_x86_64_msvc-0096a9957e0fe082
│           │   ├───windows_x86_64_msvc-045fe7691f29799d
│           │   ├───windows_x86_64_msvc-104ee4ed1131b0ba
│           │   ├───windows_x86_64_msvc-327afbf1e279218b
│           │   ├───windows_x86_64_msvc-38fed6261898cfa6
│           │   ├───windows_x86_64_msvc-78735def9ce549d2
│           │   ├───windows_x86_64_msvc-85c5ca9db9287f2d
│           │   ├───windows_x86_64_msvc-88578c4a50efca04
│           │   ├───windows_x86_64_msvc-e9eedac884c9d4a8
│           │   ├───winnow-c9af2e5863d49789
│           │   ├───winreg-c97c3cfe01dd4fa6
│           │   ├───writeable-9a0a1d921e4f7e61
│           │   ├───wry-7c99f17d9f3e52e9
│           │   ├───wry-c85672cb441bef94
│           │   ├───wry-e0faae9daaba6390
│           │   ├───yoke-derive-0cf7331e51d4023e
│           │   ├───yoke-f81c4d4bfdb73380
│           │   ├───zerocopy-908c72dfb2d07b9a
│           │   ├───zerocopy-d472eddbe14685b6
│           │   ├───zerocopy-e33356bc59f929ef
│           │   ├───zerofrom-15ea17dc6568145b
│           │   ├───zerofrom-derive-1ec98d27145b4da5
│           │   ├───zeroize-c0933e4c0b86937a
│           │   ├───zerotrie-5a2e1735be0a4bbe
│           │   ├───zerovec-89360d8e9ab0184e
│           │   └───zerovec-derive-4cf8b7fc666b5326
│           ├───build
│           │   ├───anyhow-1637c556491076f6
│           │   ├───anyhow-cf3c20f70e89be43
│           │   │   └───out
│           │   ├───camino-1f42e42fb414156d
│           │   │   └───out
│           │   ├───camino-b51df652bf17afdd
│           │   ├───cookie-84aad316634926e2
│           │   │   └───out
│           │   ├───cookie-bc683a131b4ddbd3
│           │   ├───crossbeam-utils-2915f5b5ef2a676d
│           │   ├───crossbeam-utils-52f34570ef26a1e2
│           │   │   └───out
│           │   ├───cssparser-245648d4f6282168
│           │   ├───cssparser-c2f3790229370681
│           │   │   └───out
│           │   ├───desqta-013e4646e2c70dfd
│           │   │   └───out
│           │   │       └───app-manifest
│           │   ├───desqta-06724d4cc8d7b5a1
│           │   ├───generic-array-da0fa3a3c1d67262
│           │   ├───generic-array-f7c00ba5dc107a3f
│           │   │   └───out
│           │   ├───getrandom-67b38228a8923159
│           │   ├───getrandom-7227783ca4181770
│           │   │   └───out
│           │   ├───getrandom-91e5009a59f1493b
│           │   │   └───out
│           │   ├───getrandom-f95aca9517f41846
│           │   ├───html5ever-3c5cee7a927a0998
│           │   │   └───out
│           │   ├───html5ever-bf899283ce2a3ede
│           │   ├───httparse-df7a6a039bff498a
│           │   │   └───out
│           │   ├───httparse-f14563759deb6f04
│           │   ├───icu_normalizer_data-332f9891ac4e9e61
│           │   ├───icu_normalizer_data-94cf1647bdfcde83
│           │   │   └───out
│           │   ├───icu_properties_data-666614c411bd8fff
│           │   ├───icu_properties_data-78e46d2e34107432
│           │   │   └───out
│           │   ├───indexmap-2769c497f6fc107d
│           │   ├───indexmap-55bdcc8d2468b5e5
│           │   │   └───out
│           │   ├───indexmap-5ddbcaad8ee24f20
│           │   │   └───out
│           │   ├───indexmap-df3e5eaed74a3bba
│           │   ├───libc-0c82a85a0aaaa622
│           │   ├───libc-8e49c3ac649c7b9b
│           │   │   └───out
│           │   ├───lock_api-9a26a4de427051ad
│           │   ├───lock_api-d2fd67df1cf0bc8a
│           │   │   └───out
│           │   ├───markup5ever-60f3cd8f735a97ef
│           │   ├───markup5ever-ead3671b755587cd
│           │   │   └───out
│           │   ├───native-tls-8eeb18d790d7e531
│           │   │   └───out
│           │   ├───native-tls-f21272ed536a6d04
│           │   ├───parking_lot_core-208d0614abf25f2d
│           │   ├───parking_lot_core-83220260dc85e1c4
│           │   │   └───out
│           │   ├───proc-macro-hack-96697eda46054250
│           │   ├───proc-macro-hack-fac53b41d2378f6b
│           │   │   └───out
│           │   ├───proc-macro2-3a3ce87370a3e0ce
│           │   ├───proc-macro2-ae2daf3fd4822f3e
│           │   │   └───out
│           │   ├───schemars-5dbb942307511abe
│           │   │   └───out
│           │   ├───schemars-dd2da27c3d0b8cd2
│           │   ├───selectors-ca99bf712183894a
│           │   │   └───out
│           │   ├───selectors-cbd00c1735783093
│           │   ├───semver-0077db6ee1c9d2aa
│           │   ├───semver-62f2e01527362871
│           │   │   └───out
│           │   ├───semver-68ebe31b77fadc59
│           │   ├───semver-83e4adb7e11c22c5
│           │   │   └───out
│           │   ├───serde-01d22dffb2f0f420
│           │   ├───serde-a4b52885f22edf1a
│           │   ├───serde-aa7a417492e75c30
│           │   │   └───out
│           │   ├───serde-d4ff44a625c2f4c4
│           │   │   └───out
│           │   ├───serde_json-0ddcfd2f3b722ff1
│           │   ├───serde_json-4a10cb2d7b6223f5
│           │   │   └───out
│           │   ├───serde_json-59f450e854f1edae
│           │   ├───serde_json-f775e973005b51be
│           │   │   └───out
│           │   ├───slab-79e9f1326b07f9da
│           │   ├───slab-af68c12364def23d
│           │   │   └───out
│           │   ├───softbuffer-3c15d9c9488d6c52
│           │   │   └───out
│           │   ├───softbuffer-8376d3944747bb96
│           │   ├───syn-4c18b15f9938b276
│           │   │   └───out
│           │   ├───syn-8dac8d106a163f4a
│           │   ├───tauri-559a98665d759ad1
│           │   │   └───out
│           │   │       └───permissions
│           │   │           ├───app
│           │   │           │   └───autogenerated
│           │   │           │       └───commands
│           │   │           ├───event
│           │   │           │   └───autogenerated
│           │   │           │       └───commands
│           │   │           ├───image
│           │   │           │   └───autogenerated
│           │   │           │       └───commands
│           │   │           ├───menu
│           │   │           │   └───autogenerated
│           │   │           │       └───commands
│           │   │           ├───path
│           │   │           │   └───autogenerated
│           │   │           │       └───commands
│           │   │           ├───resources
│           │   │           │   └───autogenerated
│           │   │           │       └───commands
│           │   │           ├───tray
│           │   │           │   └───autogenerated
│           │   │           │       └───commands
│           │   │           ├───webview
│           │   │           │   └───autogenerated
│           │   │           │       └───commands
│           │   │           └───window
│           │   │               └───autogenerated
│           │   │                   └───commands
│           │   ├───tauri-d1382184e1beef99
│           │   ├───tauri-plugin-deep-link-a7c29d3ecef09352
│           │   │   └───out
│           │   ├───tauri-plugin-deep-link-c36936649f222d7a
│           │   ├───tauri-plugin-notification-8ca3a356fbf75f00
│           │   ├───tauri-plugin-notification-98954df206ce3f92
│           │   │   └───out
│           │   ├───tauri-plugin-opener-39b439acd83c4a71
│           │   ├───tauri-plugin-opener-947ba2d54e38b713
│           │   │   └───out
│           │   ├───tauri-runtime-125914ebfc6cdb0e
│           │   │   └───out
│           │   ├───tauri-runtime-75943df4f0b344f2
│           │   ├───tauri-runtime-wry-d8206410fcee719a
│           │   ├───tauri-runtime-wry-ef7dc3e3853c67ba
│           │   │   └───out
│           │   ├───thiserror-1654b9df548507c7
│           │   ├───thiserror-a4adc56dd3d8d03f
│           │   ├───thiserror-d5434fa7970f8e4d
│           │   │   └───out
│           │   ├───thiserror-e41546a061920656
│           │   │   └───out
│           │   ├───typeid-4501528fe4c5fbe5
│           │   │   └───out
│           │   ├───typeid-d6fbee1d8432e325
│           │   ├───typenum-21539e70f5800262
│           │   ├───typenum-8a6e949b1837fb90
│           │   │   └───out
│           │   ├───vswhom-sys-8a387bfd3692209c
│           │   │   └───out
│           │   ├───vswhom-sys-f6186a9fcead940b
│           │   ├───webview2-com-sys-9453e3fc819c0343
│           │   │   └───out
│           │   │       ├───arm64
│           │   │       ├───x64
│           │   │       └───x86
│           │   ├───webview2-com-sys-edbf22649da77a46
│           │   ├───winapi-271376a9e24b2939
│           │   ├───winapi-b46664574246d2cd
│           │   │   └───out
│           │   ├───windows_x86_64_msvc-045fe7691f29799d
│           │   │   └───out
│           │   ├───windows_x86_64_msvc-104ee4ed1131b0ba
│           │   │   └───out
│           │   ├───windows_x86_64_msvc-38fed6261898cfa6
│           │   ├───windows_x86_64_msvc-78735def9ce549d2
│           │   ├───windows_x86_64_msvc-85c5ca9db9287f2d
│           │   │   └───out
│           │   ├───windows_x86_64_msvc-e9eedac884c9d4a8
│           │   ├───wry-c85672cb441bef94
│           │   │   └───out
│           │   ├───wry-e0faae9daaba6390
│           │   ├───zerocopy-908c72dfb2d07b9a
│           │   └───zerocopy-e33356bc59f929ef
│           │       └───out
│           ├───deps
│           ├───examples
│           └───incremental
│               ├───build_script_build-1wqc6mrf8x4mv
│               │   └───s-h7uyhxxraa-0w32ysn-bitg5bl8qrly1c8tvoy97sr6q
│               ├───desqta-074e3fssbtg5w
│               │   └───s-h7uyiw5wua-0nx5ewl-3uab0zdm9ii57uq9rfykxqwem
│               └───desqta_lib-2faj6tu3cd65e
│                   └───s-h7uyigcmpa-0fgqynl-50mjm37336nscl8y76wltf0wk
└───static
    └───images
        └───editor
            └───commands

</details>

## Contributors

<a href="https://github.com/betterseqta/desqta/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=betterseqta/desqta" />
</a>

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=BetterSEQTA/desqta&type=Date)](https://star-history.com/#BetterSEQTA/desqta&Date)
