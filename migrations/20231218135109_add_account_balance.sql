-- Add balance and initial balance column to accounts table and initial balance fill it with the sum of all records 

ALTER TABLE accounts ADD COLUMN balance FLOAT NOT NULL DEFAULT 0;
ALTER TABLE accounts ADD COLUMN initial_balance FLOAT NOT NULL DEFAULT 0;

UPDATE accounts SET balance = COALESCE((SELECT SUM(amount) FROM records WHERE account_id = accounts.id), 0);

