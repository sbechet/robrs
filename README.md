Attempt to rewrite Rob Hubbard player reversing all SIDs.

To play: `cargo run --bin robrs -- --help`.

For the record, in songs, there is a bug concerning notes outside frequency conversion table.

Help Welcome: copy and add `OriginalSong` in `DATABASE` array (see `bin/database.rs`).

- [x] 0 - Monty on the Run (1985 Gremlin Graphics)
- [x] 1 - Delta (1987)
- [x] 2 - Commando (1985 Elite)
- [x] 3 - Zoids (1986 Martech)
- [x] 4 - Sanxion (1986 Thalamus)
- [x] 5 - Lightforce (1986 Faster Than Light)
- [x] 6 - The Last V8 (1985 MAD/Mastertronic)
- [x] 7 - Spellbound (1986 MAD/Mastertronic) (W.I.P)
- [ ] 8 - Thrust (1986 Firebird)
- [x] 9 - International Karate (1986 System 3)
- [ ] 10 - One Man & His Droid
- [x] 11 - Thing on a Spring (1985 Gremlin Graphics)
- [ ] 12 - Warhawk or Proteus (1986)
- [ ] 13 - Phantoms of the Asteroids (1986)
- [x] 14 - The Human Race (1985 Mastertronic)
- [ ] 15 - Rasputin
- [ ] 12 - Master of Magic
- [ ] 13 - Formula 1 Simulator
- [x] 14 - Crazy Comets (1985 Martech)
- [ ] Confuzion
- [ ] Action Biker
- [ ] Hunter Patrol
- [ ] Chrimera
- [ ] Battle of  ritain
- [ ] Game Killer
- [ ] Gerry the Germ
- [ ] Geoff Capes
- [ ] Strongman Challenge
- [ ] Kentilla
- [ ] Bump Set and Spike
- [ ] Video Poker
- [x] ACE II (1987 Arcade)
- [ ] Knucklebuster (1986)
- [ ] Nemesis the warlok
- [ ] Convert to midi files (W.I.P)
- [ ] Full rewrite using rust idomatics
- [ ] Find a simple solution to create sid files using this rewrite? (someone ready for a LLVM 6510 target? :p)?
- [ ] Write a simple SoundTracker using `embedded-graphics`?
- [ ] Change ALSA output to be Windows and Mac compatible (maybe a day...)
