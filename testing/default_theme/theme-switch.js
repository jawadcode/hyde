document.body.attributes["data-theme"].nodeValue = localStorage.getItem("theme") || "dark";

function toggle_theme() {
  if (localStorage.getItem("theme") === "dark")
    localStorage.setItem("theme", "light");
  else
    localStorage.setItem("theme", "dark");
  document.body.attributes["data-theme"].nodeValue = localStorage.getItem("theme");
}