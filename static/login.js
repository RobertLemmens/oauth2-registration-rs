function login() {
  var username = document.getElementById('usernameField').value
  var password = document.getElementById('passwordField').value
  // process password

  // Send
  const data = { user: username, pass: password }
  fetch('/server/api/v1/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(data),
  }).then(response => response.json())
    .then(data => {
      window.location.href = '/auth/authorize.html?otp='+data['token']+'&client_id='+get_parameter('client_id')+'&username='+username+'&redirect_uri='+get_parameter('redirect_uri');
    })
    .catch((error) => {
      console.error('Error', 'Something went wrong while logging in')
    });
}

function get_parameter(sParam)
{
    var sPageURL = window.location.search.substring(1);
    var sURLVariables = sPageURL.split('&');
    for (var i = 0; i < sURLVariables.length; i++) 
    {
        var sParameterName = sURLVariables[i].split('=');
        if (sParameterName[0] == sParam) 
        {
            return sParameterName[1];
        }
    }
}

function set_parameters() {
  document.getElementById("client_id").innerHTML = get_parameter('client_id');
  document.getElementById("redirect_uri").innerHTML = get_parameter('redirect_uri');
  document.getElementById("response_type").innerHTML = get_parameter('response_type');
  document.getElementById("scope").innerHTML = get_parameter('scope');
  document.getElementById("state").innerHTML = get_parameter('state');
}
