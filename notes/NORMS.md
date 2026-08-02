# Programme-Wide Norms


## Organically Expanding UDON Conventions/Notation

> [!note] NOTE
> Much of this will eventually be part of UDON proper and this will have a much more brief explanation and point there. But while it's under development...

### One Base-name, Multiple Possible Manifestations

Here and elsewhere, unless specified otherwise, we'll use `BASENAME` to indicate an operational set of records (or items, objects, docs, etc.) in a project or subdirectory of a project/repo. It can also take the form `BASENAME.type` but often it's the same thing more or less, e.g., `TODO.todo-list` so you'll see the type often in this particular norms file, but more generally you'll just refer to "BASENAME" and the type will be inferred or known by convention detailed later here.

The actual *file*/ *directory*  that one would see indicating that directory *has* a '`BASENAME`' is one or more of the following:

| Form                                     | Example                               | Description                                                                                                                                                                                                              |
| ---------------------------------------- | ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Top-level File** *(deprecated)*        | `BASENAME.md`                         | Older noncompliant ones found in the wild (almost exclusively as of this writing)                                                                                                                                        |
| **Top-level Udon File** *(transitional)* | `BASENAME.udon`                       | Udon document, but without nailed down schema or being proper OEUDONCN (rolls right off the tongue...)                                                                                                                   |
| **Single File** *(compliant form)*       | `BASENAME.<type>.udon`                | Schema-controlled, properly OEUDONCNesque.                                                                                                                                                                               |
| **Directory**                            | `BASENAME.<type>.udon/`               | This is essentially a "directory as a doc-store." Right now there is no norm for what's *in* the directory, other than that it resolves to one or more records / parts that comply with `<type>`...                      |
| **Partitioned Form**                     | `BASENAME.<cat>[.<sub>]*.<type>.udon` | Multiple ones of these in the same directory*. Category (or partition) could also be a date at any level of chunking, could include an explicit ordering `NN-...` or no particular ordering etc.-- it's domain-specific. |
> [!warning] BEWARE
> 
> `BASENAME` is the union for that directory
> 
> So `./BASENAME-ONE.udon` and `./BASENAME-ONE.todos.udon/myTODO.md`  **should not** be seen to mean two different "things". It is instead two (although in this example ambiguously combined) parts of the **same** thing-- `BASENAME-ONE`.
> 
> So use a *different* base-name if you need two different organically expanding versions of the same "thing" (although often there's a transition period at the moment where they coexist).  `./BASENAME-ONE.fooos.udon` and  `./BASENAME-ONE.ughs.udon/` would be particularly bad and undefined ATM. `OUR-FOOOS` + separate `OUR-UGHS` (as `./OUR-FOOOS.fooos.udon` and `OUR-UGHS.ughs.udon/` or something) would be perfectly fine.

### Side-cars

*(Still a **bit** loose on how this works in conjunction with some of the permutations above etc....)*

Along with every `BASENAME` there can be a `.BASENAME-side` (which *is its own BASENAME*) -- meaning that `TODO.dag.udon` might be `TODO`'s implementation at the moment in a given directory, but it might have an sidecar currently manifested as: `TODO-side.listy.udon/*laundry*.md`  -- how the main thing and the sidecars are linked / associated is not currently specified.

### Organically Expanding?

Actually it's easier to explain with an example, which shows a healthy trajectory regardless of whether any of it is automatic or has to evolve manually. (Either way, still organic in the non-biological meaning of the word).

#### EXAMPLE:  "The project's TODO list"

The basic idea is this, presented as an example of how a thing with the base name 'TODO' or 'TODO-LIST' might evolve over time. It's not necessary to go through these steps-- especially the first or even transitional ones-- nor does it necessarily have to be in this direction, although I'm pretty sure this is the more likely direction...

1. You start *calling* it by the base-name, e.g., "Would you add such-and-such to the project's TODO?" (meaning base-name `TODO` in the *project root directory*), and that's all you ever have to call it.

*--- Early on / transitional... ---*

2. Initially there may be a `TODO.md` as the only thing, with who knows what in it-- often though headers and `- [ ] type list items` and breadcrumbs and markdown tables and lots of accumulated finished stuff and notes and sometimes adhoc and intermixed numbering/labeling conventions, etc.
3. Eventually that starts getting a bit unwieldy and you need to get it compliant so, *without changing how it's called / communicated*, you `mv TODO.md TODO.udon` and convert it from `- [ ] ...` lists and headings into a more udonic form.  Ah, much better, no more ever-evolving never-consistent TODO file. Also maybe now you add a `FINISHLOG` *separate* but related organically growing list.
4. You eventually realize there is already the perfect Udon schema for todo records, so you do a `mv TODO.udon TODO.sdag.udon` and edit it to be compliant, or maybe you've evolved the bespoke udon to where it feels like a solid and mature-enough syntax here, so you just write the schema (don't know how yet-- at least designate it) and `mv TODO.udon TODO.sdag.udon` (with 'sdag' or strategy-DAG being the hypothetical udon schema dialect in this case).
5. You now occasionally say "our s-dag TODO for the project" (or type "our TODO.sdag for the project" instead of just TODO to distinguish it from other types of TODO's that it might be otherwise confused with.

*--- Now your 'TODO list' is **COMPLIANT** ---*
*(This means that some of the following steps might eventually be automatic, potentially defined by the schema/dialect itself.)*

6. You have enough things going on where it becomes much easier to say "would you add that to TODO-meta?" or "What's next for me in TODO-joseph-urgent?" etc.  Initially-- these are just their own files:  `TODO.meta.sdag.udon` and `TODO.joseph.urgent.sdag.udon`, for example-- maybe with the unsorted residual still in `TODO.sdag.udon`.  *BUT* saying "the project's TODO" means *the union of all of those* (assuming still in this example that the files so far are in the project's top-level directory). (Now you have *Partitioned* form).
7. Eventually you have enough of these that you decide on a hybrid where there is:
    - A toplevel `TODO.sdag.udon\` directory, and, within it,
    - A whole bunch of `TODO.sdag.udon\TODO.**.sdag.udon` files.
8. Over time (or at automatic breakpoints) -- THOSE lists of udon directories might turn into their *own* directories (further subdividing)
9. Often it starts to make more sense to have the things within the subdirectory (however deep) become *one record* (TODO item / task in our example here) *per `<key>.sdag.udon` file* -- often with the key being a slug, uuid, or natural key-- or the key + the udon element type, etc.

#### Takeaway

When you go looking for the `TODO list for the spikes` you may find `spikes/TODO.**.*.udon` files there, or one but it's a *directory*, or any of a number of other permutations. A common thing that has taken the example trajectory above has been TODO lists, but also things like a project or subproject's LEXICON -- eventually turning into one `LEXICON.terms.udon/` directory filled with one defined term per document.

Ideally, as mentioned, this will all move into Udon tooling naturally, and it will be a simple matter of CLI or agentic tool "what are the TODOs for spikes" resolving to "the right thing" at *any* level of file-layout abstraction above. Same with insertions, deletions, modifications, subsearches, etc. etc.   BUT-- even *without* specialized tooling-- now there is the *convention* and expectation and even the manual operation of this (even with temporary exceptions while schemas are being developed etc. and are just implicit tags) it is *already* better than stalling at TODO.md or LEXICON.md indefinitely, or any of the subsequent stages. (LEXICON dynamic doc stores, for example, could easily continue to push into sub-sub-directories where the inner division is "bounded-contexts"). In fact, that's somewhat likely to happen in Archema here, where there is shared vocabulary across all of the subprojects and divisions, with those also able to have terms that apply only to themselves under well-defined boundaries. Hmmm... LEXICON in particular would benefit nicely from its own set of agentic tooling for things like inlining definitions not seen in context yet or something...

### Typical Examples:
- PRACTICA (higher level planning)
- TODO
- CHANGELOG
- INFLUX (stuff to process / integrate)
- LEXICON (or TERMS or TERMINOLOGY)

> [!todo] TODO
> 
> - [ ] Give this concept a name *(that isn't OEUDONCN)*

## Processing queues

> [!caution] 
> I started this *before* ending up working through the organically expanding udon stuff above! Now I wonder if this section is irrelevant, and just say "INFLUX" with standard sidecars for 'processed' / completed items, and sidecar folder for the payload itself as appropriate...

| Pri      | Path* or `BASENAME`                                          | Intent                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Old Retired Equivalent<br>Semi-equivalent                                                                            |
| -------- | ------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| REQUIRED | `INBOUND.queue`<br>or `INGEST.queue`<br>or `INTEGRATE.queue` | A manifest for unprocessed or unintegrated pieces/items. This is the *main indicator* that this directory (or sometimes it is still for the whole project although a record might migrate to a different process queue. In some cases it *can* just say "all of the current contents of `./.in-queue/`" -- although it should still be the main entry point so anyone can add items and notes about things that aren't represented in `.in-queue/` (below). | manifest, routing, index, pending, status, readme, ledger, todo, ...   (all usually in all-caps w/ various suffixes) |
|          | `./.in-queue/`                                               | A folder containing  material from any number of sources, often as directories, submodules, documents etc.  Essentially sidecars for items in the `IN-QUEUE`, and, as mentioned, sometimes with 1:1 correspondence with `IN-QUEUE`                                                                                                                                                                                                                          | inbox, to-integrate, .to-integrate, .ingest-queue, .integration-queue, .archive (yeesh), etc.                        |
\* -  Relative in this case usually to the project root *AND/OR* directories within the project




---


## INFLUX


```udon

; Obviously, while payload locations can be stacked and all apply,
  it's expected that you'd only need and only want one of these attributes---
  multiples here are for illustration as I figure out some options...
  Really you should expect something as simple as:
  
|influx :name "Audit-report Analysis, Routing, & Integration"
  :payload-list        ./audit*
  :payload-required?   true
  :payload-dup-policy  influx-suffix-dedup()
  :item-auto-remove?   false
  :item-from-payload?  true
  :item-auto-template
    |item[!{{payload.path | influx-get-key}}]
      :path !{{payload.path}}
      :created <datetime: !{{payload.path | ctime}}>
      :queued <datetime: !{{sys.now}}>
      ; ... ...

|items
  |item[P2389] :name "My 2,389'th Audit! Still no changes!"
    :path ...
    :queued <2026-01-01>
    :disposition grumpy
    
    # First off-- how dare you?
    
    It seems like:
      - you don't like me
      - you don't listen to me
      - you have failed to do the things I was pretty sure you were supposed to do when I wrote about you doing them!
    
    Please rectify *immediately*
    
    Sincerely, your doppleganger,
    TheBoat

  |item[weirdest-111]
  ; ...
  

; Example / provisional / ideation  payload specification-- a kind of sidecar or data/link attachment for a given record...
|influx


  :payload-list-glob  ./*.md ; for example
  :payload-list-dir   .INFLUX-payloads/*
  :payload-list-cmd   `find . -mindepth 1 -maxdepth 1 ! -name '.*' \( -name '*.md' -o -type d \)`
    ; Technically this would be perfect for spikes/ & audits/, for example...
      but obviously the find syntax is a bit oblique and there might be some simple aliases for this one, for example, to reuse...
      BECAUSE the basic idea is this is for the machinery--
      but also so a new agent/person can look up where to dump their document etc. for it to get processed... so...
  :payload-list *.md            ; from `./` implied
  :payload-list *.md + subdirs(*)  ; from `./` still implied, also, no `.*` prefixes for files or dirs implied
  :payload-list audit*.udon + subdirs(audit*/)
  :payload-list audit*          ; implied: relative to this file's ./, both files *and* directories as the list
  ; DIFFERENT FROM
  :payload-list audit*.md + audit**/*.md
  ; or something. This one will go into the directories and list the individual files--
    not what we want for spikes or audits, where a distinct audit/spike is
    *one* file OR *one* directory's full contents.  Not items per 
  
  
    
  
```

```
the-directory/
   .influx.conf.udon
   INFLUX.un  ; UN stands for Udon Notation (UDON stands for Universal Document Object Notation). implies INFLUX.influx.udon once schemas etc. are in place.
   .influx-inbox/
   .influx-finished/
     .audit-trail.udon
     .remnants.udon
     .remnants.archive/
     
# OR JUST

the-directory/
   INFLUX.un
   .influx/
     conf.un
     finished-log.un # Actual full audit-trail / final-disposition / remnants / notes / dates for payloads/archive items
     payloads/
       inbox/
       backlog/  # in-progress-- for intermediate states (relata does this, for example)
       archive/  # the fully processed and finished ones (many tar.gz depending on whether processor decided it would be useful to see)
       trash/    # (tar.gz and maybe config given TTL) ones that didn't even make the cut-- nothing from it "integrated"
  INFLUX/ -> ./.influx/payloads/inbox/  # symlink convention

```

