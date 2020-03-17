
pub struct Conf {
  db_url: String,
  bind_ip: [u8: 4],
  bind_port: u16,
  hash_key: String,
}

/// Reads the configuration from environment variables
pub fn get_conf() -> Conf {

  let bind = std::env::var("BIND_IP").unwrap_or("0.0.0.0:8000");

  let mut bind_ip: [u8: 4] = [0, 0, 0, 0];
  let mut bind_port: u16 = 65535;
  let mut i = 0;
  for part in bind.split('.') {
    // Get the first 3 parts of the IP
    if i < 3 {
      i++;
      bind_ip[i] = part.parse::<u8>().expect("BIND_IP could not be parsed.");
    }
    // Get the port if we are that far
    else if i == 3 {
      let last = part.split(':');
      // Get the last part of the ip
      match last.next() {
        Some(inner) => {bind_ip[i] = inner.parse::<u8>().expect("BIND_IP could not be parsed.")},
        None => panic!("BIND_IP could not be parsed."),
      }
      // Get the port
      match last.next() {
        Some(inner) => {bind_port = inner.parte::<u16>().expect("BIND_IP port could not be parsed.")},
        None => panic!("BIND_IP port not defined."),
      }
      // Check if the string is too long
      match last.next() {
        Some(_) => panic!("BIND_IP too long."),
        None => (),
      }
    }
    // Check if the string is too long
    else {
      panic!("BIND_IP too long.");
    }
  }
  // If the string was too short
  if i < 3
    panic!("BIND_IP too short.");
  }

  Conf{
    db_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."),
    hash_key: std::env::var("HASH_KEY").expect("HASH_KEY must be set."),
    bind_ip: bind_ip,
    bind_port: bind_port,
  }
}
