# Paywall for Quarto blogs (under construction)
POC for a pay-per-article paywall for [Quarto blogs](https://quarto.org/docs/websites/website-blog.html).

TypeScript React + Rust + Quarto

Since Quarto exports static websites, we need to manage frontend interactivity completely on the client-side 
and cannot rely on some server-side templating framework. Current idea is to inject jquery into the Quarto
exports to manipulate existing HTML; in addition, we overlay the static export of a React user dashboard as a modal, 
which can then be created independently of the main Quarto website.

Rough to-dos:
- Finish auth backend
- Integrate user interface into quarto site sample
- Integrate with some payment provider (probably Stripe)
- Add paywall logic 

