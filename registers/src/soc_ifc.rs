// Licensed under the Apache-2.0 license.
//
// generated by caliptra_registers_generator with rtl-caliptra repo at b8001f6cc4044a6ec48e7d3f8841b9c319e7cf58
//
#![allow(clippy::erasing_op)]
#![allow(clippy::identity_op)]
#[derive(Clone, Copy)]
pub struct RegisterBlock(*mut u32);
impl RegisterBlock {
    /// # Safety
    ///
    /// The caller is responsible for ensuring that ptr is valid for
    /// volatile reads and writes at any of the offsets in this register
    /// block.
    pub unsafe fn new(ptr: *mut u32) -> Self {
        Self(ptr)
    }
    pub fn soc_ifc_reg() -> Self {
        unsafe { Self::new(0x30030000 as *mut u32) }
    }
    /// Indicates fatal hardware error
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn hw_error_fatal(&self) -> ureg::RegRef<crate::soc_ifc::meta::HwErrorFatal> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0 / core::mem::size_of::<u32>())) }
    }
    /// Indicates non-fatal hardware error
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn hw_error_non_fatal(&self) -> ureg::RegRef<crate::soc_ifc::meta::HwErrorNonFatal> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(4 / core::mem::size_of::<u32>())) }
    }
    /// Indicates fatal firmware error
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn fw_error_fatal(&self) -> ureg::RegRef<crate::soc_ifc::meta::FwErrorFatal> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(8 / core::mem::size_of::<u32>())) }
    }
    /// Indicates non-fatal firmware error
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn fw_error_non_fatal(&self) -> ureg::RegRef<crate::soc_ifc::meta::FwErrorNonFatal> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0xc / core::mem::size_of::<u32>())) }
    }
    /// Encoded error value for hardware errors
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn hw_error_enc(&self) -> ureg::RegRef<crate::soc_ifc::meta::HwErrorEnc> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x10 / core::mem::size_of::<u32>())) }
    }
    /// Encoded error value for firmware errors
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn fw_error_enc(&self) -> ureg::RegRef<crate::soc_ifc::meta::FwErrorEnc> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x14 / core::mem::size_of::<u32>())) }
    }
    /// Reports the boot status
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn boot_status(&self) -> ureg::RegRef<crate::soc_ifc::meta::BootStatus> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x18 / core::mem::size_of::<u32>())) }
    }
    /// Reports the status of the firmware flows
    ///
    /// Read value: [`soc_ifc::regs::FlowStatusReadVal`]; Write value: [`soc_ifc::regs::FlowStatusWriteVal`]
    pub fn flow_status(&self) -> ureg::RegRef<crate::soc_ifc::meta::FlowStatus> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x1c / core::mem::size_of::<u32>())) }
    }
    /// Firmware writable register that will clear flops containing device secrets.
    /// Keyvault is flushed, along with UDS, FE, and OBF KEY.
    /// This is a failsafe mechanism for unexpected security state changes.
    ///
    /// Read value: [`soc_ifc::regs::ClearSecretsReadVal`]; Write value: [`soc_ifc::regs::ClearSecretsWriteVal`]
    pub fn clear_secrets(&self) -> ureg::RegRef<crate::soc_ifc::meta::ClearSecrets> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x20 / core::mem::size_of::<u32>())) }
    }
    /// Generic input wires connected to SoC interface
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn generic_input_wires(
        &self,
    ) -> ureg::Array<2, ureg::RegRef<crate::soc_ifc::meta::GenericInputWires>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x24 / core::mem::size_of::<u32>())) }
    }
    /// Generic output wires connected to SoC interface
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn generic_output_wires(
        &self,
    ) -> ureg::Array<2, ureg::RegRef<crate::soc_ifc::meta::GenericOutputWires>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x28 / core::mem::size_of::<u32>())) }
    }
    /// Storage for the requested TRNG Data
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn trng(&self) -> ureg::Array<12, ureg::RegRef<crate::soc_ifc::meta::Trng>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x2c / core::mem::size_of::<u32>())) }
    }
    /// Indicates that the requests TRNG Data is done and stored in the TRNG Data register
    ///
    /// Read value: [`soc_ifc::regs::TrngDoneReadVal`]; Write value: [`soc_ifc::regs::TrngDoneWriteVal`]
    pub fn trng_done(&self) -> ureg::RegRef<crate::soc_ifc::meta::TrngDone> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x38 / core::mem::size_of::<u32>())) }
    }
    /// Obfuscated UDS
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn uds_seed(&self) -> ureg::Array<12, ureg::RegRef<crate::soc_ifc::meta::UdsSeed>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x200 / core::mem::size_of::<u32>())) }
    }
    /// Obfuscated Field Entropy
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn field_entropy(
        &self,
    ) -> ureg::Array<32, ureg::RegRef<crate::soc_ifc::meta::FieldEntropy>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x20c / core::mem::size_of::<u32>())) }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn key_manifest_pk_hash_0(
        &self,
    ) -> ureg::Array<12, ureg::RegRef<crate::soc_ifc::meta::KeyManifestPkHash0>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x22c / core::mem::size_of::<u32>())) }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn key_manifest_pk_hash_1(
        &self,
    ) -> ureg::Array<12, ureg::RegRef<crate::soc_ifc::meta::KeyManifestPkHash1>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x238 / core::mem::size_of::<u32>())) }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn key_manifest_pk_hash_2(
        &self,
    ) -> ureg::Array<12, ureg::RegRef<crate::soc_ifc::meta::KeyManifestPkHash2>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x244 / core::mem::size_of::<u32>())) }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn key_manifest_pk_hash_3(
        &self,
    ) -> ureg::Array<12, ureg::RegRef<crate::soc_ifc::meta::KeyManifestPkHash3>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x250 / core::mem::size_of::<u32>())) }
    }
    /// Read value: [`soc_ifc::regs::KeyManifestPkHashMaskReadVal`]; Write value: [`soc_ifc::regs::KeyManifestPkHashMaskWriteVal`]
    pub fn key_manifest_pk_hash_mask(
        &self,
    ) -> ureg::RegRef<crate::soc_ifc::meta::KeyManifestPkHashMask> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x25c / core::mem::size_of::<u32>())) }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn key_manifest_svn(&self) -> ureg::RegRef<crate::soc_ifc::meta::KeyManifestSvn> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x260 / core::mem::size_of::<u32>())) }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn boot_loader_svn(&self) -> ureg::RegRef<crate::soc_ifc::meta::BootLoaderSvn> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x264 / core::mem::size_of::<u32>())) }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn runtime_svn(&self) -> ureg::Array<4, ureg::RegRef<crate::soc_ifc::meta::RuntimeSvn>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x268 / core::mem::size_of::<u32>())) }
    }
    /// Read value: [`soc_ifc::regs::AntiRollbackDisableReadVal`]; Write value: [`soc_ifc::regs::AntiRollbackDisableWriteVal`]
    pub fn anti_rollback_disable(&self) -> ureg::RegRef<crate::soc_ifc::meta::AntiRollbackDisable> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x26c / core::mem::size_of::<u32>())) }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn ieee_idevid_cert_chain(
        &self,
    ) -> ureg::Array<24, ureg::RegRef<crate::soc_ifc::meta::IeeeIdevidCertChain>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x270 / core::mem::size_of::<u32>())) }
    }
    /// Read value: [`soc_ifc::regs::FuseDoneReadVal`]; Write value: [`soc_ifc::regs::FuseDoneWriteVal`]
    pub fn fuse_done(&self) -> ureg::RegRef<crate::soc_ifc::meta::FuseDone> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x288 / core::mem::size_of::<u32>())) }
    }
    /// Stored De-Obfuscation key, not accessible by software
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn obf_key(&self) -> ureg::Array<8, ureg::RegRef<crate::soc_ifc::meta::ObfKey>> {
        unsafe { ureg::Array::new(self.0.wrapping_add(0x28c / core::mem::size_of::<u32>())) }
    }
    /// Lock feature gates writes to the ICCM. When lock is set to
    /// 1, writes are blocked. When cleared to 0, writes allowed.
    /// Write-once, meaning only a reset can clear it once set to 1.
    /// Writes available to Caliptra subsystem, not SoC.
    ///
    /// Read value: [`soc_ifc::regs::IccmLockReadVal`]; Write value: [`soc_ifc::regs::IccmLockWriteVal`]
    pub fn iccm_lock(&self) -> ureg::RegRef<crate::soc_ifc::meta::IccmLock> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x294 / core::mem::size_of::<u32>())) }
    }
    /// Control register to assert FW update reset. When cleared to
    /// 0, the uc core alone is reset. When set to 1, reset is deasserted.
    /// Writes available to Caliptra subsystem, not SoC.
    ///
    /// Read value: [`soc_ifc::regs::FwUpdateResetReadVal`]; Write value: [`soc_ifc::regs::FwUpdateResetWriteVal`]
    pub fn fw_update_reset(&self) -> ureg::RegRef<crate::soc_ifc::meta::FwUpdateReset> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x298 / core::mem::size_of::<u32>())) }
    }
    /// Control register to set the number of cycles for which FW update reset must be asserted. Default is 5. Max value is 255 (8 bit)
    /// Writes available to Caliptra subsystem, not SoC.
    ///
    /// Read value: [`soc_ifc::regs::FwUpdateResetWaitCyclesReadVal`]; Write value: [`soc_ifc::regs::FwUpdateResetWaitCyclesWriteVal`]
    pub fn fw_update_reset_wait_cycles(
        &self,
    ) -> ureg::RegRef<crate::soc_ifc::meta::FwUpdateResetWaitCycles> {
        unsafe { ureg::RegRef::new(self.0.wrapping_add(0x29c / core::mem::size_of::<u32>())) }
    }
}
pub mod regs {
    //! Types that represent the values held by registers.
    #[derive(Clone, Copy)]
    pub struct ClearSecretsWriteVal(u32);
    impl ClearSecretsWriteVal {
        ///
        #[inline(always)]
        pub fn clear_secrets(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
    }
    impl From<u32> for ClearSecretsWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<ClearSecretsWriteVal> for u32 {
        fn from(val: ClearSecretsWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct FlowStatusReadVal(u32);
    impl FlowStatusReadVal {
        ///
        #[inline(always)]
        pub fn status(&self) -> u32 {
            (self.0 >> 0) & 0x3fffffff
        }
        ///
        #[inline(always)]
        pub fn ready_for_fw(&self) -> bool {
            ((self.0 >> 30) & 1) != 0
        }
        ///
        #[inline(always)]
        pub fn ready_for_runtime(&self) -> bool {
            ((self.0 >> 31) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> FlowStatusWriteVal {
            FlowStatusWriteVal(self.0)
        }
    }
    impl From<u32> for FlowStatusReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<FlowStatusReadVal> for u32 {
        fn from(val: FlowStatusReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct FlowStatusWriteVal(u32);
    impl FlowStatusWriteVal {
        ///
        #[inline(always)]
        pub fn status(self, val: u32) -> Self {
            Self((self.0 & !(0x3fffffff << 0)) | ((val & 0x3fffffff) << 0))
        }
        ///
        #[inline(always)]
        pub fn ready_for_fw(self, val: bool) -> Self {
            Self((self.0 & !(1 << 30)) | (u32::from(val) << 30))
        }
        ///
        #[inline(always)]
        pub fn ready_for_runtime(self, val: bool) -> Self {
            Self((self.0 & !(1 << 31)) | (u32::from(val) << 31))
        }
    }
    impl From<u32> for FlowStatusWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<FlowStatusWriteVal> for u32 {
        fn from(val: FlowStatusWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct TrngDoneReadVal(u32);
    impl TrngDoneReadVal {
        ///
        #[inline(always)]
        pub fn done(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> TrngDoneWriteVal {
            TrngDoneWriteVal(self.0)
        }
    }
    impl From<u32> for TrngDoneReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<TrngDoneReadVal> for u32 {
        fn from(val: TrngDoneReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct TrngDoneWriteVal(u32);
    impl TrngDoneWriteVal {
        ///
        #[inline(always)]
        pub fn done(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
    }
    impl From<u32> for TrngDoneWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<TrngDoneWriteVal> for u32 {
        fn from(val: TrngDoneWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct AntiRollbackDisableReadVal(u32);
    impl AntiRollbackDisableReadVal {
        ///
        #[inline(always)]
        pub fn dis(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> AntiRollbackDisableWriteVal {
            AntiRollbackDisableWriteVal(self.0)
        }
    }
    impl From<u32> for AntiRollbackDisableReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<AntiRollbackDisableReadVal> for u32 {
        fn from(val: AntiRollbackDisableReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct AntiRollbackDisableWriteVal(u32);
    impl AntiRollbackDisableWriteVal {
        ///
        #[inline(always)]
        pub fn dis(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
    }
    impl From<u32> for AntiRollbackDisableWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<AntiRollbackDisableWriteVal> for u32 {
        fn from(val: AntiRollbackDisableWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct FuseDoneReadVal(u32);
    impl FuseDoneReadVal {
        ///
        #[inline(always)]
        pub fn done(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> FuseDoneWriteVal {
            FuseDoneWriteVal(self.0)
        }
    }
    impl From<u32> for FuseDoneReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<FuseDoneReadVal> for u32 {
        fn from(val: FuseDoneReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct FuseDoneWriteVal(u32);
    impl FuseDoneWriteVal {
        ///
        #[inline(always)]
        pub fn done(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
    }
    impl From<u32> for FuseDoneWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<FuseDoneWriteVal> for u32 {
        fn from(val: FuseDoneWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct FwUpdateResetReadVal(u32);
    impl FwUpdateResetReadVal {
        /// FW Update reset to reset core
        #[inline(always)]
        pub fn core_rst(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> FwUpdateResetWriteVal {
            FwUpdateResetWriteVal(self.0)
        }
    }
    impl From<u32> for FwUpdateResetReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<FwUpdateResetReadVal> for u32 {
        fn from(val: FwUpdateResetReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct FwUpdateResetWriteVal(u32);
    impl FwUpdateResetWriteVal {
        /// FW Update reset to reset core
        #[inline(always)]
        pub fn core_rst(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
    }
    impl From<u32> for FwUpdateResetWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<FwUpdateResetWriteVal> for u32 {
        fn from(val: FwUpdateResetWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct FwUpdateResetWaitCyclesReadVal(u32);
    impl FwUpdateResetWaitCyclesReadVal {
        /// FW Update reset wait cycles
        #[inline(always)]
        pub fn wait_cycles(&self) -> u32 {
            (self.0 >> 0) & 0xff
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> FwUpdateResetWaitCyclesWriteVal {
            FwUpdateResetWaitCyclesWriteVal(self.0)
        }
    }
    impl From<u32> for FwUpdateResetWaitCyclesReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<FwUpdateResetWaitCyclesReadVal> for u32 {
        fn from(val: FwUpdateResetWaitCyclesReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct FwUpdateResetWaitCyclesWriteVal(u32);
    impl FwUpdateResetWaitCyclesWriteVal {
        /// FW Update reset wait cycles
        #[inline(always)]
        pub fn wait_cycles(self, val: u32) -> Self {
            Self((self.0 & !(0xff << 0)) | ((val & 0xff) << 0))
        }
    }
    impl From<u32> for FwUpdateResetWaitCyclesWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<FwUpdateResetWaitCyclesWriteVal> for u32 {
        fn from(val: FwUpdateResetWaitCyclesWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct IccmLockReadVal(u32);
    impl IccmLockReadVal {
        /// Lock bit gates writes to ICCM. Write 1 to set - cannot be cleared by SW.
        #[inline(always)]
        pub fn lock(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> IccmLockWriteVal {
            IccmLockWriteVal(self.0)
        }
    }
    impl From<u32> for IccmLockReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<IccmLockReadVal> for u32 {
        fn from(val: IccmLockReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct IccmLockWriteVal(u32);
    impl IccmLockWriteVal {
        /// Lock bit gates writes to ICCM. Write 1 to set - cannot be cleared by SW.
        #[inline(always)]
        pub fn lock(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
    }
    impl From<u32> for IccmLockWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<IccmLockWriteVal> for u32 {
        fn from(val: IccmLockWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct KeyManifestPkHashMaskReadVal(u32);
    impl KeyManifestPkHashMaskReadVal {
        ///
        #[inline(always)]
        pub fn mask(&self) -> u32 {
            (self.0 >> 0) & 0xf
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> KeyManifestPkHashMaskWriteVal {
            KeyManifestPkHashMaskWriteVal(self.0)
        }
    }
    impl From<u32> for KeyManifestPkHashMaskReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<KeyManifestPkHashMaskReadVal> for u32 {
        fn from(val: KeyManifestPkHashMaskReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct KeyManifestPkHashMaskWriteVal(u32);
    impl KeyManifestPkHashMaskWriteVal {
        ///
        #[inline(always)]
        pub fn mask(self, val: u32) -> Self {
            Self((self.0 & !(0xf << 0)) | ((val & 0xf) << 0))
        }
    }
    impl From<u32> for KeyManifestPkHashMaskWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<KeyManifestPkHashMaskWriteVal> for u32 {
        fn from(val: KeyManifestPkHashMaskWriteVal) -> u32 {
            val.0
        }
    }
}
pub mod enums {
    //! Enumerations used by some register fields.
    pub mod selector {}
}
pub mod meta {
    //! Additional metadata needed by ureg.
    #[derive(Clone, Copy)]
    pub struct HwErrorFatal();
    impl ureg::RegType for HwErrorFatal {
        type Raw = u32;
    }
    impl ureg::ReadableReg for HwErrorFatal {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for HwErrorFatal {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for HwErrorFatal {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct HwErrorNonFatal();
    impl ureg::RegType for HwErrorNonFatal {
        type Raw = u32;
    }
    impl ureg::ReadableReg for HwErrorNonFatal {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for HwErrorNonFatal {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for HwErrorNonFatal {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct FwErrorFatal();
    impl ureg::RegType for FwErrorFatal {
        type Raw = u32;
    }
    impl ureg::ReadableReg for FwErrorFatal {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for FwErrorFatal {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for FwErrorFatal {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct FwErrorNonFatal();
    impl ureg::RegType for FwErrorNonFatal {
        type Raw = u32;
    }
    impl ureg::ReadableReg for FwErrorNonFatal {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for FwErrorNonFatal {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for FwErrorNonFatal {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct HwErrorEnc();
    impl ureg::RegType for HwErrorEnc {
        type Raw = u32;
    }
    impl ureg::ReadableReg for HwErrorEnc {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for HwErrorEnc {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for HwErrorEnc {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct FwErrorEnc();
    impl ureg::RegType for FwErrorEnc {
        type Raw = u32;
    }
    impl ureg::ReadableReg for FwErrorEnc {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for FwErrorEnc {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for FwErrorEnc {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct BootStatus();
    impl ureg::RegType for BootStatus {
        type Raw = u32;
    }
    impl ureg::ReadableReg for BootStatus {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for BootStatus {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for BootStatus {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct FlowStatus();
    impl ureg::RegType for FlowStatus {
        type Raw = u32;
    }
    impl ureg::ReadableReg for FlowStatus {
        type ReadVal = crate::soc_ifc::regs::FlowStatusReadVal;
    }
    impl ureg::WritableReg for FlowStatus {
        type WriteVal = crate::soc_ifc::regs::FlowStatusWriteVal;
    }
    impl ureg::ResettableReg for FlowStatus {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct ClearSecrets();
    impl ureg::RegType for ClearSecrets {
        type Raw = u32;
    }
    impl ureg::WritableReg for ClearSecrets {
        type WriteVal = crate::soc_ifc::regs::ClearSecretsWriteVal;
    }
    impl ureg::ResettableReg for ClearSecrets {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct GenericInputWires();
    impl ureg::RegType for GenericInputWires {
        type Raw = u32;
    }
    impl ureg::ReadableReg for GenericInputWires {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for GenericInputWires {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for GenericInputWires {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct GenericOutputWires();
    impl ureg::RegType for GenericOutputWires {
        type Raw = u32;
    }
    impl ureg::ReadableReg for GenericOutputWires {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for GenericOutputWires {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for GenericOutputWires {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct Trng();
    impl ureg::RegType for Trng {
        type Raw = u32;
    }
    impl ureg::ReadableReg for Trng {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for Trng {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for Trng {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct TrngDone();
    impl ureg::RegType for TrngDone {
        type Raw = u32;
    }
    impl ureg::ReadableReg for TrngDone {
        type ReadVal = crate::soc_ifc::regs::TrngDoneReadVal;
    }
    impl ureg::WritableReg for TrngDone {
        type WriteVal = crate::soc_ifc::regs::TrngDoneWriteVal;
    }
    impl ureg::ResettableReg for TrngDone {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct UdsSeed();
    impl ureg::RegType for UdsSeed {
        type Raw = u32;
    }
    impl ureg::WritableReg for UdsSeed {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for UdsSeed {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct FieldEntropy();
    impl ureg::RegType for FieldEntropy {
        type Raw = u32;
    }
    impl ureg::WritableReg for FieldEntropy {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for FieldEntropy {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct KeyManifestPkHash0();
    impl ureg::RegType for KeyManifestPkHash0 {
        type Raw = u32;
    }
    impl ureg::ReadableReg for KeyManifestPkHash0 {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for KeyManifestPkHash0 {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for KeyManifestPkHash0 {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct KeyManifestPkHash1();
    impl ureg::RegType for KeyManifestPkHash1 {
        type Raw = u32;
    }
    impl ureg::ReadableReg for KeyManifestPkHash1 {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for KeyManifestPkHash1 {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for KeyManifestPkHash1 {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct KeyManifestPkHash2();
    impl ureg::RegType for KeyManifestPkHash2 {
        type Raw = u32;
    }
    impl ureg::ReadableReg for KeyManifestPkHash2 {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for KeyManifestPkHash2 {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for KeyManifestPkHash2 {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct KeyManifestPkHash3();
    impl ureg::RegType for KeyManifestPkHash3 {
        type Raw = u32;
    }
    impl ureg::ReadableReg for KeyManifestPkHash3 {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for KeyManifestPkHash3 {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for KeyManifestPkHash3 {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct KeyManifestPkHashMask();
    impl ureg::RegType for KeyManifestPkHashMask {
        type Raw = u32;
    }
    impl ureg::ReadableReg for KeyManifestPkHashMask {
        type ReadVal = crate::soc_ifc::regs::KeyManifestPkHashMaskReadVal;
    }
    impl ureg::WritableReg for KeyManifestPkHashMask {
        type WriteVal = crate::soc_ifc::regs::KeyManifestPkHashMaskWriteVal;
    }
    impl ureg::ResettableReg for KeyManifestPkHashMask {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct KeyManifestSvn();
    impl ureg::RegType for KeyManifestSvn {
        type Raw = u32;
    }
    impl ureg::ReadableReg for KeyManifestSvn {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for KeyManifestSvn {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for KeyManifestSvn {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct BootLoaderSvn();
    impl ureg::RegType for BootLoaderSvn {
        type Raw = u32;
    }
    impl ureg::ReadableReg for BootLoaderSvn {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for BootLoaderSvn {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for BootLoaderSvn {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct RuntimeSvn();
    impl ureg::RegType for RuntimeSvn {
        type Raw = u32;
    }
    impl ureg::ReadableReg for RuntimeSvn {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for RuntimeSvn {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for RuntimeSvn {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct AntiRollbackDisable();
    impl ureg::RegType for AntiRollbackDisable {
        type Raw = u32;
    }
    impl ureg::ReadableReg for AntiRollbackDisable {
        type ReadVal = crate::soc_ifc::regs::AntiRollbackDisableReadVal;
    }
    impl ureg::WritableReg for AntiRollbackDisable {
        type WriteVal = crate::soc_ifc::regs::AntiRollbackDisableWriteVal;
    }
    impl ureg::ResettableReg for AntiRollbackDisable {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct IeeeIdevidCertChain();
    impl ureg::RegType for IeeeIdevidCertChain {
        type Raw = u32;
    }
    impl ureg::ReadableReg for IeeeIdevidCertChain {
        type ReadVal = u32;
    }
    impl ureg::WritableReg for IeeeIdevidCertChain {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for IeeeIdevidCertChain {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct FuseDone();
    impl ureg::RegType for FuseDone {
        type Raw = u32;
    }
    impl ureg::ReadableReg for FuseDone {
        type ReadVal = crate::soc_ifc::regs::FuseDoneReadVal;
    }
    impl ureg::WritableReg for FuseDone {
        type WriteVal = crate::soc_ifc::regs::FuseDoneWriteVal;
    }
    impl ureg::ResettableReg for FuseDone {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct ObfKey();
    impl ureg::RegType for ObfKey {
        type Raw = u32;
    }
    impl ureg::WritableReg for ObfKey {
        type WriteVal = u32;
    }
    impl ureg::ResettableReg for ObfKey {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct IccmLock();
    impl ureg::RegType for IccmLock {
        type Raw = u32;
    }
    impl ureg::ReadableReg for IccmLock {
        type ReadVal = crate::soc_ifc::regs::IccmLockReadVal;
    }
    impl ureg::WritableReg for IccmLock {
        type WriteVal = crate::soc_ifc::regs::IccmLockWriteVal;
    }
    impl ureg::ResettableReg for IccmLock {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct FwUpdateReset();
    impl ureg::RegType for FwUpdateReset {
        type Raw = u32;
    }
    impl ureg::ReadableReg for FwUpdateReset {
        type ReadVal = crate::soc_ifc::regs::FwUpdateResetReadVal;
    }
    impl ureg::WritableReg for FwUpdateReset {
        type WriteVal = crate::soc_ifc::regs::FwUpdateResetWriteVal;
    }
    impl ureg::ResettableReg for FwUpdateReset {
        const RESET_VAL: Self::Raw = 0;
    }
    #[derive(Clone, Copy)]
    pub struct FwUpdateResetWaitCycles();
    impl ureg::RegType for FwUpdateResetWaitCycles {
        type Raw = u32;
    }
    impl ureg::ReadableReg for FwUpdateResetWaitCycles {
        type ReadVal = crate::soc_ifc::regs::FwUpdateResetWaitCyclesReadVal;
    }
    impl ureg::WritableReg for FwUpdateResetWaitCycles {
        type WriteVal = crate::soc_ifc::regs::FwUpdateResetWaitCyclesWriteVal;
    }
    impl ureg::ResettableReg for FwUpdateResetWaitCycles {
        const RESET_VAL: Self::Raw = 0;
    }
}
