CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    date_created TIMESTAMP NOT NULL,
    salt TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT 0,
    should_initialize BOOLEAN NOT NULL DEFAULT 0,

    UNIQUE (username)
);

insert into users (username, date_created, salt, password_hash, is_admin, should_initialize) values ('admin', CURRENT_TIMESTAMP, 'lw4zlCrD80v3EFOieNDmai5kDRMByg5/3zVkjg673EgOXkXcoj97ANG7eXvHg3p3l+HFQJH3iRWeCXWmTXc1RQ==', '5cVBXsvvY9UUg3g64Q1rA5dBGRrgp7S2OQsRxye4wxZQdmM48R4jrsX2gpr3j3eX3ufSM8wrLvaf++TpfHdeO9SwV5OpB0pCOAkk+Wskwk+dssEPJiAdFPVTwgWpEIBk4nIRf30e3RoosHEXP/eTF7f1Vi7JzwxPseoSv+EJYeczCblisOSo+pU8ydM178SyuhJ5cOHMFMWUKWojxvbqQpW/bTtn302vvoKDFmEmwm9OOqN1TaNG60Wo7XzyZ6q7Un7FrbbJX/kxAjG98hHl6PTbSWMfsphxIuz10/SIZPXB1/H/McT3T6rkaEEqVLupYfjD/Ta+ty5262CP1q2OFVcbJSlpux90Hy5eIAnLHnCEGnYUkAcZYEuIkSDDHTuSTXZGJZ98CE99CxYKoS9+gMeTBbavOsqTCIyz/7ZVJ2JVRByX+Bjfl3CSlEyryfMg7YnK5iNnLwsNDdUEjs2cBwWCAyR8LwhxxpDtC0JplFWfeR7Up2zFei5DDCoYxDTTiCVrrXFI3gFqnrwBgUINCL7bPtmBWQv3WoWfIhG5c0t+MzH/x6uJZBuoVe/rB0YzkSRb3q9VufVd4Tv9sVymKwoHm3e/wciG40fAk4dzX4rM/1HEj63flKZoino++vcLqNGl5N5pGR4L4W13Txhl22jeJRfalR2o4dobBkye1zc=', 1, 1);
