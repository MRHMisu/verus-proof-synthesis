use vstd::prelude::*;
fn main() {}
verus!{
pub fn myfun(a: &mut Vec<i32>, b: &mut Vec<i32>, N: i32)
	requires
		N > 0,
		old(a).len() == N,
		old(b).len() == N,
	ensures
		forall |k:int| 0 <= k < N ==> b[k] == 2*k*k + 2*k + 1,
{
	let mut i: usize = 0;

	while (i < N as usize)
	{
		if (i == 0) {
			a.set(0, 4);
		} else {
			a.set(i, a[i-1] + 4);
		}
		i = i + 1;
	}

	i = 0;
	while (i < N as usize)
	{
		if (i == 0) {
			b.set(0, 1);
		} else {
			b.set(i, b[i-1] + a[i-1]);
		}
		i = i + 1;
	}
}
}