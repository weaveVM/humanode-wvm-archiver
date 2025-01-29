DROP TABLE IF EXISTS WeaveVMArchiverHumanode;
DROP TABLE IF EXISTS WeaveVMArchiverHumanodeBackfill;

CREATE TABLE IF NOT EXISTS WeaveVMArchiverHumanode (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    NetworkBlockId INT UNIQUE,
    WeaveVMArchiveTxid VARCHAR(66) UNIQUE
);

CREATE TABLE IF NOT EXISTS WeaveVMArchiverHumanodeBackfill (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    NetworkBlockId INT UNIQUE,
    WeaveVMArchiveTxid VARCHAR(66) UNIQUE
);

CREATE INDEX idx_archiver_txid ON WeaveVMArchiverHumanode (WeaveVMArchiveTxid);
CREATE INDEX idx_backfill_txid ON WeaveVMArchiverHumanodeBackfill (WeaveVMArchiveTxid);
CREATE INDEX idx_archiver_block_id ON WeaveVMArchiverHumanode (NetworkBlockId);
CREATE INDEX idx_backfill_block_id ON WeaveVMArchiverHumanodeBackfill (NetworkBlockId);
