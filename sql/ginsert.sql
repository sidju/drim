
INSERT INTO Clans VALUES('Tremere');
INSERT INTO Clans VALUES('Ventrue');
INSERT INTO Clans VALUES('Toreador');
INSERT INTO Clans VALUES('Malkavian');
INSERT INTO Clans VALUES('Brujah');
INSERT INTO Clans VALUES('Gangrel');
INSERT INTO Clans VALUES('Caitiff');
--The one you pretend to be part of. If sabbat spy some extra work is needed.--

INSERT INTO Sects VALUES('Camarillan');
INSERT INTO Sects VALUES('Anark');
INSERT INTO Sects VALUES('Oberoende');
INSERT INTO Sects VALUES('Sabbat');

INSERT INTO Disciplines VALUES('Thaumaturgy');
INSERT INTO Disciplines VALUES('Dominate');
INSERT INTO Disciplines VALUES('Auspex');
INSERT INTO Disciplines VALUES('Presence');
INSERT INTO Disciplines VALUES('Animalism');
INSERT INTO Disciplines VALUES('Celerity');
INSERT INTO Disciplines VALUES('Dementation');
INSERT INTO Disciplines VALUES('Fortitude');
INSERT INTO Disciplines VALUES('Obfuscate');
INSERT INTO Disciplines VALUES('Potence');
INSERT INTO Disciplines VALUES('Protean');
--More can be added when they are needed--

INSERT INTO Paths VALUES('The Green Path');
INSERT INTO Paths VALUES('Technomancy');
INSERT INTO Paths VALUES('Blood');
INSERT INTO Paths VALUES('Levinbolt');
INSERT INTO Paths VALUES('Mortal Shell');
INSERT INTO Paths VALUES('Focused Mind');
INSERT INTO Paths VALUES('Movement of Mind');
INSERT INTO Paths VALUES('Corruption');
INSERT INTO Paths VALUES('Bloods Curse');
INSERT INTO Paths VALUES('Spirit');
INSERT INTO Paths VALUES('Hearth');
INSERT INTO Paths VALUES('Oneiromancy');
INSERT INTO Paths VALUES('Flames');

--More can be added if needed, like if someone makes a whole new path--

INSERT INTO Rituals VALUES('Bind the Accusing Tongue', 1, 4, 'Mortal Shell', NULL, False);
INSERT INTO Rituals VALUES('Communicate with Kindred Sire', 1, 4, 'Blood', NULL, True);
INSERT INTO Rituals VALUES('Chime of the Unseen Spirits', 1, 4, 'Spirit', NULL, False);
INSERT INTO Rituals VALUES('Decrypt Missive', 1, 4, 'Blood', NULL, False);
INSERT INTO Rituals VALUES('Defense of the Sacred Haven', 1, 4, 'Hearth', NULL, False);
INSERT INTO Rituals VALUES('Deflection of Wooden Doom', 1, 4, 'Movement of Mind', NULL, False);
INSERT INTO Rituals VALUES('Devils Touch', 1, 4, 'Corruption', NULL, False);
INSERT INTO Rituals VALUES('Encrypt Missive', 1, 4, 'Blood', NULL, False);
INSERT INTO Rituals VALUES('Engaging the Vessel of Transferance', 1, 4, 'Blood', NULL, False);
INSERT INTO Rituals VALUES('Incantation of the Shepard', 1, 4, 'Blood', NULL, True);
INSERT INTO Rituals VALUES('Semblance of Life', 1, 4, 'Bloods Curse', NULL, False);
INSERT INTO Rituals VALUES('Learning the Mind Enslumbered', 1, 4, 'Blood', NULL, True);
INSERT INTO Rituals VALUES('Purge the Inner Demon', 1, 4, 'Focused Mind', NULL, False);
INSERT INTO Rituals VALUES('Purity of Flesh', 1, 4, 'Blood', NULL, False);
INSERT INTO Rituals VALUES('Sense the Mystical', 1, 4, 'Blood', NULL, True);
INSERT INTO Rituals VALUES('Waking with the Evenings Freshness', 1, 4, 'Bloods Curse', NULL, False);
INSERT INTO Rituals VALUES('Blood Mead', 2, 5, 'Blood', NULL, False);
INSERT INTO Rituals VALUES('Blood Walk', 2, 5, 'Blood', NULL, False);
INSERT INTO Rituals VALUES('Burning Blade', 2, 5, 'Blood', NULL, False);
INSERT INTO Rituals VALUES('Craft Bloodstone', 2, 5, 'Blood', NULL, False);
INSERT INTO Rituals VALUES('Donning the Mask of Shadows', 2, 5, 'Corruption', NULL, False);
INSERT INTO Rituals VALUES('Extinguish', 2, 5, 'Flames', NULL, False);
INSERT INTO Rituals VALUES('Principle Focus of Vitae Infusion', 2, 5, 'Blood', NULL, False);
INSERT INTO Rituals VALUES('Ward vs Ghouls', 2, 5, 'Hearth', NULL, False);
INSERT INTO Rituals VALUES('Warding circle vs Ghouls', 2, 5, 'Hearth', NULL, False);
INSERT INTO Rituals VALUES('Touch of Nightshade', 3, 6, 'Blood', NULL, False);
INSERT INTO Rituals VALUES('Flesh of Fiery Touch', 3, 6, 'Flames', NULL, False);
INSERT INTO Rituals VALUES('Gentle Mind', 3, 6, 'Blood', NULL, False);
INSERT INTO Rituals VALUES('Incorporeal Passage', 3, 6, 'Spirit', NULL, False);
INSERT INTO Rituals VALUES('Mirror of Second Sight', 3, 6, 'Blood', NULL, False);
INSERT INTO Rituals VALUES('Pavis of Foul Presence', 3, 6, 'Corruption', NULL, False);
INSERT INTO Rituals VALUES('Power of the Pyramid', 3, 6, 'Blood', NULL, False);
INSERT INTO Rituals VALUES('Shaft of Belated Quiescence', 3, 6, 'Blood', NULL, False);
INSERT INTO Rituals VALUES('Rurors hands', 3, 6, 'Mortal Shell', NULL, False);
INSERT INTO Rituals VALUES('Ward vs Lupines', 3, 6, 'Hearth', NULL, False);
INSERT INTO Rituals VALUES('Abandoning the Fetters', 5, 10, 'Blood', NULL, False);
INSERT INTO Rituals VALUES('Scrying', 5, 10, 'Blood', NULL, False);


