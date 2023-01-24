let theme_toggle = "dark";
function toggle_theme() {
  if (theme_toggle === "dark") {
    theme_toggle = "light";
  } else {
    theme_toggle = "dark";
  }
  document.body.attributes["data-theme"].nodeValue = theme_toggle;
}