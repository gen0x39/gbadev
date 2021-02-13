build:
	mkdir -p target
	arm-none-eabi-as src/crt.S -o target/crt.o
	

