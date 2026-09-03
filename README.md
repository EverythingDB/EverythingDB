# EverythingDB

A unified catalog for **all** media — films, shows, books, comics, games, albums, tracks, podcasts, stage productions, artwork and periodicals — in one schema, with no separate silo per medium.

> **Status: early development.** The PostgreSQL schema and its reference documentation are complete and stable. The Rust layer is in progress: enums and the first root models exist, the query layer has not been written yet.

---

## Why

Existing catalogs each pick a lane. AniList and MyAnimeList cover anime and manga; IMDb covers screen; Goodreads covers books; IGDB covers games; MusicBrainz covers music. Nothing covers all of it, so anything that crosses a boundary is either duplicated across sites or missing entirely:

- a manga that becomes an anime that gets a stage play and a mobile game
- a visual novel — text, software, interactive, sometimes voiced
- a music video that is simultaneously audiovisual, musical, and animated
- an audiobook, a motion comic, a concert film, a light novel, a webtoon

EverythingDB models these as first-class entities rather than exceptions.

## The core idea: facets, not a type column

A `type = 'anime'` column forces a single answer to a question that usually has several. Instead, every work gets exactly one row in `media`, and **what form the work takes** is expressed by which property tables share that same `id`. A row existing in a facet table *is* membership in that facet.

```
media row  id=42
  ├─ audiovisual  id=42   → has moving images
  ├─ animation    id=42   → those images are animated
  ├─ narrative    id=42   → it tells a story
  └─ serialized   id=42   → released in installments
```

Cascade deletes are total — dropping the `media` row drops the whole stack.

Membership is enforced by the database, not the application: a `movie` row cannot exist without an `audiovisual` row and a `narrative` row at the same `id`, because those are declared as foreign keys.

### The bitmask

Each facet holds a stable `bit_position` (0–62) in the `facet` registry table. The `media_facets` view exposes every membership boolean plus a packed `kind_mask bigint`:

```sql
-- everything that is an animated film
SELECT * FROM media_facets
WHERE kind_mask & facet_mask('animation','movie') = facet_mask('animation','movie');
```

Bit positions are permanent and never recycled; gaps (13–15, 27–31) are reserved so new facets can be added without disturbing existing ones. On the Rust side this collapses to an integer AND once the mask is cached — no round trip.

### Derived types are predicates, not tables

"Manga", "anime", "webtoon", "light novel", "doujinshi" and friends are **not** tables. They're rows in `media_type`, each stored as a boolean expression:

```
require_mask & kind_mask = require_mask
AND exclude_mask & kind_mask = 0
AND <optional extra_predicate>
```

| Slug | Defined as |
|------|------------|
| `anime` | `animation` + (`movie` or `show`) + JP origin |
| `cartoon` | `animation` + (`movie` or `show`) + not JP origin |
| `manga` / `manhwa` / `manhua` | `comic` + JP / KR / CN·TW·HK origin |
| `webtoon` | `comic` + `panel_layout = vertical_scroll` |
| `light_novel` | `book` + `print.is_illustrated` + JP origin |
| `audio_drama` | `audio` + `narrative` − `musical` − `audiobook` |
| `video_game` | `game` − `visual_novel` |

24 types are seeded. Adding OVA, ONA, eroge, zine, motion comic, gamebook, concert film, indie game or anything else is a single row — no migration, no new table. `extra_predicate` carries a small SQL fragment for the cases a mask alone can't express (country of origin, a specific enum value, an adult flag).

---

## Architecture

Four tiers over ~68 tables and 51 enums, defined in a single migration.

| Tier | Purpose | Contents |
|------|---------|----------|
| **0** | Reference vocabulary | `language`, `country`, `tag`, `credit_role`, `platform`, `rating_system`, `content_rating`, `external_source` |
| **1** | Roots | `media`, `person`, `organization`, `fictional_character` |
| **2** | Orthogonal property facets (13) | `narrative`, `print`, `sequential_art`, `audiovisual`, `animation`, `audio`, `musical`, `interactive`, `performance`, `still_image`, `publication`, `serialized`, `software` |
| **3** | Basic types (11) | `movie`, `show`, `book`, `comic`, `game`, `album`, `track`, `podcast`, `stage_production`, `artwork`, `periodical` |
| **4** | Composites (4) | `visual_novel`, `audiobook`, `music_video`, `tabletop_game` |

Plus support tables for titles, images, external IDs, installments, credits, character portrayals, relations, collections, releases, platforms, tags, content ratings, languages and awards.

Full breakdown — every column, every enum, the bit registry, the index rationale — lives in [`migrations/Schema documentation.md`](migrations/Schema%20documentation.md).

---

### Rust conventions

- Nested structs share a single `id` across their table stack, matching the shared-PK design.
- `RootStruct::insert` returns the newly assigned `i32`; `NonRootStruct::insert` expects that id to already be present on `&self`.
- Every write takes `&mut Transaction<'_, Postgres>` — nothing writes on a bare pool connection.
- Field access goes through `HasMedia` / `HasPrint`-style traits, delegated with [`ambassador`](https://crates.io/crates/ambassador) so composite types inherit their parents' accessors for free.

---

## Running it

Requires Rust (edition 2024) and PostgreSQL.

```bash
# 1. create the database, then point at it
cat > .env <<'EOF'
DB_HOST=localhost
DB_PORT=5432
DB_NAME=everythingdb
DB_USERNAME=postgres
DB_PASSWORD=yourpassword
DATABASE_URL=postgres://postgres:yourpassword@localhost:5432/everythingdb
EOF

# 2. apply the schema
cargo install sqlx-cli --no-default-features --features postgres,sqlx-toml
sqlx migrate run

# 3. build
cargo run
```

`DATABASE_URL` is needed at compile time as well as runtime — the SQLx query macros check statements against a live database during `cargo build`.

---

## Roadmap

- [x] Full four-tier schema + facet/bitmask system
- [x] Schema reference documentation
- [x] Rust enums for all Postgres enum types
- [ ] Remaining root, facet and type models
- [ ] `queries/` — typed `create_*` / `update_*` wrappers over transactions
- [ ] Search layer: a denormalized `catalog` read-model carrying `kind_mask`, plus a `SearchQuery` builder feeding one executor
- [ ] Bulk import path (`UNNEST` / `COPY`) — the per-entity inserts are fine interactively, not for ingesting a dataset
- [ ] User-defined derived types, using the same predicate mechanism
- [ ] **HTTP API**

### On the API

The API is deliberately being built as **groundwork**, not as the end product. The goal beyond this repository is a website frontend — a browsable, searchable catalog sitting on top of this schema — and the API is the layer that will make that possible. So it's being designed now with that consumer in mind: stable derived-type endpoints, cheap facet filtering, and hover-preview-sized payloads alongside full records, rather than being retrofitted later.

---

## Notes

This is a personal project under active design. The schema is the stable part; expect the Rust surface to move. Bit positions in the `facet` table are the one thing that will never change — reordering them would silently invalidate every stored `require_mask`.
