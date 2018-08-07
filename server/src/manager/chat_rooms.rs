use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub struct ChatRoom {
    pub name: String,
    pub in_room: HashSet<String>
}

#[derive(Debug)]
pub struct ChatRooms {
    pub rooms: HashMap<String, ChatRoom>
}

pub const DEFAULT_ROOM: &'static str = "Lobby";

impl ChatRooms {
    pub fn new() -> ChatRooms {
        let mut new_room = ChatRooms { rooms: HashMap::new() };
        new_room.rooms.insert(
            self::DEFAULT_ROOM.to_string(),
            ChatRoom { name: self::DEFAULT_ROOM.to_string(), in_room: HashSet::new() }
            );
        new_room
    }

    pub fn get_room_mut(&mut self, room_name: &str) -> Option<&mut ChatRoom> {
        return self.rooms.get_mut(room_name)
    }

    pub fn get_room(&self, room_name: &str) -> Option<&ChatRoom> {
        return self.rooms.get(room_name)
    }

    pub fn is_user_in_room(&self, client_name: &str, room_name: &str) -> bool {
        self.get_room(room_name).and_then(|room| room.in_room.get(client_name)).is_some()
    }

    pub fn insert_client_into_room(&mut self, client_name: &str, room_name: &str) -> Result<(),()> {
        return if self.is_user_in_room(client_name, room_name) {
            Err(())
        } else {
            if let Some(room) = self.get_room_mut(room_name) {
                return if room.in_room.insert(client_name.to_string()) { Ok(()) } else { Err(()) }
            } else {
                Err(())
            }
        }
    }

    pub fn create_room(&mut self, room_name: &str) -> Result<(), ()> {
        if let None = self.get_room(room_name) {
            self.rooms.insert(
                room_name.to_string(),
                ChatRoom { name: room_name.to_string(), in_room: HashSet::new() }
            );
            return Ok(())
        } else {
            return Err(())
        }
    }

    pub fn remove_client_from_room(&mut self, client_name: &str) -> Result<(),()> {
        let ref room_name = self.get_current_room_name(client_name).unwrap();

        return if self.is_user_in_room(client_name, room_name) {
            return if self.get_room_mut(room_name).unwrap().in_room.remove(client_name) {
                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn get_current_room_name(&self, client_name: &str) -> Result<String,()> {
        let client_room: Vec<String> = self.rooms
            .values()
            .filter(|room| (**room).in_room.contains(client_name))
            .map(|room| room.name.to_string())
            .collect();
        if let Some(room) = client_room.first() {
            Ok(room.to_string())
        } else {
            Err(())
        }
    }

    pub fn clients_room_pariticipants(&self, client_name: &str) -> Vec<String> {
        let room = self.get_current_room_name(client_name).unwrap();

        self.get_room(&room)
            .unwrap()
            .in_room
            .iter()
            .map(|name| name.to_string())
            .filter(|name| name != client_name)
            .collect()
    }
}

