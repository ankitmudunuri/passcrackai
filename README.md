# PassCrackAI
## A CLI-based Password Manager with AI-Powered Password Strength Estimation

#### This is a password manager, built in Rust, that uses a Command Line Interface (CLI) to add, remove, and display saved credentials for different domains. It uses Argon2 encryption in order to keep credentials encrypted and safe, and utilizes a trained CNN to estimate password strength. 
---

![image](https://github.com/user-attachments/assets/f16c1c05-4576-46f0-bf8a-c8a7825621c9)


### Setup
To start, download the binaries of whichever release you want (both setup.exe and passwordmanager.exe). Put both of those executables in a folder together and then run setup.exe. Finally, you can run passwordmanager.exe and it will work!
 
#### Note: If you want to update the version you can just delete the current version of the passwordmanager.exe file and replace it with the new file (I will make an easy installer soon).

### How to Use
On initialization, it will ask you to create a password. Enter the password that you want to use to log into the app. Once you have created a password, it will then print the table that will display your passwords. It will also prompt you with four options:
- 1: View a password based on keywords
- 2: Add a password to the bank
- 3: Remove a password from the bank (based on its domain)
- 4: Update a password (not working yet)
- 5: Quit and save changes

For viewing credentials, it will first prompt you with which field you want to search by (Domain, Username, or Password). It will then prompt you with what keyword or phrase you want to search by. For instance, if you select "Domain" and enter "Gmail" then it will get all entries with domains that contain "Gmail". This also works for partial searches, so if you enter "Gma" then it will still find all Gmail entries.

For adding credentials, it will ask you to enter the domain, username, and password respectively. Once that is done, it will be added to the bank and the table

For removing credentials, it will ask which domain's credentials you want to remove. Once you enter in the credentials, it will ask for confirmation. If you input 2 for "no" then it will abort the process, but if you input 1 for "yes" then it will go ahead and remove the entry from the credential bank.

If you want to quit, make sure that you don't press the X on the command prompt terminal, as that will not save your changes. Please go back to the main screen and input "5" to quit and save your changes.

Please do let me know if there are any bugs, or if you have any suggestions for features or fixes in the future. Thank you!
