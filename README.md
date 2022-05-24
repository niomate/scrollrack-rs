# Scrollrack
Scrollrack is an application to sort the cards you need by set so you can find 
them easier when searching through binders in your LGS.

## Build

```
git clone https:://github.com/ramdambo/scrollrack.git
cd scrollrack
cargo build --release
```

You can also just download the latest release as a binary.


## Example Usage


```bash
scrollrack-cli path/to/decklist.txt
```


The list should be in a simple textformat. You can comment lines by prefixing
them with "#" and also specify quantities (even though it is not really used
anywhere at the moment and this is just a remnant from how sites like moxfield
export decklists).

Example input file:

```txt
# Deck
4 Frogmite
1 Gingerbrute
4 Memnite
4 Ornithopter
4 Sojourner's Companion
4 Thought Monitor
4 Thoughtcast
1 Aether Spellbomb
4 Blood Fountain
4 Cranial Plating
2 Nettlecyst
1 Shadowspear
4 Springleaf Drum
1 Welding Jar
4 Darksteel Citadel
3 Glimmervoid
1 Island
2 Mistvault Bridge
4 Treasure Vault
4 Urza's Saga

# Sideboard
3 Dispatch
1 Etched Champion
2 Hurkyl's Recall
1 Jegantha, the Wellspring
4 Metallic Rebuke
1 Pithing Needle
1 Soul-Guide Lantern
2 Tezzeret, Agent of Bolas
```

Example output:

