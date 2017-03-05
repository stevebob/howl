use game::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum MessageType {
    Empty,
    Welcome,
    Intro,
    Title,
    PressAnyKey,
    YouDied,
    Action(ActionMessageType),
    Name(NameMessageType),
    YouRemember(Option<NameMessageType>),
    Unseen,
    Description(DescriptionMessageType),
    NameDescription(NameMessageType),
    NoDescription,
    Menu(MenuMessageType),
    ChooseDirection,
    EmptyWeaponSlotMessage,
    Front,
    Rear,
    Left,
    Right,
    EmptyWeaponSlot,
    SurvivorCamp,
    ShopTitle(usize),
    ShopTitleInsufficientFunds(usize),
    ShopTitleInventoryFull(usize),
    Inventory {
        size: usize,
        capacity: usize,
    },
    NameAndDescription(NameMessageType, DescriptionMessageType),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum NameMessageType {
    Pistol,
    Shotgun,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ActionMessageType {
    PlayerOpenDoor,
    PlayerCloseDoor,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum DescriptionMessageType {
    Pistol,
    Shotgun,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum MenuMessageType {
    NewGame,
    Continue,
    Quit,
    SaveAndQuit,
    Controls,
    Control(InputEvent, Control),
    UnboundControl(Control),
    ControlBinding(Control),
    NextDelivery,
    Shop,
    Garage,
    Inventory,
    Name(NameMessageType),
    ShopItem(NameMessageType, usize),
    Back,
    Remove,
}
