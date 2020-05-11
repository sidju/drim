drop owned by sid;

--Login entities--
--Possible roles--
CREATE TYPE roles_t AS ENUM('user', 'gm', 'admin');

--Email, password (hashed) and privilege level--
CREATE TABLE Users(
       eMail VARCHAR(255) PRIMARY KEY NOT NULL CHECK ( eMail ~ '\S*@\S*\.\S*' ),
       pass VARCHAR, --Look over if this is a good choice when writing the more exact handler in php
       role roles_t
);

--Strictly relevant off--
CREATE TABLE Players(
       Active BOOL NOT NULL,
       Paid BOOL NOT NULL,
       InEBas BOOL NOT NULL,
       Name VARCHAR NOT NULL,
       nationalID VARCHAR(10) CHECK ( nationalID ~ '\d{10}' ),
       eMail VARCHAR(255) NOT NULL UNIQUE CHECK ( eMail ~ '\S*@\S*\.\S*' ),
       telephone VARCHAR(12) NOT NULL CHECK ( telephone ~ '\+\d{1,4}|0\d{8,9}'),
       PlayerID SERIAL PRIMARY KEY
);

--Beginning of the rest--

CREATE TABLE Influences(
       Name VARCHAR PRIMARY KEY,
       URL VARCHAR
);

CREATE TABLE Cities(
       Name VARCHAR PRIMARY KEY
);

CREATE TABLE Locations(
       Name VARCHAR PRIMARY KEY,
       City VARCHAR,

       FOREIGN KEY (City) REFERENCES Cities
);

CREATE TABLE Effects(
       Name VARCHAR PRIMARY KEY
);
--Contains pengar, kontakter, information, saker--

CREATE TABLE Clans(
       Name VARCHAR PRIMARY KEY,
       URL VARCHAR
);

CREATE TABLE Sects(
       Name VARCHAR PRIMARY KEY,
       URL VARCHAR
);

CREATE TABLE Disciplines(
       Name VARCHAR PRIMARY KEY,
       URL VARCHAR
);

CREATE TABLE Paths(
       Name VARCHAR PRIMARY KEY,
       URL VARCHAR
);

CREATE TABLE Characters(
       PlayerID INTEGER, --Soft link, so that players can be deleted--
       Active BOOL NOT NULL,
       Name VARCHAR(32) NOT NULL,
       Sect VARCHAR NOT NULL,
       Clan VARCHAR NOT NULL,
       DateEmbraced DATE NOT NULL,
       TorporTime INTERVAL NOT NULL,
       Humanity INTEGER CHECK ( Humanity <= 10 AND Humanity >= 0 ) NOT NULL,
       CharacterID SERIAL PRIMARY KEY,

       FOREIGN KEY (Sect) REFERENCES Sects,
       FOREIGN KEY (Clan) REFERENCES Clans
);

CREATE TABLE Rituals(
       Name VARCHAR PRIMARY KEY,
       Lvl INTEGER NOT NULL,
       XPcost INTEGER NOT NULL,
       Path VARCHAR NOT NULL,
       URL VARCHAR,
       UsesAus BOOL DEFAULT FALSE, --If it uses auspex--
       FOREIGN KEY (Path) REFERENCES Paths
);

CREATE TABLE IsTremere(
       CharacterID INTEGER PRIMARY KEY,
       Circle INTEGER NOT NULL CHECK ( Circle <= 7 AND Circle >= 0),
       Rank INTEGER NOT NULL CHECK ( Rank <= 6 AND Rank >= 0 ),
       --Ghoul, Fledgeling, Apprentice, Regent, Lord, Pontifex, Council of seven--

       FOREIGN KEY (CharacterID) REFERENCES Characters
);
       
CREATE TABLE HasInfluence(
       CharacterID INTEGER,
       Influence VARCHAR,
       City VARCHAR,
       Lvl INTEGER NOT NULL CHECK ( Lvl <= 5 AND Lvl >= 1 ),

       FOREIGN KEY (City) REFERENCES Cities,
       FOREIGN KEY (Influence) REFERENCES Influences,
       FOREIGN KEY (CharacterID) REFERENCES Characters,
       PRIMARY KEY (CharacterID, Influence, City)
);

CREATE TABLE HasDiscipline(
       Discipline VARCHAR,
       CharacterID INTEGER,
       Lvl INTEGER NOT NULL CHECK ( Lvl <= 3 AND Lvl >= 1 ),

       FOREIGN KEY (Discipline) REFERENCES Disciplines,
       FOREIGN KEY (CharacterID) REFERENCES Characters,
       PRIMARY KEY (Discipline, CharacterID)
);