```txt
Adventures in the Forgotten Realms:
	- Treasure Vault (#261)

Adventures in the Forgotten Realms Promos:
	- Treasure Vault (#261a)

Aether Revolt:
	- Metallic Rebuke (#39)
	- Ornithopter (#167)

Antiquities:
	- Hurkyl's Recall (#10)
	- Ornithopter (#60)

Archenemy:
	- Aether Spellbomb (#102)

Born of the Gods:
	- Springleaf Drum (#162)

Classic Sixth Edition:
	- Ornithopter (#304)

Commander 2014:
	- Darksteel Citadel (#290)

Commander 2016:
	- Cranial Plating (#249)
	- Darksteel Citadel (#288)

Commander 2018:
	- Darksteel Citadel (#241)

Commander 2021:
	- Darksteel Citadel (#285)
	- Dispatch (#88)

Commander Anthology Volume II:
	- Darksteel Citadel (#243)

Darksteel:
	- Darksteel Citadel (#164)

Double Masters:
	- Cranial Plating (#245)
	- Darksteel Citadel (#315)
	- Glimmervoid (#319)
	- Metallic Rebuke (#59)
	- Springleaf Drum (#291)
	- Welding Jar (#307)

Duel Decks: Elspeth vs. Tezzeret:
	- Aether Spellbomb (#61)
	- Darksteel Citadel (#72)
	- Frogmite (#51)
	- Thoughtcast (#71)

Duel Decks: Elves vs. Inventors:
	- Darksteel Citadel (#65)

Duel Decks: Mirrodin Pure vs. New Phyrexia:
	- Dispatch (#23)
	- Memnite (#2)

Fifth Dawn:
	- Cranial Plating (#113)

Fifth Edition:
	- Hurkyl's Recall (#93)
	- Ornithopter (#393)

Foreign Black Border:
	- Hurkyl's Recall (#60)
	- Ornithopter (#270)

Fourth Edition:
	- Hurkyl's Recall (#77)
	- Ornithopter (#341)

Fourth Edition Foreign Black Border:
	- Hurkyl's Recall (#77)
	- Ornithopter (#341)

Heads I Win, Tails You Lose:
	- Shadowspear (#66)

Historic Anthology 1:
	- Ornithopter (#18)

Ikoria: Lair of Behemoths:
	- Jegantha, the Wellspring (#222)

Ikoria: Lair of Behemoths Promos:
	- Jegantha, the Wellspring (#222p)

Innistrad: Crimson Vow:
	- Blood Fountain (#95)

Innistrad: Double Feature:
	- Blood Fountain (#362)
	- Pithing Needle (#257)

Innistrad: Midnight Hunt:
	- Pithing Needle (#257)

Innistrad: Midnight Hunt Promos:
	- Pithing Needle (#257p)

Jumpstart:
	- Aether Spellbomb (#456)
	- Gingerbrute (#466)

Jumpstart: Historic Horizons:
	- Nettlecyst (#752)
	- Thought Monitor (#261)

Kaladesh Inventions:
	- Ornithopter (#42)
	- Pithing Needle (#44)

Kaladesh Remastered:
	- Metallic Rebuke (#56)
	- Ornithopter (#255)

Lorwyn:
	- Springleaf Drum (#261)

Magic 2010:
	- Ornithopter (#216)
	- Pithing Needle (#217)

Magic 2011:
	- Ornithopter (#211)

Magic 2015:
	- Darksteel Citadel (#242)
	- Ornithopter (#223)

Magic Online Promos:
	- Jegantha, the Wellspring (#80811)
	- Memnite (#37871)
	- Shadowspear (#79969)

Mirrodin:
	- Aether Spellbomb (#141)
	- Frogmite (#172)
	- Glimmervoid (#281)
	- Ornithopter (#224)
	- Thoughtcast (#54)
	- Welding Jar (#274)

Mirrodin Besieged:
	- Tezzeret, Agent of Bolas (#97)

Modern Horizons 2:
	- Mistvault Bridge (#249)
	- Nettlecyst (#231)
	- Sojourner's Companion (#235)
	- Thought Monitor (#71)
	- Urza's Saga (#259)

Modern Horizons 2 Promos:
	- Nettlecyst (#231s)
	- Thought Monitor (#71s)
	- Urza's Saga (#259s)

Modern Masters:
	- Aether Spellbomb (#196)
	- Frogmite (#207)
	- Glimmervoid (#223)

Modern Masters 2015:
	- Cranial Plating (#206)
	- Darksteel Citadel (#238)
	- Dispatch (#15)
	- Etched Champion (#209)
	- Frogmite (#215)
	- Hurkyl's Recall (#48)
	- Thoughtcast (#64)

Mystery Booster:
	- Aether Spellbomb (#1540)
	- Darksteel Citadel (#1662)
	- Frogmite (#1587)
	- Metallic Rebuke (#428)
	- Ornithopter (#1615)
	- Thoughtcast (#520)

Mystery Booster Retail Edition Foils:
	- Memnite (#108)

Mythic Edition:
	- Tezzeret, Agent of Bolas (#GR7)

Neon Dynasty Commander:
	- Dispatch (#83)
	- Thoughtcast (#99)

New Phyrexia:
	- Dispatch (#7)

Ninth Edition:
	- Ornithopter (#305)

Planechase:
	- Cranial Plating (#110)

Return to Ravnica:
	- Pithing Needle (#231)

Revised Edition:
	- Hurkyl's Recall (#60)
	- Ornithopter (#270)

Salvat 2011:
	- Ornithopter (#197)

Saviors of Kamigawa:
	- Pithing Needle (#158)

Scars of Mirrodin:
	- Etched Champion (#154)
	- Memnite (#174)

Scars of Mirrodin Promos:
	- Memnite (#174)

Secret Lair Drop:
	- Darksteel Citadel (#608)
	- Ornithopter (#604)
	- Pithing Needle (#44)

Summer Magic / Edgar:
	- Hurkyl's Recall (#60)
	- Ornithopter (#270)

Tenth Edition:
	- Hurkyl's Recall (#88)
	- Ornithopter (#336)
	- Pithing Needle (#338)

The List:
	- Cranial Plating (#342)
	- Memnite (#262)

Theros Beyond Death:
	- Shadowspear (#236)
	- Soul-Guide Lantern (#237)

Theros Beyond Death Promos:
	- Shadowspear (#236p)

Throne of Eldraine:
	- Gingerbrute (#219)

Time Spiral Remastered:
	- Cranial Plating (#392)

```
### Command Line arguments

- `-O/--ordering`: Valid values are `alpha`/`date`, order in which the sets are sorted in the output file
- `-o/--output`: Path to the output file
- `--inverted`: Inverted mapping (card -> set) (Not yet implemented)

## Future features

Check the issues to see what features I am planning to add in the future.
