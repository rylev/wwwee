use mio::net::{TcpListener, TcpStream};
use mio;
use std::net::SocketAddr;
use std;
use io;

pub const CONNECTION_COUNT : usize = 100;
const SERVER_TOKEN : mio::Token = mio::Token(0);

fn initialize_connections<T>() -> [Option<Connection<T>>; CONNECTION_COUNT] {
  let mut connections : [Option<Connection<T>>; CONNECTION_COUNT] = 
    unsafe { std::mem::uninitialized() };
  
  for conn in connections.iter_mut() {
    *conn = None;
  }
  connections
}

struct Connection<T> {
  pub handler: T,
  pub token_source: io::AsyncTokenSource
}

pub struct Server<T, F> {
  connections: [Option<Connection<T>>; CONNECTION_COUNT],
  poll: mio::Poll,
  server_socket: TcpListener,
  handler_creator: F,
}

impl<T, F> Server<T, F>
  where T: io::Handler<()>,
        F: Fn(io::Registered<TcpStream>) -> T
{
  pub fn new(addr: SocketAddr, handler_creator: F)
    -> std::io::Result<Server<T, F>>
  {
    let server_socket = TcpListener::bind(&addr)?;
    let poll = mio::Poll::new()?;
    // Start listening for incoming connections
    poll.register(&server_socket, SERVER_TOKEN, mio::Ready::readable() | mio::Ready::writable(),
      mio::PollOpt::edge())?;

    let connections = initialize_connections();

    Ok(Server {
      connections,
      poll,
      server_socket,
      handler_creator
    })
  }

  pub fn start(&mut self) -> std::io::Result<()> {
    let mut events = mio::Events::with_capacity(self.connections.len());
    loop {
      self.poll.poll(&mut events, None)?;

      self.process_events(&events);
    }
  }

  fn process_events(&mut self, events: &mio::Events) {
    for event in events.iter() {
        if event.token() == SERVER_TOKEN {
          if let Ok((socket, _)) = self.server_socket.accept() {
            self.register_connection(socket);
          }
        }
        else {
          if let Some(conn_idx) = self.handle_event(&event) {
            self.connections[conn_idx] = None;
          }
        }
      }
  }

  fn handle_event(&mut self, event: &mio::Event) -> Option<usize> {
    let token = io::Token::from_mio_token(event.token());
    let conn_id = token.connection_id();
    let conn_idx = conn_id.as_index();

    if let Some(ref mut connection) = self.connections[conn_idx] {

      let mut ctx = io::Context::new(&self.poll, conn_id, &mut connection.token_source);
      let r = event.readiness();
      // TODO: deregister socket from poll and dont call writable when readable already closed the connection
      if r.is_readable() {
        let io_event = io::Event::new(token.async_token(), io::EventKind::Readable);
        if let Some(_) = connection.handler.handle_event(&io_event, &mut ctx) {
          return Some(conn_idx);
        }
      }
      if r.is_writable() {
        let io_event = io::Event::new(token.async_token(), io::EventKind::Writable);
        if let Some(_) = connection.handler.handle_event(&io_event, &mut ctx) {
          return Some(conn_idx);
        }
      }
    }
    None
  }

  fn register_connection(&mut self, socket: TcpStream) {
    if let Some(conn_idx) = self.connections
      .iter()
      .position(|conn| conn.is_none())
    {
      let conn_id = io::ConnectionId::from_index(conn_idx);
      match self.create_and_register_connection(conn_id, socket) {
        Ok(connection) => {
          self.connections[conn_idx] = Some(connection);
          println!("added new connection with id {:?}", conn_id);
        },
        Err(err) => {
          println!("error while trying to register handler for connection {:?}: {:?}", conn_id, err);
        }
      }
    }
    else {
      println!("too many connections, dropping this one");
    }
  }

  fn create_and_register_connection(&self, conn_id: io::ConnectionId, socket: TcpStream) -> std::io::Result<Connection<T>> {
    let socket_async_token = io::AsyncToken::default();
    let token = io::Token::from_parts(conn_id, socket_async_token);
    let registered_socket = io::Registered::register(socket, token, &self.poll)?;
    let handler = (self.handler_creator)(registered_socket);
    Ok(Connection {
      handler,
      token_source: io::AsyncTokenSource::starting_from(socket_async_token)
    })
  }
}
