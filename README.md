# clover_ui
CloverUI is meant to be a GUI engine that works similarly to how the web works. This was inspired from the fact that transitioning from web development to GUI libraries like egui or imgui has been extremely difficult, specially on the styling side of things.

With this GUI library, the idea is that you can build your element tree (similar to with HTML) and you can then attach styles, event handlers, and data. The engine will then take care of everything for you and render the proper things to the screen.

Additionally, I'd like to build the GUI library such that the UI looks good by default. I'm personally not a fan of the style ImGUI and eGUI have adopted. Once again, these should be extremely easy to swap should the user want to apply their own styles.

## Future Work
- [ ] Improve styles API to mimic CSS better (currently extremely annoying to do conditional styles)
- [ ] Compute styles from classes and IDs
- [ ] Allow users to overwrite event handlers
- [ ] Allow users to bind data to an element
- [ ] Build more Element types:
  - [ ] Button
  - [ ] Radio Button
  - [ ] Checkbox
  - [ ] Input
