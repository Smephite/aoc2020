#!/bin/bash
git diff --quiet
hadNoNonStagedChanges=$?
if ! [ $hadNoNonStagedChanges -eq 0 ]
then
   echo "* Stashing non-staged changes"
   git stash --keep-index -u > /dev/null
fi
(cargo fmt)
git diff --quiet
formatted=$?

echo "* Properly formatted?"

if [ $formatted -eq 0 ]
then
   echo "* Yes"
else
    echo "* No"
    echo "The following files need formatting (in stage or commited):"
    git diff --name-only
    echo ""
    echo "Please run 'cargo fmt' to format the code."
    echo ""
fi
git stash --keep-index > /dev/null
git stash drop > /dev/null
if ! [ $hadNoNonStagedChanges -eq 0 ]
then
   echo "* Scheduling stash pop of previously stashed non-staged changes for 1 second after commit."
   sleep 1 && git stash pop --index > /dev/null & # sleep and & otherwise commit fails when this leads to a merge conflict
fi
if [ $formatted -eq 0 ]
then
   echo "... done. Proceeding with commit."
   echo ""
   exit 0
else
   echo "... done."
   echo "CANCELLING commit due to NON-FORMATTED CODE."
   echo ""
   exit 1
fi