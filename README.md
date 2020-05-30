# gdpr-authenticator

A page that displays the terms of usage/privacy policy of the site before users can enter. This software is to be used with nginx's [auth_request](https://nginx.org/en/docs/http/ngx_http_auth_request_module.html) module.

A minimal nginx configuration example is provided in `nginx.conf` (not needed for running this with BigBlueButton - for that, see the installation instructions below). The consent page that users see is in `src/index.html`.

## working principle
When nginx serves a location that is protected by an `auth_request /gdpr/check;` setting, it forwards the request to the local HTTP server provided by `gdpr-authenticator`. This tool then checks whether the consent cookie is present in the request headers. Nginx then determines based on the HTTP status code returned by `gdpr-authenticator`, whether to fulfill the original request. If the cookie is not present, `gdpr-authenticator` replies with HTTP status code 401, and this triggers nginx's own error 401, which the configuration file specifies should be turned into a "302 redirect" to the privacy policy consent page. When the user clicks the consent button on that page, a piece of JavaScript sets the consent cookie and redirects the user to their original destination.

## customizing
If you intend to use this package on a server that is not part of the Senfcall.de project, edit the file `src/setcookie.js` and change the part `domain=senfcall.de` to your domain. Edit `index.html` so it contains your own privacy policy and contact address.

### privacy policy
Note that default BigBlueButton/Greenlight installations **do not conform** to the privacy policy in `src/index.html`, and are likely not GDPR compliant. Extensive logging and recording is enabled by default. For some guidelines on how to improve BigBlueButton user privacy, see the [privacy guide](https://docs.bigbluebutton.org/admin/privacy.html). While you are welcome to use the privacy policy provided here as a basis for your own (see LICENSE file), make sure you understand both your privacy policy text and your server's configuration, in order to make sure they match.

## compiling and running
* It is recommended that you use the operating system version of your server for building this package, so that it will link to the correct versions of system libraries. So, to use this with a standard BigBlueButton server, **build on Ubuntu 16.04**.
* [Install Rust](https://www.rust-lang.org/learn/get-started)
* If you want to build the `.deb` package, install cargo-deb using `cargo install cargo-deb` (use Ubuntu 16.04 if you intend to do this)
* Compilation (choose that which applies):
  * to build for production (Ubuntu 16.04): `cargo deb`
  * to build and run when developing: `cargo run`
* The `.deb` file will end up in `target/debian/gdpr-authenticator_0.1.4_amd64.deb`. You can install it using `sudo dpkg -i target/debian/gdpr-authenticator_0.1.0_amd64.deb`, if you built it on the server. Otherwise, copy the deb file to the server and run the `dpkg` command there.
* The service will be enabled and running right after installation. To apply the consent wall to a web service, add `auth_request /gdpr/check;` to its `location` section, and make sure the file `/etc/bigbluebutton/nginx/gdpr-check.nginx` is included in its configuration. For an example on how to configure BigBlueButton's "Greenlight", see the file `greenlight.nginx` in this directory.
* The server listens on `127.0.0.1:7070`.
