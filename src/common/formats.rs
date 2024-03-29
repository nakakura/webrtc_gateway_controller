use std::fmt;
use std::net::{IpAddr, SocketAddr};

use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};

use crate::error;

/// This trait is for serializing ID to JSON.
///
/// It also has some getter functions.
pub trait SerializableId: Clone {
    /// Try to create an instance of SerializableId with String parameter.
    ///
    /// It returns None, if id is None.
    fn try_create(id: impl Into<String>) -> Result<Self, error::Error>
    where
        Self: Sized;
    /// Get internal str of the Id.
    fn as_str(&self) -> &str;
    /// Get internal String of the Id
    fn id(&self) -> String;
    /// Field name of Json. If it returns `"hoge_id"`, json will be `{"hoge_id": self.id()}`.
    fn key(&self) -> &'static str;
}

/// This trait is for serializing SocketInfo to JSON.
///
/// It also has some getter functions.
pub trait SerializableSocket<T> {
    /// Create an instance of SerializableSocket.
    ///
    /// # Failures
    /// It returns error, if the ip and port is not valid for SocketAddr.
    fn try_create(id: Option<String>, ip: &str, port: u16) -> Result<Self, error::Error>
    where
        Self: Sized;
    /// Returns id field.
    fn get_id(&self) -> Option<T>;
    /// Field name of Json.
    fn key(&self) -> &'static str;
    /// Returns SocketAddr of the socket.
    fn addr(&self) -> &SocketAddr;
    /// Returns IpAddr of the socket.
    fn ip(&self) -> IpAddr;
    /// Returns port number of the socket.
    fn port(&self) -> u16;
}

/// There are several field which has some kind of id and SocketAddr.
///
/// This struct covers all of them.
#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct SocketInfo<T: SerializableId> {
    id: Option<T>,
    socket: SocketAddr,
}

impl<T: SerializableId> SerializableSocket<T> for SocketInfo<T> {
    fn try_create(id: Option<String>, ip: &str, port: u16) -> Result<Self, error::Error> {
        let ip: IpAddr = ip.parse()?;
        let socket = SocketAddr::new(ip, port);
        match id {
            Some(id) => Ok(Self {
                id: Some(T::try_create(id)?),
                socket: socket,
            }),
            None => Ok(Self {
                id: None,
                socket: socket,
            }),
        }
    }

    fn get_id(&self) -> Option<T> {
        self.id.clone()
    }

    fn key(&self) -> &'static str {
        match self.id {
            Some(ref id) => id.key(),
            None => "",
        }
    }

    fn addr(&self) -> &SocketAddr {
        &self.socket
    }

    fn ip(&self) -> IpAddr {
        self.socket.ip()
    }

    fn port(&self) -> u16 {
        self.socket.port()
    }
}

impl<T: SerializableId> Serialize for SocketInfo<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let key = self.key();
        let id = self.get_id();
        let mut serial;
        if key.len() == 0 {
            serial = serializer.serialize_struct("SocketAddr", 2)?
        } else {
            serial = serializer.serialize_struct("SocketAddr", 3)?;
            serial.serialize_field(key, &(id.expect("no id")).id())?;
        };

        let ip = self.ip();
        if ip.is_ipv4() {
            serial.serialize_field("ip_v4", &ip.to_string())?;
        } else {
            serial.serialize_field("ip_v6", &ip.to_string())?;
        }
        serial.serialize_field("port", &self.port())?;
        serial.end()
    }
}

impl<'de, X: SerializableId> Deserialize<'de> for SocketInfo<X> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::marker::PhantomData;
        enum Field {
            IP,
            PORT,
            ID,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`ip_v4` or `ip_v6` or `port` or `*_id`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "ip_v4" => Ok(Field::IP),
                            "ip_v6" => Ok(Field::IP),
                            "port" => Ok(Field::PORT),
                            id if id.ends_with("_id") => Ok(Field::ID),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct SocketInfoVisitor<T>(PhantomData<T>);

        impl<'de, T: SerializableId> Visitor<'de> for SocketInfoVisitor<T> {
            type Value = SocketInfo<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct SocketAddr")
            }

            fn visit_map<V>(self, mut map: V) -> Result<SocketInfo<T>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut ip: Option<String> = None;
                let mut id: Option<String> = None;
                let mut port: Option<u16> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::PORT => {
                            if port.is_some() {
                                return Err(de::Error::duplicate_field("port"));
                            }
                            port = Some(map.next_value()?);
                        }
                        Field::IP => {
                            if ip.is_some() {
                                return Err(de::Error::duplicate_field("ip_v4"));
                            }
                            ip = Some(map.next_value()?);
                        }
                        Field::ID => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                    }
                }
                let ip = ip.ok_or_else(|| de::Error::missing_field("ip_v4 or ip_v6"))?;
                let port = port.ok_or_else(|| de::Error::missing_field("port"))?;
                let socket_info = SocketInfo::<T>::try_create(id, &ip, port);
                if let Err(_err) = socket_info {
                    use serde::de::Error;
                    return Err(V::Error::custom(format!("fail to deserialize socket")));
                }

                Ok(socket_info.unwrap())
            }
        }

        const FIELDS: &'static [&'static str] = &["ip_v4", "ip_v6", "port", "*_id"];
        deserializer.deserialize_struct("SocketAddr", FIELDS, SocketInfoVisitor(PhantomData))
    }
}

/// It's just a dummy Id data returning None.
///
/// There are many similar structs holding SocketAddr and a kind of ID.
/// PhantomId is for a struct which doesn't have id field.
/// It will be set as a generics parameter of `SocketInfo`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PhantomId(String);

impl SerializableId for PhantomId {
    fn try_create(id: impl Into<String>) -> Result<Self, error::Error>
    where
        Self: Sized,
    {
        Ok(PhantomId(id.into()))
    }

    fn as_str(&self) -> &str {
        ""
    }

    fn id(&self) -> String {
        String::from("")
    }

    fn key(&self) -> &'static str {
        ""
    }
}

#[cfg(test)]
mod test_socket_info {
    use super::*;

    #[test]
    fn v4() {
        let socket_info = SocketInfo::<PhantomId>::try_create(None, "127.0.0.1", 8000).unwrap();
        let json = serde_json::to_string(&socket_info).expect("serialize failed");
        let decoded_socket_info: SocketInfo<PhantomId> =
            serde_json::from_str(&json).expect("deserialize failed");
        assert_eq!(socket_info, decoded_socket_info);
    }

    #[test]
    fn v6() {
        let socket_info =
            SocketInfo::<PhantomId>::try_create(None, "2001:DB8:0:0:8:800:200C:417A", 8000)
                .unwrap();
        let json = serde_json::to_string(&socket_info).expect("serialize failed");
        let decoded_socket_info: SocketInfo<PhantomId> =
            serde_json::from_str(&json).expect("deserialize failed");
        assert_eq!(socket_info, decoded_socket_info);
    }
}
