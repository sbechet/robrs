Attempt to rewrite Rob Hubbard player reversing `Monty_on_the_Run.sid`.

To play: `cargo run --bin robrs -- --help`.

WARN: I need help with sound accuracy. I don't know how `resid-rs` delta work.

For now only 1985 musics works.

- [x] Player working for `Monty on the Run` three musics
- [_] All SoundFX working (Monty On The Run 13, 14, 16 fx bugs counting from 0 kill resid-rs?) -- 7, 12, 16 for Commando
- [ ] Add musics --- Help Welcome, copy and modify `bin/montyontherun.rs`
    - [x] 1 - Delta (1987)
    - [x] 2 - Commando
    - [x] 3 - Zoids
    - [x] 4 - Sanxion (1986 Thalamus)
    - [ ] 5 - Lightforce (1986 Faster Than Light)
    - [x] 6 - The Last V8 (1985 MAD/Mastertronic) (my first RSID :p)
    - [ ] 7 - Spellbound (1986 MAD/Mastertronic)
    - [ ] 8 - Thrust (1986 Firebird)
    - [ ] 9 - International Karate (1986 System 3)
    - [ ] 10 - One Man & His Droid
    - [x] 11 - Thing on a Spring
    - [ ] 12 - Warhawk or Proteus (1986)
    - [ ] 13 - Phantoms of the Asteroids (1986)
    - [ ] 14 - Human Race
    - [ ] 15 - Rasputin
    - [ ] 12 - Master of Magic
    - [ ] 13 - Formula 1 Simulator
    - [x] 14 - Crazy Comets
    - [ ] Confuzion
    - [ ] Action Biker
    - [ ] Hunter Patrol
    - [ ] Chrimera
    - [ ] Battle of  ritain
    - [ ] Zoids
    - [ ] Game Killer
    - [ ] Gerry the Germ
    - [ ] Geoff Capes
    - [ ] Strongman Challenge
    - [ ] Kentilla
    - [ ] Bump Set and Spike
    - [ ] Video Poker
    - [ ] ACE II
    - [ ] Knucklebuster (1986)
    - [ ] Nemesis the warlok
- [ ] Convert to midi files 
- [ ] Full rewrite using rust idomatics (look about `bitbybit` to play with it).
- [ ] Create a RHPlayer file format?
- [ ] Simplify code to be 100% no_std compatible (see `resid-rs` state)?
- [ ] Find a simple solution to create sid files using this rewrite? (someone ready for a LLVM 6510 target? :p)?
- [ ] Write a simple SoundTracker using embedded-graphics?
- [ ] Change ALSA output to be Windows and Mac compatible?

