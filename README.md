# Demo.Gen.RdataToVtable

**This is just a demo, more features or changes may be added soon!**

This project converts IDA rdata (Windows only) to json:

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
  "methods": [
    "getStatusFlag",
    "setStatusFlag",
    "hasComponent",
    "getLastHurtByMob",
  ],
  "type": "ServerPlayer"
}
```
