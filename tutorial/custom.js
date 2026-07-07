const kphisHomeIcon = document.createElement("i")
kphisHomeIcon.classList.add("fas","fa-house")
const kphisHome = document.createElement("a")
kphisHome.href = "/"
kphisHome.title = "Back to KPHIS"
kphisHome.ariaLabel = "Go to KPHIS"
kphisHome.appendChild(kphisHomeIcon)
const kphisHomeLabel = document.createElement("label")
kphisHomeLabel.classList.add("icon-button")
kphisHomeLabel.appendChild(kphisHome)
const mdbookThemeToggle = document.getElementById("theme-toggle")
mdbookThemeToggle.parentNode.insertBefore(kphisHomeLabel, mdbookThemeToggle)