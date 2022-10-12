#[cfg(target_arch = "x86")]
use super::i686;

#[cfg(target_arch = "x86")]
#[inline(always)]
pub fn init()
{
	i686::init();
}
