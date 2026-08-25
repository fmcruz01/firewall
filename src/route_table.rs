use libc::{
    AF_INET, AF_NETLINK, NETLINK_ROUTE, NLM_F_DUMP, NLM_F_REQUEST, NLMSG_DONE, RT_TABLE_MAIN,
    RTM_GETROUTE, SOCK_RAW, bind, close, getpid, ifinfomsg, nlmsghdr, recv, send, sockaddr,
    sockaddr_nl, socket, socklen_t,
};
use std::{
    mem,
    os::{fd::RawFd, raw::c_void},
};

use crate::models::{DiscoverError, RtMsg};
use anyhow::{Context, Result};

pub fn get_route_table() -> Result<()> {
    unsafe {
        let sockfd_nl: RawFd = open_nl_socket().context("failed to open route table socket")?;
        bind_nl_socket(sockfd_nl).context("failed to bind route table socket")?;
        let (rtmsg, nlh) = build_rtm_getroute();
        send_rtmsg(sockfd_nl, rtmsg, nlh).context("failed to send message rtmessage")?;
        let msg = recv_rtmsg(sockfd_nl).context("failed to receive response from netlink")?;
        for e in msg {
            println!("{}", e.nlmsg_type as u16);
        }
        close(sockfd_nl);
    }
    Ok(())
}

fn recv_rtmsg(fd: RawFd) -> Result<Vec<nlmsghdr>, DiscoverError> {
    let mut response = Vec::new();
    let mut buf = [0u8; 8192];
    let mut offset = 0;
    let received = unsafe { recv(fd, buf.as_mut_ptr() as *mut c_void, buf.len(), 0) };
    if received < 0 {
        unsafe {
            close(fd);
        }
        return Err(DiscoverError::SocketError {
            source: std::io::Error::last_os_error(),
        });
    }

    while offset + mem::size_of::<nlmsghdr>() <= buf.len() {
        let hdr = unsafe { &*(buf[offset..].as_ptr() as *const nlmsghdr) };
        if hdr.nlmsg_type == NLMSG_DONE as u16 {
            break;
        }

        let msg_len = hdr.nlmsg_len as usize;
        if msg_len < mem::size_of::<nlmsghdr>() || offset + msg_len > buf.len() {
            break;
        }

        response.push(*hdr);
        offset += (msg_len + 3) & !3;
    }
    Ok(response)
}

fn send_rtmsg(fd: RawFd, rtmsg: RtMsg, nlh: nlmsghdr) -> Result<(), DiscoverError> {
    unsafe {
        let mut buf = [0u8; size_of::<nlmsghdr>() + size_of::<RtMsg>()];
        std::ptr::copy_nonoverlapping(
            &nlh as *const _ as *const u8,
            buf.as_mut_ptr(),
            size_of::<nlmsghdr>(),
        );
        std::ptr::copy_nonoverlapping(
            &rtmsg as *const _ as *const u8,
            buf.as_mut_ptr().add(mem::size_of::<nlmsghdr>()),
            size_of::<RtMsg>(),
        );
        let sent = send(fd, buf.as_ptr() as *const c_void, buf.len(), 0);
        if sent < 0 {
            close(fd);
            return Err(DiscoverError::SocketError {
                source: std::io::Error::last_os_error(),
            });
        }
        Ok(())
    }
}

fn build_rtm_getroute() -> (RtMsg, nlmsghdr) {
    let route_msg = RtMsg {
        rtm_family: AF_INET as u8,
        rtm_dst_len: 0,
        rtm_src_len: 0,
        rtm_tos: 0,
        rtm_table: RT_TABLE_MAIN as u8,
        rtm_protocol: 0,
        rtm_scope: 0,
        rtm_type: 0,
        rtm_flags: 0,
    };
    let mut nlh: nlmsghdr = unsafe { mem::zeroed() };
    nlh.nlmsg_len = (size_of::<nlmsghdr>() + size_of::<RtMsg>()) as u32;
    nlh.nlmsg_type = RTM_GETROUTE;
    nlh.nlmsg_flags = (NLM_F_REQUEST | NLM_F_DUMP) as u16;
    nlh.nlmsg_seq = 1;

    (route_msg, nlh)
}

fn open_nl_socket() -> Result<RawFd, DiscoverError> {
    let sockfd_nl: RawFd = unsafe { socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE) };
    if sockfd_nl < 0 {
        unsafe {
            close(sockfd_nl);
        }
        return Err(DiscoverError::SocketError {
            source: std::io::Error::last_os_error(),
        });
    }

    Ok(sockfd_nl)
}

fn bind_nl_socket(sockfd_nl: RawFd) -> Result<(), DiscoverError> {
    unsafe {
        let mut saddr: sockaddr_nl = mem::zeroed();
        saddr.nl_pid = getpid() as u32;
        saddr.nl_family = AF_NETLINK as u16;
        saddr.nl_groups = 0;

        if bind(
            sockfd_nl,
            &saddr as *const _ as *const sockaddr,
            mem::size_of::<sockaddr_nl>() as socklen_t,
        ) < 0
        {
            close(sockfd_nl);
            return Err(DiscoverError::SocketError {
                source: std::io::Error::last_os_error(),
            });
        }
        Ok(())
    }
}
