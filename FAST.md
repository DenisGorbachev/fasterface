# Fasterface - a faster CLI

Fasterface is an advanced user interface for power users. It features an always-on autocomplete over an extended filesystem tree. The extended tree includes websites, APIs and in-file items (e.g. Rust functions in *.rs files).

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