CREATE TABLE HasPath(
       Path VARCHAR,
       CharacterID INTEGER,
       Lvl INTEGER NOT NULL CHECK ( Lvl <= 3 AND Lvl >= 1 ),

       FOREIGN KEY (Path) REFERENCES Paths,
       FOREIGN KEY (CharacterID) REFERENCES IsTremere,
       PRIMARY KEY (Path, CharacterID)
);

CREATE TABLE HasRitual(
       Ritual VARCHAR,
       CharacterID INTEGER,

       FOREIGN KEY (Ritual) REFERENCES Rituals,
       FOREIGN KEY (CharacterID) REFERENCES IsTremere,
       PRIMARY KEY (Ritual, CharacterID)
);

--Beware!!! This table will be inserting, modifying and deleting a lot. No data safety guaranteed.--
--This is unique since a lot of tables are either eternal entry or cleared on new round.--
CREATE TABLE Resources(
       CharacterID INTEGER PRIMARY KEY,
       Units INTEGER NOT NULL,

       FOREIGN KEY (CharacterID) REFERENCES Characters
);

--Cleared on new round.--
CREATE TABLE Assets(
       --Holds the resulting usable influences from Influences, Resources and loans--
       --A proverbial phonebook to the needed contacts--
       CharacterID INTEGER NOT NULL,
       Units INTEGER NOT NULL,
       Influence VARCHAR NOT NULL,
       City VARCHAR NOT NULL,
       
       PRIMARY KEY (CharacterID, Influence, City),
       FOREIGN KEY (CharacterID) REFERENCES Characters,
       FOREIGN KEY (Influence) REFERENCES Influences,
       FOREIGN KEY (City) REFERENCES Cities
);

--Cleared and updated from Lend on new round. Also used to cash influence into assets.--
--This table and active Openings dictate total used influences--
CREATE TABLE ConsumedInfluences(
       CharacterID INTEGER NOT NULL,
       Units INTEGER NOT NULL CHECK (Units > 0),
       Influence VARCHAR NOT NULL,
       City VARCHAR NOT NULL,
       
       PRIMARY KEY (CharacterID, Influence, City),
       FOREIGN KEY (CharacterID, Influence, City) REFERENCES hasInfluence --Thus, only influences had can be consumed.--
);

--A dangerous table. Additional loans are stacked and emptied loans must be deleted.--
CREATE TABLE Lend(
       GivingCharacterID INTEGER,
       RecievingCharacterID INTEGER,
       Influence VARCHAR,
       Units INTEGER NOT NULL CHECK ( Units > 0 ),
       City VARCHAR,

       PRIMARY KEY (GivingCharacterID, RecievingCharacterID, Influence, City),
       FOREIGN KEY (RecievingCharacterID) REFERENCES Characters,
       FOREIGN KEY (GivingCharacterID, Influence, City) REFERENCES HasInfluence --Only personal influence can be lent--
);

--Eternal insert table (which allows delete by the character)--
CREATE TABLE RecipeRequest(
       Name VARCHAR NOT NULL,
       Description VARCHAR NOT NULL,
       CharacterID INTEGER,
       Handled BOOL NOT NULL,
       Key SERIAL PRIMARY KEY,

       FOREIGN KEY (CharacterID) REFERENCES Characters
);

--Eternal insert table. Inserts made by GMs--
--HP may be modified, but rarely. Most HP changes are summed from attacks and actions--
CREATE TABLE Investments(
       Name VARCHAR(32) NOT NULL PRIMARY KEY,
       Location VARCHAR NOT NULL, --Added to the name as well in php. [Company] becomes [Company] ([Location])-- 
       CharacterID INTEGER NOT NULL,
       Effect VARCHAR NOT NULL,
       Description VARCHAR NOT NULL, --concat to allow additions--
       OneTime BOOL NOT NULL,
       --If onetime, Investment resolves once for a bigger effect and dies--
       HP INTEGER DEFAULT 0,
       Time TIME DEFAULT NOW(),

       FOREIGN KEY (CharacterID) REFERENCES Characters,
       FOREIGN KEY (Location) REFERENCES Locations,
       FOREIGN KEY (Effect) REFERENCES Effects
);

--Eternal insert table. Detete allowed for Character.--
--Paid for by Assets, as visible in the TotalAssets view.--
CREATE TABLE Actions(
       CharacterID INTEGER NOT NULL,
       Investment VARCHAR(32) NOT NULL,
       Influence VARCHAR NOT NULL,
       City VARCHAR, --Implied by investment, but too far away to be useful--
       Units INTEGER NOT NULL CHECK ( Units > 0 ),
       Description VARCHAR NOT NULL, --concat to allow additions--
       Repair BOOL NOT NULL,
       DoorOpening BOOL NOT NULL,
       Time TIME DEFAULT NOW(),
       Key SERIAL PRIMARY KEY,

       FOREIGN KEY (CharacterID) REFERENCES Characters,
       FOREIGN KEY (Investment) REFERENCES Investments,
       FOREIGN KEY (Influence) REFERENCES Influences,
       FOREIGN KEY (City) REFERENCES Cities
);

