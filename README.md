# Dockless

Dockless is a lightweight Platform as a Service (PaaS) designed for Raspberry Pi nodes, written in Rust. It enables users to easily manage, monitor, and deploy single-binary Rust applications on their Raspberry Pi devices. The project aims to simplify self-hosting for side projects by providing a predictable and minimal environment, without the complexity of containers or external runtimes. Dockless is intended to run as a system service, making it suitable for integration into custom OS images and automatic startup on boot.

## Why I am building this?

This project is to convert my Raspberry Pi into a simple server where i can host my side project with ease. This application will help in manage, monitor, and deploy the projects without and hassle. For now I have not planned to add any sandboxing or container. Also not supporting any runtime like node, or other. I want to deploy single binary rust applications for that building this as the output will be predictable.

As of now using Raspberry Pi Lite (64bit) OS but in future will make a custom so that this project can be baked into OS as it will automatically start on boot and will be the part of system.

## Progress so far

As I am currently working solo and aiming for my own use only but will generalize in future. You can checkout what is done so far in [TODO.md](./TODO.md).
