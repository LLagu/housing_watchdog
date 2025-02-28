 # Housing watchdog

This is an example of TODO.md

View the raw content of this file to understand the format.

### Todo
- [ ] Stress test
- [ ] Write README
- [ ] Add relevant comments
- [ ] Add enum to cover for different find_all(By::)
- [ ] Code clean up and refactoring
  - [ ] Names
  - [ ] Modules
- [ ] Allow non-sudoers to run the program
- [ ] Add css exclusion parameter to config
  
### In Progress

- [ ] Rework UI colors and names
- [ ] Make sparkline span its block width
- [ ] Move logo to another module

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
- [X] Add GUI or terminal control (config filepath + <s>ntfy topic</s>)
- [X] UI work 
  - [X] Move `App` definition and impl to separate module
  - [X] Define basic structure
  - [X] Parametrize main logic function to accept the config path
  - [X] Link buttons to main logic
- [X] Add ntfy topic field to configuration and remove its UI element
- [X] Preventing logic stdout to print in the UI
- [X] Move ratatui logic to another module
- [X] Fix pathing problems 
- [X] Create util fn for prev_session path retrieval
- [X] Save and load previous UI session values
- [X] Manage scraped_results fail