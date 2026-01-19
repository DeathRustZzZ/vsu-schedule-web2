

const email = document.getElementById('email'),
      password = document.getElementById('password'),
          btn = document.getElementById('loginBtn');


function loginWithLocalStorage(){
    var head = new Headers();
           head.append("Content-Type", "text/html");
           if(localStorage.getItem("token") === null) return;
           var token = localStorage.getItem("token")
                          console.log(token)
                          head.append("Authorization", "Bearer "+token);

                          console.log(token)
                           var requestOptions = {
                            method: 'GET',
                            headers: head,
                            redirect: 'follow'
                          };
                          fetch("http://127.0.0.1:9898/schedule/admin", requestOptions)
                            .then(response => response.text())
                            .then(data => {
                                    window.location.href = "/schedule/admin"
                                    }
                                )

                            .catch(error => console.log('error', error));
                        }

btn.onclick = function() {
    var myHeaders = new Headers();
    myHeaders.append("Content-Type", "application/json");

    console.log(password.value);
    var raw = JSON.stringify({
      "login": email.value + "",
      "password": password.value + ""
    });

    var requestOptions = {
      method: 'POST',
      headers: myHeaders,
      body: raw,
      redirect: 'follow'
    };



    fetch("http://127.0.0.1:9898/schedule/auth", requestOptions)
      .then(response => response.json())
      .then(data => {
      console.log(data.token)
            localStorage.setItem("token", data.token)
            loginWithLocalStorage()
        }

      )
      .catch(error => console.log('error', error));

}
