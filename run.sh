export LD_LIBRARY_PATH="$(pwd)/target/release/"
echo "LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
cd src/ 
javac -h . MainClass.java
javac MainClass.java 
java MainClass -Djava.library.path="LD_LIBRARY_PATH"
cd ../