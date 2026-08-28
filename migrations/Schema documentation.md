# EverythingDB — Schema Reference

Migration: `0001_initial_schema.sql`  
Tables: 68 · Enums: 51 · Views: 1 · Functions: 1 · Triggers: 6 · Indexes: 56

---

## Table of Contents

1. [The Facet System](#1-the-facet-system)
2. [Tier 0 — Reference Tables](#2-tier-0--reference-tables)
3. [Tier 1 — Root Tables](#3-tier-1--root-tables)
4. [Tier 2 — Property Facets](#4-tier-2--property-facets)
5. [Tier 3 — Basic Types](#5-tier-3--basic-types)
6. [Tier 4 — Composite Types](#6-tier-4--composite-types)
7. [Support Tables](#7-support-tables)
8. [Derived Types (media_type)](#8-derived-types)
9. [Index Reference](#9-index-reference)

---

## 1. The Facet System

### Core idea

Every catalogued work has exactly one row in `media`. Everything else about
**what form that work takes** is expressed by which property tables share that
same `id`. A property table row existing = membership in that facet.

```
media row  id=42
  ├─ audiovisual  id=42   → this work has moving images
  ├─ animation    id=42   → those images are animated
  ├─ narrative    id=42   → it tells a story
  └─ serialized   id=42   → it was released in installments
```

Cascade deletes are total: removing the `media` row removes the whole stack
automatically.

### Why facets instead of a type column

A `type = 'anime'` column forces a single answer to a question that often has
many. A music video is simultaneously audiovisual, musical, and (if animated)
animation. A visual novel is interactive software, text, and presentation. The
facet model lets you ask "give me everything that is both audiovisual and
musical" as a join rather than a string match, and lets the answer be correct.

### The bit mask

Each facet has a stable `bit_position` (0–62) in the `facet` registry table.
The `media_facets` view materializes all membership booleans plus a packed
`kind_mask bigint` computed from them.

```sql
SELECT kind_mask FROM media_facets WHERE media_id = 42;
-- 0b...0000_0001_0000_1001 = bits 0 (narrative) + 3 (audiovisual) + 4 (animation)
```

The helper function `facet_mask(VARIADIC text[])` converts slug names to a
mask, making predicates readable:

```sql
-- "give me things that are animated films"
WHERE kind_mask & facet_mask('animation','movie') = facet_mask('animation','movie')
```

In Rust this is an integer AND — no query round-trip needed once the mask is
cached.

### Bit position assignments (stable forever, do not reorder)

| Bit | Facet | Tier |
|-----|-------|------|
| 0 | narrative | property |
| 1 | print | property |
| 2 | sequential_art | property |
| 3 | audiovisual | property |
| 4 | animation | property |
| 5 | audio | property |
| 6 | musical | property |
| 7 | interactive | property |
| 8 | performance | property |
| 9 | still_image | property |
| 10 | publication | property |
| 11 | serialized | property |
| 12 | software | property |
| 16 | movie | basic |
| 17 | show | basic |
| 18 | book | basic |
| 19 | comic | basic |
| 20 | game | basic |
| 21 | album | basic |
| 22 | track | basic |
| 23 | podcast | basic |
| 24 | stage_production | basic |
| 25 | artwork | basic |
| 26 | periodical | basic |
| 32 | visual_novel | composite |
| 33 | audiobook | composite |
| 34 | music_video | composite |
| 35 | tabletop_game | composite |

Bits 13–15 and 27–31 are reserved gaps for future property and basic facets
without disrupting composite bit positions.

### How membership is enforced

Basic and composite tables declare their facet requirements as explicit
`FOREIGN KEY (id) REFERENCES <facet>(id)` constraints. You cannot insert a
`movie` row without first having an `audiovisual` row and a `narrative` row
with the same id. The database enforces the contract; the application doesn't
need to.

```
movie.id → audiovisual.id → media.id
movie.id → narrative.id   → media.id   (via named constraint)
```

### The extra_predicate field

Some derived types cannot be expressed with a bitmask alone — they require
checking a column value (e.g. `country_of_origin_id = JP`, or
`panel_layout = 'vertical_scroll'`). The `media_type.extra_predicate` column
holds a SQL fragment compiled by the application into a WHERE clause. The
assumed table aliases are:

| Alias | Table |
|-------|-------|
| `m` | media |
| `c` | country (joined on `m.country_of_origin_id`) |
| `f` | media_facets |
| per-facet | same short name as membership table (pr, sa, pb, sz, bk, cm, al, si, tg, ...) |

### Adding a new facet later

1. Add the table with `id PRIMARY KEY REFERENCES media(id) ON DELETE CASCADE`.
2. Insert a row into `facet` with the next free `bit_position`.
3. Add a column to the `media_facets` view.
4. Add the OR clause to the `kind_mask` expression in the same view.
5. Never move or recycle bit positions.

---

## 2. Tier 0 — Reference Tables

These are vocabulary shared across the entire schema. No cascade from `media`.

| Table | Purpose | Key columns |
|-------|---------|-------------|
| `language` | ISO-639 language registry | `iso_639_1`, `iso_639_3`, `direction` |
| `country` | ISO-3166-1 country registry | `iso_3166_1`, `region` |
| `tag` | Hierarchical folksonomy | `namespace`, `parent_id`, `is_adult`, `is_spoiler` |
| `credit_role` | Roles a person can hold | `department`, `is_primary_credit` |
| `platform` | Hardware and distribution platforms | `kind`, `manufacturer_id`, `generation` |
| `rating_system` | ESRB, PEGI, MPAA, CERO, etc. | `country_id`, `applies_to` |
| `content_rating` | Individual ratings per system | `code`, `label`, `minimum_age` |
| `external_source` | AniList, VNDB, IGDB, MusicBrainz, etc. | `url_template`, `is_authoritative` |

Tags use a `namespace` text field rather than an enum so new namespaces can be
added without a migration. Current expected namespaces: `genre`, `theme`,
`setting`, `content_warning`, `demographic`, `technique`, `origin`,
`franchise_trait`, `publishing_trait`, `mood`, `misc`.

---

## 3. Tier 1 — Root Tables

### `media`

The root of every catalogued work. Form is implicit (derived from which facet
tables share this id). Contains only data that applies to every possible form:
title, origin, lifecycle, synopsis, flags, and denormalized aggregates.

Key fields: `primary_title`, `original_title`, `romanized_title`, `sort_title`,
`country_of_origin_id`, `original_language_id`, `source_material`, `status`,
`started_on`/`ended_on`, `is_adult`, `is_official`, `is_lost_media`,
`mean_score`, `popularity`.

### `person`

Real or pseudonymous individual. Also used for groups where the group is the
credited unit (a band, an art circle) via `is_group = true`.

Key fields: `primary_name`, `native_name`, `romanized_name`, `sort_name`,
`given_name`/`family_name`, `gender`, `birth_date`/`death_date`,
`birth_country_id`, `active_from`/`active_until`, `biography`.

### `organization`

Studio, publisher, label, network, imprint, distributor, etc. Organizations
can be hierarchical (imprint → publisher) via `parent_id`. `org_type` is the
primary classifier.

Key fields: `name`, `org_type`, `parent_id`, `country_id`, `founded_on`,
`dissolved_on`, `is_defunct`.

### `fictional_character`

A character as an entity independent of any specific work. Linked to works via
`media_character`, and to performers via `character_portrayal`.

Key fields: `primary_name`, `native_name`, `romanized_name`, `aliases[]`,
`gender`, `age_description`, `species`, `occupation`, `is_spoiler_heavy`.

---

## 4. Tier 2 — Property Facets

Each facet table's PK is also a FK to `media(id)`. Presence of a row = the
work has that form. All cascade on delete.

---

### `narrative` — tells a story

> Bit: 0

Applies to: films, shows, books, comics, games, visual novels, audio dramas,
stage plays — anything with a plot.

| Column | Type | Notes |
|--------|------|-------|
| `is_fiction` | bool | false for documentaries, biographies |
| `narrative_form` | enum | linear, nonlinear, episodic, anthology, branching, vignette, frame_story, experimental |
| `point_of_view` | enum | first_person, second_person, third_limited, third_omniscient, multiple, objective, epistolary, mixed |
| `setting_period` | text | 'Edo period', '2247 CE', 'contemporary' |
| `setting_place` | text | freeform geographic / world description |
| `is_ensemble_cast` | bool | |
| `protagonist_count` | smallint | |
| `ending_count` | smallint | |
| `has_multiple_endings` | bool | generated: ending_count > 1 |
| `is_self_contained` | bool | false if it requires other works to make sense |
| `chronology_index` | numeric | in-universe timeline position |
| `is_canon` | bool | |
| `content_summary` | text | full, spoiler-inclusive description |
| `themes_summary` | text | |

---

### `print` — text carried on pages

> Bit: 1

Applies to: books, comics (partially), light novels, web novels, magazines,
scripts, lyrics sheets, screenplays.

| Column | Type | Notes |
|--------|------|-------|
| `page_count` | integer | |
| `word_count` | integer | |
| `character_count` | integer | CJK-relevant |
| `prose_format` | enum | prose, verse, script, screenplay, epistolary, diary, reference, mixed |
| `reading_direction` | enum | ltr, rtl, vertical_rtl, vertical_ltr, boustrophedon, mixed |
| `script_language_id` | FK | language of the written text |
| `is_illustrated` | bool | key for light novel derivation |
| `illustration_count` | integer | |
| `has_furigana` | bool | |
| `has_footnotes` | bool | |
| `has_index` / `has_bibliography` | bool | |
| `reading_level` | text | Lexile, JLPT, CEFR, grade |
| `estimated_reading_minutes` | integer | |
| `is_translation` | bool | |
| `translated_from_id` | FK | source language |

---

### `sequential_art` — panel-driven visual storytelling

> Bit: 2

Applies to: manga, manhwa, manhua, comics, webtoons, bande dessinée,
newspaper strips, motion comics.

| Column | Type | Notes |
|--------|------|-------|
| `panel_layout` | enum | page, double_page, **vertical_scroll** (webtoon), horizontal_strip, four_koma, single_panel, freeform, mixed |
| `page_count` / `panel_count` | integer | |
| `average_panels_per_page` | numeric | |
| `coloring` | enum | monochrome, greyscale, duotone, spot_color, full_color, mixed |
| `uses_screentone` | bool | |
| `is_digital_native` | bool | false for scanned physical works |
| `canvas_width_px` | integer | webtoon / vertical formats |
| `art_style` | text | freeform |
| `line_art_medium` | text | 'ink on bristol', 'vector', 'raster digital' |
| `has_sound_effects_inline` | bool | SFX rendered as lettered art |
| `lettering_is_typeset` | bool | false = hand-lettered |
| `is_wordless` | bool | silent / pantomime comics |

---

### `audiovisual` — moving image

> Bit: 3

Applies to: films, shows, music videos, anime, OVAs, documentaries, recorded
stage productions.

| Column | Type | Notes |
|--------|------|-------|
| `runtime_seconds` | integer | total / per episode |
| `aspect_ratio` | text | '16:9', '2.39:1', '4:3' |
| `frame_rate` | numeric | |
| `resolution_width` / `resolution_height` | integer | |
| `color_system` | enum | black_and_white, color, colorized, tinted, mixed |
| `is_hdr` / `is_stereoscopic_3d` / `is_silent` | bool | |
| `capture_medium` | enum | film_8mm through film_65mm, imax, videotape, digital, virtual |
| `audio_channel_layout` | text | 'mono', '5.1', 'Atmos' |
| `audio_codec` / `video_codec` | text | |
| `has_subtitles` / `has_closed_captions` / `has_audio_description` | bool | accessibility |
| `filming_locations` | text[] | |
| `is_live_action` | bool | false when the animation facet is also present |
| `post_credits_scenes` | smallint | |

---

### `animation` — non-photographic moving image

> Bit: 4

Applies alongside `audiovisual` for any animated work. Can also apply to
animated stills or motion comics without an `audiovisual` row.

| Column | Type | Notes |
|--------|------|-------|
| `technique` | enum | traditional_cel, digital_2d, cgi_3d, stop_motion, claymation, puppet, cutout, rotoscope, pixel_art, motion_graphics, sand, paint_on_glass, live_hybrid, mixed |
| `secondary_techniques` | enum[] | mixed-technique works |
| `animates_on` | smallint | 1 = on ones (24fps full), 2 = on twos (12fps) |
| `cel_count` / `key_frame_count` / `in_between_count` | integer | |
| `uses_motion_capture` / `uses_rotoscoping` | bool | |
| `render_engine` | text | |
| `is_hand_drawn` | bool | |
| `character_design_notes` | text | |

---

### `audio` — sound is the primary carrier

> Bit: 5

Applies to: albums, tracks, podcasts, audio dramas, field recordings, radio
broadcasts.

| Column | Type | Notes |
|--------|------|-------|
| `duration_seconds` | integer | |
| `recording_type` | enum | studio, live, field, remote, synthetic, archival, mixed |
| `sample_rate_hz` / `bit_depth` / `bitrate_kbps` | integer | technical |
| `channel_layout` | text | 'stereo', '5.1', 'binaural' |
| `is_lossless` | bool | |
| `loudness_lufs` / `dynamic_range_db` | numeric | mastering metadata |
| `is_dialogue_driven` | bool | audio drama / podcast distinction |
| `has_transcript` | bool | |
| `spoken_language_id` | FK | |
| `recorded_on` / `recording_venue` | date / text | |

---

### `musical` — organized sound with musical structure

> Bit: 6

Applies to: tracks, albums, musical stage productions, film scores (as tracks),
video game OSTs (as albums/tracks).

| Column | Type | Notes |
|--------|------|-------|
| `bpm` | numeric | |
| `musical_key` / `time_signature` / `tuning_hz` | text / numeric | theory metadata |
| `instrumentation` | text[] | |
| `is_instrumental` | bool | |
| `vocal_type` | enum | lead, duet, group, choral, spoken_word, rap, instrumental, vocaloid, mixed |
| `lyrics_language_id` | FK | |
| `has_explicit_lyrics` | bool | |
| `lyrics` | text | stored verbatim |
| `isrc` / `iswc` | text | music industry identifiers |
| `energy` / `valence` / `acousticness` / `danceability` | numeric(4,3) | 0–1 audio features |
| `is_cover` / `is_remix` | bool | |

---

### `interactive` — user input changes the state of the work

> Bit: 7

Applies to: video games, visual novels, tabletop games, interactive fiction,
gamebooks, interactive exhibits.

| Column | Type | Notes |
|--------|------|-------|
| `input_methods` | enum[] | keyboard, mouse, gamepad, touch, stylus, motion, vr, light_gun, arcade_stick, dance_pad, voice, eye_tracking, physical_component, dice, cards |
| `branching_structure` | enum | none, linear, hub, branching, open_world, procedural, sandbox |
| `choice_count` / `ending_count` | integer | |
| `player_min` / `player_max` | smallint | 1/1 for singleplayer |
| `is_multiplayer` / `is_cooperative` / `is_competitive` | bool | |
| `has_online_play` | bool | |
| `save_system` | enum | none, password, checkpoint, manual_slot, autosave, save_anywhere, permadeath, cloud_only, mixed |
| `has_difficulty_options` / `has_permadeath` | bool | |
| `uses_procedural_generation` | bool | |
| `main_playtime_minutes` / `completionist_playtime_minutes` / `session_length_minutes` | integer | |
| `accessibility_features` | text[] | |

---

### `performance` — realized live by performers

> Bit: 8

Applies to: stage plays, opera, ballet, concerts, stand-up, circus, improv,
live broadcasts.

| Column | Type | Notes |
|--------|------|-------|
| `staging` | enum | proscenium, thrust, in_the_round, black_box, immersive, site_specific, street, arena, stadium, broadcast, virtual |
| `duration_seconds` | integer | |
| `act_count` / `intermission_count` | smallint | |
| `cast_size` / `ensemble_size` / `orchestra_size` | smallint | |
| `premiere_date` / `closing_date` | date | |
| `performance_count` | integer | total number of performances |
| `is_improvised` / `is_recorded` / `requires_audience_participation` | bool | |
| `venue_name` / `venue_city` / `venue_country_id` | text / FK | |

---

### `still_image` — a fixed visual artifact

> Bit: 9

Applies to: paintings, photographs, illustrations, prints, concept art, cover
art, photobooks, infographics.

| Column | Type | Notes |
|--------|------|-------|
| `medium` | enum | oil, acrylic, watercolor, gouache, ink, pencil, charcoal, pastel, digital, photograph, screenprint, woodblock, etching, lithograph, collage, mixed_media, other |
| `is_digital` | bool | |
| `width_mm` / `height_mm` / `depth_mm` | numeric | physical dimensions |
| `width_px` / `height_px` / `dpi` | integer | digital dimensions |
| `color_space` | text | 'sRGB', 'CMYK', 'Adobe RGB' |
| `support_material` | text | canvas, washi, bristol, panel |
| `edition_number` / `edition_size` | integer | for prints |
| `is_unique_piece` | bool | one-of-a-kind |
| `current_location` | text | museum / collection |
| `exif` | jsonb | raw EXIF for photographs |

---

### `publication` — issued as an edition by someone

> Bit: 10

Applies to: books, comics, periodicals, tabletop games, artbooks, photobooks,
audiobooks, doujinshi, zines, academic papers.

This is the primary facet for distinguishing **publication style**. Doujinshi
and fan works are not separate tables — they are `publication_model =
'self_published'` or `'fan_published'` with `is_derivative = true`.

| Column | Type | Notes |
|--------|------|-------|
| `publisher_id` / `imprint_id` | FK org | |
| `publication_model` | enum | traditional, small_press, academic, **self_published**, vanity, **web_serial**, **fan_published**, commissioned, government, unpublished |
| `distribution` | enum | print_only, digital_only, print_and_digital, broadcast, streaming, physical_media, download, cartridge, disc, tape, live_only, mixed |
| `binding` | enum | hardcover, trade_paperback, mass_market, tankobon, bunko, aizoban, kanzenban, omnibus, box_set, saddle_stitch, spiral, ebook, loose_leaf, scroll, none |
| `demographic` | enum | children, middle_grade, young_adult, shounen, shoujo, seinen, josei, kodomomuke, general, adult, academic, professional |
| `published_on` | date | |
| `is_first_edition` | bool | |
| `is_official` / `is_derivative` / `is_limited` | bool | |
| `print_run` | integer | |
| `isbn_10` / `isbn_13` / `issn` / `asin` / `doi` / `barcode` / `catalog_number` | text | identifiers |
| `trim_size` / `paper_stock` | text | physical production |
| `cover_price` / `currency` | numeric / char | |
| `is_out_of_print` | bool | |
| `publishing_circle` | text | doujin circle name |
| `released_at_event` | text | 'Comiket 103', 'Kickstarter 2024' |

---

### `serialized` — released in installments

> Bit: 11

Applies to: shows, manga, podcasts, web novels, webcomics, periodicals,
episodic games.

| Column | Type | Notes |
|--------|------|-------|
| `installment_unit` | enum | episode, chapter, issue, volume, track, part, session, act, strip, entry |
| `total_installments` / `released_installments` | integer | |
| `is_ongoing` / `is_on_hiatus` | bool | |
| `schedule` | enum | daily through burst, completed, irregular |
| `release_weekday` / `release_time` / `release_timezone` | mixed | exact schedule |
| `season_number` / `part_number` | smallint | |
| `first_installment_on` / `latest_installment_on` / `next_installment_on` | date | |
| `average_installment_length` | integer | seconds or pages, unit-dependent |
| `serialized_in_id` | FK media | host periodical or anthology series |

---

### `software` — executes on hardware

> Bit: 12

Applies to: video games, interactive software, browser games. Separated from
`interactive` because some interactive works (tabletop games, gamebooks) are
not software.

| Column | Type | Notes |
|--------|------|-------|
| `engine` | text | Unity, Unreal, RPGMaker, KiriKiri, etc. |
| `programming_languages` | text[] | |
| `license` | enum | proprietary, freeware, shareware, open_source, public_domain, abandonware, subscription |
| `is_open_source` / `source_code_url` | bool / text | |
| `latest_version` / `build_size_bytes` | text / bigint | |
| `requires_internet` / `has_drm` / `drm_notes` | bool / text | |
| `server_status` | enum | not_applicable, online, sunset, preservation, private_server, announced |
| `server_shutdown_on` | date | for sunset servers |
| `minimum_requirements` / `recommended_requirements` | jsonb | |
| `supports_mods` / `supports_cross_platform` | bool | |

---

## 5. Tier 3 — Basic Types

Each table PK is simultaneously a FK to all required facets. The database
enforces facet membership at insert time.

---

### `movie`

Required facets: `audiovisual`, `narrative`

The fundamental cinematic unit. A film that is animated will also have an
`animation` row; the `is_live_action` column on `audiovisual` flips accordingly.

| Column | Notes |
|--------|-------|
| `theatrical_release_on` | |
| `is_short` / `is_feature` / `is_direct_to_video` | |
| `is_documentary` | true → narrative.is_fiction typically false |
| `festival_premiere` | 'Cannes 2023 — Competition' |
| `budget` / `box_office_gross` / `box_office_currency` | |
| `distributor_id` | FK organization |
| `film_series_position` | position within a numbered film series |
| `negative_format` / `printed_format` | archival film stock |

**Derived from movie:** anime film, animated feature, documentary, short film,
direct-to-video release.

---

### `show`

Required facets: `audiovisual`, `serialized`

All episodic television and streaming series. The `show_type` enum covers the
full range without additional tables.

| Column | Notes |
|--------|-------|
| `show_type` | tv, **ona**, **ova**, web, miniseries, special, tv_movie, pilot, anthology_series, variety, documentary_series, reality |
| `network_id` | FK organization |
| `season_count` | |
| `episode_runtime_seconds` | per-episode default |
| `original_run_start` / `original_run_end` | |
| `is_animated` | true → also has animation facet |
| `was_syndicated` / `time_slot` | broadcast metadata |

**Derived from show:** anime series (show + animation + JP origin), OVA
(show_type = ova), ONA (show_type = ona), live-action series, animated series,
documentary series.

---

### `book`

Required facets: `print`, `publication`

Every text-primary published work that is not a comic.

| Column | Notes |
|--------|-------|
| `book_type` | novel, novella, short_story, story_collection, anthology, poetry, essay_collection, memoir, biography, reference, textbook, manual, **picture_book**, **artbook**, cookbook, religious, academic_monograph, other |
| `volume_number` / `series_position` | numeric for .5 entries |
| `original_published_on` | vs. edition date in publication |
| `is_abridged` / `is_annotated` | |
| `dewey_decimal` / `library_of_congress` / `subject_headings` | library classification |

**Derived from book:** light novel (book + print.is_illustrated + JP origin),
web novel (book + publication_model = web_serial), artbook (book_type = artbook),
picture book (book_type = picture_book), light novel doujin, academic paper.

---

### `comic`

Required facets: `print`, `book`, `sequential_art`, `publication`

All panel-based visual storytelling regardless of origin or format.

| Column | Notes |
|--------|-------|
| `comic_format` | single_issue, collected_volume, one_shot, graphic_novel, strip, web_series, anthology_contribution, mini_series |
| `issue_number` / `volume_number` | numeric for special numbering |
| `chapter_range` | int4range covering chapters collected |
| `collects_issues` | freeform 'Vol.1 #1–6' |
| `is_anthology_piece` | contributed to an anthology |
| `magazine_id` | FK media → the host periodical |

**Derived from comic:** manga (JP origin), manhwa (KR origin), manhua (CN/TW/HK
origin), bande dessinée (FR/BE origin), webtoon (panel_layout =
vertical_scroll), doujinshi (publication_model = self_published /
fan_published), graphic novel (comic_format = graphic_novel), four-koma strip.

---

### `game` [needs more work]


Required facets: `interactive`, `software`

All games. DLC and expansions point back to their base game via
`base_game_id`.

| Column | Notes |
|--------|-------|
| `release_model` | retail, digital, free_to_play, early_access, subscription, shareware, arcade, browser, mod, demo, cancelled |
| `monetization` | premium, free, freemium, ad_supported, microtransaction, battle_pass, subscription, donation, pay_what_you_want, none |
| `base_game_id` | FK media → DLC/expansion parent |
| `is_expansion` / `is_remaster` / `is_port` | |
| `early_access_start` / `launch_price` / `currency` | |
| `has_achievements` / `has_level_editor` | |
| `peak_concurrent_players` | |

**Derived from game:** video games, visual novels, arcade
games, browser games, eroges (visual novel + media.is_adult), DLCs, expansions, mods.

---

### `album`

Required facets: `musical`, `audio`

A collection of music tracks released as a single work.

| Column | Notes |
|--------|-------|
| `album_type` | studio, live, compilation, ep, single, soundtrack, mixtape, remix, demo, bootleg, split, box_set |
| `label_id` | FK organization |
| `track_count` / `disc_count` | |
| `total_duration_seconds` | |
| `is_concept_album` / `is_compilation` | |
| `recorded_from` / `recorded_until` | session dates |
| `mastering_notes` | |

**Derived from album:** soundtrack (album_type = soundtrack), EP
(album_type = ep), single (album_type = single), live album, compilation,
doujin music (album + publication_model = self_published).

---

### `track`

Required facets: `musical`, `audio`

An individual piece of music. Tracks exist independently of albums and can be
linked to one.

| Column | Notes |
|--------|-------|
| `album_id` | FK album, nullable (standalone single) |
| `track_number` / `disc_number` | |
| `is_bonus_track` / `is_hidden_track` / `is_single` | |
| `original_track_id` | FK media → source work for covers/remixes |

---

### `podcast`

Required facets: `audio`, `serialized`

An episodic audio (or video) series distributed via RSS or streaming.

| Column | Notes |
|--------|-------|
| `podcast_type` | interview, narrative, audio_drama, news, educational, panel, solo, variety, rebroadcast |
| `feed_url` | RSS/Atom feed |
| `network_id` | FK organization |
| `is_video` | video podcast |
| `is_explicit` | |

---

### `stage_production`

Required facets: `performance`

A live theatrical or musical work.

| Column | Notes |
|--------|-------|
| `production_type` | play, musical, opera, operetta, ballet, dance, concert, concert_tour, standup, improv, circus, puppetry, performance_art, recital, pantomime |
| `company_id` | FK organization |
| `is_revival` / `is_touring` | |
| `original_production_id` | FK media → original production if this is a revival |
| `libretto_language_id` | for opera/musical |
| `has_live_orchestra` | |

---

### `artwork`

Required facets: `still_image`

A discrete visual work — painting, illustration, photograph, print, sculpture,
installation.

| Column | Notes |
|--------|-------|
| `artwork_type` | painting, illustration, photograph, poster, cover_art, concept_art, character_sheet, storyboard, comic_page, digital_art, sculpture, installation, print, sketch, infographic |
| `created_from` / `created_until` | date range |
| `is_commissioned` / `commissioned_by` | |
| `depicts_media_id` | FK media → cover art, fan art, promotional art |
| `signature_notes` | |

---

### `periodical`

Required facets: `publication`, `serialized`

A recurring publication — magazine, journal, newspaper, zine, newsletter.

| Column | Notes |
|--------|-------|
| `periodical_type` | magazine, journal, newspaper, **zine**, newsletter, anthology_magazine, trade_publication, **comic_magazine** |
| `frequency` | daily through annual, irregular |
| `circulation` | |
| `is_peer_reviewed` | academic journals |
| `editor_in_chief` | |
| `founded_on` / `ceased_on` | |

**Derived from periodical:** zine (periodical_type = zine), comic anthology
magazine (periodical_type = comic_magazine), academic journal, trade magazine.

---

## 6. Tier 4 — Composite Types

Only for works that genuinely span multiple basic types. Each adds FKs to all
required facets on top of a basic type's requirements.

---

### `visual_novel`

Extends: `game` (which requires `interactive` + `software`)
Additional required facets: `print`, `audiovisual`, `narrative`

The canonical multi-facet case. A VN is simultaneously a game (you make
choices), a text work (the writing is the primary content), and an
audiovisual work (sprites, CGs, music, voice acting compose the presentation).

| Column | Notes |
|--------|-------|
| `route_count` | number of distinct routes |
| `common_route_words` / `total_words` | word count breakdown |
| `is_kinetic` | no choices at all (kinetic novel) |
| `is_sound_novel` | Chunsoft-style |
| `voiced_extent` | none, partial, protagonist_excluded, full, full_including_protagonist |
| `voice_language_id` | |
| `cg_count` / `sprite_count` / `background_count` / `bgm_track_count` | asset counts |
| `has_sprite_animation` / `has_affection_system` / `has_flowchart` | |
| `has_skip_read_text` | |

---

### `audiobook`

Extends: `audio` and `book`
Additional required facets: `publication`, `narrative`
| Column | Notes |
|--------|-------|
| `source_book_id` | FK book → the text this is a reading of |
| `narration_style` | single_narrator, dual_narrator, full_cast, author_read, dramatized, synthetic |
| `is_abridged` / `is_dramatized` / `has_sound_design` | |
| `chapter_count` | |

---

### `music_video`

Extends: `audiovisual`
Additional required facets: `musical`

| Column | Notes |
|--------|-------|
| `track_id` | FK track → the musical work being visualized |
| `video_type` | official, lyric, performance, live, animated, concept, fan_made, teaser |
| `is_censored_cut` | |

---

### `tabletop_game`

Extends: `interactive`
Additional required facets: `publication`

Explicitly not software. The `interactive` facet covers the mechanical game
layer; `publication` covers the physical product.

| Column | Notes |
|--------|-------|
| `tabletop_type` | board, card, collectible_card, ttrpg, wargame, miniatures, party, dexterity, puzzle, escape_room, print_and_play, legacy |
| `minimum_age` | |
| `complexity_weight` | 0–5 (BGG scale) |
| `luck_factor` | 0–10 |
| `component_count` / `component_manifest` | |
| `rulebook_pages` | |
| `has_expansions` / `base_game_id` | |
| `is_legacy_campaign` | |

---

## 7. Support Tables

These are not media themselves; they hang off roots or basics.

| Table | Purpose |
|-------|---------|
| `media_title` | Full alias set (native, romanized, English, localized, alternate, working titles) per language/country |
| `media_image` | Cover art, posters, banners, screenshots, etc. One primary per type enforced by partial unique index |
| `person_image` / `character_image` | Images for people and characters |
| `media_external_id` | IDs in external databases (AniList, VNDB, IGDB, MusicBrainz, etc.) |
| `person_external_id` / `organization_external_id` | Same for people and orgs |
| `media_link` | Official sites, storefronts, streaming links, wikis, social |
| `person_alias` | Pen names, stage names, romanization variants |
| `installment` | Lightweight episode/chapter/issue rows that do not need their own media row. Promotes to a full media row via `promoted_media_id` when they need credits or facets |
| `media_credit` | Person ↔ work credits scoped to role, optionally to a specific installment |
| `media_organization` | Org ↔ work attachments (studio, publisher, distributor, licensor per country) |
| `media_character` | Character appearances in works, with billing |
| `character_portrayal` | Voice actor / on-stage performer ↔ character, per language |
| `media_relation` | Sequel, prequel, adaptation, port, remake, spin-off, etc. |
| `collection` | Named franchise / series / shared universe, hierarchical |
| `collection_entry` | Work ↔ collection membership with position |
| `media_release` | Regional and format-specific issuances of a work |
| `media_platform` | Platform availability |
| `media_tag` | Tag ↔ work with relevance score and spoiler flag |
| `media_language` | Languages a work is available in (audio / subtitle / text / interface) |
| `media_content_rating` | Content ratings from any rating system |
| `award` / `award_category` / `award_nomination` | Award registry and nominations |
| `facet` | Stable bit-position registry for all membership tables |
| `media_type` | Derived type definitions as bitmask + expression + SQL predicate |
| `media_type_membership` | Materialized type evaluations; refreshed by the application on write |

---

## 8. Derived Types

Seeded in `media_type`. All evaluated as `require_mask & kind_mask =
require_mask AND (exclude_mask & kind_mask = 0) AND extra_predicate`.

| Slug | Name | Defined as |
|------|------|------------|
| `film` | Film | `movie` facet |
| `anime` | Anime | `animation` + (`movie` or `show`) + JP origin |
| `cartoon` | Cartoon | `animation` + (`movie` or `show`) + not JP origin |
| `live_action_series` | Live-Action Series | `show` + not `animation` |
| `manga` | Manga | `comic` + JP origin |
| `manhwa` | Manhwa | `comic` + KR origin |
| `manhua` | Manhua | `comic` + CN/TW/HK origin |
| `webtoon` | Webtoon | `comic` + `panel_layout = vertical_scroll` |
| `doujinshi` | Doujinshi | `publication` + model in (self_published, fan_published) |
| `light_novel` | Light Novel | `book` + `print.is_illustrated` + JP origin |
| `web_novel` | Web Novel | `book` + `publication_model = web_serial` |
| `graphic_novel` | Graphic Novel | `comic` + `comic_format = graphic_novel` |
| `visual_novel` | Visual Novel | `visual_novel` composite facet |
| `video_game` | Video Game | `game` − `visual_novel` |
| `tabletop_rpg` | Tabletop RPG | `tabletop_game` + `tabletop_type = ttrpg` |
| `board_game` | Board Game | `tabletop_game` + type in (board, party, dexterity, legacy) |
| `audiobook` | Audiobook | `audiobook` composite facet |
| `audio_drama` | Audio Drama | `audio` + `narrative` − `musical` − `audiobook` |
| `podcast` | Podcast | `podcast` facet |
| `soundtrack` | Soundtrack | `album` + `album_type = soundtrack` |
| `single` | Single | `album` + type in (single, ep) |
| `music_video` | Music Video | `music_video` composite facet |
| `art_book` | Art Book | `book` + type in (artbook, picture_book) |
| `photobook` | Photobook | `publication` + `still_image` + `medium = photograph` |

Additional types trivially expressible but not seeded (add as needed):

- **OVA / ONA** → `show` + `show_type = ova/ona`
- **Eroge** → (`game` or `visual_novel`) + `media.is_adult`
- **Fan fiction** → `book` + `publication.is_derivative` + `publication_model = fan_published`
- **Doujin music** → `album` + `publication_model = self_published`
- **Zine** → `periodical` + `periodical_type = zine`
- **Motion comic** → `sequential_art` + `audiovisual`
- **Interactive fiction** → `book` (or `print`) + `interactive`
- **Gamebook** → `book` + `interactive`
- **Radio drama** → `audio` + `narrative` + recorded_at broadcast venue
- **Concert film** → `audiovisual` + `performance`
- **Recorded play** → `audiovisual` + `performance` + `stage_production`
- **Demo** → `game` + `release_model = demo`
- **Mod** → `game` + `release_model = mod`
- **Indie game** → `game` + `publication_model = self_published`

---

## 9. Index Reference

### Type and rationale

- **GIN + trgm** — trigram full-text search, handles partial matches and typos
- **GIN (array)** — membership tests on array columns
- **BTree** — range queries, equality, sorting, FK lookups
- **Partial BTree** — filtered index on a WHERE clause; smaller, faster for sparse booleans
- **Unique (partial)** — enforces a one-per-context constraint without blocking NULLs

### Root and entity search

| Index | Table | Type | Purpose |
|-------|-------|------|---------|
| `media_primary_title_trgm` | media | GIN trgm | Full-text search on primary_title |
| `media_romanized_title_trgm` | media | GIN trgm | Full-text search on romanized_title |
| `media_status_idx` | media | BTree | Filter by lifecycle status |
| `media_started_on_idx` | media | BTree DESC | Chronological browse / new releases |
| `media_popularity_idx` | media | BTree DESC | Popularity ranking |
| `media_mean_score_idx` | media | BTree DESC NULLS LAST | Score ranking |
| `media_country_idx` | media | BTree | Filter by country of origin |
| `media_language_idx` | media | BTree | Filter by original language |
| `media_adult_idx` | media | Partial BTree | Adult-only filter (sparse) |
| `person_name_trgm` | person | GIN trgm | Person name search |
| `organization_name_trgm` | organization | GIN trgm | Organization name search |
| `character_name_trgm` | fictional_character | GIN trgm | Character name search |
| `character_aliases_idx` | fictional_character | GIN (array) | Search across alias array |

### Title satellite

| Index | Table | Type | Purpose |
|-------|-------|------|---------|
| `media_title_media_idx` | media_title | BTree | Fetch all titles for a work |
| `media_title_trgm` | media_title | GIN trgm | Search across all title variants |
| `media_title_one_primary` | media_title | Unique Partial | One primary title per work |

### Image satellite

| Index | Table | Type | Purpose |
|-------|-------|------|---------|
| `media_image_media_idx` | media_image | BTree | Fetch images for a work by type |
| `media_image_one_primary` | media_image | Unique Partial | One primary image per work per type |

### Facet filtering

| Index | Table | Type | Purpose |
|-------|-------|------|---------|
| `audiovisual_runtime_idx` | audiovisual | BTree | Runtime range filter |
| `print_page_count_idx` | print | BTree | Page count filter |
| `sequential_art_layout_idx` | sequential_art | BTree | Layout type filter (webtoon, four-koma) |
| `animation_technique_idx` | animation | BTree | Technique filter |
| `publication_model_idx` | publication | BTree | Publication model filter (doujin, web serial) |
| `publication_publisher_idx` | publication | BTree | All works by a publisher |
| `publication_isbn13_idx` | publication | Partial BTree | ISBN-13 lookup (sparse) |
| `serialized_ongoing_idx` | serialized | Partial BTree | Ongoing works only (sparse) |
| `serialized_host_idx` | serialized | BTree | All serials in a host magazine |
| `interactive_playtime_idx` | interactive | BTree | Playtime range filter |
| `musical_isrc_idx` | musical | Partial BTree | ISRC music ID lookup (sparse) |

### Basic type filtering

| Index | Table | Type | Purpose |
|-------|-------|------|---------|
| `show_network_idx` | show | BTree | All shows by network |
| `show_type_idx` | show | BTree | OVA / ONA / TV filter |
| `book_type_idx` | book | BTree | Novel / artbook / textbook filter |
| `comic_format_idx` | comic | BTree | Single issue / GN / web series filter |
| `comic_magazine_idx` | comic | BTree | All comics in a host magazine |
| `game_base_idx` | game | BTree | All DLC / expansions for a base game |
| `track_album_idx` | track | BTree | All tracks in an album |
| `album_label_idx` | album | BTree | All albums by a label |
| `artwork_depicts_idx` | artwork | BTree | All artwork depicting a specific work |
| `audiobook_source_idx` | audiobook | BTree | All audiobook editions of a book |
| `music_video_track_idx` | music_video | BTree | All videos for a track |
| `tabletop_type_idx` | tabletop_game | BTree | Board / TTRPG / CCG filter |

### Installments

| Index | Table | Type | Purpose |
|-------|-------|------|---------|
| `installment_serialized_idx` | installment | BTree | All installments in a serialized work, ordered by number |
| `installment_released_idx` | installment | BTree DESC | Chronological installment browse |

### Joins (credits, relations, tags, releases)

| Index | Table | Type | Purpose |
|-------|-------|------|---------|
| `media_credit_media_idx` | media_credit | BTree | All credits for a work, in sort order |
| `media_credit_person_idx` | media_credit | BTree | All works credited to a person |
| `media_credit_role_idx` | media_credit | BTree | All credits for a role |
| `media_org_media_idx` | media_organization | BTree | All orgs attached to a work |
| `media_org_org_idx` | media_organization | BTree | All works attached to an org by role |
| `media_character_media_idx` | media_character | BTree | Characters in a work, by billing and sort |
| `media_character_char_idx` | media_character | BTree | All works a character appears in |
| `portrayal_person_idx` | character_portrayal | BTree | All portrayals by a person (voice roles) |
| `media_relation_source_idx` | media_relation | BTree | All relations from a work |
| `media_relation_target_idx` | media_relation | BTree | All relations pointing to a work |
| `collection_entry_media_idx` | collection_entry | BTree | All collections a work belongs to |
| `collection_name_trgm` | collection | GIN trgm | Collection/franchise name search |
| `media_release_media_idx` | media_release | BTree | All releases of a work |
| `media_release_country_idx` | media_release | BTree | All releases in a country |
| `media_release_platform_idx` | media_release | BTree | All releases on a platform |
| `media_tag_tag_idx` | media_tag | BTree | All works for a tag, by relevance |
| `tag_namespace_idx` | tag | BTree | Tags filtered by namespace |
| `tag_name_trgm` | tag | GIN trgm | Tag name search |
| `media_platform_platform_idx` | media_platform | BTree | All works on a platform |
| `media_content_rating_idx` | media_content_rating | BTree | All works with a given rating |
| `award_nomination_media_idx` | award_nomination | BTree | All nominations for a work |
| `award_nomination_person_idx` | award_nomination | BTree | All nominations for a person |
| `award_nomination_cat_idx` | award_nomination | BTree | All nominees in a category by year |
| `media_external_id_lookup` | media_external_id | BTree | Look up a work by external ID (AniList, VNDB, etc.) |

### Derived type system

| Index | Table | Type | Purpose |
|-------|-------|------|---------|
| `media_type_membership_type_idx` | media_type_membership | BTree | All works of a given derived type |
| `media_type_one_primary` | media_type_membership | Unique Partial | One primary type per work |
