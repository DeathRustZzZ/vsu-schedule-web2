const email = document.getElementById('email'),
      password = document.getElementById('password'),
          btn = document.getElementById('loginBtn');


btn.onclick = function() {
    var myHeaders = new Headers();
    myHeaders.append("Content-Type", "application/json");

    var raw = JSON.stringify({
      "login": email.value + "",
      "password": password.value + ""
    });

    var requestOptions = {
      method: 'POST',
      headers: myHeaders,
      body: raw,
      credentials: "include",
      redirect: 'follow'
    };



    fetch("/schedule/auth", requestOptions)
      .then(response => {
          if (response.status === 200) {
              window.location.href = "/schedule/admin";
              return;
          }
          if (response.status === 401) {
              alert("Неверный логин или пароль");
              return;
          }
          alert("Ошибка входа");
      })
      .catch(error => console.log('error', error));

}
