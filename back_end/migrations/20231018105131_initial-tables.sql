-- Initial migration

-- Types
CREATE TYPE validity AS ENUM(
    'valid',
    'invalid',
    'antivalid'
);

-- Entities

CREATE TABLE users (
    email varchar(255) PRIMARY KEY,
    username varchar(255) UNIQUE NOT NULL,
    password_hash varchar(255) NOT NULL,
    session_id_hash varchar (255)
);

CREATE TABLE profiles (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    user_email varchar(255) NOT NULL,
    propositional_access boolean DEFAULT false NOT NULL
);

CREATE TABLE propositions (
    id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY,
    profile_id uuid,
    lexical_description varchar(1023) UNIQUE NOT NULL
);

CREATE TABLE relations (
    id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY,
    premise_id uuid NOT NULL,
    conclusion_id uuid NOT NULL,
    propositional_validity validity,
    UNIQUE (premise_id, conclusion_id)
);

CREATE TABLE propositional_formalizations (
    id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY,
    profile_id uuid,
    proposition_id uuid NOT NULL,
    formalization_string varchar(255) NOT NULL,
    UNIQUE(proposition_id, formalization_string)
);

-- Relations

CREATE TABLE profiles_relations (
    profile_id uuid,
    relation_id uuid,
    is_correlated boolean NOT NULL,
    PRIMARY KEY (profile_id, relation_id)
);

CREATE TABLE profiles_propositions (
    profile_id uuid,
    proposition_id uuid,
    is_true boolean NOT NULL,
    PRIMARY KEY (profile_id, proposition_id)
);

CREATE TABLE profiles__propositional_formalizations (
    profile_id uuid,
    propositional_formalization_id uuid,
    is_correct boolean NOT NULL,
    PRIMARY KEY (profile_id, propositional_formalization_id)
);

CREATE TABLE propositional_formalizations__propositions (
    propositional_formalization_id uuid,
    proposition_id uuid,
    sentence_symbol varchar(1) NOT NULL,
    PRIMARY KEY (propositional_formalization_id, proposition_id)
);

--Alterations

ALTER TABLE profiles
ADD CONSTRAINT fk_user_emails FOREIGN KEY (user_email) REFERENCES users(email);

ALTER TABLE propositions
ADD CONSTRAINT fk_profile_id FOREIGN KEY (profile_id) REFERENCES profiles(id);

ALTER TABLE relations
ADD CONSTRAINT fk_premise_id FOREIGN KEY (premise_id) REFERENCES propositions(id),
ADD CONSTRAINT fk_conclusion_id FOREIGN KEY (conclusion_id) REFERENCES propositions(id);

ALTER TABLE propositional_formalizations
ADD CONSTRAINT fk_profile_id FOREIGN KEY (profile_id) REFERENCES profiles(id),
ADD CONSTRAINT fk_proposition_id FOREIGN KEY (proposition_id) REFERENCES propositions(id);

ALTER TABLE profiles_relations
ADD CONSTRAINT fk_profile_id FOREIGN KEY (profile_id) REFERENCES profiles(id),
ADD CONSTRAINT fk_relation_id FOREIGN KEY (relation_id) REFERENCES relations(id);

ALTER TABLE profiles_propositions
ADD CONSTRAINT fk_profile_id FOREIGN KEY (profile_id) REFERENCES profiles(id),
ADD CONSTRAINT fk_proposition_id FOREIGN KEY (proposition_id) REFERENCES propositions(id);

ALTER TABLE profiles__propositional_formalizations
ADD CONSTRAINT fk_profile_id FOREIGN KEY (profile_id) REFERENCES profiles(id),
ADD CONSTRAINT fk_propositional_formalizations FOREIGN KEY (propositional_formalization_id) REFERENCES propositional_formalizations(id);

ALTER TABLE propositional_formalizations__propositions
ADD CONSTRAINT fk_propositional_formalizations FOREIGN KEY (propositional_formalization_id) REFERENCES propositional_formalizations(id),
ADD CONSTRAINT fk_proposition_id FOREIGN KEY (proposition_id) REFERENCES propositions(id);