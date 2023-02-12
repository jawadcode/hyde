# Hyde \[***WIP***\]

A simple SSG for creating blogs.

## Roadmap:

- [x] Create new hyde projects using `hyde new <name>`
- [x] Build the current directory as a project using `hyde build`
  - [x] Able to render markdown posts to HTML including relevant metadata
  - [x] Able to list the 5 most recent posts in `static/index.html`
- [ ] Serve the project using `hyde serve`
    - [ ] Simple webserver that hosts `static/`
    - [ ] `--watch` to enable hot reloading on changes to posts
      - [ ] Re-render the markdown on change and do some magic, probably using `tower-livereload`