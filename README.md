# Scrollrack - Support your LGS

Scrollrack is an application to sort the cards you need by set so you can find them easier when buying them in your LGS

## Build

```
git clone https:://github.com/ramdambo/scrollrack.git
cd scrollrack
cargo build --release
```


## Example Usage


After building or downloading the binary, you can use the program like so

```bash
./path/to/scrollrack --path path/to/your/list.txt
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
	- Treasure Vault

Adventures in the Forgotten Realms Promos:
	- Treasure Vault

Aether Revolt:
	- Ornithopter
	- Metallic Rebuke

Antiquities:
	- Hurkyl's Recall
	- Ornithopter

Archenemy:
	- Aether Spellbomb

Born of the Gods:
	- Springleaf Drum

Classic Sixth Edition:
	- Ornithopter

Commander 2014:
	- Darksteel Citadel

Commander 2016:
	- Darksteel Citadel
	- Cranial Plating

Commander 2018:
	- Darksteel Citadel

Commander 2021:
	- Darksteel Citadel
	- Dispatch

Commander Anthology Volume II:
	- Darksteel Citadel

Darksteel:
	- Darksteel Citadel

Double Masters:
	- Glimmervoid
	- Cranial Plating
	- Welding Jar
	- Springleaf Drum
	- Metallic Rebuke
	- Darksteel Citadel

Duel Decks: Elspeth vs. Tezzeret:
	- Frogmite
	- Aether Spellbomb
	- Darksteel Citadel
	- Thoughtcast

Duel Decks: Elves vs. Inventors:
	- Darksteel Citadel

Duel Decks: Mirrodin Pure vs. New Phyrexia:
	- Memnite
	- Dispatch

Fifth Dawn:
	- Cranial Plating

Fifth Edition:
	- Hurkyl's Recall
	- Ornithopter

Foreign Black Border:
	- Ornithopter
	- Hurkyl's Recall

Fourth Edition:
	- Hurkyl's Recall
	- Ornithopter

Fourth Edition Foreign Black Border:
	- Ornithopter
	- Hurkyl's Recall

Heads I Win, Tails You Lose:
	- Shadowspear

Historic Anthology 1:
	- Ornithopter

Ikoria: Lair of Behemoths:
	- Jegantha, the Wellspring

Ikoria: Lair of Behemoths Promos:
	- Jegantha, the Wellspring

Innistrad: Crimson Vow:
	- Blood Fountain

Innistrad: Double Feature:
	- Blood Fountain
	- Pithing Needle

Innistrad: Midnight Hunt:
	- Pithing Needle

Innistrad: Midnight Hunt Promos:
	- Pithing Needle

Jumpstart:
	- Aether Spellbomb
	- Gingerbrute

Jumpstart: Historic Horizons:
	- Nettlecyst
	- Thought Monitor

Kaladesh Inventions:
	- Pithing Needle
	- Ornithopter

Kaladesh Remastered:
	- Metallic Rebuke
	- Ornithopter

Lorwyn:
	- Springleaf Drum

Magic 2010:
	- Pithing Needle
	- Ornithopter

Magic 2011:
	- Ornithopter

Magic 2015:
	- Ornithopter
	- Darksteel Citadel

Magic Online Promos:
	- Memnite
	- Shadowspear
	- Jegantha, the Wellspring

Mirrodin:
	- Thoughtcast
	- Ornithopter
	- Aether Spellbomb
	- Welding Jar
	- Frogmite
	- Glimmervoid

Mirrodin Besieged:
	- Tezzeret, Agent of Bolas

Modern Horizons 2:
	- Urza's Saga
	- Nettlecyst
	- Thought Monitor
	- Mistvault Bridge
	- Sojourner's Companion

Modern Horizons 2 Promos:
	- Thought Monitor
	- Nettlecyst
	- Urza's Saga

Modern Masters:
	- Glimmervoid
	- Frogmite
	- Aether Spellbomb

Modern Masters 2015:
	- Darksteel Citadel
	- Dispatch
	- Etched Champion
	- Hurkyl's Recall
	- Frogmite
	- Cranial Plating
	- Thoughtcast

Mystery Booster:
	- Aether Spellbomb
	- Thoughtcast
	- Metallic Rebuke
	- Darksteel Citadel
	- Ornithopter
	- Frogmite

Mystery Booster Retail Edition Foils:
	- Memnite

Mythic Edition:
	- Tezzeret, Agent of Bolas

Neon Dynasty Commander:
	- Dispatch
	- Thoughtcast

New Phyrexia:
	- Dispatch

Ninth Edition:
	- Ornithopter

Planechase:
	- Cranial Plating

Return to Ravnica:
	- Pithing Needle

Revised Edition:
	- Ornithopter
	- Hurkyl's Recall

Salvat 2011:
	- Ornithopter

Saviors of Kamigawa:
	- Pithing Needle

Scars of Mirrodin:
	- Memnite
	- Etched Champion

Scars of Mirrodin Promos:
	- Memnite

Secret Lair Drop:
	- Darksteel Citadel
	- Ornithopter
	- Pithing Needle

Summer Magic / Edgar:
	- Ornithopter
	- Hurkyl's Recall

Tenth Edition:
	- Hurkyl's Recall
	- Ornithopter
	- Pithing Needle

The List:
	- Cranial Plating
	- Memnite

Theros Beyond Death:
	- Soul-Guide Lantern
	- Shadowspear

Theros Beyond Death Promos:
	- Shadowspear

Throne of Eldraine:
	- Gingerbrute

Time Spiral Remastered:
	- Cranial Plating
```

## Future features

Check the issues to see what features I am planning to add in the future.
