touch tests//.timeout
CMD="   /home/miguevr/UGRMPBase/NetBeansProjects/mylanguage0/dist/Debug/GNU-Linux/mylanguage0  < data/SimpleText.txt 1> tests//.out5 2>&1"
eval $CMD
rm tests//.timeout
