# Housing watchdog

This is an example of TODO.md

View the raw content of this file to understand the format.

### Todo
- [ ] Stress test
- [ ] Add relevant comments
- [ ] Add enum to cover for different find_all(By::)
- [ ] Save and load previous ui session values

### In Progress
- [ ] Code clean up and refactoring
  - [ ] Names
  - [ ] Modules
- [ ] Add GUI or terminal control (config filepath + ntfy topic)
  - [ ] Move `App` definition and impl to separate module
  - [ ] Add ntfy topic field to configuration
  - [ ] Rework UI colors and names
  - [ ] Make sparkline span its block width
  - [ ] Move ratatui logic in another module

### Done ✓

- [X] Define general code structure
- [X] Basic scraping
- [X] Notifications
- [X] Error handling
- [X] Save and load previous session results
- [X] Create previous session files if not present
- [X] Create main loop
- [X] Fix chromedriver port and add autostart
- [X] Add config file to define the scrapers' parameters
- [X] UI work 
  - [X] Define basic structure
  - [X] Parametrize main logic function to accept the config path
  - [X] Link buttons to main logic