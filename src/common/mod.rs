use futures::*;
use reqwest;

use std::fmt;
use std::net::{IpAddr, SocketAddr};

use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::error;

pub async fn api_access<A: Sized, T: Sized, R: Sized>(
    success_code: reqwest::StatusCode,
    is_404_captable: bool,
    api_call: impl Fn() -> A,
    f: impl Fn(reqwest::Response) -> R,
) -> Result<T, error::Error>
where
    A: Future<Output = Result<reqwest::Response, reqwest::Error>>,
    R: Future<Output = Result<T, error::Error>>,
{
    let res = api_call().await?;
    match res.status() {
        code if code == success_code => f(res).await,
        reqwest::StatusCode::BAD_REQUEST => res
            .json::<error::ErrorResponse>()
            .await
            .map_err(Into::into)
            .and_then(|response: error::ErrorResponse| {
                let message = response
                    .params
                    .errors
                    .iter()
                    .fold("recv message".to_string(), |sum, acc| {
                        format!("{}\n{}", sum, acc.message)
                    });
                Err(error::Error::create_myerror(&message))
            }),
        reqwest::StatusCode::FORBIDDEN => Err(error::Error::create_myerror("recv Forbidden")),
        reqwest::StatusCode::NOT_FOUND if is_404_captable => {
            Err(error::Error::create_myerror("recv Not Found"))
        }
        reqwest::StatusCode::METHOD_NOT_ALLOWED => {
            Err(error::Error::create_myerror("recv Method Not Allowed"))
        }
        reqwest::StatusCode::NOT_ACCEPTABLE => {
            Err(error::Error::create_myerror("recv Not Acceptable"))
        }
        reqwest::StatusCode::REQUEST_TIMEOUT => {
            Err(error::Error::create_myerror("recv RequestTimeout"))
        }
        _ => {
            unreachable!();
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PhantomId(String);

impl SerializableId for PhantomId {
    fn new(_id: Option<String>) -> Option<Self>
    where
        Self: Sized,
    {
        None
    }

    fn as_str(&self) -> &str {
        ""
    }

    fn id(&self) -> (&'static str, String) {
        ("", String::from(""))
    }
}

pub trait SerializableId {
    fn new(id: Option<String>) -> Option<Self>
    where
        Self: Sized;
    fn as_str(&self) -> &str;
    fn id(&self) -> (&'static str, String);
}

pub trait SerializableSocket {
    fn new(id: Option<String>, socket: SocketAddr) -> Self;
    fn try_create(id: Option<String>, ip: &str, port: u16) -> Result<Self, error::Error>
    where
        Self: Sized;
    fn id(&self) -> (&'static str, String);
    fn ip(&self) -> IpAddr;
    fn port(&self) -> u16;
}

#[derive(Clone, Debug, PartialEq)]
pub struct SocketInfo<T: SerializableId> {
    id: Option<T>,
    socket: SocketAddr,
}

impl<T: SerializableId> SerializableSocket for SocketInfo<T> {
    fn new(id: Option<String>, socket: SocketAddr) -> Self {
        Self {
            id: T::new(id),
            socket: socket,
        }
    }

    fn try_create(id: Option<String>, ip: &str, port: u16) -> Result<Self, error::Error> {
        let ip: IpAddr = ip.parse()?;
        let socket = SocketAddr::new(ip, port);
        Ok(Self::new(id, socket))
    }

    fn id(&self) -> (&'static str, String) {
        match self.id {
            Some(ref id) => id.id(),
            None => ("", String::from("")),
        }
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
        let (key, id) = self.id();
        let mut serial;
        if key.len() == 0 {
            serial = serializer.serialize_struct("SocketAddr", 2)?
        } else {
            serial = serializer.serialize_struct("SocketAddr", 3)?;
            serial.serialize_field(key, &id)?;
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
        };

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
                            println!("{:?}", port);
                        }
                        Field::IP => {
                            if ip.is_some() {
                                return Err(de::Error::duplicate_field("ip_v4"));
                            }
                            ip = Some(map.next_value()?);
                            println!("{:?}", ip);
                        }
                        Field::ID => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                            println!("{:?}", id);
                        }
                    }
                }
                let ip = ip.ok_or_else(|| de::Error::missing_field("ip_v4 or ip_v6"))?;
                let ip: IpAddr = ip.parse().expect("ip field parse error");
                let port = port.ok_or_else(|| de::Error::missing_field("port"))?;
                let socket = SocketAddr::new(ip, port);
                Ok(SocketInfo::<T>::new(id, socket))
            }
        }

        const FIELDS: &'static [&'static str] = &["ip_v4", "ip_v6", "port", "*_id"];
        deserializer.deserialize_struct("SocketAddr", FIELDS, SocketInfoVisitor(PhantomData))
    }
}

#[cfg(test)]
mod test_socket_info {
    use std::net::SocketAddr;

    use super::SocketInfo;

    #[test]
    fn v4() {
        let original_addr: SocketAddr = "127.0.0.1:8000".parse().unwrap();
        let socket_info: SocketInfo = original_addr.into();
        let json = serde_json::to_string(&socket_info).expect("serialize failed");
        let decoded_socket_info: SocketInfo =
            serde_json::from_str(&json).expect("deserialize failed");
        assert_eq!(socket_info, decoded_socket_info);
    }

    #[test]
    fn v6() {
        let original_addr: SocketAddr = "[2001:DB8:0:0:8:800:200C:417A]:8000".parse().unwrap();
        let socket_info: SocketInfo = original_addr.into();
        let json = serde_json::to_string(&socket_info).expect("serialize failed");
        let decoded_socket_info: SocketInfo =
            serde_json::from_str(&json).expect("deserialize failed");
        assert_eq!(socket_info, decoded_socket_info);
    }
}
