pub const LOGIN_PAGE_HTML: &'static str = r#"
<html xmlns:th="http://www.thymeleaf.org">

<head th:fragment="head">
    <meta charset="utf-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />

    <!--[if lt IE 9]>
  <script src="https://oss.maxcdn.com/libs/html5shiv/3.7.2/html5shiv.js"></script>
  <script src="https://oss.maxcdn.com/libs/respond.js/1.4.2/respond.min.js"></script>
  <![endif]-->

    <link href="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap.min.css" rel="stylesheet"
        integrity="sha384-BVYiiSIFeK1dGmJRAkycuHAHRg32OmUcww7on3RYdg4Va+PmSTsz/K68vbdEjh4u" crossorigin="anonymous" />
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
    <title>Okta OIDC Samples for Golang!!!</title>
</head>

<body id="samples">

    <nav class="navbar navbar-default">
        <div class="container-fluid">
            <ul class="nav navbar-nav">
                <li><a href="/">Home</a></li>
            </ul>
            <form method="post" action="/logout" class="navbar-form navbar-right">
                <button id="logout-button" type="submit" class="btn btn-danger">Logout</button>
            </form>
        </div>
    </nav>

    <div id="sign-in-widget"></div>
    <script type="text/javascript">
        var config = {};
        config.baseUrl = "{{ .BaseUrl }}";
        config.clientId = "{{ .ClientId }}";
        config.redirectUri = "http://localhost:8080/authorization-code/callback";
        config.authParams = {
            issuer: "{{ .Issuer }}",
            responseType: 'code',
            state: "{{ .State }}" || false,
            display: 'page',
            scopes: ['openid', 'profile', 'email'],
            nonce: '{{ .Nonce }}',
            pkce: false,
        };
        new OktaSignIn(config).renderEl(
            { el: '#sign-in-widget' },
            function (res) {
            }
        );
    </script>

</body>
</head>
"#;
