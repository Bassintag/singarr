-- Add migration script here

CREATE TABLE job (
  "id" INTEGER PRIMARY KEY AUTOINCREMENT,
  "created_at" DATETIME DEFAULT CURRENT_TIMESTAMP,

  "payload" TEXT NOT NULL, -- JSON
  "status" TEXT NOT NULL,
  "error" TEXT,
  "completed_at" DATETIME
);
