# Paywall for Quarto blogs (under construction)
Pay-per-article paywall for [Quarto blogs](https://quarto.org/docs/websites/website-blog.html).

NOT READY for 'production', i.e. currently still a POC so don't try to monetize your content with it. There
is only a in-memory database in the backend, thus any data will be destroyed after the server shuts down.

Take a look at the [issue tracker](https://github.com/SaremS/quarto_paywall/issues) to see what is still missing - feel free to add more feature requests.

To run this on your own - see the attached `Dockerfile`, `docker-compose.yml` and `.env.example` files. 

An early stage rustdoc for the internals is available [here](https://sarems.github.io/quarto_paywall/rust_server/).

Tests are currently only run locally and coverage is still fairly poor. 
