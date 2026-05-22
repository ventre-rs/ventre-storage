use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FileSystemType {
    Btrfs,
    CryptoLuks,
    Devpts,
    Devtmpfs,
    Exfat,
    Ext2,
    Ext3,
    Ext4,
    Iso9660,
    LinuxRaidMember,
    Lvm2Member,
    Nfs,
    Ntfs,
    Overlay,
    Proc,
    Swap,
    Sysfs,
    Tmpfs,
    Udf,
    Vfat,
    Xfs,
    Other(String),
}
