use futures::*;
use reqwest;

use std::fmt;
use std::net::{IpAddr, SocketAddr};
use std::ops::Deref;

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
pub struct SocketInfo(pub SocketAddr);

impl SocketInfo {
    pub fn try_create(ip: &str, port: u16) -> Result<Self, error::Error> {
        let ip: IpAddr = ip.parse()?;
        let socket = SocketAddr::new(ip, port);
        Ok(socket.into())
    }
}

impl Deref for SocketInfo {
    type Target = SocketAddr;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<SocketAddr> for SocketInfo {
    fn from(item: SocketAddr) -> Self {
        SocketInfo(item)
    }
}

impl Serialize for SocketInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut rgb = serializer.serialize_struct("SocketAddr", 2)?;
        if self.is_ipv4() {
            rgb.serialize_field("ip_v4", &self.ip().to_string())?;
        } else {
            rgb.serialize_field("ip_v6", &self.ip().to_string())?;
        }
        rgb.serialize_field("port", &self.port())?;
        rgb.end()
    }
}

impl<'de> Deserialize<'de> for SocketInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            IP,
            PORT,
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
                        formatter.write_str("`ip_v4` or `ip_v6` or `port`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "ip_v4" => Ok(Field::IP),
                            "ip_v6" => Ok(Field::IP),
                            "port" => Ok(Field::PORT),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct SocketInfoVisitor;

        impl<'de> Visitor<'de> for SocketInfoVisitor {
            type Value = SocketInfo;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct SocketAddr")
            }

            fn visit_map<V>(self, mut map: V) -> Result<SocketInfo, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut ip: Option<String> = None;
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
                    }
                }
                let ip = ip.ok_or_else(|| de::Error::missing_field("ip_v4 or ip_v6"))?;
                let ip: IpAddr = ip.parse().expect("ip field parse error");
                let port = port.ok_or_else(|| de::Error::missing_field("port"))?;
                let socket = SocketAddr::new(ip, port);
                Ok(socket.into())
            }
        }

        const FIELDS: &'static [&'static str] = &["ip_v4", "ip_v6", "port"];
        deserializer.deserialize_struct("SocketAddr", FIELDS, SocketInfoVisitor)
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
