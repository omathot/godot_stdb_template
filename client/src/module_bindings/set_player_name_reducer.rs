// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct SetPlayerNameArgs {
    pub name: String,
}

impl From<SetPlayerNameArgs> for super::Reducer {
    fn from(args: SetPlayerNameArgs) -> Self {
        Self::SetPlayerName { name: args.name }
    }
}

impl __sdk::InModule for SetPlayerNameArgs {
    type Module = super::RemoteModule;
}

pub struct SetPlayerNameCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `set_player_name`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait set_player_name {
    /// Request that the remote module invoke the reducer `set_player_name` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_set_player_name`] callbacks.
    fn set_player_name(&self, name: String) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `set_player_name`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`SetPlayerNameCallbackId`] can be passed to [`Self::remove_on_set_player_name`]
    /// to cancel the callback.
    fn on_set_player_name(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &String) + Send + 'static,
    ) -> SetPlayerNameCallbackId;
    /// Cancel a callback previously registered by [`Self::on_set_player_name`],
    /// causing it not to run in the future.
    fn remove_on_set_player_name(&self, callback: SetPlayerNameCallbackId);
}

impl set_player_name for super::RemoteReducers {
    fn set_player_name(&self, name: String) -> __sdk::Result<()> {
        self.imp
            .call_reducer("set_player_name", SetPlayerNameArgs { name })
    }
    fn on_set_player_name(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &String) + Send + 'static,
    ) -> SetPlayerNameCallbackId {
        SetPlayerNameCallbackId(self.imp.on_reducer(
            "set_player_name",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer: super::Reducer::SetPlayerName { name },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, name)
            }),
        ))
    }
    fn remove_on_set_player_name(&self, callback: SetPlayerNameCallbackId) {
        self.imp.remove_on_reducer("set_player_name", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `set_player_name`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_set_player_name {
    /// Set the call-reducer flags for the reducer `set_player_name` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn set_player_name(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_set_player_name for super::SetReducerFlags {
    fn set_player_name(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("set_player_name", flags);
    }
}
