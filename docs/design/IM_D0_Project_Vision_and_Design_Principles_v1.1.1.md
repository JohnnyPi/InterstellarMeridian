# D0 — Project Vision and Design Principles

**Project:** Interstellar Meridian  
**Document license:** CC BY 4.0 — © 2026 JohnnyPi and Interstellar Meridian contributors, to the extent they hold rights; see [license](https://creativecommons.org/licenses/by/4.0/). Official branding and separately identified third-party material excluded.  
**Version:** 1.1.1 (greenfield migration of accepted vision; review copy)  
**Date:** 2026-11-26  
**Status:** **VISION BASELINE MIGRATION — established direction retained; editorial revision for owner review; not an implementation specification**  
**Repository:** <https://github.com/JohnnyPi/InterstellarMeridian>  
**Platform/technology direction:** Windows x64; Rust + Bevy  
**Code license:** MIT OR Apache-2.0 (project-owned code; documentation and lore CC BY 4.0).  
**Relationship to D1:** D0 establishes the authoritative product vision. The previously accepted D1 v2.0 game and engineering requirements carry forward subject to the greenfield reconciliation in D1 v2.1. D0 does not authorize implementation.

## 1. The vision

> **Interstellar Meridian is an open-ended spacefaring simulation set within a richly interconnected, evolving region of space. Players begin as independent pilots and pursue their own ambitions through exploration, commerce, combat, diplomacy, and discovery. Their decisions can change events, but the universe does not wait for them.**

Interstellar Meridian is an **independent, original, open-source spacefaring simulation**, inspired by the broader tradition of open-ended space games. It has a fresh repository and independently authored source code, architecture and content. Genre inspirations are not behavioral specifications. No inherited code, assets, data or compatibility requirements form part of the project.

The game should distinguish itself through **simulation density, causal consequences, meaningful exploration, enjoyable spaceflight, and a vibrant visual identity**. Its promise is not that it contains the most stars; it is that the systems, locations, factions, and people that exist genuinely matter to one another.

**Working design maxim:** *A universe that acts without you, reacts to you, and gives you reasons to care.*

## 2. Decisions, proposals, and open design space

This document distinguishes three levels of authority:

- **Direction established by the project owner:** goals explicitly established by the project owner. These govern the D1 revision.
- **Candidate design:** concrete proposals retained from the accepted vision and its preceding drafts for evaluation. They are **not** locked requirements.
- **Open decision:** a question requiring further design, prototyping, or deliberate approval.

### 2.1 Established direction

| Area | Established direction |
|---|---|
| Identity | Spiritual successor, not historical behavior or mod compatibility parity |
| World scope | Prefer fewer, distinctive, highly interactive systems over thousands of similar systems |
| Flight | Between full Newtonian simulation and arcade handling; local dogfighting with convincing scale and distance |
| Careers | At minimum consider mining, trading, bounty hunting, piracy, and diplomacy; continue brainstorming |
| Factions/economy | Deep interaction and consequences; avoid large numbers of shallow factions or events |
| World agency | Factions, conflicts, and other processes evolve without the player; player actions can materially alter outcomes |
| Exploration | Reward finding resource fields, gas clouds, asteroids, derelict ships/stations, and other meaningful sites |
| Visual identity | Bright, colorful, visually varied space; inspirations include *Frontier: Elite II* and *Earth & Beyond* |
| Player progression | Multi-capital-ship fleet ownership, capital ship acquisition, large-scale space battles, component upgrades and tuning, and investment in existing stations and industry; no player construction of stations or industrial installations; piloting remains central |
| Faction authority | Rank governs responsibility and command over faction assets, progressing from assigned pilot to formation leadership, target selection and strategic decisions |
| Battle missions | Authored mission types bind to actual available ships, formations, targets and battle needs |
| Equipment identity | Named reusable tuning stamps and specialized attachable parts distinguish components |
| Transport networks | Persistent civilian, commercial and military supply lines; discoverable and usable or attackable cargo lanes; shipping lanes generally safer through heavier policing |
| Technology | Independently designed Rust/Bevy architecture targeting Windows x64 |
| Project independence | Fresh repository and history; independently authored code and original or independently licensed assets; no inherited legacy materials or compatibility obligations |
| Code licensing | MIT OR Apache-2.0 for code; CC BY 4.0 for documentation and original lore; separate branding and other asset rights |
| Project identity | Interstellar Meridian; repository JohnnyPi/InterstellarMeridian; Rust package interstellar_meridian |

### 2.2 Not yet established

The exact number of systems or factions; specific flight-assist behavior; travel-drive mechanics; exact rank titles, promotion criteria and command permission matrices; economic formulas; simulation tick rates; component tuning mechanics and limits, fleet sizes and battle performance budgets; exploration sensor mechanics; procedural generation grammar; scripting language; persistence schemas; and eventual project branding all remain design questions.

## 3. Core design pillars

### P1 — Density over diameter

**Principle.** A modest region of space with distinctive geography, politics, infrastructure, cultures, resources, and histories is better than a huge map whose systems are interchangeable.

**Player-facing evidence.** A player remembers particular stations, resource fields, dangerous passages, and faction leaders because they behave differently and have consequences—not just because their names, colors, or statistics differ.

**Guardrail.** Do not increase the system count merely to advertise scale. New systems should add some new strategic, economic, exploration, narrative, or geographical relationship.

### P2 — Independent but understandable causality

**Principle.** Markets, fleets, production, transport, factions, security, and missions participate in a shared causal world model. World changes are not solely player-triggered.

**Player-facing evidence.** A convoy loss changes inventories and prices; a station capture changes access, patrols, and mission offers; the player can learn *why* through news, traffic records, market reports, and firsthand observation.

**Guardrail.** Avoid hidden world-state changes that present themselves as simulated consequences without supporting causes. The game may approximate remote outcomes, but it should record enough state to explain them.

### P3 — Meaningful individual agency, not guaranteed centrality

**Principle.** A single pilot can influence the course of events through leverage, specialization, alliances, risk, and timing, but cannot effortlessly dictate galactic outcomes.

**Player-facing evidence.** Destroying an enemy carrier can shift a front; smuggling replacement parts can keep it alive; negotiating a ceasefire can avert a battle. If the player is absent, autonomous participants still produce an outcome based on world conditions.

**Guardrail.** Avoid automatically deciding mission success or failure based only on whether the player accepted it. Avoid omniscient retaliation: an enemy hunts the player because of evidence, intelligence, reputation, or a known act.

### P4 — Satisfying flight across meaningful scales

**Principle.** Flight should offer responsive, skillful close-range maneuvers without pretending that star-system travel takes place in a small arena.

**Player-facing evidence.** Ship mass, thrust, loadout, and velocity matter in combat; stations and planets feel distant; crossing a system presents navigation, fuel, time, danger, and route choices without requiring long stretches of empty waiting.

**Guardrail.** Do not lock down Newtonian realism or arcade conventions before a graybox handling prototype and playtests.

### P5 — Discovery is an activity, not a map checkbox

**Principle.** Exploration should find useful, dangerous, unusual, or consequential phenomena. Discovery has uncertainty, method, risk, and possible downstream effects.

**Player-facing evidence.** A resource field changes trade or warfare; a derelict yields salvage and historical clues; an anomaly changes navigation; a previously unknown station becomes a new social or strategic node.

**Guardrail.** Do not fill space with collectible markers that exist only to be scanned once for a trivial payout.

### P6 — Connected careers rather than isolated minigames

**Principle.** Trading, mining, security, smuggling, piracy, diplomacy, and scientific work should be different ways to interact with the same world and its needs.

**Player-facing evidence.** Miners supply industrial ships, pirates disrupt routes, bounty hunters suppress them, diplomats negotiate access, and independent explorers reveal new resources or hazards.

**Guardrail.** Depth is preferable to promising a large catalog of shallow professions at launch.

### P7 — A luminous and legible frontier

**Principle.** Space should be colorful, diverse, and visually memorable, without losing the blackness, scale, readability, or mystery of the void.

**Player-facing evidence.** Different stellar environments and locations have recognizable palettes, silhouettes, lighting, and visual landmarks. The art direction uses color deliberately rather than applying uniform haze everywhere.

**Guardrail.** Artistic intensity must not compromise target readability, flight instruments, accessibility, performance, or physical scale.

### P8 — Open-ended progression and self-directed purpose

**Principle.** Players begin with modest resources and choose goals. There is no mandatory central storyline or one correct career path.

**Player-facing evidence.** A player can succeed as an independent trader, outlaw, explorer, contractor, diplomat, or hybrid. A long-term campaign emerges from relationships, possessions, reputation, discoveries, and changing world conditions.

**Guardrail.** Do not require faction membership or combat progression to unlock the rest of the game.

### P8A — Captain first, owner second

**Principle (owner-established).** Players may eventually build fleets containing multiple capital ships, participate in large-scale space battles, purchase capital ships, and customize existing ship designs through upgrades and tuning, and invest in existing stations and industrial operations, but may not construct stations or industrial installations. Piloting remains the central experience.

**Player-facing evidence.** Ownership expands what a captain can do without making fleet or industrial management mandatory. A single-ship career remains viable.

**Guardrail.** Do not turn progression into a station-building or empire-management game.

### P9 — Deterministic foundations, selective simulation detail

**Principle.** Content generation and important world-state transitions are seeded, versioned, reproducible, and testable where practical. Computation scales with the relevance of each world area.

**Player-facing evidence.** Worlds persist across visits and major events are coherent. Distant places evolve without simulating every individual ship maneuver at 60 frames per second.

**Guardrail.** Determinism means explicit inputs, RNG streams, ordering rules, and versioned state. Rendering and incidental audiovisual effects need not be bit-for-bit deterministic.

### P10 — A moddable, independent foundation

**Principle.** The modern game has its own content schemas and extension points, designed progressively alongside real gameplay systems.

**Player-facing evidence.** New ships, commodities, locations, missions, factions, and visual treatments can eventually be authored without changing Rust engine internals.

**Guardrail.** Native extension interfaces are the target; no external game compatibility requirement. Avoid building a speculative scripting platform before concrete extension needs are identified.

## 4. The world's shape: a living region, not an empty galaxy

**Candidate design:** start with a **connected frontier region**: established routes, contested borders, industrial centers, isolated outposts, unexplored pockets, and hazardous shortcuts. The region should have physical and political topology, not just randomly scattered destinations.

A potential *design exploration range*, not a committed content target, is **6–12 deeply differentiated systems** with **3–4 consequential major factions** and smaller local interests. The eventual game may grow beyond this; the key constraint is adding interconnections before adding map area.

Each system should have a reason to exist, potentially combining:

- A **geographic role** (junction, dead end, frontier gate, hazardous passage, isolated pocket).
- An **economic role** (agriculture, extraction, shipbuilding, research, fuel, trade, pilgrimage, military depot).
- A **political role** (capital, frontier dispute, neutral buffer, occupied territory, pirate refuge).
- An **exploration signature** (rare gas cloud, debris belt, anomaly, wreck field, ancient installation, active survey zone).
- A **social identity** (institutions, attitudes, local services, history, conflicts, named recurring actors).

A system can have several roles, but should not feel algorithmically identical to its neighbor.

## 5. The living-universe causal model

### 5.1 Key systems to interconnect

1. **Resources:** deposits, reserves, extraction rate, exhaustion/regeneration where appropriate.
2. **Production and consumption:** facilities, inputs, outputs, capacity, demand, maintenance.
3. **Logistics:** routes, cargo, travel time, escorts, interdiction, shipping risk.
4. **Markets:** stocks, orders or abstractions of supply/demand, availability, prices, access restrictions.
5. **Factions:** assets, leadership/policy, relationships, claims, military force, economic capacity.
6. **Security and conflict:** patrols, piracy, blockade, attacks, damage, occupation, recovery.
7. **Agents:** independent traders, miners, pirates, mercenaries, authorities, explorers and named notable NPCs.
8. **Opportunities:** contracts and missions derived from needs, shortages, threats, relationships and discoveries.
9. **Consequences and history:** persisted world changes, records of significant events, and player reputation.

**Illustrative causality chain:** A pirate ambush destroys two ore freighters → the shipyard's input inventory falls below its repair threshold → a military carrier's refit is delayed → a border fleet loses its intended support → a disputed station becomes vulnerable → security deteriorates → demand for escorts, salvage, replacement ore, intelligence, and ceasefire negotiations increases.

A world simulation may abstract a remote convoy rather than instantiate a detailed ship, but it must respect that the cargo existed, the shipyard needed it, and the outcome changed relevant state.

### 5.1A Persistent supply lines and shipping lanes

**Established direction:** Civilian, commercial and military transport must exist as continuing world activity, not merely combat encounter decoration. Players can discover cargo lanes, travel along them or attack shipping. Shipping lanes should generally be safer because they receive heavier policing, not because they guarantee immunity.

**Candidate design:** Distinguish a route (navigable corridor between locations), a service or supply line (recurring transport obligation), and a shipment (actual cargo moving between endpoints). Civilian passenger and essential-goods services, commercial raw-material and finished-goods freight, and military fuel, ammunition, personnel and repair shipments share the same logistics substrate but have different access, scheduling, security and priority policies. Route knowledge can come from charts, traffic observation, sensors, informants or exploration; military routes need not be public. Lanes need not be physical rails or gates.

- Each shipment has a source, destination, cargo or passenger manifest, assigned transport capacity, departure, estimated arrival, progress and outcome. Remote aggregation is permitted if quantities, timing and material outcomes reconcile with local encounters.
- Dispatch reserves or removes stock exactly once; arrival transfers delivered quantities; capture, loss, diversion and delay change the appropriate ledgers. Destroyed or captured cargo cannot also be delivered.
- Patrols, escorts, checkpoints and response times depend on jurisdiction, available ships, risk and operational priorities. A blockade or military redeployment can make a normally safe lane dangerous. Patrol protection is an emergent service, not invulnerability.
- Players may follow traffic, escort or join convoys, trade, intercept, raid, investigate missing shipments, or bypass routes using riskier shortcuts. Attacks may provoke evidence-based enforcement and faction responses.
- Persistent history retains significant deliveries, losses and route changes so revisiting a system reveals consequences rather than resetting traffic. No real-time progression while the game is closed is implied.

**Candidate validation:** A shipment moving remotely becomes the same shipment when encountered locally; a raid changes destination stock and subsequent demand, and altered patrol availability changes risk. Compare unattended arrival, player escort, cargo capture and reroute scenarios. Avoid double-counting at simulation-detail transitions and save/load.

### 5.2 The carrier-war thought experiment

**This scenario is an acceptance prototype proposal, not authored campaign canon.**

- Two factions contest a strategically useful station and its supply corridor.
- One faction has a carrier with a recorded readiness state, escorts, fuel, ammunition, and mission assignments.
- The opposing faction schedules an operation against the carrier.
- The player may join the strike, deliver crucial supplies, sabotage escorts, provide intelligence, defend the carrier, negotiate, or stay away.
- The operation resolves from the available forces, resources, intelligence, player intervention, and deterministic outcome rules—not from an arbitrary binary story flag.

**Outcome A:** Carrier destroyed. Local force projection and morale/command capacity drop; the opponents may take territory; survivors retreat; new salvage and reconstruction opportunities appear.

**Outcome B:** Strike fails. Carrier remains operational, conducts reprisals or advances, and may capture nearby assets. If the player's role is known, the enemy can place a bounty or assign pursuers.

**Outcome C:** Player is absent. The same world rules determine success, partial success, stalemate, retreat, or defeat. Events are not frozen waiting for player participation.

**Outcome D:** Player finds an alternative. Interdicting fuel deliveries, extracting an officer, bribing a neutral port, or brokering a ceasefire may change the battle conditions before it begins.

**Crucial requirement:** Each meaningful outcome persists and changes at least two other systems (for example, economy and navigation; faction control and security; production and mission availability). The player can inspect evidence of what happened.

### 5.3 Simulation levels (candidate architecture concept)

| Level | Scope | Typical detail | Main purpose |
|---|---|---|---|
| **Active** | Player's immediate vicinity | Responsive flight, physics, targeting, collisions, weapon effects, tactical NPC behavior | Hands-on gameplay |
| **Operational** | Current system outside active area | Route progress, docks, production, patrol assignments, encounter scheduling, selected named actors | Coherent local activity |
| **Strategic** | Other systems in connected region | Cargo and inventory movements, military readiness, production, influence, contracts, territorial change | Continuing world history |
| **Dormant/Deferred** | Low-relevance or undiscovered areas | Versioned state plus scheduled or analytical catch-up when time advances or events occur | Efficient scale |

Design requirements for later D3/D4: stable persistent identities; explicit time domains; local/remote transfer of ownership; event ordering; deterministic tie-breaks; reconcilable resource ledgers; and explainable transition history. Bevy ECS entities need not exist for every inactive resource, faction process, or remote ship.

No fixed rates, simulation formulas, concurrency model, or ECS component API are approved here.

## 6. Careers worth exploring

All careers should tap into shared systems; only a carefully chosen subset should be required for the first playable slice.

| Career | Core player activity | Effects on the shared world |
|---|---|---|
| **Trader / logistics pilot** | Buy, route, contract, haul, insure, reroute | Supplies industries and populations; shortages and route reliability |
| **Miner / prospector** | Survey deposits, extract, refine, sell claims | Unlocks production, drives commodity supply, attracts competition |
| **Salvager / wreck specialist** | Locate wrecks, stabilize, recover, restore | Returns materials, secrets and equipment; legal ownership disputes |
| **Bounty hunter** | Investigate targets, track, intercept, apprehend/destroy | Suppresses crime, changes threat and reputation |
| **Pirate / raider** | Ambush, intimidate, ransom, steal, evade | Interrupts trade, triggers security responses, changes prices |
| **Smuggler / blockade runner** | Conceal shipments, forge routes, evade scans | Bypasses sanctions, feeds illicit markets, changes conflict outcomes |
| **Escort / private security** | Protect fleets, stations, surveyors and civilians | Lowers route risk and cargo losses |
| **Military contractor / privateer** | Patrol, recon, strike, support, sabotage | Changes battle readiness and territorial control |
| **Diplomat / political operative** | Negotiate, gather leverage, mediate, broker access | Alters alliances, permits, ceasefires, embargoes and intelligence |
| **Explorer / surveyor** | Detect, triangulate, map, analyze, register discoveries | Reveals routes, resources, anomalies and strategic opportunities |
| **Scientist / anomaly investigator** | Sample, experiment, investigate phenomena | Unlocks information, technologies and environmental responses |
| **Courier / intelligence operative** | Deliver sensitive data, spy, decrypt, misdirect | Changes faction knowledge and decision-making |
| **Rescue / recovery pilot** | Distress response, medical evacuation, repair assistance | Preserves assets, reputations, and relationships |

**Candidate first playable combination:** trading/logistics + prospecting/mining + one faction/security conflict, with salvage/exploration connecting them. This deliberately prioritizes systemic overlap over career count.

**Open choice:** Diplomacy can range from negotiations and influence conducted as a pilot to a much larger strategic-command interface. Start with interpersonal and contract-level agency unless broader command proves central to the experience.

### 6.1 Long-term ownership and progression

**Established capabilities:** multiple owned ships forming fleets including multiple capital ships; purchasable capital ships; large-scale space battles inspired by the spectacle of Star Wars; customization of existing ship designs via equipment upgrades and component tuning; investments in existing stations and industrial operations. Players cannot build stations or industrial operations. These are long-term product goals, not first-slice requirements.

**Candidate mechanics:**

- Fleet: hire captains and crews, issue high-level assignments for trade, mining, escort or survey; autonomous ships face the same operating costs, risks and world rules as NPC ships. Avoid mandatory micromanagement.
- Capital ships: fleets may contain multiple capital ships, not just one flagship. Personally command a selected ship from its bridge; helm, navigation, tactical and engineering controls may be delegated. Capital ships should feel distinct from fighters, with momentum, crew, logistics and possible carried craft. Exact classes and carrier mechanics remain open.
- Fleet battles: large engagements with opposing capital ships, fighter squadrons and supporting vessels are a long-term product goal. Players may participate as individual pilots or fleet commanders. Candidate mechanics include formation orders, target priorities, subsystem damage, fighter deployment, withdrawal and reinforcement. Battles should affect persistent military strength, logistics and territorial control. Exact ship counts, rank-specific command limits, tactical interface and performance budgets require prototypes; no arbitrary scale promise is made.
- Ship customization: start from existing ship hull designs, purchase or replace compatible weapons, shields, engines, hull upgrades and other equipment, and optionally tune component characteristics. Candidate trade-offs include thrust versus heat or fuel use, shield capacity versus recharge, armor versus mass, and weapon output versus power draw. Shipyards may install or refit equipment. A full ship construction simulator, freeform hull design and structural assembly are not planned requirements. Named reusable tuning stamps and attachable specialized parts are required (section 6.3). Exact slots, tuning ranges and constraints remain open.
- Investment: purchase stakes in existing installations or enterprises, with returns linked to actual economic performance. Candidate opportunities include financing repairs, production expansion by NPC owners, or delivery contracts; dividends, ownership rights and governance remain open. No player-placed factories or stations.
- Progression: independent pilot → established captain → multi-ship owner → fleet or capital ship commander → investor is an illustrative path, not a mandatory rank system. Every stage should preserve meaningful cockpit gameplay.

**Candidate causal example:** A player invests in an existing refinery, purchases and refits a mining ship, and personally escorts its first delivery. A faction blockade threatens both the shipment and refinery returns. The player can fly an escort mission, negotiate access, or redirect the fleet. NPC owners retain responsibility for infrastructure construction.

**Later acceptance proposal:** fleet ships operate without continuous player input; upgrades and tuning obey validated constraints; capital ships can be personally commanded; fleets can contain multiple capital ships; large battles preserve meaningful pilot participation and persist their outcomes; investment outcomes respond to simulated production and trade; no station construction interface is required; single-ship progression remains viable. D5–D7, D9 and D10 must refine and schedule these capabilities.

### 6.2 Faction rank and command authority

**Established direction:** Responsibility within a faction is rank-dependent. Low-ranking pilots execute assignments; subsequent ranks unlock formation leadership and formation selection, then target changes, then full strategic decisions within authorized faction scope. Rank is distinct from reputation, personal wealth and ownership. Personal fleet ownership does not automatically confer authority over faction-owned assets.

**Candidate progression:** individual pilot → formation leader → squadron commander → task group commander → fleet commander → strategic commander. Titles, promotion rules, qualifications, delegation and faction-specific variations remain open. Authority should be checked against the issuing actor, assigned force, operational scope and current rank or delegated role; no automatic authority over unrelated forces. Orders are executed by autonomous agents; player command replaces eligible NPC decisions rather than bypassing the simulation. Piloting remains central and high-level command can be optional.

### 6.3 Named tuning stamps and specialized component parts

**Established direction:** Players can stamp a tuned component configuration with a custom name and reference and reuse it on other ships. Specialized parts attach to components and make them distinctive. Existing hull designs, equipment upgrades and refits remain the scope; no freeform hull construction.

**Candidate model:** distinguish component definition, owned physical component instance, attached part instance and named tuning stamp. A stamp is a reusable configuration recipe, not an item duplication mechanism. It records compatible component family/model, tuning settings and optionally desired part requirements. Applying a stamp to another component validates compatibility, limits, available parts and any fitting costs. A stamp never creates rare parts, transfers wear, or overwrites physical ownership. Missing parts or incompatible equipment produce an explicit report rather than silently substituting. A named, versioned stamp can be revised or copied; existing fitted equipment retains its applied settings unless explicitly retuned. Exact stamp portability, versioning, part sockets, acquisition and fitting costs remain proposals.

Specialized parts might include an experimental cooling manifold, reinforced emitter, precision focusing lens, rare sensor crystal or improved regulator. They can alter tuning bounds or trade-offs as well as base characteristics. Avoid universal best-in-slot upgrades; emphasize specialization, provenance and meaningful choices. Preserve instance identity and configuration across ship transfer and save/load.

## 7. Exploration: finding things with consequences

### 7.1 Discovery categories (brainstorm backlog)

| Category | Potential finds | Meaningful payoff or complication |
|---|---|---|
| **Resource fields** | Asteroid belts, rare ore pockets, ice, gas clouds, volatile elements | Claims, extraction, industrial demand, competition |
| **Wrecks and debris** | Lost freighters, ancient battles, derelict carriers, ruined stations | Salvage, evidence, hazards, ownership disputes |
| **Hidden infrastructure** | Smuggler depots, abandoned listening posts, sealed research platforms | Faction secrets, markets, access codes, missions |
| **Natural hazards** | Radiation pockets, dust storms, magnetic turbulence, unusual gravity | Navigation choices, equipment risks, safe routes |
| **Anomalies** | Unusual emissions, gravitational lenses, artificial signals, exotic matter | Scientific investigation, unique rules, rare opportunities |
| **Historical sites** | Monumental wrecks, old frontier battlefields, isolated monuments | Local history, long-form mysteries, faction identity |
| **Transient sites** | Distress calls, migrating debris, active collisions, convoy aftermath | Time-sensitive rescue, salvage, danger |
| **Uncharted routes** | Hidden passages, unstable jump corridors, anomalous shortcuts | Trading advantage, invasion paths, piracy opportunities |
| **Life and ecosystems** | Exotic microbial signatures or spaceborne phenomena, if consistent with setting | Research, ethical disputes, ecological protection |

### 7.2 A proposed discovery loop

**Notice → Investigate → Interpret → Decide → Change.**

For example: anomalous sensor return → approach and triangulate → find a rare gas pocket beside unstable debris → decide whether to sell coordinates, claim it, mine it, or conceal it → the new resource affects industry, faction attention, traffic, or hazard maps.

The same site may be relevant to several professions. An abandoned station is a salvage location to one player, an intelligence source to another, and a politically explosive artifact to a diplomat.

**Design preference:** A mixture of authored landmark patterns and procedural variation, rather than entirely undifferentiated random placement. Not every site must change regional politics; small atmospheric discoveries are welcome, but significant sites should link back into the world.

## 8. Flight, distance, and travel

**Established feel:** responsive local flying and dogfighting, between arcade and full realism; convincingly vast distances.

**Candidate three-scale design:**

1. **Tactical flight:** manual or assisted high-responsiveness movement near ships, stations, asteroids, and combat zones. Mass, acceleration, maneuverability, heat/energy and loadout should affect handling. Optional assisted vs. freer-inertia modes are for prototyping, not a settled requirement.
2. **System transit:** a distinct cruise/travel capability for meaningful interplanetary distance. Route choice, signature, acceleration/interdiction risk, fuel/energy, and dangerous approach/exits provide gameplay; distance should not mean minutes of inactivity.
3. **Interstellar transit:** jumps constrained by an understandable navigation topology, range, cost, known/unknown routes, and world time. The topology becomes part of trade, piracy, politics, and exploration.

**Playtest question:** Can travel communicate scale *and* produce decisions, without turning journey time into either a long empty wait or a purely decorative menu click?

## 9. Visual and audio identity

**Art statement:** *Luminous space opera with credible scale and readable action.*

The inspirations are the expressive colors of *Frontier: Elite II* and the memorable environments and space features of *Earth & Beyond*, reinterpreted into an original style rather than copied assets or imagery.

**Candidate art grammar:**

- Recognizable regional palettes: warm industrial stars, saturated ion clouds, icy blue shadowed belts, violet exotic zones, dramatic gold/teal station lighting.
- Rich near-field structures and clear far-field depth: asteroid textures, dust ribbons, gas clouds, wreck silhouettes, visible traffic lanes and energetic phenomena.
- Signature locations: stations, shipyards, resource fields, carriers, research sites, and anomalies recognizable at a glance.
- Dark negative space to preserve scale and contrast; vibrant regions should be special rather than a constant bright fog.
- Legible silhouettes and instrument contrast; accessible color encodings never depend on hue alone.
- Diegetic world information: broadcasts, station signage, hull markings, traffic patterns, communications and sound design reveal politics and history.

**Guardrail:** Photorealism is not a goal. Visual clarity, expressive art direction, and performance matter more than physically exhaustive rendering.

## 10. Missions and authored content

**Candidate system:** Missions represent opportunities arising from factual conditions—*who wants what, why now, what resources or permissions are needed, what alternatives exist, and what changes if the attempt succeeds or fails.*

Use a blend of authored narrative framing, data-driven world facts, and bounded procedural composition. A generated mission must not invent a nonexistent station, commodity, need, or reachable target to satisfy a template.

Missions should permit multiple approaches when justified by the world: a blockade may be broken by fighting, deception, rerouting, diplomacy, or depriving its fleet of fuel. Outcomes should be consumed by the same world simulation that created the opportunity.

### 10.1 Military mission templates and operational binding

**Established direction:** Battle assignments are generated from predetermined mission types using actual available ships, formations and targets. Example: an anti-capital bombing-run template becomes “Join Bomber Wing Delta Against Cap Ship Frigate Nevermind.” Player responsibility and permissible order changes depend on faction rank (section 6.2).

**Candidate model:** Strategic objective → operation → battle plan → tactical objective → formation assignment → individual assignment. Authored templates define eligibility, required capabilities, target predicates, objectives, completion/failure conditions and text slots. Bind stable IDs to existing assets; do not create a target merely to satisfy a template. NPC commanders use the same mission and order model. Candidate families include intercept, escort, anti-capital strike, reconnaissance, defensive screening, resupply, rescue, withdrawal and salvage.

Assignments and operational outcomes are distinct: a target destroyed by another wing may fulfill the operational objective while the player's individual participation is recorded separately. A changed or destroyed target, unavailable formation, withdrawn authorization or completed battle triggers reevaluation, cancellation or authorized reassignment. Forces cannot be allocated to incompatible simultaneous assignments without an explicit policy. No mission freezes the battle waiting for the player.

**Candidate validation:** With identical battle state, low-rank player receives an assignment, formation leader can select formation but not unauthorized targets, higher rank can redirect assigned forces, strategic authority can change operation objectives. Repeat with player absent and with target destroyed externally. Persist assignments, authority and outcomes across simulation levels and save/load.

**Non-goal:** An unrestricted universal quest generator in the first implementation milestone. Begin with a few coherent mission grammars using real world state.

## 11. Scope discipline and delivery strategy

### 11.1 Explicit early non-goals (proposed)

- No compatibility with another game’s behavior, content, expansion packages or save files is required.
- No enormous homogeneous generated galaxy as a launch requirement.
- No multiplayer/networked economy in the initial project.
- No seamless atmospheric landing or on-foot station/planet exploration in the initial scope.
- No full per-ship high-frequency simulation across all remote systems.
- No simultaneous implementation of every career, faction mechanic, ship subsystem, or mission type.
- No player construction of stations or industrial installations (established product boundary).
- No full ship construction simulator or freeform hull assembly requirement; focus on upgrading and tuning existing ship designs.
- No requirement for fleet ownership, capital ships, large fleet battles, advanced ship customization or investment in the initial vertical slice.
- No dependency on photorealism, AAA asset volume, or massive fleet-control UI.

### 11.2 Earliest differentiated proof: the causal sandbox

**Proposed headless experiment, prior to building the full living universe:**

- Two neighboring systems, two opposed factions, several stations and transport links.
- A small commodity and production chain, real inventory changes, one military asset with readiness state.
- Autonomous trade/transport and an impending contested operation; candidate extension: a persistent supply shipment and rank-bound assignment to demonstrate logistics and military mission causality.
- Four deterministic interventions: supply, sabotage, direct combat outcome, and player non-participation.
- State transitions recorded with inputs, outcomes, and player-visible explanation fields.
- Save/load and replay of the same seed and choices yield equivalent material outcomes.

**Pass when:** The carrier's survival or loss creates coherent, persistent downstream changes to at least two other systems, and the outcome can also progress without the player. Fail if the content is just hardcoded branches with no common world facts.

This experiment does **not** establish the final architecture or require beautiful visuals. In parallel or shortly afterward, a minimalist flight graybox must test the tactile feel before elaborate world or content systems are locked down.

### 11.3 Later vertical slice proposal

A tiny region with a dockable station, one controllable ship, system transit, a small market, prospectable resource site, an autonomous trade route, a faction border conflict, and save/load. The player can complete a legitimate trading/prospecting/security loop and observe one persistent cross-system consequence.

All proposed numbers and features require D10 scheduling, dependency analysis, test criteria, and performance budgets before they become milestones.

## 12. Technical principles

**Retained unless revised later:** Rust, Bevy, Windows x64 first; modular ECS; simulation/presentation/persistence separation; headless tests; seeded and versioned procedural generation; durable IDs separate from ECS handles; explicit subsystem interfaces; data-driven content; profiling-led optimization; incremental, test-first work; accessibility and mod-aware design.

**Simulation detail:** Strategic simulation need not be represented as Bevy ECS entities at all times; abstract persistent models may feed and receive active ECS instances. Simulation accuracy is measured by coherent state transitions and player-observable consequences, not literal continuous ticking of distant objects.

**To be evaluated, not chosen in D0:** Bevy version, physics library, spatial coordinate representation, save encoding, script runtime, deterministic scheduler design, economic algorithm, or ECS crate boundaries.

## 13. Independent development, provenance, and licensing

- Interstellar Meridian is established in a fresh Git repository with no inherited application history. Prior research remains outside the active project and imposes no implementation gates.
- Project-owned source code is dual-licensed under MIT OR Apache-2.0, at the recipient’s option. Maintain both license texts and the corresponding Cargo SPDX expression when a manifest is introduced.
- Code, data, art, music and fiction must be independently authored or obtained under appropriate documented rights. Do not import materials from the earlier reference project.
- Verify third-party dependencies and assets for license compatibility, attribution, notices, modification and distribution obligations before integration. Record provenance and preserve required notices. Unresolved rights block inclusion.
- Project-owned documentation and original fictional lore are licensed under Creative Commons Attribution 4.0 International (CC BY 4.0), https://creativecommons.org/licenses/by/4.0/. Project-owned source code remains MIT OR Apache-2.0. The game name, official logos and other source-identifying branding are excluded from the CC grant as branding; no trademark or endorsement permission is granted. Artwork, music and audio not expressly released as documentation or lore require separate licensing before distribution. Third-party materials retain their applicable terms.
- Interstellar Meridian is the owner-selected development title. Formal trademark clearance remains outstanding. The proposed tagline “Chart your own course” is branding guidance, not a gameplay requirement.

## 14. Documentation hierarchy and transition

| ID | Purpose | Disposition / next action |
|---|---|---|
| **D0** | Project Vision and Design Principles | Formal vision baseline; preserve candidate/open distinctions |
| **D1** | High-Level Game and Engine Design v2.1 | Reconcile accepted v2.0 with independent development |
| **D2** | Retired identifier | Not part of active project documentation |
| **D3** | Universe and World Simulation | Define the region, generator, causality, simulation levels, history and test sandbox |
| **D4** | Technical Architecture | Choose current Rust/Bevy versions and state/scheduling/ownership boundaries after validation |
| **D5** | Flight and Ship Systems | Prototype handling, travel modes, equipment upgrades, tuning, capital ship controls, and feedback |
| **D6** | Economy and Faction Simulation | Stocks, production, logistics, relationships, conflict and control |
| **D6A / D6B** | Factions, institutions, history and civilization | Setting references; preserve established versus proposed lore |
| **D7** | Autonomous Agents and AI | Strategic actor goals, operational plans, fleet coordination and tactical execution |
| **D8** | Procedural Mission System | Opportunities generated from world needs and resulting consequences |
| **D9** | Persistence and Modding | Versioned state, schemas, content APIs and extensibility |
| **D10** | Implementation Roadmap | Small milestones, explicit acceptance gates, measurable baselines |

**Change control:** Record independent project establishment in ADR-001. Reconcile D1 v2.1.1 and D10 with this baseline. Downstream documents must not silently contradict accepted requirements. Preserve historical documents outside the active project; record substantive changes explicitly.

## 15. Vision acceptance criteria and baseline disposition

The following accepted review criteria are retained as the vision-level baseline. Items describing prototypes establish validation intent, not approved implementation scope or test results:

1. The game is an independently developed original project with no inherited implementation or compatibility obligations.
2. Depth and interdependence are more important than raw star or faction counts.
3. The player's meaningful agency coexists with outcomes that occur in their absence.
4. Economic, political, military and exploration consequences share persisted world state.
5. Flight and travel balance enjoyable local handling with vast-feeling interplanetary distances.
6. Exploration is interactive and capable of generating other gameplay, not merely map completion.
7. A vivid, varied, original visual identity is a product requirement.
8. The initial implementation proves a small causal world and tactile flight before scaling content.
9. Open-source license compliance and asset provenance remain explicit obligations.
10. Long-term multi-capital-ship fleet, large battle, ship customization and investment progression preserves the pilot-first identity and excludes player construction of stations or industrial installations.
11. Faction rank governs earned command authority; battle missions bind authored types to available actual forces and targets.
12. Named reusable component tuning stamps and attachable specialized parts support customization without duplicating physical items.
13. Civilian, commercial and military supply lines persist; discoverable cargo lanes support travel and interdiction with generally stronger policing.
14. All detailed mechanics and numerical targets not expressly confirmed remain proposals until later documents and tests validate them.

## 16. Deferred design questions (not T0 blockers)

1. **Regional identity:** Does the game begin in a settled pocket with a contested frontier, an entirely unexplored frontier, or a mixture?
2. **Player leverage (direction resolved):** Multi-capital-ship fleets, large battles, ship upgrades and tuning, and investments are allowed; player-built stations/industry and full ship construction simulation are excluded from planned scope. Open: investment rights, fleet autonomy, battle scale, exact rank permissions, stamp/part compatibility and component tuning interface.
3. **Diplomacy:** Should negotiation be mostly face-to-face contracts and reputation, or may players act as formal faction envoys with authority to broker system-scale agreements?
4. **Exploration rewards:** Should discoveries usually become public knowledge and produce new traffic, or can players keep them secret, sell exclusivity, or establish claims?
5. **Travel risk:** How interruptible should high-speed system travel be—and what activities remain available while traversing immense distances?
6. **Historical time and logistics:** When an important event occurs far away, how much detail should the game retain so it can later be investigated?
7. **Visual direction:** Should the dominant aesthetic be retro-saturated, painterly science fiction, or a modern illustrative hybrid?

---

**v0.3 change record:** Owner clarified that fleets may include multiple capital ships and should support spectacular large battles. Earlier custom ship construction proposal is superseded by upgrades, refits and optional component tuning. Detailed combat, fleet and tuning mechanics remain proposals.

**v0.4 change record:** Owner established rank-dependent faction responsibility, operationally bound battle mission templates, named reusable tuning stamps, specialized attachable component parts, and persistent civilian/commercial/military supply lines with discoverable, generally more heavily policed shipping lanes. Proposed implementation semantics, ranks, logistics ledgers and validation scenarios remain subject to review. No repository changes or implementation authorization.

## 17. Vision baseline and migration record — 2026-11-26

T0 was accepted at vision scope in D0 v1.0. The owner subsequently accepted D1 v2.0, chose a completely independent greenfield project, approved MIT / Apache 2.0 dual code licensing and selected Interstellar Meridian as its title. These decisions are carried forward here; no candidate mechanic or numerical target is promoted.

**Setting handoff:** D6A and D6B have been migrated to v0.2.1. Established setting constraints include humanity’s Earth origins, settlement in another galaxy, seven governments descended from distinct generation-ship expeditions, human-origin contemporary civilizations, other life and remnants of vanished civilizations, and fragile postwar peace. D6B supersedes inconsistent earlier founding proposals in D6A. Exact chronology, technology mechanics and other proposed lore remain open. The candidate regional faction count does not replace the seven-government setting.

**Development status:** T0 and T1 remain accepted at their original substantive scope. T2 independent establishment is in progress; the owner supplied evidence of a fresh synchronized repository and tracked license files. Documentation migration and owner-confirmed Project setup are complete at the prior versions; this reconciled package requires final owner review, Source replacement and repository synchronization. D1 v2.1.1 and D10 v0.1.1 establish the development gates. T3 technical contracts and T4 implementation are not authorized by this document alone.

**v1.1 change record:** Rebrand D0 v1.0, replace obsolete legacy obligations with the greenfield and licensing decisions, update document hierarchy and gate status, and retain the original gameplay pillars, examples, candidate mechanics and open questions. Editorial migration awaits owner review. No dependency selection, code changes, asset clearance or runtime tests performed.

**Source basis:** D0 v1.0 (substantive vision); D1 v2.0 (accepted high-level scope); D6B v0.1 (setting constraints); D6A v0.1 (institutional reference); owner decisions on independent repository, licensing and name in the project establishment discussion. No external gameplay specification is incorporated.

**Next document:** D1 — High-Level Game and Engine Design v2.1.


**Reconciliation record (2026-11-26):** Editorial patch of 1.1 to 1.1.1; active references updated, owner-approved CC BY 4.0 documentation/lore policy incorporated, and outstanding T2 evidence distinguished from completed checks. Earlier migration records describe the preceding revision. No new gameplay, canon or technical implementation is authorized.
