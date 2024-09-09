This is my take on emulating the 6502 in Rust, which can play snake! I wrote the functions for the opcodes, the massive reference table for the opcodes, the unit tests, and the overall structure for the code. 
I treated it mostly as a learning experience in terms of Rust and the layout of the 6502 microprocessor. 

# References and Sources Used 
Here are my major references concerning the overall organization and direction of the code. 
- [The Rust NES Book, by bugzmanov](https://bugzmanov.github.io/nes_ebook/chapter_1.html)
  <br>*Especially helpful when starting out in need of a guideline. Big help with this as well as using the actual SDL library*
- [The Github repository of the aforementioned Rust NES Book](https://github.com/bugzmanov/nes_ebook)
- [The NES Emulator from Scratch Series by javidx9](https://www.youtube.com/@javidx9)
  <br> *Explains the general concepts of the 6502 architecture very well*
- [The 6502 Instruction Set by Masswerk](https://www.masswerk.at/6502/6502_instruction_set.html)
- [The Snake Assembly Code by wkjagt](https://gist.github.com/wkjagt/9043907)
