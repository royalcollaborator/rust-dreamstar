document.querySelectorAll(".user-btn").forEach((button) => {
    try {
        button.addEventListener("click", function (event) {
            event.preventDefault(); // Prevent default link behavior
            const dropdown = this.nextElementSibling;
            dropdown.classList.toggle("hidden");
        });
    } catch (e) {

    }
});

// Hide dropdown when the mouse leaves the dropdown area
document.querySelectorAll(".dropdown-menu").forEach((dropdown) => {
    try {
        dropdown.addEventListener("mouseleave", function () {
            this.classList.add("hidden");
        });
    } catch (e) {

    }
});

// Prevent dropdown from hiding when hovered over
document.querySelectorAll(".dropdown-menu").forEach((dropdown) => {
    try {
        dropdown.addEventListener("mouseenter", function () {
            this.classList.remove("hidden");
        });
    } catch (e) {

    }
});