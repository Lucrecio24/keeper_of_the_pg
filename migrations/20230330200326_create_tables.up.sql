-- Add up migration script here
CREATE TABLE cumulative_commands (
    command_id VARCHAR(10),
    command_name VARCHAR(20),
    CONSTRAINT cumulative_commands_pk PRIMARY KEY (command_id)
);

CREATE TABLE cumulative_command_data (
    account_id VARCHAR(18),
    command_id VARCHAR(10),
    amount int NOT NULL,
    CONSTRAINT cumulative_command_data_pk PRIMARY KEY (account_id , command_id),
    FOREIGN KEY (account_id) REFERENCES members (account_id),
    FOREIGN KEY (command_id) REFERENCES cumulative_commands (command_id)
);

CREATE TABLE members (
    account_id VARCHAR(18) NOT NULL,
    rank_id VARCHAR(18) NOT NULL,
    lanas_coin INT NOT NULL DEFAULT 0,
    CONSTRAINT members_pk PRIMARY KEY (account_id)
);

CREATE TABLE seasons (
    season_id SMALLINT AUTO_INCREMENT,
    season_name TEXT NOT NULL,
    start_date DATE,
    finish_date DATE,
    CONSTRAINT seasons_pk PRIMARY KEY(season_id)
);

CREATE TABLE aliases (
    alias_str VARCHAR(30),
    account_id VARCHAR(18) NOT NULL,
    season_id SMALLINT NOT NULL,
    is_current BOOLEAN NOT NULL DEFAULT FALSE,
    CONSTRAINT aliases_pk PRIMARY KEY (account_id , season_id),
    FOREIGN KEY (account_id) REFERENCES members (account_id),
    FOREIGN KEY (season_id) REFERENCES seasons (season_id)
);