--Eternal Insert.--
--Created to give more power to the SLs and separate the players a bit
--Written by the SLs based on a pending action and linked to an ActiveAction?
CREATE TABLE AttackTowards(
       Victim VARCHAR(32),
       Attack VARCHAR(32),
       Damage INTEGER NOT NULL,
       Description VARCHAR NOT NULL,
       Time TIME DEFAULT NOW(),

       PRIMARY KEY (Victim, Attack),
       FOREIGN KEY (Victim) REFERENCES Investments,
       FOREIGN KEY (Attack) REFERENCES Investments
);

--Insert and delete. Used as a workflow flag.--
--Investment confirmation relations
CREATE TABLE PendingInvestments(
       Name VARCHAR(32),

       PRIMARY KEY (Name),
       FOREIGN KEY (Name) REFERENCES Investments
);

--Insert and delete. Used as a workflow flag.--
--Also disabled investments by the player--
CREATE TABLE DeniedInvestments(
       Name VARCHAR(32),
       Comment VARCHAR, --Why it was denied--
       
       PRIMARY KEY (Name),
       FOREIGN KEY (Name) REFERENCES Investments
);

--Insert and delete. Used as a workflow flag.--
--Signifies that the Investment is finished.
CREATE TABLE ActiveInvestments(
       Name VARCHAR(32),
       Time TIME DEFAULT NOW(), --If current time is after time, it becomes visible next round--
       Visible BOOL, --Wether or not it has entered play--
       Revenue INTEGER NOT NULL CHECK (Revenue >= 0),
       Consequence INTEGER NOT NULL DEFAULT 0,
       --The effect it has on the Location (- if negative)--
       Comment VARCHAR, --For returning information or telling about new contacts--
       
       PRIMARY KEY (Name),
       FOREIGN KEY (Name) REFERENCES Investments
);


--##################################################################
-- Done this far --
--##################################################################
--Action confirmation relations
--Insert and delete. Used as a workflow flag.--
CREATE TABLE PendingActions(
       Key INTEGER PRIMARY KEY,
       
       FOREIGN KEY (Key) REFERENCES Actions
);

--Insert and delete. Used as a workflow flag.--
CREATE TABLE DeniedActions(
       Key INTEGER PRIMARY KEY,
       Comment VARCHAR, --Why it was denied--
       
       FOREIGN KEY (Key) REFERENCES Actions
);

--Insert and delete. Used as a workflow flag.--
CREATE TABLE ActiveActions(
       Key INTEGER PRIMARY KEY,

       FOREIGN KEY (Key) REFERENCES Actions
);

--Insert and delete. Used as a workflow flag.--
CREATE TABLE ApprovedActions(
       Key INTEGER PRIMARY KEY,

       FOREIGN KEY (Key) REFERENCES Actions
);

--DoorOpening confirmation relations--
--Insert and delete. Used as a workflow flag.--
CREATE TABLE PendingOpenings(
       Key INTEGER PRIMARY KEY,
       
       FOREIGN KEY (Key) REFERENCES Actions
);

--Insert and delete. Used as a workflow flag.--
CREATE TABLE DeniedOpenings(
       Key INTEGER PRIMARY KEY,
       Comment VARCHAR, --Why it was denied--
       
       FOREIGN KEY (Key) REFERENCES Actions
);

--Insert and delete. Used as a workflow flag.--
CREATE TABLE ApprovedOpenings(
       Key INTEGER PRIMARY KEY,
       OpenedTo INTEGER NOT NULL,

       FOREIGN KEY (Key) REFERENCES Actions
);

--Insert and delete. Used as a workflow flag.--
CREATE TABLE ActiveOpenings(
       Key INTEGER PRIMARY KEY,
       OpenedTo INTEGER NOT NULL,

       FOREIGN KEY (Key) REFERENCES Actions
);

--Modify the power of influences (perform at action resolve)--
--Insert and delete. Used to change system behavior in a php "new_round" method.--
CREATE TABLE InfluenceModifier(
       Influence VARCHAR,
       City VARCHAR,
       Modifier INTEGER NOT NULL DEFAULT 0,

       PRIMARY KEY (Influence, City),
       FOREIGN KEY (Influence) REFERENCES Influences,
       FOREIGN KEY (City) REFERENCES Cities
);

--Insert and delete. Used to change system behavior in a php "new_round" method.--
CREATE TABLE LocalInfluenceModifier(
       Influence VARCHAR,
       Location VARCHAR,
       Modifier INTEGER NOT NULL DEFAULT 0,

       PRIMARY KEY (Influence, Location),
       FOREIGN KEY (Influence) REFERENCES Influences,
       FOREIGN KEY (Location) REFERENCES Locations
);
