@echo off

echo -------
echo -------

set Wildcard=*.rs

echo TODOS FOUND:
findstr -s -n -i -l "TODO" %Wildcard%

echo -------
echo -------
