This document describes breaking changes of `teloxide` crate, as well as the ways to update code.
Note that the list of required changes is not fully exhaustive and it may lack something in rare cases.

## 0.4 -> 0.5

### core

#### Field type changes

Types of some fields were changed to be more accurate. 
If you used them, you may need to change types in your code too.

Example:
```diff
let ps: PhotoSize = /* ... */;
-let w: i32 = ps.width;
+let w: u32 = ps.width;
```

List of changed types:
- `PhotoSoze::width`: `i32` -> `u32`
- `PhotoSoze::height`: `i32` -> `u32`
- `Restricted::until_date`: `i32` -> `DateTime<Utc>`
- `Kicked::until_date` (`Banned::until_date`): `i32` -> `DateTime<Utc>`
- `PublicChatSupergroup::slow_mode_delay`: `Option<i32>` -> `Option<u32>`
- `User::id`: `i32` -> `i64` (note: all methods which are accepting `user_id` were changed too)


#### Method output types

In teloxide `v0.4` (core `v0.2`) some API methods had wrong return types.
This made them practically unusable as they've always returned parsing error.
On the offchance you were using the methods, you may need to adjust types in your code.

List of changed return types:
- `get_chat_administrators`: `ChatMember` -> `Vec<ChatMember>`
- `send_chat_action`: `Message` -> `True`
- `leave_chat`: `String` -> `True`
- `pin_chat_message`: `String` -> `True`
- `set_chat_description`: `String` -> `True`
- `set_chat_photo`: `String` -> `True`
- `set_chat_title`: `String` -> `True`
- `unpin_all_chat_messages`: `String` -> `True`
- `unpin_chat_message`: `String` -> `True`


#### Method parameter types

Some API methods accept different types now. 
If you've used changed parameters, you need to adjust code for new types.

Examples:
```diff
let bot = Bot::new("TOKEN").auto_send();

-bot.set_webhook("url").await?;
+bot.set_webhook(Url::parse("url").unwrap()).await?;

let link = bot
    .create_chat_invite_link(chat_id)
-    .expire_date(timestamp)
# Note: this is not the only way to create `DateTime`. Refer to `chrono` docs for more.
+    .expire_date(DateTime::<Utc>::from_utc(
+        NaiveDateTime::from_timestamp(timestamp, 0), Utc)
+    )
    .await?;
```

See also: [teloxide examples fixes](https://github.com/teloxide/teloxide/pull/408/files/369e43aa7ed1b192d326e6bdfe76f3560001353f..18f88cc034e97fd437c48930728c1d5d2da7a14d).

List of changed required params:
- `SetWebhook::url`: `String` -> `Url`

List of changed optional params:
- `AnswerCallbackQuery::url`: `String` -> `Url`
- `SendInvoice::photo_url`: `String` -> `Url`
- `CreateChatInviteLink::expire_date`: `i64` -> `DateTime<Utc>` 
- `EditChatInviteLink::expire_date`: `i64` -> `DateTime<Utc>` 
- `KickChatMember::until_date`: `u64` -> `DateTime<Utc>` 
- `RestrictChatMember::until_date`: `u64` -> `DateTime<Utc>` 
- `SendPoll::close_date`: `u64` -> `DateTime<Utc>` 


#### Renamed items

Some items (fields, variants, types, methods) were renamed.
If you used them, you should start using new names.

Example:
```diff
-bot.send_chat_action(chat, ChatAction::RecordAudio).await?;
+bot.send_chat_action(chat, ChatAction::RecordVoice).await?;

-if chat_member.is_kicked() {
+if chat_member.is_banned() {
    /* ... */
}
```

List of renamed items:
- `ChatAction::RecordAudio` -> `RecordVoice`
- `ChatAction::UploadAudio` -> `UploadVoice`
- `ChatMemberKind::Creator` -> `Owner`
- `ChatMemberKind::Kicked` -> `Banned`
- `Creator` -> `Owner`
- `Kicked` -> `Banned`
- `ChatMemberKind::is_Creator` -> `is_owner` *
- `ChatMemberKind::is_kicked` -> `is_banned` *
- `ChatMemberStatus::Creator` -> `Owner`
- `ChatMemberStatus::Kicked` -> `Banned`
- `kick_chat_member` -> `ban_chat_member` *
- `get_chat_members_count` -> `get_chat_member_count` *

\* Old methods are still accessible, but deprecated


#### Added `impl Clone` for {`CacheMe`, `DefaultParseMode`, `Throttle`}

Previously said bot adaptors were lacking `Clone` implementation. 
To workaround this issue it was proposed to wrap bot in `Arc`.
Now it's not required, so you can remove the `Arc`:

```diff
let bot = Bot::new(token).parse_mode(ParseMode::MarkdownV2);
-let bot = Arc::new(bot);
```


### teloxide

#### Mutable reference for dispatching

`Dispatcher::dispatch` and `Dispatcher::dispatch_with_listener` now require mutable (unique) reference to self.
If you've used variable to store `Dispatcher`, you need to make it mutable:

```diff
-let dp = Dispatcher::new();
+let mut dp = Dispatcher::new();
/* ... */
dp.dispatch();
```


#### Listener refactor

`UpdateListener` trait was refactored.
If you've used `polling`/`polling_default` provided by teloxide, no changes are required.
If, however, you've used or implemented `UpdateListener` directly or used a `Stream` as a listener, 
then you need to refactor your code too.

See also: [teloxide examples fixes](https://github.com/teloxide/teloxide/pull/385/files/8785b8263cb4caebf212e2a66a19f73e653eb060..c378d6ef4e524da96718beec6f989e8ac51d1531).


#### `polling_default`

`polling_default` is now async, but removes webhook.

Example fix:
```diff
-let listener = polling_default(bot);
+let listener = polling_default(bot).await;
```
