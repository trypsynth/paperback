# Translating

To translate Paperback, the most important thing you need is the will to do it. This guide proves that the process isn't very difficult.

## Prerequisites

Before you begin, you must set up the infrastructure for translations so you can download, edit, and eventually upload them to GitHub. You will need the following:

* Git for Windows: [https://git-scm.com/install/windows](https://git-scm.com/install/windows)
* Poedit: [https://poedit.net/download](https://poedit.net/download)

Git is a GitHub tool that allows you to manage repositories on GitHub, while Poedit is software used for editing translations, most commonly `.po` files.

To set up Git, please refer to the [official documentation](https://docs.github.com/en/get-started/git-basics/set-up-git).

Once you have completed the setup and installed Poedit, you are ready for the first steps.

## Step 1: Forking the Project

To create a copy of the project where you can make changes, you must "fork" it. You can do this by going to the project's main page and selecting the "See your forks of this repository" option in the top right corner. Then, select "Create a new fork" and fill in the required details. This will associate the project with your GitHub profile under your own name.

## Step 2: Creating a Local Copy of the Repository

Before you start translating, you need to create a local copy of the project on your computer.

1. Open the Command Prompt (CMD);

2. Enter `cd` followed by the path where you want the repository to be located. For example:

```
cd C:\GitHub
```

3. Clone the repository:

```
git clone https://github.com/your-username/paperback.git
```

4. Git will create a folder with the repository's name (in this case, `paperback`), so you don't need to create one yourself.

5. Enter that folder:

```
cd paperback
```

Now you have a complete local copy of the project and are ready for the next steps.

## Translating

We will assume this is your first time translating and that you are adding a language that doesn't exist yet—for example, Danish.

Once the project is ready, what you need for the translation is the template file `paperback.pot`. It contains empty strings. The first time you open it, you will be asked to create a new language; select the language you are translating into. You can change this, along with other information, later using `Alt+Enter`.

Now, translate everything within that file, paying close attention to warnings and errors. Save the file using the format `<language-code>.po`. In this case, `da.po`. You do not need the file with the `.mo` extension. These are compiled translation files that cannot be edited directly but are used by programs to extract and display the translations.

## Working with Branches and Translations

Paperback uses specific branches for translations. For example, the Bosnian language uses a branch called `bosnian-translation`. Now we need to create a branch for Danish. You can name it anything, as long as it is meaningful and clearly describes the branch's purpose.

```
git checkout -b danish-translation
```

The branch is now created, and you have been automatically switched to it.

Once that's done, move the file you previously translated (in this case, `da.po`) into the `po` folder.

With the file in place, simply add it to a commit, commit the changes, and open a pull request. Follow these steps:

1. To add all changes in the entire project, use:

```
git add .
```

To add only a specific file, use:

```
git add <path/to/file>
```

In this case:

```
git add po/da.po
```

2. Next, commit the changes:

```
git commit -m "Added Danish translation"
```

The commit message in quotes should briefly explain what was changed.

3. Now, send the local changes to the remote repository using the following command:

```
git push origin danish-translation
```

Git will provide a link to create a pull request, which you can follow if you wish.

4. Finally, go to your repository on GitHub. A message will appear at the top stating that new changes have been made and that you can create a pull request.

5. Fill out the template and ensure all information is correct:

* **base repository:** the original project;
* **base branch:** `master`;
* **compare repository:** your fork of the original project;
* **compare branch:** your branch (e.g., `danish-translation`).

**Note:** If you are working with a fork and want to pull new changes from the original project, add the "upstream" remote:

```
git remote add upstream https://github.com/trypsynth/paperback.git
```

To check if it already exists, use:

```
git remote -v
```

To pull changes, use:

```
git fetch upstream
```

This allows you to get the latest updates before updating your translation. To push these updates to your own repository, type:

```
git merge upstream/master
```

## Updating an Existing Translation

If a translation already exists for a language and you want to update it, the process is similar to adding a new one, but you work on the same branch and with the existing `.po` file.

1. Switch to the branch containing the translation you wish to update. For example:

```
git checkout danish-translation
```

If you didn't start the translation yourself and don't know the branch name, you can check the remote branches:

```
git branch -r
```

Then switch to that branch.

2. Pull the latest changes from the remote repository to ensure you are working on the most recent version:

```
git pull origin danish-translation
```

3. Open the existing `.po` file in the `po` folder (e.g., `po/da.po`) using Poedit. Make your changes, add new translations, or fix errors.

4. Save the changes in the same file. Poedit will automatically generate a `.mo` file; you can delete it as it is not needed here.

5. Add the modified file to the commit:

```
git add po/da.po
```

6. Create a commit with a description of the changes:

```
git commit -m "Updated Danish translation"
```

7. Push the changes to the remote repository:

```
git push origin danish-translation
```

8. If a Pull Request (PR) for this branch is already open, the changes will be added automatically. If not, open a new PR to the original project following the same procedure used for adding a new translation.
