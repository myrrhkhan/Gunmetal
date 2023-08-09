# Roadmap/code organization

## Adding Environment Variables

### Backend:

- Make an outer function to input environment variable and add -> string status
  - Function to check if already there (to prevent duplicates)
  - Function to check OS
  - Mac/Linux:
    - Helper function to check for config files
    - Function to make directory and config files if they don't exist
  - Check for settings file and give error if not and return "set a thing"
  - Set the thingy

### What to do when add env var is called

- Check if settings file exists - If settinge file does not exist - return "file does not exist" error - Redirect to settings page and prompt user to set everything - Check if setting for bash file exists - if yes, set the variable - if not, go to settings menu
  If cannot make file, don't crash but send a separate error message that gives a bigger alert
