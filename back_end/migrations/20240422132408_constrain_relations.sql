
ALTER TABLE relations
ADD CONSTRAINT different_values CHECK (premise_id <> conclusion_id)