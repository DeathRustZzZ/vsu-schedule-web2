const body = document.querySelector("body");
const modeToggle = body.querySelector(".mode-toggle");
const sidebar = body.querySelector("nav");
const logoutBtn = document.getElementById("logoutBtn");
const submitBtn = document.getElementById("submit_button");
const fileInput = document.getElementById("file-input");
const dropZone = document.getElementById("upload-container");
const facultySelect = document.getElementById("facult");

logoutBtn.onclick = function () {
    fetch("/schedule/auth/logout", { method: "GET", credentials: "include" })
        .then(() => {
            localStorage.removeItem("token");
            window.location.href = "/schedule/login";
        })
        .catch(error => console.log("error", error));
}

submitBtn.onclick = function () {

    sendFiles();
}


let mode = localStorage.getItem("mode");
if(mode ==="dark"){
    body.classList.toggle("dark");
}

let getStatus = localStorage.getItem("status");
if(getStatus && getStatus ==="close"){
    sidebar.classList.toggle("close");
}

modeToggle.addEventListener("click", () =>{
    body.classList.toggle("dark");
    if(body.classList.contains("dark")){
        localStorage.setItem("mode", "dark");
    }else{
        localStorage.setItem("mode", "light");
    }
});

fileInput.addEventListener("focus", () => {
    document.querySelector("label").classList.add("focus");
});
fileInput.addEventListener("blur", () => {
    document.querySelector("label").classList.remove("focus");
});

["drag", "dragstart", "dragend", "dragover", "dragenter", "dragleave", "drop"].forEach(eventName => {
    dropZone.addEventListener(eventName, (e) => {
        e.preventDefault();
        e.stopPropagation();
    });
});

dropZone.addEventListener("dragover", () => {
    dropZone.classList.add("dragover");
});
dropZone.addEventListener("dragenter", () => {
    dropZone.classList.add("dragover");
});
dropZone.addEventListener("dragleave", () => {
    dropZone.classList.remove("dragover");
});
dropZone.addEventListener("drop", (e) => {
    dropZone.classList.remove("dragover");
    const files = e.dataTransfer.files;
    sendFiles(files);
});

fileInput.addEventListener("change", () => {
    sendFiles(fileInput.files);
});

function sendFiles(files) {
    const facult = facultySelect ? facultySelect.value : "select";
    if (facult === "select") {
        alert("вы не выбрали факультет!");
        return;
    }
    if (!fileInput) {
        return;
    }
    const file = files && files.length ? files[0] : fileInput.files[0];
    if (!file) {
        alert("вы не выбрали файл!");
        return;
    }

    const data = new FormData();
    data.append("file", file, file.name);

    fetch(`/api/v1/schedule/uploadFile?f=${encodeURIComponent(facult)}`, {
        method: "POST",
        body: data,
        credentials: "include",
        redirect: "follow"
    })
        .then(response => {
            if (response.status === 200) {
                alert("Успешно!");
            } else if (response.status === 400) {
                alert("Ошибка заполнения или формата таблицы!");
            } else {
                alert("Ошибка загрузки.");
            }
        })
        .catch(error => console.log("error", error));
}
