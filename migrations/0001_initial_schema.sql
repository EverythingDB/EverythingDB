-- =============================================================================
-- 0001_initial_schema.sql
-- EverythingDB — unified media catalog
-- =============================================================================
--
-- LAYERING MODEL
-- --------------
--   Tier 0  reference / vocabulary   language, country, tag, platform, ...
--   Tier 1  root                     media, person, organization, fictional_character
--   Tier 2  property (facet)         orthogonal *forms* a work can take.
--                                    PK is also an FK to media(id).
--   Tier 3  basic                    the everyday nameable types. PK is an FK to
--                                    every facet the type requires, so membership
--                                    is enforced by the FK graph itself.
--   Tier 4  composite                genuinely hybrid works that are not
--                                    expressible as a single basic type.
--
-- WHAT IS *NOT* A TABLE
-- ---------------------
-- Most "media types" people name are not forms, they are qualifiers over an
-- existing form. These are derived, never tabled:
--
--   doujinshi   = comic/book with publication.publication_model='self_published'
--                 (+ optionally publication.is_derivative and a 'derived_from'
--                 media_relation to the parent work)
--   manga       = comic with media.country_of_origin='JP'
--   manhwa      = comic with media.country_of_origin='KR'
--   webtoon     = comic with sequential_art.panel_layout='vertical_scroll'
--   anime       = (movie|show) + animation + country_of_origin='JP'
--   light novel = book + print.is_illustrated + demographic + JP origin
--   web novel   = book with publication.publication_model='web_serial'
--   OVA/ONA     = show.show_type
--   eroge       = visual_novel/game with media.is_adult
--   fan works   = publication.is_derivative + publication.is_official=false
--   soundtrack  = album.album_type='soundtrack'
--   zine        = periodical.periodical_type='zine'
--
-- Those live in `media_type`, as boolean expressions over table membership,
-- evaluated either as a bitmask test (Rust) or a compiled SQL fragment.
-- See section 11.
--
-- Every child table cascades from its parent, so deleting a media row deletes
-- the whole stack.
-- =============================================================================

CREATE EXTENSION IF NOT EXISTS citext;
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- -----------------------------------------------------------------------------
-- 0. Helpers
-- -----------------------------------------------------------------------------

CREATE FUNCTION set_updated_at() RETURNS trigger AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- =============================================================================
-- 1. ENUMERATED DOMAINS
-- =============================================================================

CREATE TYPE date_precision   AS ENUM ('day','month','season','year','decade','unknown');
CREATE TYPE media_status     AS ENUM ('announced','in_production','releasing','on_hiatus','completed','cancelled','lost','unknown');
CREATE TYPE source_material  AS ENUM ('original','novel','light_novel','web_novel','manga','comic','game','visual_novel','tabletop','film','show','music','folklore','religious_text','historical','biography','news','other','unknown');
CREATE TYPE gender           AS ENUM ('male','female','other','unspecified');
CREATE TYPE org_type         AS ENUM ('studio','publisher','imprint','developer','distributor','record_label','network','streaming_service','production_committee','licensor','printer','theatre_company','collective','museum','other');

CREATE TYPE title_type       AS ENUM ('primary','native','romanized','english','localized','alternative','abbreviation','working','translated');
CREATE TYPE image_type       AS ENUM ('cover','poster','banner','backdrop','logo','thumbnail','screenshot','still','character_art','concept_art','promotional','spine','back_cover');

CREATE TYPE relation_type    AS ENUM ('sequel','prequel','side_story','parent_story','adaptation','adapted_from','alternative_version','alternative_setting','spin_off','summary','full_story','character_shared','setting_shared','remake','remaster','port','localization','abridgement','derived_from','contains','contained_in','soundtrack_of','has_soundtrack','other');
CREATE TYPE collection_type  AS ENUM ('franchise','series','shared_universe','trilogy','box_set','discography','anthology','arc','crossover_event');

CREATE TYPE credit_department AS ENUM ('direction','writing','art','animation','production','performance','voice','music','sound','photography','editing','design','engineering','translation','publishing','other');
CREATE TYPE character_billing AS ENUM ('main','supporting','recurring','minor','background','cameo','antagonist','narrator');

-- Facet-specific
CREATE TYPE narrative_form      AS ENUM ('linear','nonlinear','episodic','anthology','branching','vignette','frame_story','experimental');
CREATE TYPE point_of_view       AS ENUM ('first_person','second_person','third_limited','third_omniscient','multiple','objective','epistolary','mixed');
CREATE TYPE prose_format        AS ENUM ('prose','verse','script','screenplay','epistolary','diary','reference','mixed');
CREATE TYPE reading_direction   AS ENUM ('ltr','rtl','vertical_rtl','vertical_ltr','boustrophedon','mixed');
CREATE TYPE panel_layout        AS ENUM ('page','double_page','vertical_scroll','horizontal_strip','four_koma','single_panel','freeform','mixed');
CREATE TYPE coloring_mode       AS ENUM ('monochrome','greyscale','duotone','spot_color','full_color','mixed');
CREATE TYPE color_system        AS ENUM ('black_and_white','color','colorized','tinted','mixed');
CREATE TYPE capture_medium      AS ENUM ('film_8mm','film_16mm','film_35mm','film_65mm','imax','videotape','digital','virtual','mixed','unknown');
CREATE TYPE animation_technique AS ENUM ('traditional_cel','digital_2d','cgi_3d','stop_motion','claymation','puppet','cutout','rotoscope','pixel_art','motion_graphics','sand','paint_on_glass','live_hybrid','mixed');
CREATE TYPE recording_type      AS ENUM ('studio','live','field','remote','synthetic','archival','mixed');
CREATE TYPE vocal_type          AS ENUM ('lead','duet','group','choral','spoken_word','rap','instrumental','vocaloid','mixed');
CREATE TYPE input_method        AS ENUM ('keyboard','mouse','gamepad','touch','stylus','motion','vr','light_gun','arcade_stick','dance_pad','voice','eye_tracking','physical_component','dice','cards');
CREATE TYPE save_system         AS ENUM ('none','password','checkpoint','manual_slot','autosave','save_anywhere','permadeath','cloud_only','mixed');
CREATE TYPE branching_structure AS ENUM ('none','linear','hub','branching','open_world','procedural','sandbox');
CREATE TYPE software_license    AS ENUM ('proprietary','freeware','shareware','open_source','public_domain','abandonware','subscription','unknown');
CREATE TYPE server_status       AS ENUM ('not_applicable','online','sunset','preservation','private_server','announced');
CREATE TYPE staging_type        AS ENUM ('proscenium','thrust','in_the_round','black_box','immersive','site_specific','street','arena','stadium','broadcast','virtual');
CREATE TYPE visual_medium       AS ENUM ('oil','acrylic','watercolor','gouache','ink','pencil','charcoal','pastel','digital','photograph','screenprint','woodblock','etching','lithograph','collage','mixed_media','other');

CREATE TYPE publication_model   AS ENUM ('traditional','small_press','academic','self_published','vanity','web_serial','fan_published','commissioned','government','unpublished');
CREATE TYPE distribution_format AS ENUM ('print_only','digital_only','print_and_digital','broadcast','streaming','physical_media','download','cartridge','disc','tape','live_only','mixed');
CREATE TYPE binding_format      AS ENUM ('hardcover','trade_paperback','mass_market','tankobon','bunko','aizoban','kanzenban','omnibus','box_set','saddle_stitch','spiral','ebook','loose_leaf','scroll','none');
CREATE TYPE audience_demographic AS ENUM ('children','middle_grade','young_adult','shounen','shoujo','seinen','josei','kodomomuke','general','adult','academic','professional');

CREATE TYPE installment_unit    AS ENUM ('episode','chapter','issue','volume','track','part','session','act','strip','entry');
CREATE TYPE release_schedule    AS ENUM ('daily','weekdays','weekly','biweekly','monthly','bimonthly','quarterly','seasonal','annual','irregular','burst','completed');

-- Basic-tier specific
CREATE TYPE show_type           AS ENUM ('tv','ona','ova','web','miniseries','special','tv_movie','pilot','anthology_series','variety','documentary_series','reality');
CREATE TYPE book_type           AS ENUM ('novel','novella','short_story','story_collection','anthology','poetry','essay_collection','memoir','biography','reference','textbook','manual','picture_book','artbook','cookbook','religious','academic_monograph','other');
CREATE TYPE comic_format        AS ENUM ('single_issue','collected_volume','one_shot','graphic_novel','strip','web_series','anthology_contribution','mini_series');
CREATE TYPE game_release_model  AS ENUM ('retail','digital','free_to_play','early_access','subscription','shareware','arcade','browser','mod','demo','cancelled');
CREATE TYPE monetization_model  AS ENUM ('premium','free','freemium','ad_supported','microtransaction','battle_pass','subscription','donation','pay_what_you_want','none');
CREATE TYPE album_type          AS ENUM ('studio','live','compilation','ep','single','soundtrack','mixtape','remix','demo','bootleg','split','box_set');
CREATE TYPE podcast_type        AS ENUM ('interview','narrative','audio_drama','news','educational','panel','solo','variety','rebroadcast');
CREATE TYPE production_type     AS ENUM ('play','musical','opera','operetta','ballet','dance','concert','concert_tour','standup','improv','circus','puppetry','performance_art','recital','pantomime');
CREATE TYPE artwork_type        AS ENUM ('painting','illustration','photograph','poster','cover_art','concept_art','character_sheet','storyboard','comic_page','digital_art','sculpture','installation','print','sketch','infographic');
CREATE TYPE periodical_type     AS ENUM ('magazine','journal','newspaper','zine','newsletter','anthology_magazine','trade_publication','comic_magazine');

-- Composite-tier specific
CREATE TYPE voiced_extent       AS ENUM ('none','partial','protagonist_excluded','full','full_including_protagonist');
CREATE TYPE narration_style     AS ENUM ('single_narrator','dual_narrator','full_cast','author_read','dramatized','synthetic');
CREATE TYPE music_video_type    AS ENUM ('official','lyric','performance','live','animated','concept','fan_made','teaser');
CREATE TYPE tabletop_type       AS ENUM ('board','card','collectible_card','ttrpg','wargame','miniatures','party','dexterity','puzzle','escape_room','print_and_play','legacy');

