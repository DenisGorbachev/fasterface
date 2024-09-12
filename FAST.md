# Fasterface - a faster UI

Fasterface is a UI for power users. It features an always-on autocomplete over an extended filesystem tree. The extended tree includes websites, APIs and in-file items (e.g. Rust functions in *.rs files).

* Always-on search
* Extended tree
  * Examples
    * APIs
    * Databases
    * Functions within files
* Contextual search within the primary selected tree node
* Autofocus of newly created objects
  * Examples
    * Create a struct
    * Add a field on that struct

## Notes

* Indexing may take some time

## Questions

* How do we know if the user wants to execute the command on the currently selected node or on another node?
  * Options
    * Require to specify the selected node but allow

## Examples

### Add an argument to a Rust function

* Launch
* (maybe wait until the indexing is completed)
* cd to project directory

## Principles

* Zero-dep functions
* Unification of function interfaces by using the [most general type](#most-general-type)

## Most general type

Suppose we have two functions:

* `fn files(dir: &Path) -> Vec<PathBuf>`
* `fn issues(assignee: &str) -> Vec<Issue>`

Suppose we want to display the results in a table. However, `PathBuf` and `Issue` are different types, so we want to display a different table for each function. We can use a custom type `Table<T>`:

* `fn files(dir: &Path) -> Table<PathBuf>`
* `fn issues(assignee: &str) -> Table<Issue>`

Of course, both functions may fail, so they must return a `Result`:

* `fn files(dir: &Path) -> Result<Table<PathBuf>, FilesError>`
* `fn issues(assignee: &str) -> Result<Table<Issue>, IssuesError>`

Now we have two different error types: `FilesError` and `IssuesError`. We can unify the signatures by using a `Box<dyn Error>`:

* `fn files(dir: &Path) -> Result<Table<PathBuf>, Box<dyn Error>>`
* `fn issues(assignee: &str) -> Result<Table<Issue>, Box<dyn Error>>`

This is better, but still leaves us with different `Table<PathBuf>` and `Table<Issue>`. Let's replace them with `Table<dyn Row>`:

* `fn files(dir: &Path) -> Result<Table<dyn Row>, Box<dyn Error>>`
* `fn issues(assignee: &str) -> Result<Table<dyn Row>, Box<dyn Error>>`

Let's think about the implementation. The `issues` function needs to send an API request via `octocrab` (an external crate). The `octocrab` crate exposes `async` functions, so we have to make our `issues` async, too. To maintain compatibility, we'll make the `files` function async as well.

* `async fn files(dir: &Path) -> Result<Table<dyn Row>, Box<dyn Error>>`
* `async fn issues(assignee: &str) -> Result<Table<dyn Row>, Box<dyn Error>>`

The list of files can be very long, and it may take a while to load it. Same with issues. Ideally, we would want to show the first results as soon as they are available. For that, we need to return an async stream. The stream has to be encapsulated in the `Table` type because we would also need the metadata to display the table (a stream doesn't contain metadata, only data).

Other things to think about:

* Caching
* Background refresh (don't make the UI jump)
* Reusing results

General approaches:

* Wrap every type in `Result<T, fasterface::Error>`
* Wrap most types in async stream

Maybe approaches:

* Use a strict message-passing paradigm
  * Every item of the stream is a message

Next, notice that files can be loaded via local filesystem, but issues have to be loaded via network. Network calls

Notice that `Option<T>` can be converted to `Result<T, ()>`. Such conversion is lossless - we preserve full information. At the same time, we gain the ability to use `?` operator.

Notice that `Result<T, ()>` can't be converted to `Result<T, Box<dyn Error>>`. 
