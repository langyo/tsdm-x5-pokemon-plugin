# Repairs for existing seed data

These scripts repair specific rows from the repository's original seed data.
They are separate from the X3 schema migration and must be selected explicitly.

## Item evolution IDs

Use `001_item_evolution_ids.sql` only for an existing X5 database that imported
the original `docker/init.d/08-pokemon-evolutions.sql`. That seed stored legacy
`pm_itemdata.upitem` codes in item evolution conditions, while the game expects
the catalog's `pm_itemdata.id`. For example, using a Water Stone on Eevee could
produce Flareon, and the communication item could fail to evolve Kadabra.

The repair updates 77 original rows. Two other item rules already use the same
number in both numbering systems and need no change. Each update requires an
exact match on the original rule's ID, source species, target species, method,
condition value and priority. Rules with different values in any of those fields
are left unchanged. Applying the repair again makes no further changes.

New databases created with the corrected seed do not need this repair. Do not
use it to convert arbitrary X3 exports or custom evolution catalogs. The X3
schema migration does not infer which item numbering system a rule uses.

### Apply to an existing seeded database

1. Back up the selected site's database and retain the backup until verified.
1. Review the guarded rows in the SQL file against the site's `pm_evolution`
   table. Confirm that it uses the original item catalog and seed rules.
1. From the repository root, connect to that database. The client prompts for
   the password; replace the placeholders below with the site's own settings.

   ```sh
   mariadb --host='<database-host>' --user='<database-user>' -p '<site-database>'
   ```

1. Check the selected database, then apply the targeted repair:

   ```sql
   SELECT DATABASE();
   SOURCE migrations/seed-fixes/001_item_evolution_ids.sql;
   ```

1. Check the affected rules against the corrected seed and verify an intended
   item evolution in a test copy of the site.

Do not reimport the full installation seeds into an existing site. The targeted
repair does not replace the catalog or modify player inventory.

## Regression coverage

`scripts/test/seed_upgrade_database.php` imports the full seed into isolated
databases, exercises the actual evolution item functions, verifies this repair
against the original rows, and checks reruns and customized rules. See
[Game reliability and interface tests](../../docs/game-reliability-testing.md)
for the local database requirements and command.
