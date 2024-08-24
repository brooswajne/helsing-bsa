# Something Awesome

Built for the Helsing "Bring Something Awesome" interview.

This is a demo of some concepts which I have built in my role at Intrasonics / Ipsos, which I am particularly excited about due to the massive developer-experience improvements they have brought:

1. **Automated full end-to-end testing against live (ephemeral) environments.**  \
   While unit tests / integration tests have a huge amount of value, I'm a strong proponent of making sure your test suite includes full end-to-end tests.
   There is no test more representative of actual user experience than a full end-to-end test which runs the application on a live environment (as it would in production).  \
   These can traditionally be quite difficult to orchestrate, but modern infrastructure-as-code tools have made it trivial to reproducibly spin up production-ready environments (and clean them up once done with them).
2. **Automated (continuous) deployments.**  \
   Setting up your project with CD early on not only saves valuable operational effort and time, but also enables a much quicker feedback and iteration cycle.
   Having your changes deployed to a staging environment as soon as they're merged makes it much easier to ask for feedback from stakeholders.

These leverage [Pulumi](https://www.pulumi.com/) as an infrastructure-as-code tool, but the same functionality could be achieved using any IaC tool.

The "product" being tested / deployed is also a demo of a cool concept in itself: dynamic, file-based routing.
This is a concept which has become very popular in the JavaScript front-end framework world, and I'm a big believer that it should make its way to the back-end side too.
It's an intuitive way to organise your files (removes ambiguity), which inherently forces you to consider which pieces of functionality and which routes belong together.

The file-based routing mechanism demonstrated uses the Rust language to build a stand-alone binary for each API route, which can then be deployed as a serverless (AWS Lambda) endpoint.
This mechanism is again inspired by something I built at Intrasonics / Ipsos, where our goals were the following:

- We wanted to migrate away from an existing API (built on technology which we could no longer afford to maintain).
- We wanted this migration to be as incremental as possible.
  To avoid the risk involved in one massive rewrite and switch-over, our goal was to migrate away from the existing API endpoint-by-endpoint.
  To make the migration seamless to clients, we chose to put an API Gateway in front of our existing API, which we could use to control (at an endpoint level) whether requests would be sent to the old API or to the new versions of the endpoint.
- We wanted this new API to have very low maintenance requirements, while still scaling to demand of individual endpoints.
  Our team was small, and a lot of our time was being spent on maintaining the existing API's servers.
  We decided to leverage cloud serverless technology (AWS Lambda) to allow individual endpoints to scale up / down as needed, and not require active maintenance.

## Other Ideas

The following is a small list of other things I considered using for this interview.
I'd still be happy to talk about any of them.

### Work Projects

These are things I built at work and stand out to me as being particularly cool, but obviously I can't share their source code.

- XLSX file templating engine
- In-browser ZIP extraction / resume-able upload
- Postgres-based queue

### Side Projects

These are some of the side projects I've started over the years.
I don't allocate much of my non-work time to programming, so have been terrible at actually finishing any of them.

- https://github.com/brooswajne/Blossom  \
  A toy programming language, built using Rust.
  Nowhere near working.
- https://github.com/brooswajne/ulti-td  \
  Tool to help organise and run Ultimate Frisbee tournaments.
  Nothing special, but I do really want to re-visit this at some point.
- https://github.com/brooswajne/nosh  \
  A shopping list application for me and my partner.
  Unfinished.
- https://github.com/brooswajne/wine  \
  An application for taking wine tasting notes, to help me slowly learn the differences between different grapes / regions.

### Other Ideas

These are some other side project ideas I've had, and considered building for the purpose of this BSA interview.
In the end, I decided that demo-ing something that I've already built, know quite well, and am proud of, would probably be a better idea.

- Savings tracker (with scraping)
- Holiday waypoints
