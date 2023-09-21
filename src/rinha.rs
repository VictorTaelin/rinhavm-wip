#[derive(Debug, Clone)]
pub struct File {
  pub name: String,
  pub expr: Term,
}

#[derive(Debug, Clone)]
pub struct Parameter {
  pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
  Rem,
  Eq,
  Neq,
  Lt,
  Gt,
  Lte,
  Gte,
  And,
  Or,
}

#[derive(Debug, Clone)]
pub enum Term {
  Int {
    value: i32,
  },
  Str {
    value: String,
  },
  Call {
    callee: Box<Term>,
    args: Vec<Term>,
  },
  Binary {
    lhs: Box<Term>,
    op: BinaryOp,
    rhs: Box<Term>,
  },
  Function {
    params: Vec<Parameter>,
    value: Box<Term>,
  },
  Let {
    name: Parameter,
    value: Box<Term>,
    next: Box<Term>,
  },
  If {
    cond: Box<Term>,
    then: Box<Term>,
    els_: Box<Term>,
  },
  Print {
    value: Box<Term>,
  },
  First {
    value: Box<Term>,
  },
  Second {
    value: Box<Term>,
  },
  Bool {
    value: bool,
  },
  Tuple {
    first: Box<Term>,
    second: Box<Term>,
  },
  Var {
    text: String,
  },
}

impl std::fmt::Display for BinaryOp {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      BinaryOp::Add => write!(f, "+"),
      BinaryOp::Sub => write!(f, "-"),
      BinaryOp::Mul => write!(f, "*"),
      BinaryOp::Div => write!(f, "/"),
      BinaryOp::Rem => write!(f, "%"),
      BinaryOp::Eq => write!(f, "=="),
      BinaryOp::Neq => write!(f, "!="),
      BinaryOp::Lt => write!(f, "<"),
      BinaryOp::Gt => write!(f, ">"),
      BinaryOp::Lte => write!(f, "<="),
      BinaryOp::Gte => write!(f, ">="),
      BinaryOp::And => write!(f, "&&"),
      BinaryOp::Or => write!(f, "||"),
    }
  }
}

impl BinaryOp {
  pub fn from_str(s: &str) -> Option<Self> {
    match s {
      "Add" => Some(BinaryOp::Add),
      "Sub" => Some(BinaryOp::Sub),
      "Mul" => Some(BinaryOp::Mul),
      "Div" => Some(BinaryOp::Div),
      "Rem" => Some(BinaryOp::Rem),
      "Eq" => Some(BinaryOp::Eq),
      "Neq" => Some(BinaryOp::Neq),
      "Lt" => Some(BinaryOp::Lt),
      "Gt" => Some(BinaryOp::Gt),
      "Lte" => Some(BinaryOp::Lte),
      "Gte" => Some(BinaryOp::Gte),
      "And" => Some(BinaryOp::And),
      "Or" => Some(BinaryOp::Or),
      _ => None,
    }
  }
}

impl Parameter {
  pub fn from_json(json: &serde_json::Value) -> Option<Self> {
    let text = json["text"].as_str()?.to_string();
    Some(Parameter { text })
  }
}

pub fn term_to_string(term: &Term) -> String {
  match term {
    Term::Int { value, .. } => value.to_string(),
    Term::Str { value, .. } => format!("\"{}\"", value),
    Term::Call { callee, args, .. } => {
      let callee_str = term_to_string(callee);
      let args_str = args.iter().map(term_to_string).collect::<Vec<String>>().join(", ");
      format!("{}({})", callee_str, args_str)
    },
    Term::Binary { lhs, op, rhs, .. } => {
      let lhs_str = term_to_string(lhs);
      let rhs_str = term_to_string(rhs);
      format!("{} {} {}", lhs_str, op, rhs_str)
    },
    Term::Function { params, value, .. } => {
      let params_str = params.iter().map(|p| p.text.clone()).collect::<Vec<String>>().join(", ");
      let value_str = term_to_string(value);
      format!("fn ({}) => {{ {} }}", params_str, value_str)
    },
    Term::Let { name, value, next, .. } => {
      let name_str = &name.text;
      let value_str = term_to_string(value);
      let next_str = term_to_string(next);
      format!("let {} = {}; {}", name_str, value_str, next_str)
    },
    Term::If { cond, then, els_, .. } => {
      let cond_str = term_to_string(cond);
      let then_str = term_to_string(then);
      let els_str = term_to_string(els_);
      format!("if ({}) {{ {} }} else {{ {} }}", cond_str, then_str, els_str)
    },
    Term::Print { value, .. } => {
      let value_str = term_to_string(value);
      format!("print({})", value_str)
    },
    Term::First { value, .. } => {
      let value_str = term_to_string(value);
      format!("first({})", value_str)
    },
    Term::Second { value, .. } => {
      let value_str = term_to_string(value);
      format!("second({})", value_str)
    },
    Term::Bool { value, .. } => value.to_string(),
    Term::Tuple { first, second, .. } => {
      let first_str = term_to_string(first);
      let second_str = term_to_string(second);
      format!("({}, {})", first_str, second_str)
    },
    Term::Var { text, .. } => text.clone(),
  }
}

pub fn json_to_file(json: &serde_json::Value) -> Option<File> {
  let name = json["name"].as_str()?.to_string();
  let expr = json_to_term(&json["expression"])?;
  Some(File { name, expr })
}

