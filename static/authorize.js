
function authorize() {
  const data = { token: get_parameter('otp')}
  fetch('/server/api/v1/authorize?device=' + get_parameter('device') + '&pcke=' + get_parameter('pcke') + '&client_id='+get_parameter('client_id')+'&username='+get_parameter('username'), {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(data),
  }).then(response => response.json())
    .then(data => {
      console.log('Ok!')
      console.log(data);
      window.location.href = get_parameter('redirect_uri')+'?token='+data['code'];
    })
    .catch((error) => {
      console.log(error)
      console.error('Error', 'Something went wrong while authorizing')
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
