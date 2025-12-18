const body = document.querySelector("body"),
      modeToggle = body.querySelector(".mode-toggle");
      sidebar = body.querySelector("nav");
      sidebarToggle = body.querySelector(".sidebar-toggle");
      logoutBtn = document.getElementById("logoutBtn");
      submitBtn = document.getElementById("submit_button")

logoutBtn.onclick = function () {
    token = localStorage.getItem("token");
    var myHeaders = new Headers();
    myHeaders.append("Content-Type", "application/json");
    myHeaders.append("Authorization", "Bearer " + token);
    myHeaders.append("Cookie", "JSESSIONID=C9A5B398E23E08EBC2392F912302C741");

let files = "";

    var requestOptions = {
      method: 'GET',
      headers: myHeaders,
      redirect: 'follow'
    };

    fetch("http://127.0.0.1:9898/rest/auth/logout", requestOptions)
      .then(response => response.text())
      .then(result => {
            localStorage.removeItem("token");
            window.location.href = "/login";


            })

      .catch(error => console.log('error', error));
}

submitBtn.onclick = function () {

    sendFiles()
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

   $('#file-input').focus(function() {
       $('label').addClass('focus');
   })
   .focusout(function() {
       $('label').removeClass('focus');
   });
   var dropZone = $('#upload-container');
   dropZone.on('drag dragstart dragend dragover dragenter dragleave drop', function(){
       return false;
   });
   dropZone.on('dragover dragenter', function() {
       dropZone.addClass('dragover');
   });

   dropZone.on('dragleave', function(e) {
       dropZone.removeClass('dragover');
   });
   dropZone.on('dragleave', function(e) {
       let dx = e.pageX - dropZone.offset().left;
       let dy = e.pageY - dropZone.offset().top;
       if ((dx < 0) || (dx > dropZone.width()) || (dy < 0) || (dy > dropZone.height())) {
            dropZone.removeClass('dragover');
       };
   });
   dropZone.on('drop', function(e) {
       dropZone.removeClass('dragover');
       files = e.originalEvent.dataTransfer.files;
       sendFiles(files);
   });
   $('#file-input').change(function() {
       files = this.files;
       sendFiles(files);
   });
function sendFiles() {
    let facult = document.getElementById("facult").value
    console.log(facult)
    if(facult === "select"){
        alert("вы не выбрали факультет!")
    }
    let Data = new FormData();
    var myHeaders = new Headers();
    myHeaders.append("Authorization", "Bearer " + localStorage.getItem("token"));
    myHeaders.append("Cookie", "JSESSIONID=32625BC457E59FAB133BD2B9C60A08A8");
    if(document.getElementById("file-input") === null){
        return // show exception on the page
    }
    let file =  document.getElementById("file-input").files[0]
    if(file === undefined){
        alert("вы не выбрали файл!");
    }

    Data.append('file', file,file.name);

    var requestOptions = {
      method: 'POST',
      headers: myHeaders,
      body: Data,
      redirect: 'follow'
    };

    fetch("http://127.0.0.1:9898/rest/uploadFile?f=" + facult, requestOptions)
      .then(response => {
            if(response.status == 200){
                alert("Успешно!")
            }
            if(response.status == 400){
                alert("Ошибка заполнения или формата таблицы!")
            }
      })
      .then(result => console.log(result))
      .catch(error => console.log('error', error));
};
let descr = document.querySelector('.descr');
let src = document.querySelector('.descr_src');
let button = document.querySelector('.switch');
let container = document.querySelector('.container ');

