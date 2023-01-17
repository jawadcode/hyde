# The Process of Building a Hyde Project:

## Reading the config

1) Bail with an error saying the current directory is not a project if `hyde.toml` does not exist
2) Read `hyde.toml` into a `Config`

## Render the `index.html` template

1) Iterate over `$proj_dir/posts/`.entries()
  1) Filter out any entries that are `Err` (perhaps report using `eprintln!` or `log::warn!`)
  2) Collect into a list of recent posts (leaving the summaries blank)
  3) Unstable (not exactly likely 2 posts will be written at exactly the same time) sort the list
  n-place, using `datetime` from each post's frontmatter
  4) list = the last 5 posts from the list
2) Render the index template, passing the config (hyde.toml) and the list of recent posts, writing
it to `$proj_dir/static/index.html`

## Copy over auxilliary theme files

Check if `$proj_dir/static/` exists, if not, copy over all auxilliary entries in
`$proj_dir/themes/` and render all of the posts to `$proj_dir/static/posts/` and exit.

1) Iterate over the non-template/auxilliary entries in `$proj_dir/themes/`:
Just recursively copy the files from `$proj_dir/stat`
  1) ~~Get the filename, and grab the corresponding entry from `$proj_dir/static/`~~
  2) ~~if it exists, compare the last modified timestamps between the theme and static entry and copy~~
  ~~over the entry accordingly~~
  3) ~~if it does not exist, copy the entry over~~
4) Grab the template files' contents

## Re-rendering changes

Check if `$proj_dir/static/posts` exists, if not render all of the posts to
`$proj_dir/static/posts` and exit.

Define a list of posts

1) Iterate over `$proj_dir/posts/`.entries()
  1) Extract the file stem and grab the corresponding file from `$proj_dir/static/posts`
  2) if it exists, compare the markdown file's last modified timestamp with the html's and
  render the file over to the corresponding file accordingly
  3) if it does not, render the file over to `static/posts`
