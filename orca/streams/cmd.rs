
use ::currency::Pair;

/// Market stream command.
#[derive(Clone)]
pub enum Command {
    /// Subscription command.
    Subscribe(Pair),

    /// Unsubscription command.
    Unsubscribe(Pair),
}

/// Market stream command sender.
pub type CmdSender = ::multiqueue::BroadcastFutSender<Command>;

/// Market stream command receiver.
pub type CmdReceiver = ::multiqueue::BroadcastFutReceiver<Command>;
