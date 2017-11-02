
/// Market stream command.
#[derive(Clone)]
pub enum Command {
    /// Subscription command.
    Subscribe(::core::CurrencyPair),

    /// Unsubscription command.
    Unsubscribe(::core::CurrencyPair),
}

/// Market stream command sender.
pub type CommandSender = ::multiqueue::BroadcastFutSender<Command>;

/// Market stream command receiver.
pub type CommandReceiver = ::multiqueue::BroadcastFutReceiver<Command>;
