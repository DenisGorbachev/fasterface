# Secrets

Suppose the user wants to see his GitHub issues. For that, the user would have to provide a GitHub authentication token.

* The user should be able to enter the token in the UI if it's not present.
* The user should be able to provide the token via external source (e.g. env var, vault program)
* The token may expire, so the user might need to reenter the token.

The best solution is to accept the token from the context.
