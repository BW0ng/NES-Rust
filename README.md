# Rust Emulation of Nintendo Entertainment System (NES)

## Description
Creating an Rust implementation of the NES

## Information
In 1985 Nintendo released a revolutionary gaming console called the NES (Nintendo Entertainment System). The NES was the first true color console to be released, and featured a 6-bit color scheme. This, coupled with other state of the art features, led the console to sell millions of units and become one of the most emulated consoles in history.

The Rust NES aims to recreate the original feel of the NES while also improving the outputs to modern interfaces. 

The project was divided into three major subsystems with two additional sections that were worked on in unison. These components are the Central Processing Unit (CPU), Audio Processing Unit (APU), and the Picture Processing Unit (PPU) along with the minor sections being the Controllers and the Memory Mappers. The CPU takes the data from the cartridges and converts it into actionable instructions that are passed through to the APU and PPU. The APU takes these instructions and uses its four channels to produce 8-bit audio and the PPU uses them to create the sprites and background to be outputted as a video signal.

## Authors
* Brandon Wong
