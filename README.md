![cdlogo](https://carefuldata.com/images/cdlogo.png)

# morpho-web-lt

A version of morpho-web without curl and with higher port binding for non-root users.

See the regular morpho-web here: https://github.com/jpegleg/morpho-web

cURL is used in the regular morpho-web as a health checking function which can perform
custom database healthchecks and so forth. Eliminating cURL reduces the
compilation complexity and eliminates openssl dependencies from the build. All
of the main TLS is with rustls.

The other difference from morpho-web with morpho-web-lt is that we bind port 3443 instead of 443.
This facilities non-root users binding the port, which we'll use on on the FreeBSD compute nodes.

## Compiling with cross

To compile for multiple architectures, we'll use `cargo cross`. We'll do the compiling on
a separate FreeBSD build node and deploy the appropriate binary to the FreeBSD compute nodes.

