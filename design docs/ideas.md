* TODO : 
- a map descriptor from the gdscript side => setup of the map on the rust side
- a spell for creating destructible walls
- a input descriptor on the rust side => a gui selector from the gdscript side
- a spell that pushes away, and attacks
- portals
- destructible terrains

spell && objects could be implemented via

- a description :
    - target : HashSet -> Any, Team, Enemy, Neutral
    - range : Force, option DictWeight
    - for objects : tags : hash_set HealObject (heal_force), DamageObject (damage), EffectObject(effect)
    - for spells : dyn code (scripting or closures) that returns a Vec<Intent> 

It is likely that a lot of actions would then be implemented as spell (attack enemy x then zone effect malus)

Lets think about the context that should be available for Entity when resolving intents into actions
- fog of war && others zone of effect
- current malus/bonus
- who is dead? close? still alive?
- when did i attack/ got attacked last?