pub fn json_to_term(json: &serde_json::Value) -> Option<Term> {
  let kind = json["kind"].as_str()?.to_string();

  match kind.as_str() {
    "Int" => {
      let value = json["value"].as_i64()? as i32;
      Some(Term::Int { value })
    },
    "Str" => {
      let value = json["value"].as_str()?.to_string();
      Some(Term::Str { value })
    },
    "Call" => {
      let callee = json_to_term(&json["callee"])?;
      let args = json["arguments"].as_array()?.iter().filter_map(json_to_term).collect();
      Some(Term::Call { callee: Box::new(callee), args })
    },
    "Binary" => {
      let lhs = json_to_term(&json["lhs"])?;
      let op = BinaryOp::from_str(json["op"].as_str()?)?;
      let rhs = json_to_term(&json["rhs"])?;
      Some(Term::Binary { lhs: Box::new(lhs), op, rhs: Box::new(rhs) })
    },
    "Function" => {
      let params = json["parameters"].as_array()?.iter().filter_map(Parameter::from_json).collect();
      let value = json_to_term(&json["value"])?;
      Some(Term::Function { params, value: Box::new(value) })
    },
    "Let" => {
      let name = Parameter::from_json(&json["name"])?;
      let value = json_to_term(&json["value"])?;
      let next = json_to_term(&json["next"])?;
      Some(Term::Let { name, value: Box::new(value), next: Box::new(next) })
    },
    "If" => {
      let cond = json_to_term(&json["condition"])?;
      let then = json_to_term(&json["then"])?;
      let els_ = json_to_term(&json["otherwise"])?;
      Some(Term::If { cond: Box::new(cond), then: Box::new(then), els_: Box::new(els_) })
    },
    "Print" => {
      let value = json_to_term(&json["value"])?;
      Some(Term::Print { value: Box::new(value) })
    },
    "First" => {
      let value = json_to_term(&json["value"])?;
      Some(Term::First { value: Box::new(value) })
    },
    "Second" => {
      let value = json_to_term(&json["value"])?;
      Some(Term::Second { value: Box::new(value) })
    },
    "Bool" => {
      let value = json["value"].as_bool()?;
      Some(Term::Bool { value })
    },
    "Tuple" => {
      let first = json_to_term(&json["first"])?;
      let second = json_to_term(&json["second"])?;
      Some(Term::Tuple { first: Box::new(first), second: Box::new(second) })
    },
    "Var" => {
      let text = json["text"].as_str()?.to_string();
      Some(Term::Var { text })
    },
    _ => {
      None
    },
  }
}

pub fn rinha_term_to_hvm_lang_term(term: &Term) -> hvm_lang::ast::Term {
  match term {
    Term::Int { value, .. } => todo!(),
    Term::Str { .. } => todo!(),
    Term::Call { callee, args, .. } => {
      let mut term = rinha_term_to_hvm_lang_term(callee);
      for arg in args {
        term = hvm_lang::ast::Term::App {
          fun: Box::new(term),
          arg: Box::new(rinha_term_to_hvm_lang_term(arg)),
        };
      }
      term
    },
    Term::Binary { lhs, op, rhs, .. } => todo!(),
    Term::Function { params, value, .. } => {
      let mut term = rinha_term_to_hvm_lang_term(value);
      for param in params.iter().rev() {
        term = hvm_lang::ast::Term::Lam {
          nam: Some(hvm_lang::ast::Name(param.text.clone())),
          bod: Box::new(term),
        };
      }
      term
    },
    Term::Let { name, value, next, .. } => {
      let lam = hvm_lang::ast::Term::Lam {
        nam: Some(hvm_lang::ast::Name(name.text.clone())),
        bod: Box::new(rinha_term_to_hvm_lang_term(next)),
      };
      hvm_lang::ast::Term::App {
        fun: Box::new(lam),
        arg: Box::new(rinha_term_to_hvm_lang_term(value)),
      }
    },
    Term::If { cond, then, els_, .. } => todo!(),
    Term::Print { value, .. } => todo!(),
    Term::First { value, .. } => todo!(),
    Term::Second { value, .. } => todo!(),
    Term::Bool { value, .. } => todo!(),
    Term::Tuple { first, second, .. } => todo!(),
    Term::Var { text, .. } => hvm_lang::ast::Term::Var { nam: hvm_lang::ast::Name(text.clone()) },
  }
}

pub fn hvm_lang_term_to_rinha_term(term: &hvm_lang::ast::Term) -> Term {
  match term {
    hvm_lang::ast::Term::Var { nam } => Term::Var { text: nam.0.clone() },
    hvm_lang::ast::Term::Lam { nam, bod } => {
      let param = Parameter { text: match nam { None => "_".to_string(), Some(x) => x.0.clone() } };
      let value = hvm_lang_term_to_rinha_term(bod);
      Term::Function { params: vec![param], value: Box::new(value) }
    },
    hvm_lang::ast::Term::App { fun, arg } => {
      let callee = hvm_lang_term_to_rinha_term(fun);
      let args = vec![hvm_lang_term_to_rinha_term(arg)];
      Term::Call { callee: Box::new(callee), args }
    },
    _ => Term::Var { text: "TODO".to_string() },
  }
}

pub fn rinha_to_net(term: &Term) -> hvm_core::core::Net {
  let lnet = hvm_lang::to_core::term_to_hvm_core(&rinha_term_to_hvm_lang_term(term).try_into_affine(&std::collections::HashSet::new()).unwrap()).unwrap();
  return hvm_core::lang::lnet_to_net(&lnet, Some(1 << 26));
}

pub fn net_to_rinha(net: &hvm_core::core::Net) -> Term {
  let lnet = hvm_core::lang::readback_lnet(net);
  return hvm_lang_term_to_rinha_term(&hvm_lang::from_core::readback_net(&lnet).unwrap().0);
}
