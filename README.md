This is my take on emulating the 6502 in Rust, which can now execute a 6502 assembly script (in hexadecimal) from a text file! It can also alternatively run snake (assembly code for snake provided by wkjagt). Simply:

1. Run the program on the terminal, choose an option to either run a script from a text file or snake
2. If you choose to run a script, type and enter the name of your text file (including the extension)
3. The results of the script (the data of the registers after each instruction) will be printed on the terminal!

I treated it mostly as a learning experience in terms of Rust and the layout of the 6502 microprocessor. I also hope that my explanations in the code for the structure of the 6502 would be helpful for anyone else who would be interested.

The code already comes in with a built-in text file called script.txt. This text file has some example code to show you how to format your script and even comment it! It's almost like an interpreter. You can use this text file or create your own.

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


<img width="518" alt="assembly_text" src="https://github.com/user-attachments/assets/6d0a20e3-f94f-41eb-93fb-f4b48e65a066">
<img width="853" alt="6502_intro_part1" src="https://github.com/user-attachments/assets/b1de1c7f-6d1a-4618-baff-99ebad23a9b0">
<img width="837" alt="program_6502" src="https://github.com/user-attachments/assets/330bb7b3-71be-46ea-ab35-0a8e3c455ebf">
