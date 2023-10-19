function toggle(id) {

    const boxes = document.getElementsByClassName('toggle');

    for (const box of boxes) {
        box.style.display = 'none';
    }

    var x = document.getElementById(id);
    if (x.style.display === "none") {
        x.style.display = "block";
    } else {
        x.style.display = "none";
    }
}
