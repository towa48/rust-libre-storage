(function() {
    'use strict';

    document.addEventListener('DOMContentLoaded', onInit);

    function onInit() {
        var signInBtn = document.querySelector('#signin-btn');
        signInBtn.addEventListener('click', onSignInClick);
    }

    function onSignInClick() {
        var userNameField = document.querySelector('input[name="user"]');
        var passwordField = document.querySelector('input[name="password"]');

        var credentials = {
            user: userNameField.value,
            password: passwordField.value
        }

        fetch('/auth/token', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(credentials)
        })
        .then(response => response.json())
        .then(data => {
            console.log(data.token);
        });
    }

})();