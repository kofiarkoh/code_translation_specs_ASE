// AOJ 1373: Placing Medals on a Binary Tree
// 2018.1.7 bal4u@uu

#include <stdio.h>

#define N 500000
int bit[500003];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

void add(int i, int v)
{
    while (i <= N) bit[i] += v, i |= i+1;
}
 
int sum(int i)
{
    int s = 0;
    while (i) s += bit[i-1], i &= i-1;
    return s;
}

#define sub(a,b) (sum(a)-sum(b))

int main()
{
	int n, i, x, k, ans;

	k = 0;
	n = in(); while (n--) {
		x = in();
		ans = 1;
		if (sub(1, 0) == 1) ans = 0;
		else if (x >= N) k = 1;
		else if (sub(x+1, 0) < x) {
			for (i = x; sub(i+1, i); i--) add(i, -1); add(i, 1);
		} else if (sub(x+1, 0) == x) {
			if (k || sub(N, x+1)) ans = 0;
			else { for (i = 1; i <= x; i++) add(i, -1); add(0, 1); }
		}
		puts(ans? "Yes": "No");
	}
	return 0;
}

