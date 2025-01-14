use serde::{Serialize, Deserialize};

pub trait Component<'de> {
    type Properties: Serialize + Deserialize<'de>;
    
    fn init(props: Self::Properties, pauser: &mut Pauser) -> String;
}

pub struct Pauser {
    r: u32,
    n: u32,
    ctx: Vec<String>,
    objs: Vec<String>,
    subs: Vec<Vec<String>>,
}

impl Pauser {
    pub fn new() -> Self {
        Self {r: 0, n: 0, ctx: vec![], objs: vec![], subs: vec![]}
    }
    pub fn id(&self) -> u32 {
        self.n
    }
    pub fn add(&mut self) -> u32 {
        self.n += 1;
        self.ctx.push(format!("\"{}\":{{\"R\":\"{}\"}}", self.n, self.r));
        self.n
    }
    pub fn push_obj(&mut self, v: String) {
        self.objs.push(v);
    }
    pub fn push_subs(&mut self, v: Vec<String>) {
        self.subs.push(v);
    }
    pub fn to_string(&self) -> String {
		let mut s = String::from("<script type=\"module\" src=\"/static/main.js\"></script><script type=\"app/json\">");
        s.push_str("{\"ctx\":{");
        for c in &self.ctx {
            s.push_str(&format!("{}", c));
        }
        s.push_str("},\"objs\":[");
        let mut b = false;
        for o in &self.objs {
            if b {
                s.push_str(&format!(",{}", o));
            } else {
                b = true;
                s.push_str(&o);
            }
        }
        s.push_str("],\"subs\":[");
        b = false;
        for sub in &self.subs {
            s.push('[');
            for sb in sub {
                if b {
                    s.push_str(&format!(",{}", sb));
                } else {
                    b = true;
                    s.push_str(&sb);
                }
            }
            s.push(']');
        }
        s.push_str("]}</script>");
        s
    }
}

#[macro_export]
macro_rules! components {
    ($($name:ident,)*) => {
        $(pub mod $name {
            include!(concat!(".parsed", anansi::main_separator!(), stringify!($name), ".in"));
        })*
    }
}