--More to come, there will always be more to come here, Fucking Tremeres, man--

--Influence and related:--
INSERT INTO Influences VALUES('Societet');
INSERT INTO Influences VALUES('Media');
INSERT INTO Influences VALUES('Finans');
INSERT INTO Influences VALUES('Juridik');
INSERT INTO Influences VALUES('Byråkrati');
INSERT INTO Influences VALUES('Industri');
INSERT INTO Influences VALUES('Gatuliv');
INSERT INTO Influences VALUES('Herd');
INSERT INTO Influences VALUES('Lokaler');
INSERT INTO Influences VALUES('Lyx');
INSERT INTO Influences VALUES('Ockult');
INSERT INTO Influences VALUES('Polis');
INSERT INTO Influences VALUES('Politik');
INSERT INTO Influences VALUES('Tillgångar');
INSERT INTO Influences VALUES('Transport');
INSERT INTO Influences VALUES('Trossamfund');
INSERT INTO Influences VALUES('Undre Världen');
INSERT INTO Influences VALUES('Universitet');
INSERT INTO Influences VALUES('Vården');
--Special case-- Except, no
--INSERT INTO Influences VALUES('Resurser');

INSERT INTO Effects VALUES('Resurser');
INSERT INTO Effects VALUES('Kontakter');
INSERT INTO Effects VALUES('Information');
INSERT INTO Effects VALUES('Utrustning');

INSERT INTO Locations VALUES('Centrum');
INSERT INTO Locations VALUES('Angered');
INSERT INTO Locations VALUES('Ale-Surte');
INSERT INTO Locations VALUES('Askim');
INSERT INTO Locations VALUES('Bergsjön');
INSERT INTO Locations VALUES('Örgryte');
INSERT INTO Locations VALUES('Majorna-Linne');
INSERT INTO Locations VALUES('Norra Hisingen');
INSERT INTO Locations VALUES('Lundby');
INSERT INTO Locations VALUES('Västra Hisingen');
INSERT INTO Locations VALUES('Norra Skärgården');
INSERT INTO Locations VALUES('Partille');
INSERT INTO Locations VALUES('Härryda');
INSERT INTO Locations VALUES('Mölndal');
INSERT INTO Locations VALUES('Högsbo-Frölunda');
INSERT INTO Locations VALUES('Västra Göteborg');
INSERT INTO Locations VALUES('Södra Skärgården');


INSERT INTO Cities VALUES('Göteborg');
INSERT INTO Cities VALUES('Kungälv');
INSERT INTO Cities VALUES('Kungsbacka');

--Done thanks to Liv--

--Some user stuff--
INSERT INTO roles VALUES('ROLE_USER');
INSERT INTO roles VALUES('ROLE_GM');
INSERT INTO roles VALUES('ROLE_ADMIN');
