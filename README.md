This is my take on emulating the 6502 in Rust, which can execute an assembly script (written in hexadecimal) from the user or play snake! Simply run the program on the terminal, choose an option (either writing or playing snake), and see the results. The results (the data of the registers after the last instruction) will be printed onto the terminal after each instruction. 

I treated it mostly as a learning experience in terms of Rust and the layout of the 6502 microprocessor. I also hope that my explanations in the code for the structure of the 6502 would be helpful for anyone else who would be interested.

In the future, I plan to make the program more functional by giving the option to execute assembly from a provided text file that is modifiable by the user!

# References and Sources Used 
Here are my major references concerning the overall organization and direction of the code. 
- [The Rust NES Book, by bugzmanov](https://bugzmanov.github.io/nes_ebook/chapter_1.html)
  <br>Especially helpful when it came to implementing the SDL library for snake
- [The Github repository of the aforementioned Rust NES Book](https://github.com/bugzmanov/nes_ebook)
- [The NES Emulator from Scratch Series by javidx9](https://www.youtube.com/@javidx9)
- [The 6502 Instruction Set by Masswerk](https://www.masswerk.at/6502/6502_instruction_set.html)
- [The Snake Assembly Code by wkjagt](https://gist.github.com/wkjagt/9043907)
  <br> The assembly code used for snake
- [6502 NES Reference](https://www.nesdev.org/obelisk-6502-guide/reference.html)
