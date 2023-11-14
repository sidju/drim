-- Required for proper OIDC login, and enables redirecting users back to where
-- they were before they were redirected to log in again.
CREATE TABLE Logins (
	StateID VARCHAR PRIMARY KEY, -- Randomly generated, collisions improbable
	Nonce VARCHAR NOT NULL, -- Used to validate the OIDC response
    TargetPath VARCHAR NOT NULL, -- Where we redirect after successful auth
	TargetQuery VARCHAR -- Can be included in the redirect, so why not
);
