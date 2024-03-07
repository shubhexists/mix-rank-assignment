# Overview

The goal of this project is to build a web app to render competitive matrices in a browser:

![Payments Competitive Matrix](payments-attrition-matrix.png "Payments Competitive Matrix")

This matrix was generated from a sample of apps that use Stripe, PayPal, or Braintree for payments, or none of the above.

Examples of facts we can read from this matrix:
* Stripe has 10,572 apps currently using their SDK (of the sampled apps)
* PayPal has 11,892 apps currently using their SDK
* Stripe churned 22 customers to PayPal
* Stripe churned 8,082 customers to another solution not covered in this matrix
* PayPal acquired 11,844 app integrations from another solution not covered in this matrix
* 879,587 apps from the sample haven't integrated any of these three payments SDKs

This matrix was generated using matplotlib. We'd like to add a similar matrix into our web-based product, but matplotlib isn't a good solution because the images it generates aren't interactive. We'd prefer a solution that was rendered client-side.

# Objectives

* Please build an API in Python that serves requests using the data in the SQlite file in this repository.
* The client should render a competitive matrix similar in spirit to the example above.
* Consider shading the cells according to magnitude.
* Allow the user to select a set of SDKs to include in the matrix, and re-render as they change their selection.
* Allow the user to view example apps for a cell of the matrix they select.
* Compute a normalized matrix (by row) to better visualize retention and attrition rates. Ex: ![Normalized Payments Matrix](payments-norm-matrix.png "Normalized Payments Matrix")
* Be prepared to discuss the bottlenecks as you scale up to a larger dataset (1e5 `sdk`s, 1e7 `app`'s, and 1e9 `app_sdk`'s).  Will your implementation still perform well? What would you have to change to improve performance? You don't need to implement all the optimizations, but be able to talk about the next steps to scale it for production.
* Consider how the app might support updating the view as the data in the database updates, without requiring a page refresh. You don't have to implement this, but keep it in mind for discussion.
* We use [Nix](https://nixos.org/nix/) for package management. If you add your dependencies to `default.nix`, then it's easy for us to run your code. Install nix and launch the environment with `nix-shell` (works on Linux, macOS, and most unixes).

Feel free to complete as much or as little of the project as you'd like. Spare your time from implementing features that would be time consuming or uninteresting, and focus instead on parts that would make for better discussion when reviewing together. Make notes of ideas, bugs, and deficiencies to discuss together.

We recommend not using GPT, Copilot, or similar tools to generate code for this project, but if you do, please label that code clearly so we know what code you personally wrote. That code is rarely up to our standard, so we don't want it to reflect negatively on our assessment.

There's no time limit. Spend as much or as little time on it as you'd like. Clone this git repository (don't fork), and push to a new repository when you're ready to share. We'll schedule a follow-up call to review.
