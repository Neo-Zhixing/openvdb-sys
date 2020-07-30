## Installing Dependencies

### Ubuntu

```shell script
apt-get install cmake
apt-get install libilmbase-dev
apt-get install libtbb-dev
apt-get install libboost-system-dev     # Boost::system
apt-get install libboost-iostreams-dev  # Boost::iostream

wget https://github.com/Blosc/c-blosc/archive/v1.20.0.tar.gz
tar -xf v1.20.0.tar.gz
cd c-blosc-1.20.0
cmake
make
make install
cd ..
rm -rf v1.20.0.tar.gz c-blosc-1.20.0
```