-- Registry
CREATE TYPE facet_tier          AS ENUM ('property','basic','composite');
CREATE TYPE platform_kind       AS ENUM ('console','handheld','arcade','computer','mobile','browser','vr','streaming_service','broadcast_network','print_channel','tabletop','other');

-- =============================================================================
-- 2. REFERENCE / VOCABULARY
-- =============================================================================

CREATE TABLE language (
    id          integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    iso_639_1   char(2) UNIQUE,
    iso_639_3   char(3) NOT NULL UNIQUE,
    name        text NOT NULL,
    native_name text,
    script      text,
    direction   reading_direction NOT NULL DEFAULT 'ltr'
);

CREATE TABLE country (
    id          integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    iso_3166_1  char(2) NOT NULL UNIQUE,
    name        text NOT NULL,
    native_name text,
    region      text
);

CREATE TABLE tag (
    id            integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    slug          citext NOT NULL UNIQUE,
    name          text NOT NULL,
    namespace     text NOT NULL,   -- genre | theme | setting | content_warning
                                   -- demographic | technique | origin | franchise_trait
                                   -- publishing_trait | mood | misc
    description   text,
    parent_id     integer REFERENCES tag(id) ON DELETE SET NULL,
    is_adult      boolean NOT NULL DEFAULT false,
    is_spoiler    boolean NOT NULL DEFAULT false,
    is_moderated  boolean NOT NULL DEFAULT true,
    usage_count   integer NOT NULL DEFAULT 0,
    created_at    timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE credit_role (
    id          integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    slug        citext NOT NULL UNIQUE,
    name        text NOT NULL,
    department  credit_department NOT NULL,
    description text,
    -- roles are form-scoped hints, not hard constraints (a "director" exists for
    -- film, stage, audio drama and games alike)
    is_primary_credit boolean NOT NULL DEFAULT false
);

CREATE TABLE platform (
    id             integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    slug           citext NOT NULL UNIQUE,
    name           text NOT NULL,
    kind           platform_kind NOT NULL,
    manufacturer_id integer,           -- FK added after organization
    released_on    date,
    discontinued_on date,
    generation     smallint
);

CREATE TABLE rating_system (
    id          integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    slug        citext NOT NULL UNIQUE,
    name        text NOT NULL,         -- ESRB, PEGI, MPAA, CERO, BBFC, TV Parental
    country_id  integer REFERENCES country(id) ON DELETE SET NULL,
    applies_to  text                   -- freeform scope hint
);

CREATE TABLE content_rating (
    id               integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    rating_system_id integer NOT NULL REFERENCES rating_system(id) ON DELETE CASCADE,
    code             text NOT NULL,    -- 'M', '18', 'PG-13', 'Z'
    label            text,
    minimum_age      smallint,
    sort_order       smallint NOT NULL DEFAULT 0,
    UNIQUE (rating_system_id, code)
);

CREATE TABLE external_source (
    id           integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    slug         citext NOT NULL UNIQUE,   -- anilist, mal, imdb, vndb, igdb, isbndb,
                                           -- musicbrainz, tmdb, bgg, openlibrary, discogs
    name         text NOT NULL,
    base_url     text,
    url_template text,                     -- e.g. 'https://anilist.co/anime/{id}'
    is_authoritative boolean NOT NULL DEFAULT false
);

-- =============================================================================
-- 3. ROOT TIER
-- =============================================================================

CREATE TABLE media (
    id                   integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    slug                 citext UNIQUE,

    -- naming (full alias set lives in media_title)
    primary_title        text NOT NULL,
    original_title       text,
    romanized_title      text,
    sort_title           text,

    -- provenance
    original_language_id integer REFERENCES language(id) ON DELETE SET NULL,
    country_of_origin_id integer REFERENCES country(id)  ON DELETE SET NULL,
    source_material      source_material NOT NULL DEFAULT 'unknown',

    -- lifecycle
    status               media_status NOT NULL DEFAULT 'unknown',
    started_on           date,
    ended_on             date,
    date_precision       date_precision NOT NULL DEFAULT 'day',
    is_indefinite        boolean NOT NULL DEFAULT false,

    -- description
    tagline              text,
    synopsis             text,
    synopsis_language_id integer REFERENCES language(id) ON DELETE SET NULL,
    notes                text,

    -- classification flags that apply to every form
    is_adult             boolean NOT NULL DEFAULT false,
    is_official          boolean NOT NULL DEFAULT true,
    is_lost_media        boolean NOT NULL DEFAULT false,
    is_unreleased        boolean NOT NULL DEFAULT false,

    -- denormalized aggregates, maintained by the application
    mean_score           numeric(5,2) CHECK (mean_score BETWEEN 0 AND 100),
    score_count          integer NOT NULL DEFAULT 0,
    popularity           integer NOT NULL DEFAULT 0,
    favorite_count       integer NOT NULL DEFAULT 0,

    -- curation
    data_completeness    smallint NOT NULL DEFAULT 0 CHECK (data_completeness BETWEEN 0 AND 100),
    is_locked            boolean NOT NULL DEFAULT false,
    verified_at          timestamptz,
    created_at           timestamptz NOT NULL DEFAULT now(),
    updated_at           timestamptz NOT NULL DEFAULT now(),

    CONSTRAINT media_date_order CHECK (ended_on IS NULL OR started_on IS NULL OR ended_on >= started_on)
);

COMMENT ON TABLE media IS 'Root of every catalogued work. Form is expressed by which facet tables share this id.';

CREATE TABLE person (
    id              integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    slug            citext UNIQUE,
    primary_name    text NOT NULL,
    native_name     text,
    romanized_name  text,
    sort_name       text,
    given_name      text,
    family_name     text,

    gender          gender NOT NULL DEFAULT 'unspecified',
    pronouns        text,
    birth_date      date,
    birth_precision date_precision NOT NULL DEFAULT 'day',
    death_date      date,
    birth_country_id integer REFERENCES country(id) ON DELETE SET NULL,
    hometown        text,
    height_cm       smallint,
    blood_type      text,

    is_group        boolean NOT NULL DEFAULT false,   -- bands, art collectives, circles
    active_from     date,
    active_until    date,
    primary_language_id integer REFERENCES language(id) ON DELETE SET NULL,

    biography       text,
    website         text,
    created_at      timestamptz NOT NULL DEFAULT now(),
    updated_at      timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE organization (
    id             integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    slug           citext UNIQUE,
    name           text NOT NULL,
    native_name    text,
    romanized_name text,
    org_type       org_type NOT NULL,
    parent_id      integer REFERENCES organization(id) ON DELETE SET NULL,  -- imprints, subsidiaries
    country_id     integer REFERENCES country(id) ON DELETE SET NULL,
    founded_on     date,
    dissolved_on   date,
    headquarters   text,
    website        text,
    description    text,
    is_defunct     boolean NOT NULL DEFAULT false,
    created_at     timestamptz NOT NULL DEFAULT now(),
    updated_at     timestamptz NOT NULL DEFAULT now()
);

ALTER TABLE platform
    ADD CONSTRAINT platform_manufacturer_fk
    FOREIGN KEY (manufacturer_id) REFERENCES organization(id) ON DELETE SET NULL;

CREATE TABLE fictional_character (
    id              integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    slug            citext UNIQUE,
    primary_name    text NOT NULL,
    native_name     text,
    romanized_name  text,
    aliases         text[] NOT NULL DEFAULT '{}',
    gender          gender NOT NULL DEFAULT 'unspecified',
    pronouns        text,
    age_description text,          -- '17', 'ageless', 'approx. 400'
    birthday_month  smallint CHECK (birthday_month BETWEEN 1 AND 12),
    birthday_day    smallint CHECK (birthday_day BETWEEN 1 AND 31),
    height_cm       smallint,
    blood_type      text,
    species         text,
    occupation      text,
    description     text,
    is_spoiler_heavy boolean NOT NULL DEFAULT false,
    created_at      timestamptz NOT NULL DEFAULT now(),
    updated_at      timestamptz NOT NULL DEFAULT now()
);

-- =============================================================================
-- 4. ROOT SATELLITES (titles, images, ids, links)
-- =============================================================================

CREATE TABLE media_title (
    id          integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    media_id    integer NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    title       text NOT NULL,
    title_type  title_type NOT NULL,
    language_id integer REFERENCES language(id) ON DELETE SET NULL,
    country_id  integer REFERENCES country(id) ON DELETE SET NULL,
    is_primary  boolean NOT NULL DEFAULT false,
    UNIQUE (media_id, title, title_type, language_id)
);

CREATE TABLE person_alias (
    id          integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    person_id   integer NOT NULL REFERENCES person(id) ON DELETE CASCADE,
    alias       text NOT NULL,
    language_id integer REFERENCES language(id) ON DELETE SET NULL,
    is_pen_name boolean NOT NULL DEFAULT false,
    notes       text,
    UNIQUE (person_id, alias)
);

CREATE TABLE media_image (
    id          integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    media_id    integer NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    url         text NOT NULL,
    image_type  image_type NOT NULL,
    language_id integer REFERENCES language(id) ON DELETE SET NULL,
    width_px    integer,
    height_px   integer,
    blurhash    text,
    dominant_color char(7),
    is_primary  boolean NOT NULL DEFAULT false,
    is_adult    boolean NOT NULL DEFAULT false,
    is_spoiler  boolean NOT NULL DEFAULT false,
    attribution text
);

CREATE TABLE character_image (
    id            integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    character_id  integer NOT NULL REFERENCES fictional_character(id) ON DELETE CASCADE,
    url           text NOT NULL,
    image_type    image_type NOT NULL DEFAULT 'character_art',
    is_primary    boolean NOT NULL DEFAULT false,
    is_spoiler    boolean NOT NULL DEFAULT false
);

CREATE TABLE person_image (
    id         integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    person_id  integer NOT NULL REFERENCES person(id) ON DELETE CASCADE,
    url        text NOT NULL,
    is_primary boolean NOT NULL DEFAULT false
);

CREATE TABLE media_external_id (
    media_id           integer NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    external_source_id integer NOT NULL REFERENCES external_source(id) ON DELETE CASCADE,
    external_id        text NOT NULL,
    url                text,
    last_synced_at     timestamptz,
    PRIMARY KEY (media_id, external_source_id, external_id)
);

CREATE TABLE person_external_id (
    person_id          integer NOT NULL REFERENCES person(id) ON DELETE CASCADE,
    external_source_id integer NOT NULL REFERENCES external_source(id) ON DELETE CASCADE,
    external_id        text NOT NULL,
    url                text,
    PRIMARY KEY (person_id, external_source_id, external_id)
);

CREATE TABLE organization_external_id (
    organization_id    integer NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    external_source_id integer NOT NULL REFERENCES external_source(id) ON DELETE CASCADE,
    external_id        text NOT NULL,
    url                text,
    PRIMARY KEY (organization_id, external_source_id, external_id)
);

CREATE TABLE media_link (
    id       integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    media_id integer NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    url      text NOT NULL,
    label    text,
    kind     text,   -- official_site, store, streaming, wiki, social, archive
    language_id integer REFERENCES language(id) ON DELETE SET NULL,
    country_id  integer REFERENCES country(id) ON DELETE SET NULL
);

-- =============================================================================
-- 5. PROPERTY TIER — orthogonal forms
--    id is PK *and* FK to media(id). Presence of the row == membership.
-- =============================================================================

-- 5.1 narrative — the work tells a story
CREATE TABLE narrative (
    id                integer PRIMARY KEY REFERENCES media(id) ON DELETE CASCADE,
    is_fiction        boolean NOT NULL DEFAULT true,
    narrative_form    narrative_form NOT NULL DEFAULT 'linear',
    point_of_view     point_of_view,
    setting_period    text,          -- 'Edo period', '2247 CE', 'contemporary'
    setting_place     text,
    is_ensemble_cast  boolean NOT NULL DEFAULT false,
    protagonist_count smallint,
    ending_count      smallint NOT NULL DEFAULT 1,
    has_multiple_endings boolean GENERATED ALWAYS AS (ending_count > 1) STORED,
    is_self_contained boolean NOT NULL DEFAULT true,
    chronology_index  numeric(8,2),  -- position in an in-universe timeline
    is_canon          boolean NOT NULL DEFAULT true,
    content_summary   text,          -- full, spoiler-inclusive
    themes_summary    text
);

-- 5.2 print — text carried on pages
CREATE TABLE print (
    id                    integer PRIMARY KEY REFERENCES media(id) ON DELETE CASCADE,
    page_count            integer,
    word_count            integer,
    character_count       integer,
    prose_format          prose_format NOT NULL DEFAULT 'prose',
    reading_direction     reading_direction NOT NULL DEFAULT 'ltr',
    script_language_id    integer REFERENCES language(id) ON DELETE SET NULL,
    is_illustrated        boolean NOT NULL DEFAULT false,
    illustration_count    integer,
    has_furigana          boolean NOT NULL DEFAULT false,
    has_footnotes         boolean NOT NULL DEFAULT false,
    has_index             boolean NOT NULL DEFAULT false,
    has_bibliography      boolean NOT NULL DEFAULT false,
    reading_level         text,      -- Lexile, JLPT, CEFR, grade
    estimated_reading_minutes integer,
    is_translation        boolean NOT NULL DEFAULT false,
    translated_from_id    integer REFERENCES language(id) ON DELETE SET NULL
);

-- 5.3 sequential_art — panel-driven visual storytelling
CREATE TABLE sequential_art (
    id                     integer PRIMARY KEY REFERENCES media(id) ON DELETE CASCADE,
    panel_layout           panel_layout NOT NULL DEFAULT 'page',
    page_count             integer,
    panel_count            integer,
    average_panels_per_page numeric(5,2),
    coloring               coloring_mode NOT NULL DEFAULT 'monochrome',
    uses_screentone        boolean NOT NULL DEFAULT false,
    is_digital_native      boolean NOT NULL DEFAULT false,
    canvas_width_px        integer,   -- webtoon/vertical formats
    art_style              text,
    line_art_medium        text,      -- 'ink on bristol', 'vector', 'raster digital'
    has_sound_effects_inline boolean NOT NULL DEFAULT true,
    lettering_is_typeset   boolean,
    is_wordless            boolean NOT NULL DEFAULT false
);

-- 5.4 audiovisual — moving image
CREATE TABLE audiovisual (
    id                  integer PRIMARY KEY REFERENCES media(id) ON DELETE CASCADE,
    runtime_seconds     integer,
    aspect_ratio        text,          -- '16:9', '2.39:1', '4:3'
    frame_rate          numeric(6,3),
    resolution_width    integer,
    resolution_height   integer,
    color_system        color_system NOT NULL DEFAULT 'color',
    is_hdr              boolean NOT NULL DEFAULT false,
    is_stereoscopic_3d  boolean NOT NULL DEFAULT false,
    is_silent           boolean NOT NULL DEFAULT false,
    capture_medium      capture_medium NOT NULL DEFAULT 'digital',
    audio_channel_layout text,         -- 'mono', '5.1', 'Atmos'
    audio_codec         text,
    video_codec         text,
    has_subtitles       boolean NOT NULL DEFAULT false,
    has_closed_captions boolean NOT NULL DEFAULT false,
    has_audio_description boolean NOT NULL DEFAULT false,
    filming_locations   text[] NOT NULL DEFAULT '{}',
    is_live_action      boolean NOT NULL DEFAULT true,   -- false when `animation` present
    post_credits_scenes smallint NOT NULL DEFAULT 0
);

-- 5.5 animation — non-photographic moving image (refines audiovisual, but also
--     applies to animated stills, motion comics, animated interactive art)
CREATE TABLE animation (
    id                    integer PRIMARY KEY REFERENCES media(id) ON DELETE CASCADE,
    technique             animation_technique NOT NULL,
    secondary_techniques  animation_technique[] NOT NULL DEFAULT '{}',
    animates_on           smallint,     -- 1 = on ones, 2 = on twos
    cel_count             integer,
    key_frame_count       integer,
    in_between_count      integer,
    uses_motion_capture   boolean NOT NULL DEFAULT false,
    uses_rotoscoping      boolean NOT NULL DEFAULT false,
    render_engine         text,
    is_hand_drawn         boolean NOT NULL DEFAULT false,
    character_design_notes text
);

-- 5.6 audio — sound is the primary carrier
CREATE TABLE audio (
    id                integer PRIMARY KEY REFERENCES media(id) ON DELETE CASCADE,
    duration_seconds  integer,
    recording_type    recording_type NOT NULL DEFAULT 'studio',
    sample_rate_hz    integer,
    bit_depth         smallint,
    bitrate_kbps      integer,
    channel_layout    text,
    is_lossless       boolean NOT NULL DEFAULT false,
    loudness_lufs     numeric(5,2),
    dynamic_range_db  numeric(5,2),
    is_dialogue_driven boolean NOT NULL DEFAULT false,
    has_transcript    boolean NOT NULL DEFAULT false,
    spoken_language_id integer REFERENCES language(id) ON DELETE SET NULL,
    recorded_on       date,
    recording_venue   text
);

-- 5.7 musical — organized sound with musical structure
CREATE TABLE musical (
    id                 integer PRIMARY KEY REFERENCES media(id) ON DELETE CASCADE,
    bpm                numeric(6,2),
    musical_key        text,           -- 'F# minor'
    time_signature     text,           -- '4/4', '7/8'
    tuning_hz          numeric(6,2),
    instrumentation    text[] NOT NULL DEFAULT '{}',
    is_instrumental    boolean NOT NULL DEFAULT false,
    vocal_type         vocal_type,
    lyrics_language_id integer REFERENCES language(id) ON DELETE SET NULL,
    has_explicit_lyrics boolean NOT NULL DEFAULT false,
    lyrics             text,
    isrc               text,
    iswc               text,
    energy             numeric(4,3) CHECK (energy BETWEEN 0 AND 1),
    valence            numeric(4,3) CHECK (valence BETWEEN 0 AND 1),
    acousticness       numeric(4,3) CHECK (acousticness BETWEEN 0 AND 1),
    danceability       numeric(4,3) CHECK (danceability BETWEEN 0 AND 1),
    is_cover           boolean NOT NULL DEFAULT false,
    is_remix           boolean NOT NULL DEFAULT false
);

-- 5.8 interactive — user input changes the state of the work
CREATE TABLE interactive (
    id                        integer PRIMARY KEY REFERENCES media(id) ON DELETE CASCADE,
    input_methods             input_method[] NOT NULL DEFAULT '{}',
    branching_structure       branching_structure NOT NULL DEFAULT 'linear',
    choice_count              integer,
    ending_count              smallint,
    player_min                smallint NOT NULL DEFAULT 1,
    player_max                smallint,
    is_multiplayer            boolean NOT NULL DEFAULT false,
    is_cooperative            boolean NOT NULL DEFAULT false,
    is_competitive            boolean NOT NULL DEFAULT false,
    has_online_play           boolean NOT NULL DEFAULT false,
    save_system               save_system NOT NULL DEFAULT 'none',
    has_difficulty_options    boolean NOT NULL DEFAULT false,
    has_permadeath            boolean NOT NULL DEFAULT false,
    uses_procedural_generation boolean NOT NULL DEFAULT false,
    main_playtime_minutes     integer,
    completionist_playtime_minutes integer,
    session_length_minutes    integer,
    accessibility_features    text[] NOT NULL DEFAULT '{}',
    control_notes             text,
    CONSTRAINT interactive_player_range CHECK (player_max IS NULL OR player_max >= player_min)
);

-- 5.9 performance — realized live by performers
CREATE TABLE performance (
    id                 integer PRIMARY KEY REFERENCES media(id) ON DELETE CASCADE,
    staging            staging_type NOT NULL DEFAULT 'proscenium',
    duration_seconds   integer,
    act_count          smallint,
    intermission_count smallint NOT NULL DEFAULT 0,
    cast_size          smallint,
    ensemble_size      smallint,
    orchestra_size     smallint,
    premiere_date      date,
    closing_date       date,
    performance_count  integer,
    is_improvised      boolean NOT NULL DEFAULT false,
    is_recorded        boolean NOT NULL DEFAULT false,
    requires_audience_participation boolean NOT NULL DEFAULT false,
    venue_name         text,
    venue_city         text,
    venue_country_id   integer REFERENCES country(id) ON DELETE SET NULL
);

-- 5.10 still_image — a fixed visual artifact
CREATE TABLE still_image (
    id                integer PRIMARY KEY REFERENCES media(id) ON DELETE CASCADE,
    medium            visual_medium NOT NULL DEFAULT 'digital',
    is_digital        boolean NOT NULL DEFAULT true,
    width_mm          numeric(8,2),
    height_mm         numeric(8,2),
    depth_mm          numeric(8,2),
    width_px          integer,
    height_px         integer,
    dpi               integer,
    color_space       text,
    support_material  text,           -- canvas, washi, bristol, panel
    technique_notes   text,
    edition_number    integer,
    edition_size      integer,
    is_unique_piece   boolean NOT NULL DEFAULT false,
    current_location  text,           -- museum / collection
    exif              jsonb
);

-- 5.11 publication — issued as an edition by someone
CREATE TABLE publication (
    id                  integer PRIMARY KEY REFERENCES media(id) ON DELETE CASCADE,
    publisher_id        integer REFERENCES organization(id) ON DELETE SET NULL,
    imprint_id          integer REFERENCES organization(id) ON DELETE SET NULL,
    publication_model   publication_model NOT NULL DEFAULT 'traditional',
    distribution        distribution_format NOT NULL DEFAULT 'print_and_digital',
    binding             binding_format,
    demographic         audience_demographic,
    published_on        date,
    date_precision      date_precision NOT NULL DEFAULT 'day',
    edition_label       text,
    edition_number      smallint,
    is_first_edition    boolean NOT NULL DEFAULT false,
    is_official         boolean NOT NULL DEFAULT true,
    is_derivative       boolean NOT NULL DEFAULT false,  -- fan works, doujin of a parent
    is_limited          boolean NOT NULL DEFAULT false,
    print_run           integer,
    isbn_10             text,
    isbn_13             text,
    issn                text,
    asin                text,
    doi                 text,
    barcode             text,
    catalog_number      text,
    trim_size           text,          -- 'A5', '6x9in', 'B6'
    paper_stock         text,
    cover_price         numeric(10,2),
    currency            char(3),
    is_out_of_print     boolean NOT NULL DEFAULT false,
    -- self-published / circle-published works: the "doujin" case.
    -- Circle name goes here, the genre reading goes in tags.
    publishing_circle   text,
    released_at_event   text           -- 'Comiket 103', 'Kickstarter'
);

COMMENT ON COLUMN publication.publication_model IS
    'doujinshi / indie / zine are publication_model = self_published or fan_published, not separate tables';

-- 5.12 serialized — released in installments
CREATE TABLE serialized (
    id                     integer PRIMARY KEY REFERENCES media(id) ON DELETE CASCADE,
    installment_unit       installment_unit NOT NULL,
    total_installments     integer,
    released_installments  integer NOT NULL DEFAULT 0,
    is_ongoing             boolean NOT NULL DEFAULT false,
    is_on_hiatus           boolean NOT NULL DEFAULT false,
    schedule               release_schedule NOT NULL DEFAULT 'irregular',
    release_weekday        smallint CHECK (release_weekday BETWEEN 0 AND 6),
    release_time           time,
    release_timezone       text,
    season_number          smallint,
    part_number            smallint,
    first_installment_on   date,
    latest_installment_on  date,
    next_installment_on    date,
    average_installment_length integer,   -- seconds or pages, unit-dependent
    serialized_in_id       integer REFERENCES media(id) ON DELETE SET NULL,  -- host periodical
    CONSTRAINT serialized_count_sane CHECK (total_installments IS NULL OR released_installments <= total_installments)
);

-- 5.13 software — executes on hardware
CREATE TABLE software (
    id                  integer PRIMARY KEY REFERENCES media(id) ON DELETE CASCADE,
    engine              text,
    programming_languages text[] NOT NULL DEFAULT '{}',
    license             software_license NOT NULL DEFAULT 'proprietary',
    is_open_source      boolean NOT NULL DEFAULT false,
    source_code_url     text,
    latest_version      text,
    build_size_bytes    bigint,
    requires_internet   boolean NOT NULL DEFAULT false,
    has_drm             boolean NOT NULL DEFAULT false,
    drm_notes           text,
    server_status       server_status NOT NULL DEFAULT 'not_applicable',
    server_shutdown_on  date,
    minimum_requirements jsonb,
    recommended_requirements jsonb,
    supports_mods       boolean NOT NULL DEFAULT false,
    supports_cross_platform boolean NOT NULL DEFAULT false
);

-- =============================================================================
-- 6. BASIC TIER
--    PK is an FK into every facet the type requires.
-- =============================================================================

CREATE TABLE movie (
    id                    integer PRIMARY KEY REFERENCES audiovisual(id) ON DELETE CASCADE,
    theatrical_release_on date,
    is_short              boolean NOT NULL DEFAULT false,
    is_feature            boolean NOT NULL DEFAULT true,
    is_direct_to_video    boolean NOT NULL DEFAULT false,
    is_documentary        boolean NOT NULL DEFAULT false,
    festival_premiere     text,
    budget                numeric(14,2),
    box_office_gross      numeric(14,2),
    box_office_currency   char(3),
    distributor_id        integer REFERENCES organization(id) ON DELETE SET NULL,
    film_series_position  smallint,
    negative_format       text,
    printed_format        text,
    CONSTRAINT movie_narrative_fk FOREIGN KEY (id) REFERENCES narrative(id) ON DELETE CASCADE
);

CREATE TABLE show (
    id                  integer PRIMARY KEY REFERENCES audiovisual(id) ON DELETE CASCADE,
    show_type           show_type NOT NULL DEFAULT 'tv',
    network_id          integer REFERENCES organization(id) ON DELETE SET NULL,
    season_count        smallint,
    episode_runtime_seconds integer,
    original_run_start  date,
    original_run_end    date,
    is_animated         boolean NOT NULL DEFAULT false,
    was_syndicated      boolean NOT NULL DEFAULT false,
    time_slot           text,
    CONSTRAINT show_serialized_fk FOREIGN KEY (id) REFERENCES serialized(id) ON DELETE CASCADE
);

CREATE TABLE book (
    id                     integer PRIMARY KEY REFERENCES print(id) ON DELETE CASCADE,
    book_type              book_type NOT NULL DEFAULT 'novel',
    volume_number          numeric(6,2),
    series_position        numeric(6,2),
    original_published_on  date,
    is_abridged            boolean NOT NULL DEFAULT false,
    is_annotated           boolean NOT NULL DEFAULT false,
    dewey_decimal          text,
    library_of_congress    text,
    subject_headings       text[] NOT NULL DEFAULT '{}',
    CONSTRAINT book_publication_fk FOREIGN KEY (id) REFERENCES publication(id) ON DELETE CASCADE
);

CREATE TABLE comic (
    id               integer PRIMARY KEY REFERENCES sequential_art(id) ON DELETE CASCADE,
    comic_format     comic_format NOT NULL DEFAULT 'collected_volume',
    issue_number     numeric(6,2),
    volume_number    numeric(6,2),
    chapter_range    int4range,
    collects_issues  text,
    is_anthology_piece boolean NOT NULL DEFAULT false,
    magazine_id      integer REFERENCES media(id) ON DELETE SET NULL,
    CONSTRAINT comic_publication_fk FOREIGN KEY (id) REFERENCES publication(id) ON DELETE CASCADE
);

COMMENT ON TABLE comic IS
    'manga / manhwa / manhua / bande dessinee are comic + media.country_of_origin. '
    'webtoon is comic + sequential_art.panel_layout = vertical_scroll. '
    'doujinshi is comic + publication.publication_model = self_published.';

CREATE TABLE game (
    id                  integer PRIMARY KEY REFERENCES interactive(id) ON DELETE CASCADE,
    release_model       game_release_model NOT NULL DEFAULT 'digital',
    monetization        monetization_model NOT NULL DEFAULT 'premium',
    base_game_id        integer REFERENCES media(id) ON DELETE SET NULL,  -- DLC / expansion parent
    is_expansion        boolean NOT NULL DEFAULT false,
    is_remaster         boolean NOT NULL DEFAULT false,
    is_port             boolean NOT NULL DEFAULT false,
    early_access_start  date,
    launch_price        numeric(10,2),
    currency            char(3),
    has_achievements    boolean NOT NULL DEFAULT false,
    has_level_editor    boolean NOT NULL DEFAULT false,
    peak_concurrent_players integer,
    CONSTRAINT game_software_fk FOREIGN KEY (id) REFERENCES software(id) ON DELETE CASCADE
);

CREATE TABLE album (
    id               integer PRIMARY KEY REFERENCES musical(id) ON DELETE CASCADE,
    album_type       album_type NOT NULL DEFAULT 'studio',
    label_id         integer REFERENCES organization(id) ON DELETE SET NULL,
    track_count      smallint,
    disc_count       smallint NOT NULL DEFAULT 1,
    total_duration_seconds integer,
    is_concept_album boolean NOT NULL DEFAULT false,
    is_compilation   boolean NOT NULL DEFAULT false,
    recorded_from    date,
    recorded_until   date,
    mastering_notes  text,
    CONSTRAINT album_audio_fk FOREIGN KEY (id) REFERENCES audio(id) ON DELETE CASCADE
);

CREATE TABLE track (
    id             integer PRIMARY KEY REFERENCES musical(id) ON DELETE CASCADE,
    album_id       integer REFERENCES album(id) ON DELETE SET NULL,
    track_number   smallint,
    disc_number    smallint NOT NULL DEFAULT 1,
    is_bonus_track boolean NOT NULL DEFAULT false,
    is_hidden_track boolean NOT NULL DEFAULT false,
    is_single      boolean NOT NULL DEFAULT false,
    original_track_id integer REFERENCES media(id) ON DELETE SET NULL,  -- for covers/remixes
    CONSTRAINT track_audio_fk FOREIGN KEY (id) REFERENCES audio(id) ON DELETE CASCADE
);

CREATE TABLE podcast (
    id            integer PRIMARY KEY REFERENCES audio(id) ON DELETE CASCADE,
    podcast_type  podcast_type NOT NULL DEFAULT 'interview',
    feed_url      text,
    network_id    integer REFERENCES organization(id) ON DELETE SET NULL,
    is_video      boolean NOT NULL DEFAULT false,
    is_explicit   boolean NOT NULL DEFAULT false,
    CONSTRAINT podcast_serialized_fk FOREIGN KEY (id) REFERENCES serialized(id) ON DELETE CASCADE
);

CREATE TABLE stage_production (
    id                integer PRIMARY KEY REFERENCES performance(id) ON DELETE CASCADE,
    production_type   production_type NOT NULL,
    company_id        integer REFERENCES organization(id) ON DELETE SET NULL,
    is_revival        boolean NOT NULL DEFAULT false,
    is_touring        boolean NOT NULL DEFAULT false,
    original_production_id integer REFERENCES media(id) ON DELETE SET NULL,
    libretto_language_id integer REFERENCES language(id) ON DELETE SET NULL,
    has_live_orchestra boolean NOT NULL DEFAULT false
);

CREATE TABLE artwork (
    id             integer PRIMARY KEY REFERENCES still_image(id) ON DELETE CASCADE,
    artwork_type   artwork_type NOT NULL,
    created_from   date,
    created_until  date,
    is_commissioned boolean NOT NULL DEFAULT false,
    commissioned_by text,
    depicts_media_id integer REFERENCES media(id) ON DELETE SET NULL,   -- cover art, fan art
    signature_notes text
);

CREATE TABLE periodical (
    id              integer PRIMARY KEY REFERENCES publication(id) ON DELETE CASCADE,
    periodical_type periodical_type NOT NULL,
    frequency       release_schedule NOT NULL DEFAULT 'monthly',
    circulation     integer,
    is_peer_reviewed boolean NOT NULL DEFAULT false,
    editor_in_chief text,
    founded_on      date,
    ceased_on       date,
    CONSTRAINT periodical_serialized_fk FOREIGN KEY (id) REFERENCES serialized(id) ON DELETE CASCADE
);

-- =============================================================================
-- 7. COMPOSITE TIER
--    Only for works that cannot be reduced to one basic type.
-- =============================================================================

-- visual novel: interactive + text + presentation. This is the canonical
-- multi-form case, hence a table rather than a derived predicate.
CREATE TABLE visual_novel (
    id                    integer PRIMARY KEY REFERENCES game(id) ON DELETE CASCADE,
    route_count           smallint,
    common_route_words    integer,
    total_words           integer,
    is_kinetic            boolean NOT NULL DEFAULT false,   -- no choices at all
    is_sound_novel        boolean NOT NULL DEFAULT false,
    voiced_extent         voiced_extent NOT NULL DEFAULT 'none',
    voice_language_id     integer REFERENCES language(id) ON DELETE SET NULL,
    cg_count              integer,
    sprite_count          integer,
    background_count      integer,
    bgm_track_count       integer,
    has_sprite_animation  boolean NOT NULL DEFAULT false,
    has_affection_system  boolean NOT NULL DEFAULT false,
    has_flowchart         boolean NOT NULL DEFAULT false,
    has_skip_read_text    boolean NOT NULL DEFAULT true,
    CONSTRAINT vn_print_fk       FOREIGN KEY (id) REFERENCES print(id) ON DELETE CASCADE,
    CONSTRAINT vn_audiovisual_fk FOREIGN KEY (id) REFERENCES audiovisual(id) ON DELETE CASCADE,
    CONSTRAINT vn_narrative_fk   FOREIGN KEY (id) REFERENCES narrative(id) ON DELETE CASCADE
);

-- audiobook: an audio edition of a text work
CREATE TABLE audiobook (
    id                integer PRIMARY KEY REFERENCES audio(id) ON DELETE CASCADE,
    source_book_id    integer REFERENCES book(id) ON DELETE SET NULL,
    narration_style   narration_style NOT NULL DEFAULT 'single_narrator',
    is_abridged       boolean NOT NULL DEFAULT false,
    is_dramatized     boolean NOT NULL DEFAULT false,
    has_sound_design  boolean NOT NULL DEFAULT false,
    chapter_count     smallint,
    CONSTRAINT audiobook_publication_fk FOREIGN KEY (id) REFERENCES publication(id) ON DELETE CASCADE,
    CONSTRAINT audiobook_narrative_fk   FOREIGN KEY (id) REFERENCES narrative(id) ON DELETE CASCADE
);

-- music video: a moving-image work whose subject is a musical work
CREATE TABLE music_video (
    id             integer PRIMARY KEY REFERENCES audiovisual(id) ON DELETE CASCADE,
    track_id       integer REFERENCES track(id) ON DELETE SET NULL,
    video_type     music_video_type NOT NULL DEFAULT 'official',
    is_censored_cut boolean NOT NULL DEFAULT false,
    CONSTRAINT music_video_musical_fk FOREIGN KEY (id) REFERENCES musical(id) ON DELETE CASCADE
);

-- tabletop game: interactive but not software; a published physical object
CREATE TABLE tabletop_game (
    id                 integer PRIMARY KEY REFERENCES interactive(id) ON DELETE CASCADE,
    tabletop_type      tabletop_type NOT NULL,
    minimum_age        smallint,
    complexity_weight  numeric(4,2) CHECK (complexity_weight BETWEEN 0 AND 5),
    luck_factor        smallint CHECK (luck_factor BETWEEN 0 AND 10),
    component_count    integer,
    component_manifest text,
    rulebook_pages     smallint,
    has_expansions     boolean NOT NULL DEFAULT false,
    base_game_id       integer REFERENCES media(id) ON DELETE SET NULL,
    is_legacy_campaign boolean NOT NULL DEFAULT false,
    CONSTRAINT tabletop_publication_fk FOREIGN KEY (id) REFERENCES publication(id) ON DELETE CASCADE
);

-- =============================================================================
-- 8. INSTALLMENTS
--    Episodes / chapters / issues that do not warrant their own media row.
--    Promote to a full media row (and link back) only when they need their own
--    credits, ratings and facets.
-- =============================================================================

CREATE TABLE installment (
    id                integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    serialized_id     integer NOT NULL REFERENCES serialized(id) ON DELETE CASCADE,
    promoted_media_id integer REFERENCES media(id) ON DELETE SET NULL,
    number            numeric(8,2) NOT NULL,
    absolute_number   integer,
    season_number     smallint,
    title             text,
    native_title      text,
    synopsis          text,
    released_on       date,
    duration_seconds  integer,
    page_count        integer,
    word_count        integer,
    is_special        boolean NOT NULL DEFAULT false,
    is_recap          boolean NOT NULL DEFAULT false,
    is_filler         boolean NOT NULL DEFAULT false,
    mean_score        numeric(5,2) CHECK (mean_score BETWEEN 0 AND 100),
    UNIQUE (serialized_id, number, season_number)
);

-- =============================================================================
-- 9. PEOPLE, ORGS, CHARACTERS ATTACHED TO WORKS
-- =============================================================================

CREATE TABLE media_credit (
    id              integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    media_id        integer NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    person_id       integer NOT NULL REFERENCES person(id) ON DELETE CASCADE,
    credit_role_id  integer NOT NULL REFERENCES credit_role(id) ON DELETE RESTRICT,
    credited_as     text,
    installment_id  integer REFERENCES installment(id) ON DELETE CASCADE,
    is_uncredited   boolean NOT NULL DEFAULT false,
    is_lead         boolean NOT NULL DEFAULT false,
    sort_order      smallint NOT NULL DEFAULT 0,
    notes           text,
    UNIQUE (media_id, person_id, credit_role_id, installment_id)
);

CREATE TABLE media_organization (
    id              integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    media_id        integer NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    organization_id integer NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    org_role        org_type NOT NULL,
    country_id      integer REFERENCES country(id) ON DELETE SET NULL,  -- regional licensor
    is_primary      boolean NOT NULL DEFAULT false,
    notes           text,
    UNIQUE (media_id, organization_id, org_role, country_id)
);

CREATE TABLE media_character (
    id            integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    media_id      integer NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    character_id  integer NOT NULL REFERENCES fictional_character(id) ON DELETE CASCADE,
    billing       character_billing NOT NULL DEFAULT 'supporting',
    role_notes    text,
    sort_order    smallint NOT NULL DEFAULT 0,
    UNIQUE (media_id, character_id)
);

-- One character can be voiced/played by different people per language and per work.
CREATE TABLE character_portrayal (
    id                  integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    media_character_id  integer NOT NULL REFERENCES media_character(id) ON DELETE CASCADE,
    person_id           integer NOT NULL REFERENCES person(id) ON DELETE CASCADE,
    language_id         integer REFERENCES language(id) ON DELETE SET NULL,
    is_voice_only       boolean NOT NULL DEFAULT true,
    is_motion_capture   boolean NOT NULL DEFAULT false,
    is_understudy       boolean NOT NULL DEFAULT false,
    credited_as         text,
    UNIQUE (media_character_id, person_id, language_id)
);

-- =============================================================================
-- 10. RELATIONS, COLLECTIONS, RELEASES, TAGS, RATINGS, AWARDS
-- =============================================================================

CREATE TABLE media_relation (
    id            integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    source_id     integer NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    target_id     integer NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    relation      relation_type NOT NULL,
    notes         text,
    UNIQUE (source_id, target_id, relation),
    CONSTRAINT media_relation_no_self CHECK (source_id <> target_id)
);

CREATE TABLE collection (
    id              integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    slug            citext UNIQUE,
    name            text NOT NULL,
    native_name     text,
    collection_type collection_type NOT NULL DEFAULT 'series',
    parent_id       integer REFERENCES collection(id) ON DELETE SET NULL,
    description     text,
    started_on      date,
    ended_on        date,
    created_at      timestamptz NOT NULL DEFAULT now(),
    updated_at      timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE collection_entry (
    collection_id integer NOT NULL REFERENCES collection(id) ON DELETE CASCADE,
    media_id      integer NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    position      numeric(8,2),
    is_canon      boolean NOT NULL DEFAULT true,
    is_core_entry boolean NOT NULL DEFAULT true,
    notes         text,
    PRIMARY KEY (collection_id, media_id)
);

-- A regional / format-specific issuing of an existing work.
CREATE TABLE media_release (
    id              integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    media_id        integer NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    country_id      integer REFERENCES country(id) ON DELETE SET NULL,
    platform_id     integer REFERENCES platform(id) ON DELETE SET NULL,
    released_on     date,
    date_precision  date_precision NOT NULL DEFAULT 'day',
    distribution    distribution_format,
    edition_label   text,
    language_id     integer REFERENCES language(id) ON DELETE SET NULL,
    is_dubbed       boolean NOT NULL DEFAULT false,
    is_subtitled    boolean NOT NULL DEFAULT false,
    is_censored     boolean NOT NULL DEFAULT false,
    censorship_notes text,
    distributor_id  integer REFERENCES organization(id) ON DELETE SET NULL,
    price           numeric(10,2),
    currency        char(3),
    catalog_number  text,
    notes           text
);

CREATE TABLE media_platform (
    media_id    integer NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    platform_id integer NOT NULL REFERENCES platform(id) ON DELETE CASCADE,
    is_original boolean NOT NULL DEFAULT false,
    added_on    date,
    removed_on  date,
    PRIMARY KEY (media_id, platform_id)
);

CREATE TABLE media_tag (
    media_id    integer NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    tag_id      integer NOT NULL REFERENCES tag(id) ON DELETE CASCADE,
    relevance   smallint NOT NULL DEFAULT 100 CHECK (relevance BETWEEN 0 AND 100),
    is_spoiler  boolean NOT NULL DEFAULT false,
    vote_count  integer NOT NULL DEFAULT 0,
    added_at    timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (media_id, tag_id)
);

CREATE TABLE media_content_rating (
    media_id          integer NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    content_rating_id integer NOT NULL REFERENCES content_rating(id) ON DELETE CASCADE,
    descriptors       text[] NOT NULL DEFAULT '{}',  -- 'Blood', 'Strong Language'
    rated_on          date,
    PRIMARY KEY (media_id, content_rating_id)
);

CREATE TABLE media_language (
    media_id    integer NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    language_id integer NOT NULL REFERENCES language(id) ON DELETE CASCADE,
    is_original boolean NOT NULL DEFAULT false,
    is_audio    boolean NOT NULL DEFAULT false,
    is_subtitle boolean NOT NULL DEFAULT false,
    is_text     boolean NOT NULL DEFAULT false,
    is_interface boolean NOT NULL DEFAULT false,
    PRIMARY KEY (media_id, language_id)
);

CREATE TABLE award (
    id              integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    slug            citext NOT NULL UNIQUE,
    name            text NOT NULL,
    awarding_body   text,
    country_id      integer REFERENCES country(id) ON DELETE SET NULL,
    established_on  date,
    description     text
);

CREATE TABLE award_category (
    id         integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    award_id   integer NOT NULL REFERENCES award(id) ON DELETE CASCADE,
    name       text NOT NULL,
    is_for_person boolean NOT NULL DEFAULT false,
    UNIQUE (award_id, name)
);

CREATE TABLE award_nomination (
    id                integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    award_category_id integer NOT NULL REFERENCES award_category(id) ON DELETE CASCADE,
    year              smallint NOT NULL,
    media_id          integer REFERENCES media(id) ON DELETE CASCADE,
    person_id         integer REFERENCES person(id) ON DELETE CASCADE,
    is_winner         boolean NOT NULL DEFAULT false,
    notes             text,
    CONSTRAINT award_nomination_subject CHECK (media_id IS NOT NULL OR person_id IS NOT NULL)
);

-- =============================================================================
-- 11. DERIVED TYPE SYSTEM
--     Media "types" are predicates over table membership, not tables.
-- =============================================================================

CREATE TABLE facet (
    id           integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    slug         citext NOT NULL UNIQUE,
    table_name   text NOT NULL UNIQUE,
    tier         facet_tier NOT NULL,
    bit_position smallint NOT NULL UNIQUE CHECK (bit_position BETWEEN 0 AND 62),
    description  text
);

COMMENT ON TABLE facet IS
    'Registry mapping every membership table to a bit in the kind mask. '
    'Keep bit_position stable forever; append new facets at the next free bit.';

INSERT INTO facet (slug, table_name, tier, bit_position, description) VALUES
    ('narrative',       'narrative',       'property',  0,  'tells a story'),
    ('print',           'print',           'property',  1,  'text on pages'),
    ('sequential_art',  'sequential_art',  'property',  2,  'panel-based visual storytelling'),
    ('audiovisual',     'audiovisual',     'property',  3,  'moving image'),
    ('animation',       'animation',       'property',  4,  'non-photographic imagery'),
    ('audio',           'audio',           'property',  5,  'sound as primary carrier'),
    ('musical',         'musical',         'property',  6,  'musical structure'),
    ('interactive',     'interactive',     'property',  7,  'input changes state'),
    ('performance',     'performance',     'property',  8,  'realized live'),
    ('still_image',     'still_image',     'property',  9,  'fixed visual artifact'),
    ('publication',     'publication',     'property', 10,  'issued as an edition'),
    ('serialized',      'serialized',      'property', 11,  'released in installments'),
    ('software',        'software',        'property', 12,  'executes on hardware'),
    ('movie',           'movie',           'basic',    16,  NULL),
    ('show',            'show',            'basic',    17,  NULL),
    ('book',            'book',            'basic',    18,  NULL),
    ('comic',           'comic',           'basic',    19,  NULL),
    ('game',            'game',            'basic',    20,  NULL),
    ('album',           'album',           'basic',    21,  NULL),
    ('track',           'track',           'basic',    22,  NULL),
    ('podcast',         'podcast',         'basic',    23,  NULL),
    ('stage_production','stage_production','basic',    24,  NULL),
    ('artwork',         'artwork',         'basic',    25,  NULL),
    ('periodical',      'periodical',      'basic',    26,  NULL),
    ('visual_novel',    'visual_novel',    'composite',32,  NULL),
    ('audiobook',       'audiobook',       'composite',33,  NULL),
    ('music_video',     'music_video',     'composite',34,  NULL),
    ('tabletop_game',   'tabletop_game',   'composite',35,  NULL);

-- Build a mask from facet slugs: facet_mask('book','game') -> bigint
CREATE FUNCTION facet_mask(VARIADIC slugs text[]) RETURNS bigint AS $$
    SELECT COALESCE(bit_or((1::bigint << f.bit_position)), 0::bigint)
    FROM facet f
    WHERE f.slug = ANY (slugs);
$$ LANGUAGE sql STABLE;

-- Membership view. One row per media, boolean per facet plus the packed mask.
CREATE VIEW media_facets AS
SELECT
    m.id AS media_id,
    (nr.id  IS NOT NULL) AS is_narrative,
    (pr.id  IS NOT NULL) AS is_print,
    (sa.id  IS NOT NULL) AS is_sequential_art,
    (av.id  IS NOT NULL) AS is_audiovisual,
    (an.id  IS NOT NULL) AS is_animation,
    (au.id  IS NOT NULL) AS is_audio,
    (mu.id  IS NOT NULL) AS is_musical,
    (it.id  IS NOT NULL) AS is_interactive,
    (pf.id  IS NOT NULL) AS is_performance,
    (si.id  IS NOT NULL) AS is_still_image,
    (pb.id  IS NOT NULL) AS is_publication,
    (sz.id  IS NOT NULL) AS is_serialized,
    (sw.id  IS NOT NULL) AS is_software,
    (mv.id  IS NOT NULL) AS is_movie,
    (sh.id  IS NOT NULL) AS is_show,
    (bk.id  IS NOT NULL) AS is_book,
    (cm.id  IS NOT NULL) AS is_comic,
    (gm.id  IS NOT NULL) AS is_game,
    (al.id  IS NOT NULL) AS is_album,
    (tr.id  IS NOT NULL) AS is_track,
    (pc.id  IS NOT NULL) AS is_podcast,
    (sp.id  IS NOT NULL) AS is_stage_production,
    (aw.id  IS NOT NULL) AS is_artwork,
    (pe.id  IS NOT NULL) AS is_periodical,
    (vn.id  IS NOT NULL) AS is_visual_novel,
    (ab.id  IS NOT NULL) AS is_audiobook,
    (mvd.id IS NOT NULL) AS is_music_video,
    (tg.id  IS NOT NULL) AS is_tabletop_game,
    (
        (CASE WHEN nr.id  IS NOT NULL THEN 1::bigint <<  0 ELSE 0 END) |
        (CASE WHEN pr.id  IS NOT NULL THEN 1::bigint <<  1 ELSE 0 END) |
        (CASE WHEN sa.id  IS NOT NULL THEN 1::bigint <<  2 ELSE 0 END) |
        (CASE WHEN av.id  IS NOT NULL THEN 1::bigint <<  3 ELSE 0 END) |
        (CASE WHEN an.id  IS NOT NULL THEN 1::bigint <<  4 ELSE 0 END) |
        (CASE WHEN au.id  IS NOT NULL THEN 1::bigint <<  5 ELSE 0 END) |
        (CASE WHEN mu.id  IS NOT NULL THEN 1::bigint <<  6 ELSE 0 END) |
        (CASE WHEN it.id  IS NOT NULL THEN 1::bigint <<  7 ELSE 0 END) |
        (CASE WHEN pf.id  IS NOT NULL THEN 1::bigint <<  8 ELSE 0 END) |
        (CASE WHEN si.id  IS NOT NULL THEN 1::bigint <<  9 ELSE 0 END) |
        (CASE WHEN pb.id  IS NOT NULL THEN 1::bigint << 10 ELSE 0 END) |
        (CASE WHEN sz.id  IS NOT NULL THEN 1::bigint << 11 ELSE 0 END) |
        (CASE WHEN sw.id  IS NOT NULL THEN 1::bigint << 12 ELSE 0 END) |
        (CASE WHEN mv.id  IS NOT NULL THEN 1::bigint << 16 ELSE 0 END) |
        (CASE WHEN sh.id  IS NOT NULL THEN 1::bigint << 17 ELSE 0 END) |
        (CASE WHEN bk.id  IS NOT NULL THEN 1::bigint << 18 ELSE 0 END) |
        (CASE WHEN cm.id  IS NOT NULL THEN 1::bigint << 19 ELSE 0 END) |
        (CASE WHEN gm.id  IS NOT NULL THEN 1::bigint << 20 ELSE 0 END) |
        (CASE WHEN al.id  IS NOT NULL THEN 1::bigint << 21 ELSE 0 END) |
        (CASE WHEN tr.id  IS NOT NULL THEN 1::bigint << 22 ELSE 0 END) |
        (CASE WHEN pc.id  IS NOT NULL THEN 1::bigint << 23 ELSE 0 END) |
        (CASE WHEN sp.id  IS NOT NULL THEN 1::bigint << 24 ELSE 0 END) |
        (CASE WHEN aw.id  IS NOT NULL THEN 1::bigint << 25 ELSE 0 END) |
        (CASE WHEN pe.id  IS NOT NULL THEN 1::bigint << 26 ELSE 0 END) |
        (CASE WHEN vn.id  IS NOT NULL THEN 1::bigint << 32 ELSE 0 END) |
        (CASE WHEN ab.id  IS NOT NULL THEN 1::bigint << 33 ELSE 0 END) |
        (CASE WHEN mvd.id IS NOT NULL THEN 1::bigint << 34 ELSE 0 END) |
        (CASE WHEN tg.id  IS NOT NULL THEN 1::bigint << 35 ELSE 0 END)
    ) AS kind_mask
FROM media m
LEFT JOIN narrative        nr  ON nr.id  = m.id
LEFT JOIN print            pr  ON pr.id  = m.id
LEFT JOIN sequential_art   sa  ON sa.id  = m.id
LEFT JOIN audiovisual      av  ON av.id  = m.id
LEFT JOIN animation        an  ON an.id  = m.id
LEFT JOIN audio            au  ON au.id  = m.id
LEFT JOIN musical          mu  ON mu.id  = m.id
LEFT JOIN interactive      it  ON it.id  = m.id
LEFT JOIN performance      pf  ON pf.id  = m.id
LEFT JOIN still_image      si  ON si.id  = m.id
LEFT JOIN publication      pb  ON pb.id  = m.id
LEFT JOIN serialized       sz  ON sz.id  = m.id
LEFT JOIN software         sw  ON sw.id  = m.id
LEFT JOIN movie            mv  ON mv.id  = m.id
LEFT JOIN show             sh  ON sh.id  = m.id
LEFT JOIN book             bk  ON bk.id  = m.id
LEFT JOIN comic            cm  ON cm.id  = m.id
LEFT JOIN game             gm  ON gm.id  = m.id
LEFT JOIN album            al  ON al.id  = m.id
LEFT JOIN track            tr  ON tr.id  = m.id
LEFT JOIN podcast          pc  ON pc.id  = m.id
LEFT JOIN stage_production sp  ON sp.id  = m.id
LEFT JOIN artwork          aw  ON aw.id  = m.id
LEFT JOIN periodical       pe  ON pe.id  = m.id
LEFT JOIN visual_novel     vn  ON vn.id  = m.id
LEFT JOIN audiobook        ab  ON ab.id  = m.id
LEFT JOIN music_video      mvd ON mvd.id = m.id
LEFT JOIN tabletop_game    tg  ON tg.id  = m.id;

-- Named types, defined as expressions rather than tables.
--   expression      canonical boolean tree, evaluable in Rust
--   require_mask    all of these bits must be set   (fast path)
--   exclude_mask    none of these bits may be set   (fast path)
--   extra_predicate SQL fragment over `media` / facet tables for anything the
--                   mask cannot express (country of origin, a tag, a column value)
CREATE TABLE media_type (
    id              integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    slug            citext NOT NULL UNIQUE,
    name            text NOT NULL,
    native_name     text,
    description     text,
    expression      jsonb NOT NULL,
    require_mask    bigint NOT NULL DEFAULT 0,
    exclude_mask    bigint NOT NULL DEFAULT 0,
    extra_predicate text,
    parent_type_id  integer REFERENCES media_type(id) ON DELETE SET NULL,
    is_builtin      boolean NOT NULL DEFAULT true,
    is_user_defined boolean NOT NULL DEFAULT false,
    created_by      integer,          -- FK to a future user table
    sort_order      smallint NOT NULL DEFAULT 0,
    created_at      timestamptz NOT NULL DEFAULT now(),
    updated_at      timestamptz NOT NULL DEFAULT now()
);

COMMENT ON COLUMN media_type.extra_predicate IS
    'SQL fragment appended with AND. Assumes these aliases: m = media, '
    'c = country (joined on m.country_of_origin_id), f = media_facets, and one '
    'alias per membership table matching media_facets (pr, sa, pb, sz, bk, cm, '
    'al, si, tg, ...). Compiled, never executed as-is.';

INSERT INTO media_type (slug, name, description, expression, require_mask, exclude_mask, extra_predicate) VALUES
    ('film', 'Film', 'Any movie, live action or animated.',
     '{"op":"facet","name":"movie"}', facet_mask('movie'), 0, NULL),

    ('anime', 'Anime', 'Japanese animated film or series.',
     '{"op":"and","args":[{"op":"facet","name":"animation"},{"op":"or","args":[{"op":"facet","name":"movie"},{"op":"facet","name":"show"}]},{"op":"origin","country":"JP"}]}',
     facet_mask('animation'), 0,
     'c.iso_3166_1 = ''JP'' AND (f.is_movie OR f.is_show)'),

    ('cartoon', 'Cartoon', 'Animated series or film of non-Japanese origin.',
     '{"op":"and","args":[{"op":"facet","name":"animation"},{"op":"or","args":[{"op":"facet","name":"movie"},{"op":"facet","name":"show"}]},{"op":"not","args":[{"op":"origin","country":"JP"}]}]}',
     facet_mask('animation'), 0,
     'c.iso_3166_1 IS DISTINCT FROM ''JP'' AND (f.is_movie OR f.is_show)'),

    ('live_action_series', 'Live-Action Series', NULL,
     '{"op":"and","args":[{"op":"facet","name":"show"},{"op":"not","args":[{"op":"facet","name":"animation"}]}]}',
     facet_mask('show'), facet_mask('animation'), NULL),

    ('manga', 'Manga', 'Comic of Japanese origin.',
     '{"op":"and","args":[{"op":"facet","name":"comic"},{"op":"origin","country":"JP"}]}',
     facet_mask('comic'), 0, 'c.iso_3166_1 = ''JP'''),

    ('manhwa', 'Manhwa', 'Comic of Korean origin.',
     '{"op":"and","args":[{"op":"facet","name":"comic"},{"op":"origin","country":"KR"}]}',
     facet_mask('comic'), 0, 'c.iso_3166_1 = ''KR'''),

    ('manhua', 'Manhua', 'Comic of Chinese origin.',
     '{"op":"and","args":[{"op":"facet","name":"comic"},{"op":"origin","country":"CN"}]}',
     facet_mask('comic'), 0, 'c.iso_3166_1 IN (''CN'',''TW'',''HK'')'),

    ('webtoon', 'Webtoon', 'Vertical-scroll digital comic.',
     '{"op":"and","args":[{"op":"facet","name":"comic"},{"op":"column","table":"sequential_art","field":"panel_layout","eq":"vertical_scroll"}]}',
     facet_mask('comic'), 0, 'sa.panel_layout = ''vertical_scroll'''),

    ('doujinshi', 'Doujinshi', 'Self-published comic or book, derivative or original.',
     '{"op":"and","args":[{"op":"facet","name":"publication"},{"op":"column","table":"publication","field":"publication_model","in":["self_published","fan_published"]}]}',
     facet_mask('publication'), 0,
     'pb.publication_model IN (''self_published'',''fan_published'')'),

    ('light_novel', 'Light Novel', 'Illustrated Japanese prose novel.',
     '{"op":"and","args":[{"op":"facet","name":"book"},{"op":"column","table":"print","field":"is_illustrated","eq":true},{"op":"origin","country":"JP"}]}',
     facet_mask('book'), 0, 'pr.is_illustrated AND c.iso_3166_1 = ''JP'''),

    ('web_novel', 'Web Novel', 'Prose serialized online.',
     '{"op":"and","args":[{"op":"facet","name":"book"},{"op":"column","table":"publication","field":"publication_model","eq":"web_serial"}]}',
     facet_mask('book'), 0, 'pb.publication_model = ''web_serial'''),

    ('graphic_novel', 'Graphic Novel', NULL,
     '{"op":"and","args":[{"op":"facet","name":"comic"},{"op":"column","table":"comic","field":"comic_format","eq":"graphic_novel"}]}',
     facet_mask('comic'), 0, 'cm.comic_format = ''graphic_novel'''),

    ('visual_novel', 'Visual Novel', 'Text-driven interactive work with presentation layer.',
     '{"op":"facet","name":"visual_novel"}', facet_mask('visual_novel'), 0, NULL),

    ('video_game', 'Video Game', 'Interactive software, excluding visual novels.',
     '{"op":"and","args":[{"op":"facet","name":"game"},{"op":"not","args":[{"op":"facet","name":"visual_novel"}]}]}',
     facet_mask('game'), facet_mask('visual_novel'), NULL),

    ('tabletop_rpg', 'Tabletop RPG', NULL,
     '{"op":"and","args":[{"op":"facet","name":"tabletop_game"},{"op":"column","table":"tabletop_game","field":"tabletop_type","eq":"ttrpg"}]}',
     facet_mask('tabletop_game'), 0, 'tg.tabletop_type = ''ttrpg'''),

    ('board_game', 'Board Game', NULL,
     '{"op":"and","args":[{"op":"facet","name":"tabletop_game"},{"op":"column","table":"tabletop_game","field":"tabletop_type","in":["board","party","dexterity","legacy"]}]}',
     facet_mask('tabletop_game'), 0,
     'tg.tabletop_type IN (''board'',''party'',''dexterity'',''legacy'')'),

    ('audiobook', 'Audiobook', NULL,
     '{"op":"facet","name":"audiobook"}', facet_mask('audiobook'), 0, NULL),

    ('audio_drama', 'Audio Drama', 'Narrative audio that is not an adaptation of a book.',
     '{"op":"and","args":[{"op":"facet","name":"audio"},{"op":"facet","name":"narrative"},{"op":"not","args":[{"op":"facet","name":"musical"}]},{"op":"not","args":[{"op":"facet","name":"audiobook"}]}]}',
     facet_mask('audio','narrative'), facet_mask('musical','audiobook'), NULL),

    ('podcast', 'Podcast', NULL,
     '{"op":"facet","name":"podcast"}', facet_mask('podcast'), 0, NULL),

    ('soundtrack', 'Soundtrack', NULL,
     '{"op":"and","args":[{"op":"facet","name":"album"},{"op":"column","table":"album","field":"album_type","eq":"soundtrack"}]}',
     facet_mask('album'), 0, 'al.album_type = ''soundtrack'''),

    ('single', 'Single', NULL,
     '{"op":"and","args":[{"op":"facet","name":"album"},{"op":"column","table":"album","field":"album_type","in":["single","ep"]}]}',
     facet_mask('album'), 0, 'al.album_type IN (''single'',''ep'')'),

    ('music_video', 'Music Video', NULL,
     '{"op":"facet","name":"music_video"}', facet_mask('music_video'), 0, NULL),

    ('art_book', 'Art Book', NULL,
     '{"op":"and","args":[{"op":"facet","name":"book"},{"op":"column","table":"book","field":"book_type","in":["artbook","picture_book"]}]}',
     facet_mask('book'), 0, 'bk.book_type IN (''artbook'',''picture_book'')'),

    ('photobook', 'Photobook', NULL,
     '{"op":"and","args":[{"op":"facet","name":"publication"},{"op":"facet","name":"still_image"},{"op":"column","table":"still_image","field":"medium","eq":"photograph"}]}',
     facet_mask('publication','still_image'), 0, 'si.medium = ''photograph''');

CREATE TABLE media_type_membership (
    media_id      integer NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    media_type_id integer NOT NULL REFERENCES media_type(id) ON DELETE CASCADE,
    is_primary    boolean NOT NULL DEFAULT false,
    computed_at   timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (media_id, media_type_id)
);

COMMENT ON TABLE media_type_membership IS
    'Materialized result of evaluating media_type predicates. Refreshed by the '
    'application on write; never edited by hand.';

-- =============================================================================
-- 12. TRIGGERS
-- =============================================================================

CREATE TRIGGER media_set_updated_at        BEFORE UPDATE ON media        FOR EACH ROW EXECUTE FUNCTION set_updated_at();
CREATE TRIGGER person_set_updated_at       BEFORE UPDATE ON person       FOR EACH ROW EXECUTE FUNCTION set_updated_at();
CREATE TRIGGER organization_set_updated_at BEFORE UPDATE ON organization FOR EACH ROW EXECUTE FUNCTION set_updated_at();
CREATE TRIGGER character_set_updated_at    BEFORE UPDATE ON fictional_character FOR EACH ROW EXECUTE FUNCTION set_updated_at();
CREATE TRIGGER collection_set_updated_at   BEFORE UPDATE ON collection   FOR EACH ROW EXECUTE FUNCTION set_updated_at();
CREATE TRIGGER media_type_set_updated_at   BEFORE UPDATE ON media_type   FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- =============================================================================
-- 13. INDEXES
-- =============================================================================

-- root lookup
CREATE INDEX media_primary_title_trgm ON media USING gin (primary_title gin_trgm_ops);
CREATE INDEX media_romanized_title_trgm ON media USING gin (romanized_title gin_trgm_ops);
CREATE INDEX media_status_idx        ON media (status);
CREATE INDEX media_started_on_idx    ON media (started_on DESC NULLS LAST);
CREATE INDEX media_popularity_idx    ON media (popularity DESC);
CREATE INDEX media_mean_score_idx    ON media (mean_score DESC NULLS LAST);
CREATE INDEX media_country_idx       ON media (country_of_origin_id);
CREATE INDEX media_language_idx      ON media (original_language_id);
CREATE INDEX media_adult_idx         ON media (is_adult) WHERE is_adult;

CREATE INDEX person_name_trgm        ON person USING gin (primary_name gin_trgm_ops);
CREATE INDEX organization_name_trgm  ON organization USING gin (name gin_trgm_ops);
CREATE INDEX character_name_trgm     ON fictional_character USING gin (primary_name gin_trgm_ops);
CREATE INDEX character_aliases_idx   ON fictional_character USING gin (aliases);

CREATE INDEX media_title_media_idx   ON media_title (media_id);
CREATE INDEX media_title_trgm        ON media_title USING gin (title gin_trgm_ops);
CREATE UNIQUE INDEX media_title_one_primary ON media_title (media_id) WHERE is_primary;

CREATE INDEX media_image_media_idx   ON media_image (media_id, image_type);
CREATE UNIQUE INDEX media_image_one_primary ON media_image (media_id, image_type) WHERE is_primary;

-- facet columns worth filtering on
CREATE INDEX audiovisual_runtime_idx     ON audiovisual (runtime_seconds);
CREATE INDEX print_page_count_idx        ON print (page_count);
CREATE INDEX sequential_art_layout_idx   ON sequential_art (panel_layout);
CREATE INDEX animation_technique_idx     ON animation (technique);
CREATE INDEX publication_model_idx       ON publication (publication_model);
CREATE INDEX publication_publisher_idx   ON publication (publisher_id);
CREATE INDEX publication_isbn13_idx      ON publication (isbn_13) WHERE isbn_13 IS NOT NULL;
CREATE INDEX serialized_ongoing_idx      ON serialized (is_ongoing) WHERE is_ongoing;
CREATE INDEX serialized_host_idx         ON serialized (serialized_in_id);
CREATE INDEX interactive_playtime_idx    ON interactive (main_playtime_minutes);
CREATE INDEX musical_isrc_idx            ON musical (isrc) WHERE isrc IS NOT NULL;

-- basic / composite
CREATE INDEX show_network_idx        ON show (network_id);
CREATE INDEX show_type_idx           ON show (show_type);
CREATE INDEX book_type_idx           ON book (book_type);
CREATE INDEX comic_format_idx        ON comic (comic_format);
CREATE INDEX comic_magazine_idx      ON comic (magazine_id);
CREATE INDEX game_base_idx           ON game (base_game_id);
CREATE INDEX track_album_idx         ON track (album_id);
CREATE INDEX album_label_idx         ON album (label_id);
CREATE INDEX artwork_depicts_idx     ON artwork (depicts_media_id);
CREATE INDEX audiobook_source_idx    ON audiobook (source_book_id);
CREATE INDEX music_video_track_idx   ON music_video (track_id);
CREATE INDEX tabletop_type_idx       ON tabletop_game (tabletop_type);

-- joins
CREATE INDEX installment_serialized_idx ON installment (serialized_id, number);
CREATE INDEX installment_released_idx   ON installment (released_on DESC NULLS LAST);

CREATE INDEX media_credit_media_idx  ON media_credit (media_id, sort_order);
CREATE INDEX media_credit_person_idx ON media_credit (person_id);
CREATE INDEX media_credit_role_idx   ON media_credit (credit_role_id);

CREATE INDEX media_org_media_idx     ON media_organization (media_id);
CREATE INDEX media_org_org_idx       ON media_organization (organization_id, org_role);

CREATE INDEX media_character_media_idx ON media_character (media_id, billing, sort_order);
CREATE INDEX media_character_char_idx  ON media_character (character_id);
CREATE INDEX portrayal_person_idx      ON character_portrayal (person_id);

CREATE INDEX media_relation_source_idx ON media_relation (source_id, relation);
CREATE INDEX media_relation_target_idx ON media_relation (target_id, relation);

CREATE INDEX collection_entry_media_idx ON collection_entry (media_id);
CREATE INDEX collection_name_trgm       ON collection USING gin (name gin_trgm_ops);

CREATE INDEX media_release_media_idx    ON media_release (media_id, released_on);
CREATE INDEX media_release_country_idx  ON media_release (country_id);
CREATE INDEX media_release_platform_idx ON media_release (platform_id);

CREATE INDEX media_tag_tag_idx          ON media_tag (tag_id, relevance DESC);
CREATE INDEX tag_namespace_idx          ON tag (namespace);
CREATE INDEX tag_name_trgm              ON tag USING gin (name gin_trgm_ops);

CREATE INDEX media_platform_platform_idx ON media_platform (platform_id);
CREATE INDEX media_content_rating_idx    ON media_content_rating (content_rating_id);
CREATE INDEX award_nomination_media_idx  ON award_nomination (media_id);
CREATE INDEX award_nomination_person_idx ON award_nomination (person_id);
CREATE INDEX award_nomination_cat_idx    ON award_nomination (award_category_id, year);

CREATE INDEX media_external_id_lookup ON media_external_id (external_source_id, external_id);
CREATE INDEX media_type_membership_type_idx ON media_type_membership (media_type_id);
CREATE UNIQUE INDEX media_type_one_primary ON media_type_membership (media_id) WHERE is_primary;
