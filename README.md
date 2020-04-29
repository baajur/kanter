# Kanter
A node based image editor for texture creation. It uses [Kanter Core](https://github.com/lukors/kanter_core) and [OrbTk](https://github.com/redox-os/orbtk).

# Focus
- Responsiveness - It should be fast to start, and snappy to use
- Simplicity - It should be easy to understand and not be bloated with unnecessary nodes or cluttered UI
- Completeness - It should have the tools to generate every kind of texture

# Progress
I'm currently building a basic user interface, here is the progress on that.

- [x] Saving & Loading graphs
- [x] Manipulating edges
- [ ] Adding & Deleting nodes
- [ ] Graph nodes support
- [ ] Exporting outputs
- [ ] Thumbnail previews on nodes
- [ ] Automatically recalculate changed nodes and their children
- [ ] Make it prettier

The plan for the 3D viewport is to create an addon for Blender so I can use their Eevee renderer as the 3D viewport instead of making my own.
