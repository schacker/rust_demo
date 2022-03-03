use regex::{Regex};
use log::{LevelFilter, info, warn, trace};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

pub fn testlog() {
    // log4rs::init_file("../log4rs.yml", Default::default()).unwrap();

    let stdout = ConsoleAppender::builder().build();

    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("log/requests.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder().build("app::backend::db", LevelFilter::Info))
        .logger(Logger::builder()
            .appender("requests")
            .additive(false)
            .build("app::requests", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(LevelFilter::Warn))
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();
    
    info!("ceshi info");
    warn!("ceshi info");
    trace!("ceshi info");
    

    // use handle to change logger configuration at runtime
}

pub fn is_app_webview(user_agent: &str, check_type: &str) -> bool {
  println!("{}", check_type);
  match check_type {
    "link" => {
      println!("命中link");
      return is_link(user_agent)
    },
    "lianjia" => {
      println!("命中lianjia");
      return is_lianjia(user_agent)
    },
    "deyou" => return is_deyou(user_agent),
    "baichuan" => return is_baichuan(user_agent),
    "atom" => return is_atom(user_agent),
    "linkxinfang" => return is_link_xinfang(user_agent),
    "vrstudio" => return is_vr_studio(user_agent),
    _ => return is_link(user_agent) || 
      is_link(user_agent) || 
      is_deyou(user_agent) || 
      is_baichuan(user_agent) || 
      is_atom(user_agent) || 
      is_link_xinfang(user_agent) ||
      is_vr_studio(user_agent)
  }
}

fn is_link(user_agent: &str) -> bool {
  let r = Regex::new(r"(?i)Lianjia/(?i)Home(?i)Link/([0-9a-z\.\-]+)").unwrap();
  return r.is_match(user_agent)
}
fn is_vr_studio(user_agent: &str) -> bool {
  let r = Regex::new(r"(?i)V(?i)R(?i)Studio\s*([0-9a-z\.\-]+)").unwrap();
  return r.is_match(user_agent)
}
fn is_link_xinfang(user_agent: &str) -> bool {
  let r = Regex::new(r"(?i)Home(?i)Link/([0-9a-z\.\-]+)").unwrap();
  return r.is_match(user_agent)
}
fn is_deyou(user_agent: &str) -> bool {
  let r = Regex::new(r"(?i)Lianjia/(?i)Alliance/?([0-9a-z\.\-]+)").unwrap();
  let r1 = Regex::new(r"(?i)Lianjia(?i)Alliance/?([0-9a-z\.\-]+)").unwrap();
  return r.is_match(user_agent) || r1.is_match(user_agent)
}
fn is_baichuan(user_agent: &str) -> bool {
  let r = Regex::new(r"(?i)Lianjia/(?i)lianjiabaichuan/([0-9a-z\.\-]+)").unwrap();
  return r.is_match(user_agent)
}
fn is_beike(user_agent: &str) -> bool {
  let r = Regex::new(r"(?i)lianjiabeike/([0-9a-z\.\-]+)").unwrap();
  return r.is_match(user_agent)
}
fn is_atom(user_agent: &str) -> bool {
  let r = Regex::new(r"(?i)lianjiaatom/([0-9a-z\.\-]+)").unwrap();
  return r.is_match(user_agent)
}
fn is_lianjia(user_agent: &str) -> bool {
  let r = Regex::new(r"(?i)Lianjia").unwrap();
  println!("{}", r.is_match(user_agent));
  if r.is_match(user_agent) {
    return !(
      is_deyou(user_agent) || 
      is_baichuan(user_agent) || 
      is_beike(user_agent) ||
      is_link(user_agent) ||
      is_atom(user_agent)
    )
  }
  return false
}

pub fn facilities(n: i32) -> i32 {
  if n <= 0 {
    return 1;
  }
  return n * facilities(n - 1);
}

pub fn append_search(uri: &str, search: &str) -> String {

  let mut _uri: Option<&str> = Some(uri);
  if !uri.is_empty() && (uri.ends_with("?") || uri.ends_with("&")) {

    _uri = uri.get(0..(uri.len()-1));
  }

  let s_first = search.starts_with("?") || search.starts_with("&");

  let mut _search: Option<&str> = Some(search);
  if s_first {
    _search = search.get(1..(search.len()));
  }

  // let mut m_uri: &str;
  match _search {
    Some(s) => {
      if s.len() == 0 { 
        match _uri {
          Some(_s) => {
            return format!("{}", _s);
          },
          None => {
            
          }
        }
      }

    },
    None => {
            
    }
  };

  let spliter = match uri.contains("?") {
    true => "&",
    false => "?"
  };
  // match _uri {
  //   Some(s) => {
  //     m_uri = s
  //   },
  //   None => {
  //     m_uri = ""
  //   }
  // };
    // println!("{}, {}, {}", _uri.unwrap_or_default(), spliter, _search.unwrap_or_default());

  return format!("{}{}{}", _uri.unwrap_or_default(), spliter, _search.unwrap_or_default())
}
