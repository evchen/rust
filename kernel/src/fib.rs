#[allow(dead_code)]

pub fn fib(n: u32)-> u32{
 if n ==0{
  0
 }else{
  if n==1{
    1
  }else{
    fib(n-1)+fib(n-2)
  }
 }
}

pub fn fub(n: u32) -> u32{
  if n == 0{
    0
  }
  else{
    fub(n-1)+n
  }
}
