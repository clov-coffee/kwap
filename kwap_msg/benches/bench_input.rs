use kwap_msg::{alloc::*, no_alloc};
use tinyvec::ArrayVec;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct TestInput {
  pub tkl: u8,
  pub n_opts: usize,
  pub opt_size: usize,
  pub payload_size: usize,
}

impl TestInput {
  pub fn get_bytes(&self) -> Vec<u8> {
    self.get_alloc_message().into()
  }
  pub fn get_alloc_message(&self) -> Message {
    self.into()
  }
  pub fn get_no_alloc_message<const P: usize, const N: usize, const O: usize>(&self) -> no_alloc::Message<P, N, O> {
    self.into()
  }
  pub fn get_coap_lite_packet(&self) -> coap_lite::Packet {
    coap_lite::Packet::from_bytes(&self.get_bytes()).unwrap()
  }
}

impl<'a> Into<Message> for &'a TestInput {
  fn into(self) -> Message {
    let opts: Vec<Opt> = (0..self.n_opts).map(|n| Opt { delta: OptDelta(n as _),
                                                        value: OptValue(core::iter::repeat(1).take(self.opt_size)
                                                                                             .collect()) })
                                         .collect();

    let token = core::iter::repeat(1u8).take(self.tkl as _)
                                       .collect::<tinyvec::ArrayVec<[_; 8]>>();

    Message { id: Id(1),
              ty: Type(0),
              ver: Version(0),
              token: Token(token),
              code: Code { class: 2, detail: 5 },
              opts,
              payload: Payload(core::iter::repeat(1u8).take(self.payload_size).collect()) }
  }
}

impl<'a, const P: usize, const N: usize, const O: usize> Into<no_alloc::Message<P, N, O>> for &'a TestInput {
  fn into(self) -> no_alloc::Message<P, N, O> {
    let opts: ArrayVec<[_; N]> =
      (0..self.n_opts).map(|n| no_alloc::Opt { delta: OptDelta(n as _),
                                               value: no_alloc::OptValue(core::iter::repeat(1).take(self.opt_size)
                                                                                              .collect()) })
                      .collect();

    let token = core::iter::repeat(1u8).take(self.tkl as _)
                                       .collect::<tinyvec::ArrayVec<[_; 8]>>();

    no_alloc::Message { id: Id(1),
                        ty: Type(0),
                        ver: Version(0),
                        token: Token(token),
                        code: Code { class: 2, detail: 5 },
                        opts,
                        payload: no_alloc::Payload(core::iter::repeat(1u8).take(self.payload_size).collect()) }
  }
}