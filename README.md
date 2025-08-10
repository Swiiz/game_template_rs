# **🦀 Rust Game Template 🎮**

This repository offers a **minimal Rust game template** to speed-up your development by providing already setup wgpu (rendering), winit (windowing), and egui (debug UI). Use this if you want to build a game or gpu application entirely from scratch.

## **Features ✨**

* **wgpu context:** Ready-to-use rendering instance, adapter, device, and queue.  
* **winit window:** Basic window creation and event loop handling.  
* **egui integration:** For debug interfaces and simple in-game tools.

## **How to Use 🚀**

### **1\. Clone the Repository**

```
git clone https://github.com/Swiiz/game_template_rs.git  
cd game_template_rs
```

### **2\. Rename Your Game 🎮**

Update `Cargo.toml` with your game's name:

```TOML
[package]  
name = "your_game_name" # <--- Change this line!  
version = "0.1.0"  
edition = "2024"

[dependencies]  
...
```

### **3\. Make your game 🛠️**

After cloning and renaming, dive into src/lib.rs to add your game logic, use wgpu for rendering and create debug UIs with egui.


```
cargo run
```