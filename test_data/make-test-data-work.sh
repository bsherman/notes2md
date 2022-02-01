#
# To make the dest data work as expecited, given it requires non-readable and non-file objects,
# run this script.
#
mkdir dir_you_cant_write; chmod 500 dir_you_cant_write
mkdir dir_you_cant_read; chmod 000 dir_you_cant_read
touch file_you_cant_read.txt; chmod 000 file_you_cant_read.txt
sudo mknod -m 444 tty-device c 4 0
