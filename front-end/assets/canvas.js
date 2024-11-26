var pressedMouse = false;
var x = 0, y = 0;
var colorLine = "#dc3545";
var key = { C: 67 };

document.addEventListener("mousedown", startDrawing);
document.addEventListener("mousemove", drawLine);
document.addEventListener("mouseup", stopDrawing);
document.addEventListener("keydown", clearCanvas);

function startDrawing(eventvs01) {
    try {
        let square = document.getElementById("drawPlace");
        let paper = square.getContext("2d");
        pressedMouse = true;
        var rect = square.getBoundingClientRect();
        x = eventvs01.clientX - rect.left;
        y = eventvs01.clientY - rect.top;
    } catch (e) {
        x = 0;
        y = 0;
        pressedMouse = false;
        console.log(e);
    }
}

function drawLine(eventvs02) {
    try {
        if (pressedMouse) {
            let square = document.getElementById("drawPlace");
            let paper = square.getContext("2d");
            square.style.cursor = "crosshair";
            var rect = square.getBoundingClientRect();
            console.log(rect.left);
            var xM = eventvs02.clientX - rect.left;
            var yM = eventvs02.clientY - rect.top;
            drawing_line(colorLine, x, y, xM, yM, paper);
            x = xM;
            y = yM;
        }
    } catch (e) {
        x = 0;
        y = 0;
        pressedMouse = false;
        console.log(e);
    }
}

function stopDrawing(eventvs03) {
    try {
        let square = document.getElementById("drawPlace");
        let paper = square.getContext("2d");
        pressedMouse = false;
        x = 0;
        y = 0;
        square.style.cursor = "default";
    } catch (e) {
        x = 0;
        y = 0;
        pressedMouse = false;
        console.log(e);
    }

}

function clearCanvas(whenPressKey) {
    try {
        let square = document.getElementById("drawPlace");
        let paper = square.getContext("2d");
        if (whenPressKey.keyCode == key.C) {
            paper.clearRect(0, 0, square.width, square.height);
        }
    } catch (e) {
        console.log(e);

    }
}

function drawing_line(
    color,
    x_start,
    y_start,
    x_end,
    y_end,
    board
) {
    try {
        board.beginPath();
        board.strokeStyle = color;
        board.lineWidth = 2;
        board.moveTo(x_start, y_start);
        board.lineTo(x_end, y_end);
        board.stroke();
        board.closePath();
    } catch (e) {
        console.log(e);
    }
}

// Set the canvas width and height to match the CSS dimensions
window.addEventListener("load", function () {
    try {
        let square = document.getElementById("drawPlace");
        let paper = square.getContext("2d");
        let rect = square.getBoundingClientRect();
        square.width = rect.width;
        square.height = rect.height;
    } catch (e) {
        console.log(e);
    }
});