Attempt to rewrite Rob Hubbard player reversing `Monty_on_the_Run.sid`.

To play: `cargo run --bin robrs`.

To change music: edit `src/main.rs` and modify `player.init()` argument.



- [x] Player working for `Monty on the Run` three musics
- [-] All SoundFX working (13, 14, 16 fx bugs counting from 0 kill resid-rs?)
- [ ] Add musics --- Help Welcome, copy and modify `bin/montyontherun.rs`
    - [ ] Confuzion
    - [ ] Thing on a Spring
    - [ ] Action Biker
    - [ ] Crazy Comets
    - [ ] Commando
    - [ ] Hunter Patrol
    - [ ] Chrimera
    - [ ] The Last V8
    - [ ] Battle of  ritain
    - [ ] Human Race
    - [ ] Zoids
    - [ ] Rasputin,
    - [ ] Master of Magic
    - [ ] One Man & His Droid
    - [ ] Game Killer
    - [ ] Gerry the Germ
    - [ ] Geoff Capes
    - [ ] Strongman Challenge
    - [ ] Phantoms of the Asteroids
    - [ ] Kentilla
    - [ ] Thrust
    - [ ] International Karate
    - [ ] Spellbound
    - [ ] Bump Set and Spike
    - [ ] Formula 1 Simulator
    - [ ] Video Poker
    - [ ] Warhawk or Proteus
    - [ ] ...
    - [ ] ACE II seems to be a different code
- [ ] Full rewrite using rust idomatics (look about `bitbybit` to play with it).
- [ ] Create a RHPlayer file format?
- [ ] Simplify code to be 100% no_std compatible (see `resid-rs` state)?
- [ ] Find a simple solution to create sid files using this rewrite? (someone ready for a LLVM 6510 target? :p)?
- [ ] Write a simple SoundTracker using embedded-graphics?
- [ ] Change ALSA output to be Windows and Mac compatible?

