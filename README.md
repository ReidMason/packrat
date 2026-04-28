# Features
## Core
- A tree will need to be generated somehow - Entities only know their direct parent
    - Home > Garage > Toolbox > Spanner
        - NOTE: lookup parent_id until it returns None
- A tree that shows all the children of that tree
- Deleting an entity that has to make sure any child updates their parent_id
- Updating a parent should be simple, it's just updating the parent_id
- Would be nice to include additional information to an entity
    - Such as warranty expiry (get a nofity when its close to expiring?)

## UI
- Fuzzy searching
