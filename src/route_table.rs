use libc::{
    AF_INET, AF_INET6, AF_NETLINK, IFNAMSIZ, NETLINK_ROUTE, NLM_F_DUMP, NLM_F_REQUEST, NLMSG_DONE,
    NLMSG_ERROR, RT_TABLE_MAIN, RTA_OIF, RTA_PREFSRC, RTM_GETROUTE, SOCK_RAW, bind, close, getpid,
    if_indextoname, ifinfomsg, nlmsghdr, recv, rtattr, send, sockaddr, sockaddr_nl, socket,
    socklen_t,
};
use std::{
    collections::HashMap, ffi::{CStr, c_char}, mem, net::{IpAddr, Ipv4Addr, Ipv6Addr}, os::{fd::RawFd, raw::c_void}
};

use crate::models::{DiscoverError, NetworkInterface, RtMsg};
use anyhow::{Context, Result};

pub fn get_route_table() -> Result<HashMap<String, NetworkInterface>> {
    let sockfd_nl: RawFd = open_nl_socket().context("failed to open route table socket")?;
    bind_nl_socket(sockfd_nl).context("failed to bind route table socket")?;
    let (rtmsg, nlh) = build_rtm_getroute();
    send_rtmsg(sockfd_nl, rtmsg, nlh).context("failed to send message rtmessage")?;
    let iface = recv_rtmsg(sockfd_nl).context("failed to receive response from netlink")?;
    unsafe {
        close(sockfd_nl);
    }
    Ok(iface)
}

fn parse_rtaattr_data(rtmsg: &RtMsg, rta: &rtattr, data: &[u8], iface: &mut NetworkInterface) {
    match rta.rta_type {
        RTA_OIF => {
            let ifindex = u32::from_ne_bytes(data[..4].try_into().unwrap());
            let mut name = [0 as c_char; IFNAMSIZ];
            unsafe {
                if_indextoname(ifindex, name.as_mut_ptr());
            }
            let name = unsafe { CStr::from_ptr(name.as_ptr()) };

            iface.set_name(&name.to_string_lossy());
        }
        RTA_PREFSRC => match rtmsg.rtm_family as i32 {
            AF_INET => {
                let addr = Ipv4Addr::new(data[0], data[1], data[2], data[3]);
                iface.add_addr(IpAddr::V4(addr));
            }
            AF_INET6 => {
                let bytes: [u8; 16] = data[..16].try_into().unwrap();
                let addr = Ipv6Addr::from(bytes);
                iface.add_addr(IpAddr::V6(addr));
            }
            _ => {}
        },
        _ => {}
    }
}

fn recv_rtmsg(fd: RawFd) -> Result<HashMap<String, NetworkInterface>, DiscoverError> {
    let mut interfaces: HashMap<String, NetworkInterface> = HashMap::new();
    let mut buf = [0u8; 8192];
    loop {
        let received = unsafe { recv(fd, buf.as_mut_ptr() as *mut c_void, buf.len(), 0) };
        if received < 0 {
            unsafe {
                close(fd);
            }
            return Err(DiscoverError::SocketError {
                source: std::io::Error::last_os_error(),
            });
        }
        if received == 0 {
            break;
        }

        let mut offset = 0usize;

        while offset < received as usize {
            let hdr = unsafe { &*(buf[offset..].as_ptr() as *const nlmsghdr) };
            if hdr.nlmsg_type == NLMSG_DONE as u16 {
                return Ok(interfaces);
            }
            let msg_len = hdr.nlmsg_len as usize;
            if msg_len < mem::size_of::<nlmsghdr>() || offset + msg_len > buf.len() {
                break;
            }

            let msg_end = offset + msg_len;
            let attrs_offset = offset + mem::size_of::<nlmsghdr>();
            let mut data_offset = attrs_offset + mem::size_of::<RtMsg>();
            if data_offset > msg_end {
                break;
            }
            let rtmsg = unsafe { &*(buf[attrs_offset..].as_ptr() as *const RtMsg) };
            if rtmsg.rtm_dst_len == 0 {
                offset += (msg_len + 3) & !3;
                continue;
            }

            let mut ntw_if = NetworkInterface::new();
            while data_offset + mem::size_of::<rtattr>() <= msg_end {
                let rta = unsafe { &*(buf[data_offset..].as_ptr() as *const rtattr) };
                let attr_len = rta.rta_len as usize;
                if attr_len < mem::size_of::<rtattr>() || data_offset + attr_len > msg_end {
                    break;
                }
                let attr_data_start = data_offset + mem::size_of::<rtattr>();
                let attr_data_end = data_offset + attr_len;
                let attr_data = &buf[attr_data_start..attr_data_end];
                parse_rtaattr_data(rtmsg, rta, attr_data, &mut ntw_if);
                data_offset += (attr_len + 3) & !3;
            }
            if let Some(existing_if) = interfaces.get_mut(&ntw_if.name) {
                existing_if.add_addr(existing_if.addresses[0]);
            } else {
                interfaces.insert(ntw_if.name.clone(), ntw_if);
            }
            offset += (msg_len + 3) & !3;
        }
    }
    if interfaces.is_empty() {
        return Err(DiscoverError::NetworkInterfaceNotFound {
            iface: "default".to_string(),
        });
    }
    Ok(interfaces)
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
