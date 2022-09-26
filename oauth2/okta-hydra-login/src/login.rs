pub const LOGIN_PAGE_HTML: &'static str = r#"
<html>

<head th:fragment="head">
    <meta charset="utf-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />

    <script src="https://global.oktacdn.com/okta-signin-widget/5.2.0/js/okta-sign-in.min.js"
        type="text/javascript"></script>
    <link href="https://global.oktacdn.com/okta-signin-widget/5.2.0/css/okta-sign-in.min.css" type="text/css"
        rel="stylesheet" />
    <style>
        body.login {
            background-color: #f9f9f9;
        }

        #okta-sign-in {
            min-height: 0 !important;
        }
    </style>
    <title>Okta OIDC Sample for Rust</title>
</head>

<body>
<h1>OAuth2 Login</h1>
Please login with your credentials:
    <div id="sign-in-widget"></div>
<script type="text/javascript">
    var config = { };
    config.baseUrl = "{{ .baseURL }}";
    config.clientId = "{{ .clientID }}";
    config.redirectUri = "{{ .redirectUri }}";
    config.authParams = {
        issuer: "{{ .issuer }}",
    responseType: 'code',
    state: "{{ .state }}" || false,
    display: 'page',
    scopes: ['openid', 'profile', 'email'],
    nonce: '{{ .nonce }}',
    pkce: false,
    };
    new OktaSignIn(config).renderEl(
    {el: '#sign-in-widget' },
    function (res) {
    }
    );
</script>

</body>
"#;
