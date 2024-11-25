## EzGit

# About
Terminal User Interface util to help manage github repositories

# Installation Guide
EzGit depends on FTXUI:

- Navigate to EzGit/dependencies/ftxui
```
git clone https://github.com/ArthurSonzogni/FTXUI.git
cd FTXUI
mkdir build
cd build
cmake ..
make
cd ../
cp -r * ../
cd ../
rm -rf FTXUI
```
(Optional) 
```
sudo make install
```
To run the application you can create a build folder, navigate into it,
run 
```
cmake ..
make
```

