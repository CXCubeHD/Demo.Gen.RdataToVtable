# Demo.Gen.RdataToVtable

**This is just a demo, more features or changes may be added soon!**

This project converts IDA rdata to json:

```
.rdata:000000014233D498 ; const ServerPlayer::`vftable'
.rdata:000000014233D498 ??_7ServerPlayer@@6B@ dq offset ?getStatusFlag@Actor@@UEBA_NW4ActorFlags@@@Z
.rdata:000000014233D498                                         ; DATA XREF: ServerPlayer::ServerPlayer(Level &,PacketSender &,NetworkSystem &,ClientBlobCache::Server::ActiveTransfersManager &,GameType,NetworkIdentifier const &,SubClientId,std::function<void (ServerPlayer &)>,mce::UUID,std::basic_string<char,std::char_traits<char>,std::allocator<char>> const &,std::basic_string<char,std::char_traits<char>,std::allocator<char>> const &,std::unique_ptr<Certificate,std::default_delete<Certificate>>,int,bool,EntityContext &):loc_140AAEA01↑o
.rdata:000000014233D498                                         ; ServerPlayer::~ServerPlayer(void)+D↑o
.rdata:000000014233D498                                         ; Actor::getStatusFlag(ActorFlags)
.rdata:000000014233D4A0                 dq offset ?setStatusFlag@Actor@@UEAAXW4ActorFlags@@_N@Z ; Actor::setStatusFlag(ActorFlags,bool)
.rdata:000000014233D4A8                 dq offset ?hasComponent@Mob@@UEBA_NAEBVHashedString@@@Z ; Mob::hasComponent(HashedString const &)
.rdata:000000014233D4B0                 dq offset ?getLastHurtByMob@Actor@@UEAAPEAVMob@@XZ ; Actor::getLastHurtByMob(void)
```

```json
{
  "type": "ServerPlayer",
  "methods": [
    {
      "symbol": "?getStatusFlag@Actor@@UEBA_NW4ActorFlags@@@Z",
      "undecorated_symbol": "public: virtual bool __cdecl Actor::getStatusFlag(enum ActorFlags)const __ptr64",
      "cleaned_symbol": "bool Actor::getStatusFlag(enum ActorFlags)const ",
      "name": "getStatusFlag",
      "scoped_name": "Actor::getStatusFlag",
      "parameter_types": [
        "enum ActorFlags"
      ],
      "return_type": "bool",
      "visibility": "public"
    },
    {
      "symbol": "?setStatusFlag@Actor@@UEAAXW4ActorFlags@@_N@Z",
      "undecorated_symbol": "public: virtual void __cdecl Actor::setStatusFlag(enum ActorFlags,bool) __ptr64",
      "cleaned_symbol": "void Actor::setStatusFlag(enum ActorFlags,bool)",
      "name": "setStatusFlag",
      "scoped_name": "Actor::setStatusFlag",
      "parameter_types": [
        "enum ActorFlags",
        "bool"
      ],
      "return_type": "void",
      "visibility": "public"
    },
    {
      "symbol": "?hasComponent@Mob@@UEBA_NAEBVHashedString@@@Z",
      "undecorated_symbol": "public: virtual bool __cdecl Mob::hasComponent(class HashedString const & __ptr64)const __ptr64",
      "cleaned_symbol": "bool Mob::hasComponent(class HashedString const &)const ",
      "name": "hasComponent",
      "scoped_name": "Mob::hasComponent",
      "parameter_types": [
        "class HashedString const &"
      ],
      "return_type": "bool",
      "visibility": "public"
    },
    {
      "symbol": "?getLastHurtByMob@Actor@@UEAAPEAVMob@@XZ",
      "undecorated_symbol": "public: virtual class Mob * __ptr64 __cdecl Actor::getLastHurtByMob(void) __ptr64",
      "cleaned_symbol": "class Mob * Actor::getLastHurtByMob(void)",
      "name": "getLastHurtByMob",
      "scoped_name": "Actor::getLastHurtByMob",
      "parameter_types": [],
      "return_type": "class Mob *",
      "visibility": "public"
    }
  ]
}
```
