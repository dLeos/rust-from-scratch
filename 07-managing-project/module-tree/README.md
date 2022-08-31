# Paths for Referring to an Item in the Module Tree

See project [../restaurant](../restaurant) for examples.

Module tree:
  - the *module tree* describes the structure of modules in the project, and it is similar to the filesystem’s directory tree on a computer;
  - the root of module tree is implicit module named *crate*;
  - if module A is contained inside module B, we say that module A is the *child* of module B and that module B is the *parent* of module A;
  - child modules are defined inside parent modules;
  - modules can also hold definitions for other items, such as structs, enums, constants, traits, and functions.

Paths:
  - to show Rust where to find an item in a module tree, we use a path. A path can take two forms:
    - an *absolute path* is a full path starting from a *crate* root;
    - a *relative path* starts from the current module and uses *self*, *super*, or an identifier in the current module;
  - both absolute and relative paths are followed by one or more identifiers separated by double colons (`::`);
  - the keyword `super` allows you to reference an item in parent module.

Private and public:
  - in Rust, all items (functions, methods, structs, enums, modules, and constants) are *private* to parent modules by default;
  - items in a parent module can’t use the private items inside child modules, **but** items in child modules can use the items in their ancestor modules;
  - keyword `pub` used to make an item public;
  - *struct* field by default private, even if the struct is public. Each field should be explicitly described with the `pub` key;
  - *enum* variants are automatically public if the enum is public.
