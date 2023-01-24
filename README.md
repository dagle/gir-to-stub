gir-to-stub
===========
gir-to-stub is a tool for working with gobjects in dynamic languages. 

For static languages (like go, rust, etc) gir generates code that the user can
then include in their project. When working with a dynamic language, a typelib is used instead.

The idea behind gir-to-stub is to generate a stub file for the gir file for your dynamic language. 
Think of it like a header file to your dynamic language. The reasoning for this is that having code
(the header file) in the native language lets you use the native tools that the language would give. 
These include (but not limited to) the ability to generate documentation using native tools, 
lsp support and debug information.

gir-to-stub tries to gather as much information as possible that makes sense to language you are using,
so not only types are gathered but also documentation.

gir-to-stub currently only work with lua and that is my focus (because that my usecase).
When the interface is ergonomic enough, support for more languages will be added.
