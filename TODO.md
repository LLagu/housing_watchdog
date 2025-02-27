# Housing watchdog

This is an example of TODO.md

View the raw content of this file to understand the format.

### Todo
- [ ] Stress test
- [ ] Add relevant comments
- [ ] Add enum to cover for different find_all(By::)
- [ ] Save and load previous UI session values
- [ ] Code clean up and refactoring
  - [ ] Names
  - [ ] Modules
- [ ] Manage scraped_results fail
### In Progress

- [ ] Add GUI or terminal control (config filepath + <s>ntfy topic</s>)
  - [ ] Move `App` definition and impl to separate module
  - [ ] Rework UI colors and names
  - [ ] Make sparkline span its block width
  - [ ] Move logo to another module
- [ ] Add css exclusion parameter to config
- [ ] Fix pathing problems 

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
- [X] Add ntfy topic field to configuration and remove its UI element
- [X] Preventing logic stdout to print in the UI
- [X] Move ratatui logic to another module