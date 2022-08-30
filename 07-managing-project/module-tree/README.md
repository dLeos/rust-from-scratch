# Paths for Referring to an Item in the Module Tree

See project [../restaurant](../restaurant) for examples.

In this lesson I learned:
  - the *module tree* might remind you of the filesystem’s directory tree on your computer;
  - the root of module tree is implicit module named *crate*;
  - if module A is contained inside module B, we say that module A is the *child* of module B and that module B is the *parent* of module A;
  - we can define child modules inside parent modules;
  - modules can also hold definitions for other items, such as structs, enums, constants, traits, and functions;
  - to show Rust where to find an item in a module tree, we use a path. A path can take two forms:
    - an *absolute path* is the full path starting from a *crate* root;
    - a *relative path* starts from the current module and uses *self*, *super*, or an identifier in the current module;
  - both absolute and relative paths are followed by one or more identifiers separated by double colons (`::`).
  - in Rust, all items (functions, methods, structs, enums, modules, and constants) are *private* to parent modules by default;
  - items in a parent module can’t use the private items inside child modules, **but** items in child modules can use the items in their ancestor modules;
  - keyword `pub` used to make an item public.
