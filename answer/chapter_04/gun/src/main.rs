use std::cell::{Cell, RefCell};
use std::io;
use std::io::prelude::*;

#[derive(Debug, Clone, Copy)]
enum GunExtra {
    Silencer, // 消音器
    Scope, // 瞄准镜
    ExtendedMag { round: u32, extra_recoil_time: f64 }, // 扩展弹匣
    None
}

#[derive(Debug)]
struct Gun {
    gun_type: String,
    recoil_time: Cell<f64>, // second
    magazine_size: Cell<u32>, // round
    extra: RefCell<GunExtra>
}

fn main() {
    let mut gun = Gun {
        gun_type: "AK47".to_string(),
        recoil_time: Cell::new(1.2),
        magazine_size: Cell::new(30),
        extra: RefCell::new(GunExtra::None)
    };
    let ak47_ex_mag = GunExtra::ExtendedMag { round: 30, extra_recoil_time: 1.6 };
    println!("Gun: {:?}", gun);
    loop {
        
        println!("1 -> Silencer");
        println!("2 -> Scope");
        println!("3 -> ExtendedMag");
        print!("Choose an Extra(1-3): ");
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).ok().expect("Failed to read input.");

        // unload extra
        match *gun.extra.borrow() {
            GunExtra::ExtendedMag { round, extra_recoil_time } => {
                gun.magazine_size.set(gun.magazine_size.get() - round);
                *gun.recoil_time.get_mut() -= extra_recoil_time;
            },
            _ => {}
        }

        // load extra according to user input
        match input.trim() {
            "1" => {
                *gun.extra.get_mut() = GunExtra::Silencer;
            },
            "2" => {
                *gun.extra.get_mut() = GunExtra::Scope;
            },
            "3" => {
                match ak47_ex_mag {
                    GunExtra::ExtendedMag { round, extra_recoil_time } => {
                        gun.magazine_size.set(gun.magazine_size.get() + round);
                        gun.recoil_time.set(gun.recoil_time.get() + extra_recoil_time);
                        *gun.extra.get_mut() = ak47_ex_mag;
                    },
                    _ => {}
                }
            },
            _ => {
                *gun.extra.get_mut() = GunExtra::None;                
                println!("GUN EXTRA UNLOADED!");
            }
        }
        println!("Gun: {:?}", gun);
    }
}
