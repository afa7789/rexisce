# Tasks: xmpp-start Dead Code Wiring + Remaining Features

## dc-5 (PENDING)
**Priority:** 1 | **Agent:** builder
Wire message_mutations.rs into engine — use MutationManager for XEP-0308 corrections, XEP-0424 retractions, XEP-0444 reactions parsing

## dc-9 (PENDING)
**Priority:** 1 | **Agent:** builder
Wire account.rs XMPP module into engine — handle account info IQ queries/responses

## dc-22 (PENDING)
**Priority:** 2 | **Agent:** builder
Wire remaining OMEMO methods — bundle auto-fetch for new contacts, pre-key rotation when count drops, store.load_devices/sync_device_list on PEP events

## dc-11 (PENDING)
**Priority:** 2 | **Agent:** builder
Wire xmpp_uri.rs into app — parse xmpp: URIs, dispatch actions (open chat, join room)

## dc-17 (PENDING)
**Priority:** 2 | **Agent:** builder
Wire store repos fully — ensure avatar_crop, thumbnail, roster_repo CRUD methods are called from app lifecycle

## dc-18 (PENDING)
**Priority:** 2 | **Agent:** builder | **Depends on:** DC-19 (done)
Wire connection/proxy.rs — apply proxy settings from config when connecting

## dc-21 (PENDING)
**Priority:** 3 | **Agent:** senior-developer | **Depends on:** dc-9
Replace single engine with MultiEngineManager in App — full multi-account runtime

## dc-23 (PENDING)
**Priority:** 4 | **Agent:** builder | **Depends on:** all DC tasks
Remove ALL #![allow(dead_code)] annotations — final pass after all wiring done

## omemo-pep-autopub (PENDING)
**Priority:** 2 | **Agent:** builder | **Depends on:** dc-22
OMEMO PEP bundle auto-publish on XMPP connect

## omemo-key-exchange (PENDING)
**Priority:** 2 | **Agent:** senior-developer | **Depends on:** omemo-pep-autopub
OMEMO key exchange flow — fetch peer device list + bundles, create outbound Olm sessions

## omemo-ui-lock (PENDING)
**Priority:** 3 | **Agent:** builder | **Depends on:** omemo-key-exchange
Show OMEMO lock icon on encrypted messages, show trust fingerprint dialog

## multi-event-routing (PENDING)
**Priority:** 3 | **Agent:** senior-developer | **Depends on:** dc-21
Route all XmppEvent to correct per-account AccountState

## multi-keychain (PENDING)
**Priority:** 3 | **Agent:** builder | **Depends on:** multi-event-routing
Store XMPP credentials per-account in OS keychain

## push-notifications (PENDING)
**Priority:** 4 | **Agent:** builder
K7: Push notifications (XEP-0357) — enable/disable push on server

## moderator-retract (PENDING)
**Priority:** 4 | **Agent:** builder
P2: Moderator retract button in MUC + reason in tombstone
