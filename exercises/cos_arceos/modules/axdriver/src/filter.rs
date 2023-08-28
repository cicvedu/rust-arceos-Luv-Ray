//! NetFilter

use crate::NetDriverOps;
use driver_common::{BaseDriverOps, DevResult, DeviceType};
use driver_net::{EthernetAddress, NetBufPtr};

pub struct NetFilter<T> {
    pub inner: T,
}

impl<T: BaseDriverOps> BaseDriverOps for NetFilter<T> {
    fn device_type(&self) -> DeviceType { self.inner.device_type() }
    fn device_name(&self) -> &str { self.inner.device_name() }
}

impl<T: driver_net::NetDriverOps> NetDriverOps for NetFilter<T> {
    fn mac_address(&self) -> EthernetAddress { self.inner.mac_address() }
    fn can_transmit(&self) -> bool { self.inner.can_transmit() }
    fn can_receive(&self) -> bool { self.inner.can_receive() }
    fn rx_queue_size(&self) -> usize { self.inner.rx_queue_size() }
    fn tx_queue_size(&self) -> usize { self.inner.tx_queue_size() }
    fn recycle_rx_buffer(&mut self, rx_buf: NetBufPtr) -> DevResult { self.inner.recycle_rx_buffer(rx_buf) }
    fn recycle_tx_buffers(&mut self) -> DevResult { self.inner.recycle_tx_buffers() }
    fn transmit(&mut self, tx_buf: NetBufPtr) -> DevResult { self.inner.transmit(tx_buf) }
    fn receive(&mut self) -> DevResult<NetBufPtr> { self.inner.receive() }
    fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<NetBufPtr> { self.inner.alloc_tx_buffer(size) }
}