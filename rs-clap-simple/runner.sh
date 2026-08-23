cargo build

echo -e "\n\n---------------------------------------------------------------------"
echo "target/debug/rs-clap-simple -h"
target/debug/rs-clap-simple -h

echo -e "\n\n---------------------------------------------------------------------"
echo "target/debug/rs-clap-simple 0 -s 1 --can-be-long 2"
target/debug/rs-clap-simple 0 -s 1 --can-be-long 2


echo -e "\n\n---------------------------------------------------------------------"
echo "target/debug/rs-clap-simple 3 -s 4 -c 5"
target/debug/rs-clap-simple 3 -s 4 -c 5


echo -e "\n\n---------------------------------------------------------------------"
echo "target/debug/rs-clap-simple 6 -s 7 -c 8 'other text'"
target/debug/rs-clap-simple 6 -s 7 -c 8 'other text'

echo -e "\n\nError----------------------------------------------------------------"
echo "target/debug/rs-clap-simple"
target/debug/rs-clap-simple
