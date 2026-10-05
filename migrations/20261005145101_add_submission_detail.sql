CREATE TABLE IF NOT EXISTS submission_details(
    submission_id uuid NOT NULL REFERENCES submissions(id),
    test_case_id uuid NOT NULL REFERENCES test_cases(id),

    verdict verdict NOT NULL,
    run_time int NOT NULL,
    memory_usage int NOT NULL
);

ALTER TABLE submissions
ADD run_time int,
ADD memory_usage int;
