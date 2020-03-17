CREATE VIEW ViewCharacters AS
       SELECT CharacterID, PlayerID, Name, Sect, AGE(DateEmbraced + TorporTime) AS Age, Clan, Humanity
       FROM Characters
       WHERE Active
;

--Needs a rewrite for door openings--
CREATE VIEW TotalInvestments AS
       --A summarising subview for description and HP--
       SELECT Name, string_agg(Location, '\n') AS Location, SUM(Units) AS HP, SUM(Addition) AS MaxHP , string_agg(Description, '\n\n' ORDER BY Time) AS Descriptions
       FROM (
       --Select the long term investment for base description--
       		SELECT Name, Location, 0 AS Addition, 0 AS Units, Description, FALSE AS Repair, Time 
		FROM Investments
		WHERE OneTime = FALSE 

		UNION
       --Select actions for max and current HP--
		SELECT Investment AS Name, NULL AS Location, Units AS Addition, Units, Actions.Description, Repair, Actions.Time
		FROM ActiveActions
		JOIN Actions
		ON Actions.Key = ActiveActions.Key
		JOIN Investments
		ON Actions.Investment = Investments.name
		WHERE Repair = False

		UNION
       --Select repairs for current HP--
		SELECT Investment AS Name, NULL AS Location, 0 AS Addition, Units, Actions.Description, Repair, Actions.Time
		FROM ActiveActions
		JOIN Actions
		ON Actions.Key = ActiveActions.Key
		JOIN Investments
		ON Actions.Investment = Investments.name
		WHERE Repair = True
		
		UNION
       --Select attacks for current HP--
       		SELECT Victim AS Name, NULL AS Location, 0 AS Addition, (Damage * -1) AS Units, Description, TRUE AS Repair, Time
		FROM AttackTowards
       ) AS InvSum
       GROUP BY Name
;

--Influence summary views--
CREATE VIEW TotalInfluences AS
       SELECT characterid, influence, city, sum(units) AS unitsleft
       FROM (
		SELECT characterid, influence, city,( (lvl * (lvl + 1)) / 2) AS units
		FROM hasinfluence

		UNION

		SELECT characterid, influence, city, (-units) AS units
		FROM ApprovedOpenings
		JOIN Actions
		ON Actions.Key = ApprovedOpenings.Key

		UNION

		SELECT characterid, influence, city, (-units) AS units
		FROM ConsumedInfluences
	) AS InfSum
	GROUP BY (characterid, influence, city)
;

CREATE VIEW ProjectedInfluences AS
       SELECT characterid, influence, city, sum(units) AS unitsleft
       FROM (
		SELECT characterid, influence, city,( (lvl * (lvl + 1)) / 2) AS units
		FROM hasinfluence

		UNION

		SELECT characterid, influence, city, (-units) AS units
		FROM PendingOpenings
		JOIN Actions
		ON Actions.Key = PendingOpenings.Key

		UNION

		SELECT characterid, influence, city, (-units) AS units
		FROM ConsumedInfluences
	) AS InfSum
	GROUP BY (characterid, influence, city)
;

--Asset summary views--
CREATE VIEW TotalAssets AS
       SELECT characterid, influence, city, sum(units) AS unitsleft
       FROM (
		SELECT characterid, influence, city, units
		FROM assets

		UNION

		SELECT characterid, influence, city, (-units) AS units
		FROM ApprovedActions
		JOIN Actions
		ON Actions.Key = ApprovedActions.Key
	) AS InfSum
	GROUP BY (characterid, influence, city)
;

CREATE VIEW ProjectedAssets AS
       SELECT characterid, influence, city, sum(units) AS unitsleft
       FROM (
		SELECT characterid, influence, city, units
		FROM assets

		UNION

		SELECT characterid, influence, city, (-units) AS units
		FROM PendingActions
		JOIN Actions
		ON Actions.Key = PendingActions.Key
	) AS InfSum
	GROUP BY (characterid, influence, city)
;

--Use active investments, not all investments, and use consequence--
CREATE VIEW ViewLocations AS
       SELECT Location, SUM(HP) AS TotalHP, SUM(MaxHP) AS TotalMaxHP
       FROM TotalInvestments
       GROUP BY Location
;

--Actions first put in pending, then active or denied, last archived with their final properties (repair, door-opener, addition)--

CREATE VIEW ViewUsedActions AS
-- 6 is to be changed for a age based formula for the character --
       SELECT ViewCharacters.CharacterID, 3 AS Base, Used, (3 - Used) AS Balance
       FROM (
       	    SELECT CharacterID, Count(Investment) AS Used
	    FROM (
       	    	 SELECT Actions.CharacterID, Actions.Investment
	    	 FROM PendingActions
       	    	 JOIN Actions
       	    	 ON PendingActions.Key = Actions.Key

	    	 UNION

	    	 SELECT Actions.CharacterID, Actions.Investment
	    	 FROM ApprovedActions
       	    	 JOIN Actions
       	    	 ON ApprovedActions.Key = Actions.Key

		 UNION

		 SELECT Actions.CharacterID, Actions.Investment
	    	 FROM PendingOpenings
       	    	 JOIN Actions
       	    	 ON PendingOpenings.Key = Actions.Key

	    	 UNION

	    	 SELECT Actions.CharacterID, Actions.Investment
	    	 FROM ApprovedOpenings
       	    	 JOIN Actions
       	    	 ON ApprovedOpenings.Key = Actions.Key
		 ) AS TakenActions
		 GROUP BY CharacterID
	    ) AS ActionCount
       JOIN ViewCharacters
       ON ViewCharacters.CharacterID = ActionCount.CharacterID
;
