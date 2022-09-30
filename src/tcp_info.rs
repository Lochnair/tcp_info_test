use std::{fmt, mem};
use std::error::Error;
use std::fmt::Display;
use std::mem::MaybeUninit;
use std::os::unix::io::RawFd;

use libc::{__errno_location, c_void, socklen_t, SOL_TCP, TCP_INFO};
use modular_bitfield::prelude::*;

#[derive(Default, Debug)]
pub struct GetSockOptError {
    pub errno: i32,
}

impl Error for GetSockOptError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for GetSockOptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Errno: {}",
            self.errno
        )
    }
}

#[bitfield]
#[derive(Clone, Copy, Debug)]
pub struct TcpInfo {
    pub tcpi_state: u8,
    pub tcpi_ca_state: u8,
    pub tcpi_retransmits: u8,
    pub tcpi_probes: u8,
    pub tcpi_backoff: u8,
    pub tcpi_options: u8,
    pub tcpi_snd_wscale: B4,
    pub tcpi_rcv_wscale: B4,
    pub tcpi_delivery_rate_app_limited: bool,

    pub tcpi_rto: u32,
    pub tcpi_ato: u32,
    pub tcpi_snd_mss: u32,
    pub tcpi_rcv_mss: u32,

    pub tcpi_unacked: u32,
    pub tcpi_sacked: u32,
    pub tcpi_lost: u32,
    pub tcpi_retrans: u32,
    pub tcpi_fackets: u32,

    /* Times. */
    pub tcpi_last_data_sent: u32,
    pub tcpi_last_ack_sent: u32,
    pub tcpi_last_data_recv: u32,
    pub tcpi_last_ack_recv: u32,

    /* Metrics. */
    pub tcpi_pmtu: u32,
    pub tcpi_rcv_ssthresh: u32,
    pub tcpi_rtt: u32,
    pub tcpi_rttvar: u32,
    pub tcpi_snd_ssthresh: u32,
    pub tcpi_snd_cwnd: u32,
    pub tcpi_advmss: u32,
    pub tcpi_reordering: u32,

    pub tcpi_rcv_rtt: u32,
    pub tcpi_rcv_space: u32,

    pub tcpi_total_retrans: u32,

    pub tcpi_pacing_rate: u64,
    pub tcpi_max_pacing_rate: u64,
    pub tcpi_bytes_acked: u64,
    pub tcpi_bytes_received: u64,
    pub tcpi_segs_out: u32,
    pub tcpi_segs_in: u32,

    pub tcpi_notsent_bytes: u32,
    pub tcpi_min_rtt: u32,
    pub tcpi_data_segs_in: u32,
    pub tcpi_data_segs_out: u32,

    pub tcpi_delivery_rate: u64,

    pub tcpi_busy_time: u64,
    pub tcpi_rwnd_limited: u64,
    pub tcpi_sndbuf_limited: u64,

    pub tcpi_delivered: u32,
    pub tcpi_delivered_ce: u32,

    pub tcpi_bytes_sent: u64,
    pub tcpi_bytes_retrans: u64,
    pub tcpi_dsack_dups: u32,
    pub tcpi_reord_seen: u32,

    // modular-bitfield requires the struct to be N * byte
    // add some padding at the end to satisfy the requirement
    #[allow(unused)]
    padding: B7,
}

fn get_tcp_info_len() -> *mut socklen_t {
    let mut len: socklen_t = mem::size_of::<TcpInfo>() as socklen_t;
    &mut len
}

pub fn get_tcp_info(fd: RawFd) -> Result<TcpInfo, GetSockOptError> {
    let mut val: MaybeUninit<TcpInfo> = MaybeUninit::uninit();

    unsafe {
        let res = libc::getsockopt(fd, SOL_TCP, TCP_INFO, val.as_mut_ptr() as *mut c_void, get_tcp_info_len());

        if res == -1 {
            let errno = __errno_location().read();
            return Err(GetSockOptError { errno });
        }

        Ok(val.assume_init())
    }
}