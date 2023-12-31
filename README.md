![cdlogo](https://carefuldata.com/images/cdlogo.png)

# morpho-web-lt

A version of morpho-web without curl and with higher port binding for non-root users.
This version of morpho also has no transactional logging.
If logging is needed, either add your own or see the regular morpho logging.

See the regular morpho-web here: https://github.com/jpegleg/morpho-web

cURL is used in the regular morpho-web as a health checking function which can perform
custom database healthchecks and so forth. Eliminating cURL reduces the
compilation complexity and eliminates openssl dependencies from the build. All
of the main TLS is with rustls.

The other difference from morpho-web with morpho-web-lt is that we bind port 3443 instead of 443.
This facilities non-root users binding the port.

## Compiling with cross

To compile for multiple architectures, we'll use `cargo cross`. Of course it can
be compiled normally as well.

## Project bobcat

See https://github.com/jpegleg/project-bobcat regarding small scale, low power, BSD deployments.
